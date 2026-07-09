# 链表实现 · 第五章

> 译自 [too-many-lists](https://rust-unofficial.github.io/too-many-lists/fifth.html)。本章从第四章 `Rc<RefCell<Node>>` 的沉重枷锁中解脱出来，退回到单链表，用**裸指针**和**Unsafe Rust**实现一个 O(1) 的队列。这是整个教程中第一次真正踏入 unsafe  territory。

---

## 目标：为什么需要 unsafe

第四章用 `Rc<RefCell<Node>>` 实现了双向 deque，虽然安全，但代价惨重：

- 每个节点额外携带两个引用计数器和一个借用标志
- 运行时检查 `borrow_mut()` 是否 panic
- 代码冗长，难以阅读和维护

> **译注**：`Rc` 和 `RefCell` 适合处理简单场景，但一旦你想隐藏这些运行时开销，它们就会变得笨重。Rust 的设计哲学是：如果安全抽象无法表达某个高效的数据结构，那就用 unsafe 写出一个安全的外壳。

队列（Queue）与栈（Stack）的区别只有一个：栈在同端 push/pop，队列在一端 push、另一端 pop。对于单链表，这意味着我们要么把 push 移到尾部，要么把 pop 移到尾部——两者都需要遍历整个链表，变成 O(n)。

为了做到 O(1)，我们需要**缓存尾指针**，直接跳转到链表末尾。

---

## 布局设计：双指针单链表

```rust
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,  // 新增：裸指针指向尾节点
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
```

注意 `tail` 的类型：`*mut Node<T>`，这是一个**可变裸指针**。与 `Box` 或 `&mut` 不同，裸指针：

- 没有生命周期检查
- 可以为 null（`null_mut()`）
- 不拥有指向的数据（不会自动 drop）
- 解引用时必须包裹在 `unsafe` 块中

> **译注**：裸指针本质上就是 C 指针。Rust 刻意把解引用标记为 unsafe，因为 null、dangling、aliasing  violations 都可能导致**未定义行为（Undefined Behavior）**。但"创建裸指针"、"比较裸指针"、"赋值裸指针"这些操作本身是安全的——你只是在做整数运算。

---

## 第一次尝试：用引用做尾指针

在引入裸指针之前，作者尝试了一个更"安全"的方案：用 `Option<&mut Node<T>>` 做尾指针。

```rust
pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}
```

编译器立刻报错：

```
error[E0495]: cannot infer an appropriate lifetime for autoref
```

问题出在 `push` 方法里。我们需要从 `self.head` 或 `old_tail.next` 中借出一个 `&mut Node<T>`，然后存到 `self.tail` 中。但这个引用的生命周期必须与 `List<'a, T>` 的 `'a` 一样长，而 `self` 在方法结束时就失效了。

如果强行把 `push` 改成 `pub fn push(&'a mut self, elem: T)`，编译器倒是通过了，但使用体验极差：

```rust
let mut list = List::new();
list.push(1);  // 借用了 list 的 'a 生命周期
list.push(2);  // 错误！list 仍然被第一个 push 借用着
```

这就是**自引用结构**的经典困境：结构体内部的一个字段引用了另一个字段。Rust 的生命周期系统无法描述"指向自己内部"的引用。详细分析见 [自引用结构](../../ownership-lifetimes/self-referential.md)。

---

## 转向裸指针

既然引用无法描述自引用，那就用裸指针绕过编译器的生命周期检查，同时用 unsafe 块明确标记危险区域。

### 初始化

```rust
use std::ptr;

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),  // 空链表，尾指针为 null
        }
    }
}
```

`ptr::null_mut()` 返回一个 `*mut T` 的空指针，等价于 C 的 `NULL`。

### push：在尾部插入

```rust
pub fn push(&mut self, elem: T) {
    let mut new_tail = Box::new(Node {
        elem,
        next: None,
    });

    // 从 Box 内部获取裸指针
    let raw_tail: *mut _ = &mut *new_tail;

    if !self.tail.is_null() {
        unsafe {
            (*self.tail).next = Some(new_tail);
        }
    } else {
        self.head = Some(new_tail);
    }

    self.tail = raw_tail;
}
```

关键步骤：

1. `Box::new(Node { ... })` 在堆上分配新节点
2. `&mut *new_tail` 从 `Box` 中借出可变引用，然后**强制转换**为 `*mut Node<T>`
3. `Box` 本身被挂到链表上（`old_tail.next = Some(new_tail)` 或 `self.head = Some(new_tail)`），所以堆内存仍然被拥有
4. `raw_tail` 只是额外保存了一个指向同一块内存的裸指针，用于下次 O(1) 插入

> **译注**：`(*self.tail).next` 需要 `unsafe` 块，因为编译器无法证明 `self.tail` 不是 null。但我们通过 `!self.tail.is_null()` 做了检查，所以这是 sound 的。这种"在 safe 代码中做检查，在 unsafe 块中执行操作"的模式是 Rust unsafe 代码的典型风格。

### pop：从头部弹出

```rust
pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|head| {
        let head = *head;  // 解包 Box，拿到 Node
        self.head = head.next;

        if self.head.is_none() {
            self.tail = ptr::null_mut();
        }

        head.elem
    })
}
```

这里 `pop` 本身不需要 unsafe，因为操作的是 `head`（`Option<Box<Node<T>>>`），与第二章的栈完全相同。唯一需要注意的是：**当链表被弹空时，必须把 `tail` 也置为 null**。如果忘记这一步，下次 `push` 会往一个已经释放的地址写入，造成 use-after-free。

这就是 **unsafe taint（unsafe 污染）**：一旦模块里用了 unsafe，整个模块的代码都必须正确维护不变量，否则 safe 代码也会崩溃。

---

## Miri 与 Stacked Borrows

> **译注**：本节涉及 Rust 内存模型的深层细节。如果你只想了解实现，可以跳过；如果你想写出真正正确的 unsafe 代码，这是必修课。

上述代码在常规测试下工作正常，但用 **Miri**（Rust 的内存安全解释器）运行时会报错。问题出在这一行：

```rust
let raw_tail: *mut _ = &mut *new_tail;
```

这里我们先创建了一个 `&mut Node<T>`（可变引用），然后把它转成裸指针。在 Rust 的 **Stacked Borrows** 模型中，可变引用具有唯一性：从它创建裸指针后，如果继续使用原始引用或 `Box`，就可能违反 aliasing 规则。

正确的写法是避免通过引用中转，直接用 `Box::into_raw`：

```rust
pub fn push(&mut self, elem: T) {
    let new_tail = Box::new(Node {
        elem,
        next: ptr::null_mut(),
    });

    let raw_tail = Box::into_raw(new_tail);  // Box -> *mut，不 drop

    if !self.tail.is_null() {
        unsafe {
            (*self.tail).next = raw_tail;
        }
    } else {
        self.head = raw_tail;
    }

    self.tail = raw_tail;
}
```

等等，这样 `head` 和 `tail` 都变成了裸指针？那 `pop` 怎么把 `Box` 拿回来？

答案是：`pop` 需要用 `Box::from_raw` 把裸指针重新包装成 `Box`，这样 Rust 才能正确管理内存：

```rust
pub fn pop(&mut self) -> Option<T> {
    unsafe {
        if self.head.is_null() {
            None
        } else {
            let head = Box::from_raw(self.head);  // *mut -> Box
            self.head = head.next;
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            Some(head.elem)
        }
    }
}
```

> **译注**：`Box::into_raw` 和 `Box::from_raw` 是一对逆操作。`into_raw` 交出所有权但不释放内存；`from_raw` 重新获得所有权，当 `Box` 离开作用域时会正常 drop。这种"所有权在 Box 和裸指针之间转移"的技巧是 unsafe 链表的核心。

---

## 最终布局：纯裸指针链表

经过 Miri 的洗礼，我们最终放弃 `Option<Box<Node<T>>>`，全部改用裸指针：

```rust
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
```

现在 `head` 和 `tail` 都是 `*mut Node<T>`，空链表时用 `ptr::null_mut()` 表示。整个结构更加统一，也更容易推理。

### 完整 push/pop

```rust
impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }
                Some(head.elem)
            }
        }
    }
}
```

注意 `push` 里的 `unsafe` 块包裹了全部逻辑，因为 `Box::into_raw` 和裸指针解引用都是 unsafe 操作。`pop` 同理。

---

## peek 与 peek_mut

```rust
pub fn peek(&self) -> Option<&T> {
    unsafe {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub fn peek_mut(&mut self) -> Option<&mut T> {
    unsafe {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}
```

`as_ref()` 和 `as_mut()` 是裸指针上的安全方法：如果指针非 null，返回 `Some(&Node)` 或 `Some(&mut Node)`；如果为 null，返回 `None`。它们本身不 unsafe，但因为调用它们的对象是裸指针，所以整个表达式需要在 unsafe 块中。

---

## 迭代器

### IntoIter（消耗所有权）

```rust
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
```

与第二章完全相同，复用 `pop` 即可。

### Iter（不可变借用）

```rust
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter { next: self.head.as_ref() }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}
```

### IterMut（可变借用）

```rust
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut { next: self.head.as_mut() }
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}
```

> **译注**：`IterMut` 用 `Option<&'a mut Node<T>>` 而不是 `*mut Node<T>`，因为迭代器需要返回 `&'a mut T`。这里的关键是 `self.next.take()`：它从 `Option` 中取出当前节点，然后我们从 `node.next` 中借出下一个节点的可变引用。由于裸指针没有生命周期，所有生命周期约束都落在 `as_mut()` 返回的引用上。详见 [生命周期进阶](../../ownership-lifetimes/lifetime-advanced.md) 中型变（variance）的讨论。

---

## Drop：必须显式遍历

```rust
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}
```

复用 `pop` 是最简洁的方案。每次 `pop` 用 `Box::from_raw` 把裸指针重新包装成 `Box`，然后 `Box` 离开作用域时自动 drop 节点。当链表为空时，`pop` 返回 `None`，循环结束。

> **译注**：如果你试图写 `while !self.head.is_null() { ... }` 并手动 `drop(Box::from_raw(self.head))`，请注意 `drop` 函数会消费 `Box`，而 `Box::from_raw` 创建的 `Box` 在 `drop` 调用后就已经被释放了。直接用 `pop` 是最不容易出错的方式。

---

## 测试

```rust
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // 检查空链表后重新 push 是否正常
        list.push(6);
        list.push(7);
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
```

> **译注**：原教程还包含一个 `miri_food` 测试，专门用来在 Miri 下验证各种边界操作（push/pop/peek/iter_mut 的交错组合）。如果你打算修改这段代码，强烈建议安装 Miri 并运行 `cargo +nightly miri test`。

---

## 关键教训

| 主题 | 要点 |
|------|------|
| 裸指针 `*mut T` | 没有生命周期、不拥有数据、解引用需 unsafe |
| `Box::into_raw` / `from_raw` | 在 `Box` 和裸指针之间转移所有权，必须成对使用 |
| unsafe taint | 一个模块里的 unsafe 会影响整个模块，靠 privacy 隔离风险 |
| 自引用结构 | 引用无法描述"指向自己内部"，裸指针是唯一出路 |
| Miri | 常规测试通过不代表内存安全，Miri 能发现 Stacked Borrows 违规 |
| 空链表处理 | `pop` 空后必须将 `tail` 置 null，否则 push 会写入 dangling 指针 |

---

## 与第四章的对比

| 特性 | 第四章（Safe Deque） | 第五章（Unsafe Queue） |
|------|-------------------|----------------------|
| 指针类型 | `Rc<RefCell<Node>>` | `*mut Node<T>` |
| 运行时开销 | 引用计数 + 借用检查 | 无 |
| 代码复杂度 | 冗长，嵌套调用 | 简洁，但需要 unsafe |
| 正确性保障 | 编译器保证 | 程序员保证 + Miri 验证 |
| 迭代器 | 简单 | 需要 unsafe 块 |
| 适用场景 | 原型、教学 | 生产、性能敏感 |

> **译注**：本章虽然是"unsafe"实现，但对外暴露的 API 仍然是 safe 的。`List::push`、`List::pop`、`List::iter` 等方法都不需要调用方写 unsafe。这就是 Rust 的 unsafe 哲学：用 unsafe 构建安全的抽象，而不是让所有人都去写 unsafe。

---

## 下一章

第六章将构建一个**生产级的不安全双向链表（Production Unsafe Doubly-Linked Deque）**，引入 `NonNull`、panic safety、cursor API，以及 `Send`/`Sync` 的编译期验证。那是 too-many-lists 的终极挑战。

---

> 原文：[An Ok Unsafe Singly-Linked Queue](https://rust-unofficial.github.io/too-many-lists/fifth.html) by Gankra
