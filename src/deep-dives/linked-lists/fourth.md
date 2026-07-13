# 链表实现 · 第四章 · 不安全单链表队列

> 译自 [too-many-lists](https://rust-unofficial.github.io/too-many-lists/fifth.html) 的 "An Ok Unsafe Singly-Linked Queue"。本章从单链表栈出发，实现一个**带尾指针的单链表队列**，并首次引入裸指针、Unsafe Rust、`NonNull` 与 `PhantomData` 等概念。
>
> **译注**：too-many-lists 原书的[第四章](https://rust-unofficial.github.io/too-many-lists/fourth.html)是 `Rc<RefCell<Node>>` 安全双向 Deque，本系列跳过了该章（其核心概念已在第三章 `Rc` 和智能指针章节中覆盖），直接进入 unsafe 队列实现。有兴趣的读者建议对照阅读原书第四章。

## 为什么需要 unsafe

在第三章中，我们用 `Rc` 实现了持久化链表。如果我们继续用 `Rc<RefCell<Node>>` 来实现一个支持双向操作的链表，虽然安全，但代价惨重：

- 每个节点额外携带两个引用计数器和一个借用标志
- 运行时检查 `borrow_mut()` 是否 panic
- 代码冗长，难以阅读和维护

> **译注**：`Rc` 和 `RefCell` 适合处理简单场景，但一旦你想隐藏这些运行时开销，它们就会变得笨重。Rust 的设计哲学是：如果安全抽象无法表达某个高效的数据结构，那就用 unsafe 写出一个安全的外壳。原书第四章详细展示了 `Rc<RefCell>` 双向 Deque 的笨重实现，此处直接跳到 unsafe 方案。

---

## 目标：一个队列

第二章的栈实现把所有操作都放在链表的**头部**：`push` 在头部插入，`pop` 在头部弹出。队列（Queue）与栈的区别仅仅在于：**队列从另一头弹出**。因此，我们需要把 `push` 或 `pop` 中的一个移动到链表的尾部。

```text
input list:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)

stack push X:
[Some(ptr)] -> (X, Some(ptr)) -> (A, Some(ptr)) -> (B, None)

stack pop:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)
```

对于单链表，我们可以把 `push` 挪到尾部（尾插），也可以把 `pop` 挪到尾部（尾删），两种做法的“代码复杂度”看起来差不多。但一个朴素的尾删/尾插实现都需要**遍历整条链表**才能找到尾部或尾前节点，时间复杂度是 $O(n)$。

> **NARRATOR**：链表教程里说 "some would argue this is a queue"，但作者认为性能保证也是接口的一部分。这里我们选择更优雅的方案：把 `push` 放在尾部，同时用一根指针指向尾节点，让 `push` 和 `pop` 都是 $O(1)$。

---

## 第一次尝试：尾指针 + Box 节点

延续第二章的风格，我们先用 `Option<Box<Node>>` 做指针，再加一个 `tail` 字段指向尾节点：

```rust
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>, // NEW!
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem,
            next: None,
        });

        let old_tail = std::mem::replace(&mut self.tail, Some(new_tail));

        match old_tail {
            Some(mut old_tail) => {
                old_tail.next = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail);
            }
        }
    }
}
```

### 编译错误：不能同时持有两个 Box

```text
error[E0382]: use of moved value: `new_tail`
  --> src/fifth.rs:38:38
   |
38 |                 old_tail.next = Some(new_tail);
   |                                      ^^^^^^^^ value used here after move
```

`Box` 代表所有权，把它赋给 `self.tail` 后，所有权已经转移，不能再把它塞给 `old_tail.next`。更糟糕的是，如果这段代码能通过，我们会同时让 `List` 和旧尾节点的 `next` 都指向同一个 `Box`——当 `tail` 被重新赋值时，旧 `Box` 会被 drop，接着旧尾节点又会 drop 同一个 `Box`，导致**双重释放**。

> **关键教训**：`Box` 是**拥有型指针**（owning pointer），不能同时让两个地方都“拥有”同一个节点。链表需要一种**非拥有型指针**，让 `tail` 只是指向尾节点的地址，而不负责释放它。

---

## 第二次尝试：尾指针改用 `&mut Node`

非拥有型指针在 Safe Rust 里就是引用：

```rust
pub struct List<T> {
    head: Link<T>,
    tail: Option<&mut Node<T>>, // NEW!
}
```

编译器立刻报错：

```text
error[E0106]: missing lifetime specifier
 --> src/fifth.rs:3:18
  |
3 |     tail: Option<&mut Node<T>>, // NEW!
  |                  ^ expected lifetime parameter
```

引用必须标注生命周期。于是我们习惯性地给 `List` 加上 `'a`：

```rust
pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}
```

但 `push` 里把 `self.head.as_deref_mut()` 返回的 `&mut Node` 赋给 `self.tail` 时，编译器发现这个引用的生命周期来自 `self` 的匿名借用，而字段要求 `'a`。为了统一，我们把 `push` 写成 `pub fn push(&'a mut self, elem: T)`——让 `self` 的借用时间与 `'a` 一样长。

这能编译！然而测试立刻爆炸：

```text
error[E0499]: cannot borrow `list` as mutable more than once at a time
  --> src/fifth.rs:68:9
   |
65 |         assert_eq!(list.pop(), None);
   |                    ---- first mutable borrow occurs here
...
68 |         list.push(1);
   |        ^^^^ second mutable borrow occurs here
```

---

## 自引用结构：尾巴指向自己

我们遇到了一个 Safe Rust 无法直接表达的问题：**自引用结构（self-referential struct）**。

`List` 的 `tail` 字段指向的是 `List` 自己拥有的某个节点。当我们写 `pub fn push(&'a mut self)` 时，实际上是在说：

- `List` 内部保存了一个生命周期为 `'a` 的引用；
- 这个引用指向 `List` 自己体内的某个节点；
- 因此 `List` 必须被借用 `'a` 那么久；
- 但 `'a` 又是 `List` 自己的生命周期参数，于是 `List` 一旦调用一次 `push` 就被“钉死”了，再也不能再次可变借用。

这种“自己引用自己”的结构在 Safe Rust 中几乎无法正常工作。它正是 [`Pin`](https://doc.rust-lang.org/std/pin/index.html) 试图解决的问题——异步状态机和自引用 future 需要保证内部指针始终有效。但我们的队列并不想被钉在原地，我们需要更灵活的方案。

> **交叉链接**：关于 `Pin` 与自引用结构的更多细节，参见 Rust 官方 [`Pin` 文档](https://doc.rust-lang.org/std/pin/index.html) 与 [Nomicon 关于型变的章节](https://doc.rust-lang.org/nomicon/subtyping.html)。

---

## 正确方案：使用裸指针 `*mut Node`

Safe Rust 的引用无法描述“指向自身体内节点”的尾指针，因为借用检查器会把 `&mut Node` 的生命周期与 `self` 的借用绑定得太死。我们改用**裸指针（raw pointer）**：

```rust
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // DANGER DANGER
}

type Link<T> = Option<Box<Node<T>>>;
```

裸指针 `*mut Node<T>` 不携带生命周期，也不负责释放内存。它只表示“这是一个内存地址，指向一个 Node”。这样 `tail` 只是指向尾节点的位置，不影响所有权。

> **NARRATOR**：这版实现依然“危险地错了”，但还没到揭晓的时候。本章接下来的版本会把它修对。

### 为什么第一次的裸指针版仍不够

混合使用 `Box`（拥有型安全指针）和 `*mut`（非拥有型裸指针）依然危险。原因是：Safe Rust 的引用和 `Box` 给编译器提供了严格的别名规则（alias rules），而裸指针虽然不受借用检查器约束，但在 LLVM 层面依然受 **Stacked Borrows** 内存模型管辖。`Box` 拥有独占访问权（noalias），当 `Box` 还存活时，通过并存的 `*mut` 指针访问同一块内存会触发未定义行为（UB）。`Box::into_raw` 的关键作用正是**消费掉 Box、交出独占权**，此后这块内存仅由裸指针管理，直到 `Box::from_raw` 重新接管。这个「所有权交接」的边界是 Unsafe Rust 安全编程的核心。

> **Stacked Borrows 摘要**：Rust 的内存模型把对同一块内存的访问视为一个栈。`Box`/`&mut` 作为独占引用压入栈顶，此时任何通过裸指针的读写都相当于在栈底偷窥—— Miri 会标记为 UB。`into_raw` 把独占引用弹出栈，裸指针才能合法操作。`as_ref()`/`as_mut()` 返回的引用生命周期是无界的（unbounded），必须被调用者的借用上下文立即约束，否则同样可能破坏别名规则。运行 `MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo miri test` 可以自动检测这类问题。

> **核心原则**：在任意时刻，同一块内存要么由安全指针（`Box`/`&mut`）管理，要么由裸指针管理，不能同时存在。`Box::into_raw` 与 `Box::from_raw` 是两者之间的安全交接点。

---

## 把 `Link` 也改成裸指针

去掉 `Box` 后，`Link` 直接用 `*mut Node<T>`：

```rust
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

`Option<Box<Node>>` 被 `Option` 替换为更朴素的空指针：`ptr::null_mut()`。空链表就是 `head == tail == null`。

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

> **译注**：`Box::into_raw` 和 `Box::from_raw` 是一对逆操作。`into_raw` 交出所有权但不释放内存；`from_raw` 重新获得所有权，当 `Box` 离开作用域时会正常 drop。这种“所有权在 Box 和裸指针之间转移”的技巧是 unsafe 链表的核心。

---

## 用 `Box::into_raw` 分配节点

不再用 `Box` 作为链表的一部分，但我们仍希望用 Rust 的内存分配器。标准库提供了一对非常合适的工具：

- `Box::into_raw(b: Box<T>) -> *mut T`：消费 Box，返回一个裸指针。此后 Box 不再负责释放这块内存，调用者自行管理。
- `Box::from_raw(r: *mut T) -> Box<T>`：把裸指针重新包成 Box，让它在作用域结束时 drop。

```rust
use std::ptr;

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
}
```

### 安全注释（SAFETY）

这里首次出现 `unsafe` 块。代码虽然简单，但有几条不变式（invariants）必须人工维护：

1. `self.tail` 要么为空，要么指向一个**由本 List 拥有且仍然存活**的节点；
2. `new_tail` 是新分配的节点，不会有其他指针指向它；
3. `(*self.tail).next = new_tail` 解引用尾节点，前提是它确实非空且有效。

只要这些不变式成立，这个实现就是安全的。链表/图/树等数据结构是把 Unsafe Rust 用得“有章法”的典型场景。

---

## `pop` 与尾指针的重置

```rust
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
```

注意最后重置 `tail` 的逻辑。如果链表被弹空了，`head` 会变成空指针，此时 `tail` 也必须同步为空。否则 `tail` 会指向一个已经被释放的节点，形成**悬空指针（dangling pointer）**。

> **unsafe taint（unsafe 污染）**：一旦模块里用了 unsafe，整个模块的代码都必须正确维护不变量，否则 safe 代码也会崩溃。`pop` 本身不需要 unsafe（操作的是 head），但当链表被弹空时，必须把 `tail` 也置为 null——如果忘记这一步，下次 `push` 会往一个已经释放的地址写入，造成 use-after-free。

```rust
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}
```

通过反复 `pop` 来释放所有节点，简洁且正确。每个节点在 `Box::from_raw` 后由 Box 的析构函数释放。

---

## 迭代器：`peek`、 `iter`、 `iter_mut`

由于我们全用裸指针，迭代器也不得不与安全指针交互。这里采用保守策略：把裸指针**临时**转换为引用，并在返回前立即使用，不长期保存引用。

```rust
pub fn peek(&self) -> Option<&T> {
    // SAFETY: self.head 要么为空，要么指向由本 List 拥有且仍然存活的节点。
    // as_ref() 返回的无界生命周期被 &self 的借用上下文约束，不会产生别名冲突。
    unsafe {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub fn peek_mut(&mut self) -> Option<&mut T> {
    // SAFETY: self.head 要么为空，要么指向由本 List 拥有且仍然存活的节点。
    // as_mut() 返回的无界生命周期被 &mut self 的借用上下文约束，不会产生别名冲突。
    unsafe {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

pub fn iter(&self) -> Iter<'_, T> {
    // SAFETY: self.head 要么为空，要么指向由本 List 拥有且仍然存活的节点。
    // as_ref() 返回的无界生命周期被 &self 的借用上下文约束。
    unsafe {
        Iter { next: self.head.as_ref() }
    }
}

pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    // SAFETY: self.head 要么为空，要么指向由本 List 拥有且仍然存活的节点。
    // as_mut() 返回的无界生命周期被 &mut self 的借用上下文约束。
    unsafe {
        IterMut { next: self.head.as_mut() }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
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

`ptr::as_ref` 和 `ptr::as_mut` 会把 `*mut T` 转成 `Option<&T>` / `Option<&mut T>`。它们的签名是 `unsafe fn as_ref<'a>(self) -> Option<&'a T>`，返回的生命周期是**无界**的（unbounded），因此必须尽快被更短的上下文限制住。在这里，我们把它作为 `Iter` / `IterMut` 的构造参数，由调用者的借用生命周期来限定，这是安全的。

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
}
```

> **译注**：原教程还包含 `into_iter` / `iter` / `iter_mut` 的测试以及一个 `miri_food` 测试，专门用来在 Miri 下验证各种边界操作。如果你打算修改这段代码，强烈建议安装 Miri 并运行 `cargo +nightly miri test`。

---

## 进一步：`NonNull<T>` 的展望

`NonNull<T>` 是标准库对 `*mut T` 的一层薄包装，它的核心优势包括：

1. **非空保证**：`NonNull<T>` 永远不会是 null。`NonNull::new(ptr)` 返回 `Option<NonNull<T>>`，若 `ptr` 为 null 则返回 `None`；`NonNull::new_unchecked(ptr)` 跳过检查（unsafe），调用者必须保证非空。
2. **型变（Covariance）**：`NonNull<T>` 对 `T` 是协变的，而裸指针 `*mut T` 对 `T` 是不变的。这对泛型集合非常重要——它允许 `NonNull<Cat>` 在合适的地方当作 `NonNull<Animal>` 使用。
3. **空指针优化**：`Option<NonNull<T>>` 与 `NonNull<T>` 大小相同（都等于 `*mut T` 的大小），编译器用 null 值表示 `None`。

> **本章注**：`NonNull` 是**下一章**才正式引入并作为主线实现的数据结构工具；本章的完整实现仍采用裸指针 `*mut Node<T>`，以保证读者先理解 unsafe 与裸指针本身。本章提到的 `Option<NonNull<Node<T>>>` 仅作为理论展望：它可以用更类型化的方式表达 `Link`——空指针用 `None` 表示，非空指针保证指向有效内存，需要时再通过 `NonNull::as_ptr` 取回裸指针进行 `unsafe` 操作。例如：
>
> ```rust
> use std::ptr::NonNull;
>
> type Link<T> = Option<NonNull<Node<T>>>;
> ```
>
> 这一改造会改变本章主线的类型签名，因此留待下一章统一展开。

---

## `PhantomData`：型变与所有权的控制旋钮

`PhantomData<T>` 是一个**零大小类型（ZST）**，用于告诉编译器：

- 这个类型“表现得像”拥有/借用了一个 `T`；
- 影响型变（variance）；
- 影响 drop 检查；
- 影响 `Send` / `Sync` 等自动 trait 推导。

例如，标准库中自定义迭代器：

```rust
use std::marker::PhantomData;

pub struct Iter<'a, T> {
    ptr: *const Node<T>,
    end: *const Node<T>,
    _marker: PhantomData<&'a Node<T>>, // 让 Iter 对 'a 和 T 协变
}
```

> **交叉链接**：`PhantomData` 与型变、自引用结构的关系，在 Rust 官方 [Nomicon 关于型变的章节](https://doc.rust-lang.org/nomicon/subtyping.html) 与 [`Pin` 文档](https://doc.rust-lang.org/std/pin/index.html) 中有更深入的展开。它本质上是在类型系统里“伪造”一个字段，让编译器按我们的意图进行生命周期推理。

在本章的队列中，如果我们把 `Iter` 的 `next` 字段从 `Option<&'a Node<T>>` 改成裸指针 `*mut Node<T>`，就需要加 `PhantomData<&'a Node<T>>` 来约束 `'a` 并防止类型参数未使用的错误（`E0392: parameter 'a is never used`）。

> **本章注**：`Iter`/`IterMut` 的 `next` 字段最终使用了 `Option<&'a Node<T>>`（而非裸指针），因此**不需要** `PhantomData`。`PhantomData` 的真正战场在下一章的双端队列中。

---

## 完整实现（最终版）

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

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

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

    pub fn peek(&self) -> Option<&T> {
        // SAFETY: self.head 要么为空，要么指向由本 List 拥有且仍然存活的节点。
        // as_ref() 返回的无界生命周期被 &self 的借用上下文约束，不会产生别名冲突。
        unsafe {
            self.head.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // SAFETY: self.head 要么为空，要么指向由本 List 拥有且仍然存活的节点。
        // as_mut() 返回的无界生命周期被 &mut self 的借用上下文约束，不会产生别名冲突。
        unsafe {
            self.head.as_mut().map(|node| &mut node.elem)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        // SAFETY: self.head 要么为空，要么指向由本 List 拥有且仍然存活的节点。
        // as_ref() 返回的无界生命周期被 &self 的借用上下文约束。
        unsafe {
            Iter { next: self.head.as_ref() }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        // SAFETY: self.head 要么为空，要么指向由本 List 拥有且仍然存活的节点。
        // as_mut() 返回的无界生命周期被 &mut self 的借用上下文约束。
        unsafe {
            IterMut { next: self.head.as_mut() }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

---

## 关键教训

- **`Box` 不能同时被两个地方拥有**：双向指针需要非拥有型指针，否则会出现双重释放或借用错误。
- **Safe Rust 无法表达自引用**：`&mut self` 里的引用指向自身体内节点会导致“自引用结构”，触发无法再次借用的问题。裸指针跳过了生命周期检查，让数据结构可行。
- **不要混用安全指针与裸指针**：`Box` 和 `*mut` 混用会破坏别名规则，应统一使用裸指针。
- **`Box::into_raw` / `Box::from_raw`** 是安全分配与裸指针之间的桥梁，负责“把所有权交给裸指针”和“把所有权还给 Box 以释放”。
- **`NonNull<T>`** 提供非空保证和正确的型变，是 `*mut T` 的上位替代。
- **`PhantomData`** 是控制型变、所有权和自动 trait 的零成本工具。用裸指针实现泛型结构时通常不可或缺。
- **Miri 是 Unsafe 代码的盟友**：它能抓到很多人工审查会遗漏的别名/生命周期问题。

| 主题 | 要点 |
|------|------|
| 裸指针 `*mut T` | 没有生命周期、不拥有数据、解引用需 unsafe |
| `Box::into_raw` / `from_raw` | 在 `Box` 和裸指针之间转移所有权，必须成对使用 |
| unsafe taint | 一个模块里的 unsafe 会影响整个模块，靠 privacy 隔离风险 |
| 自引用结构 | 引用无法描述“指向自己内部”，裸指针是唯一出路 |
| Miri | 常规测试通过不代表内存安全，Miri 能发现 Stacked Borrows 违规 |
| 空链表处理 | `pop` 空后必须将 `tail` 置 null，否则 push 会写入 dangling 指针 |

---

## 下一章

第五章将构建一个**生产级的不安全双向链表（Production Unsafe Doubly-Linked Deque）**，引入 `NonNull`、panic safety、cursor API，以及 `Send`/`Sync` 的编译期验证。那是 too-many-lists 的终极挑战。

---

> 原文：[An Ok Unsafe Singly-Linked Queue](https://rust-unofficial.github.io/too-many-lists/fifth.html) by Gankra
