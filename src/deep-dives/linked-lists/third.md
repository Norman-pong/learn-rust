# 链表实现 · 第三章

> 译自 [too-many-lists](https://rust-unofficial.github.io/too-many-lists/third.html)。

---

## 从可变到持久

前两章我们已经掌握了**可变单链表**的写法。第二章用 `&mut self` 和 `Option::take()` 实现了实用的栈式链表，但有一个根本限制：每次操作都修改原列表，无法安全地共享尾部。

本章我们要写一个**持久化（persistent）不可变单链表**——这正是函数式程序员所熟悉的那种数据结构。你可以获取头部、获取尾部、把一个人的头部接到另一个人的尾部……仅此而已。不可变性是一种强大的约束。

在这个过程中，我们将深入理解 `Rc` 和 `Arc` 这两个智能指针。这也为下一章的"改变游戏规则"的链表打下基础。

---

## 为什么需要持久化链表

持久化数据结构的核心特征是：**修改操作返回新版本，旧版本仍然可用**。多个版本之间可以共享未被修改的部分。

考虑这样一个典型场景：

```
list1 = A -> B -> C -> D
list2 = tail(list1)     = B -> C -> D
list3 = prepend(list2, X) = X -> B -> C -> D
```

我们期望内存布局是这样的：

```
list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+
```

三个列表共享 `B -> C -> D` 这段尾部！这在函数式语言（如 Haskell、Lisp、OCaml）中是家常便饭——它们靠垃圾回收器（GC）来管理共享节点的生命周期。当没有任何列表引用 `B` 时，GC 才会回收它。

但 Rust 没有追踪式 GC。Rust 只有**引用计数**（Reference Counting）。

---

## 引用计数：Rc

`Rc<T>`（Reference Counted）是 Rust 标准库提供的单线程引用计数指针。它和 `Box<T>` 类似，都指向堆上的数据，但有一个关键区别：

- `Box<T>`：独占所有权，只有一个所有者
- `Rc<T>`：共享所有权，可以有多个 `Rc` 指向同一块数据，当最后一个 `Rc` 被 drop 时，内存才被释放

`Rc` 的 `clone()` 不会深拷贝数据，只是增加引用计数。这使得共享尾部变得极其廉价。

不过，灵活性是有代价的：`Rc` 只提供**不可变共享引用**（`&T`）。这意味着：

1. 不能修改 `Rc` 内部的数据
2. 不能从 `Rc` 中取出数据（除非你是最后一个引用者）

对于持久化链表来说，这恰好是我们需要的——不可变性正是持久化数据结构的前提。

---

## 结构设计

第二章的结构是：

```rust
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
```

我们能把 `Box` 直接换成 `Rc` 吗？

```rust
// 在 third.rs 中

type Link<T> = Option<Rc<Node<T>>>;
```

编译：

```
error[E0412]: cannot find type `Rc` in this scope
 --> src/third.rs:5:23
  |
5 | type Link<T> = Option<Rc<Node<T>>>;
  |                       ^^ not found in this scope
help: possible candidate is found in another module
  |
1 | use std::rc::Rc;
  |
```

和 `Box`、`Option`、`String` 这些"明星类型"不同，`Rc` 不是自动导入的，需要显式引入：

```rust
use std::rc::Rc;
```

再次编译，只有几个未使用字段的警告——结构本身没问题。但直接把 `Box` 换成 `Rc` 就能工作吗？

不，完全不行。`push` 和 `pop` 的概念在不可变链表上不再适用。我们需要全新的 API。

---

## API 设计：prepend / tail / head

持久化链表的操作语义和可变链表完全不同：

| 可变链表（第二章） | 持久化链表（本章） |
|------------------|------------------|
| `push(&mut self, elem)` → 修改原列表 | `prepend(&self, elem)` → 返回新列表 |
| `pop(&mut self)` → 修改原列表 | `tail(&self)` → 返回新列表（去掉头部） |
| `peek(&self)` → 看头部 | `head(&self)` → 看头部 |

### prepend：在头部添加元素

```rust
pub fn prepend(&self, elem: T) -> List<T> {
    List {
        head: Some(Rc::new(Node {
            elem,
            next: self.head.clone(),  // 克隆 Rc = 增加引用计数
        }))
    }
}
```

关键点：

- `self.head.clone()` 不是深拷贝整个链表！它只是把 `Option<Rc<Node<T>>>` 克隆一份
- `Option` 的 `Clone` 实现会克隆内部的值，而 `Rc` 的 `Clone` 只是增加引用计数
- 新节点指向旧的头部，旧列表完全不受影响

这体现了 Rust 所有权模型的精妙之处：在 [所有权模型](../../ownership-lifetimes/ownership.md) 中，我们学习了 `Move` 和 `Clone` 的区别。`Rc` 的 `Clone` 是一种特殊的浅拷贝——它创建一个新的共享所有权句柄，而不是复制堆数据。这与 [引用与借用](../../ownership-lifetimes/reference-borrow.md) 中讨论的不可变借用（`&T`）有相似之处：多个读者可以共存，但没有人可以修改数据。

### tail：获取去掉头部的列表

```rust
pub fn tail(&self) -> List<T> {
    List {
        head: self.head.as_ref().map(|node| node.next.clone())
    }
}
```

等等，编译报错：

```
error[E0308]: mismatched types
  --> src/third.rs:27:22
   |
27 |         List { head: self.head.as_ref().map(|node| node.next.clone()) }
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                      expected struct `std::rc::Rc`, found enum `std::option::Option`
   |
```

`map` 期望我们返回一个 `Y`，但 `node.next` 本身就是 `Option<Rc<Node<T>>>`，所以 `clone()` 返回的也是 `Option<...>`。`map` 会把结果再包一层 `Option`，变成 `Option<Option<Rc<...>>>`。

这是 `Option` 的另一个常见模式：用 `and_then` 替代 `map`，它允许我们返回 `Option` 而不会被二次包装：

```rust
pub fn tail(&self) -> List<T> {
    List {
        head: self.head.as_ref().and_then(|node| node.next.clone())
    }
}
```

### head：查看第一个元素

```rust
pub fn head(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem)
}
```

这种写法使用了 `as_ref().map(...)` 模式：先用 `as_ref()` 把 `Option<Rc<Node<T>>>` 转成 `Option<&Rc<Node<T>>>`（避免消费 `self.head`），再用 `map` 提取节点元素的引用 `&T`。这和第二章 `push`/`pop` 内部处理 `Option` 的模式类似——由于我们只有共享引用，只能返回 `&T`，不能返回所有权。

---

## 完整基础实现

```rust
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}
```

---

## 测试

```rust
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // 空列表的 tail 仍然是空列表
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
```

注意：用 `let list = list.prepend(1)` 重新绑定后，旧的 `list` 变量名被**遮蔽**（shadowed）了——但数据本身仍然存在。如果想让多个版本共存，给它们不同的名字即可：

```rust
let list1 = List::new().prepend(1).prepend(2);  // 2 -> 1
let list2 = list1.prepend(3);                   // 3 -> 2 -> 1
let list3 = list1.tail();                       // 1

// list1, list2, list3 同时存在，共享节点 1
```

---

## 验证引用计数

`Rc` 的引用计数变化是隐式的：`clone` 增加计数，drop 减少计数。为了验证这种共享机制，我们可以用 `Rc::strong_count` 直接观察某个节点的引用计数。

由于 `List` 把 `head` 和 `Node` 都封装为私有字段，外部无法直接拿到 `Rc<Node<T>>`。为了演示，我们可以临时在 `Node` 上实现一个调试方法，或者直接用 `Rc::strong_count` 来观察独立的 `Rc`：

```rust
use std::rc::Rc;

let shared = Rc::new(42);
assert_eq!(Rc::strong_count(&shared), 1);      // 只有 shared 一个引用

let cloned = shared.clone();
assert_eq!(Rc::strong_count(&shared), 2);      // shared 和 cloned 共同指向同一个值
assert_eq!(Rc::strong_count(&cloned), 2);       // 从任意一个 Rc 看计数都一样

// 回到链表的语境：
// let list1 = List::new().prepend(1).prepend(2);  // 2 -> 1
// 此时 Node(2) 的 strong_count = 1, Node(1) 的 strong_count = 1
// let list2 = list1.prepend(3);                   // 3 -> 2 -> 1
// 此时 Node(2) 和 Node(1) 的 strong_count 都变成 2，因为 list1 和 list2 共享尾部
```

在 `prepend` 中，`self.head.clone()` 并不会复制任何节点数据，只是把头部节点的 `Rc` 引用计数加一。多个列表版本可以共享同一个尾部，而 `Rc` 会在所有引用者都 drop 后自动释放节点。

---

## 迭代器

不可变链表的迭代器和第二章几乎一样：

```rust
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
```

注意：

- `as_deref()` 把 `Option<Rc<Node<T>>>` 转成 `Option<&Node<T>>`（`as_deref` 利用 `Rc<T>` 实现了 `Deref<Target = T>`，将 `Rc<Node<T>>` 自动解引用为 `&Node<T>`）
- 迭代器持有的是**引用**而非 `Rc`，所以迭代过程中不会影响引用计数
- 只能实现 `Iter`（不可变迭代），**不能**实现 `IterMut` 或 `IntoIter`——因为我们只有共享访问权

测试：

```rust
#[test]
fn iter() {
    let list = List::new().prepend(1).prepend(2).prepend(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
}
```

---

## Drop 优化：处理共享所有权

和可变链表一样，递归 Drop 可能导致栈溢出。但持久化链表的情况更微妙：

- 如果某个节点被多个列表共享，你不能随意释放它
- 如果某个节点**只被当前列表引用**，你可以安全地释放它，并继续检查下一个节点

`Rc` 提供了一个关键方法：`Rc::try_unwrap`。它尝试从 `Rc` 中取出内部值，**只有当引用计数为 1 时才会成功**：

```rust
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {          // node: Rc<Node<T>>
            if let Ok(mut node) = Rc::try_unwrap(node) {  // node: Node<T>（遮蔽）
                // 这里内层的 mut node 遮蔽了外层的 node，类型从 Rc<Node<T>> 变为 Node<T>
                // try_unwrap 在成功时消费了 Rc 并返回内部值，因此可以安全取出并继续
                head = node.next.take();
            } else {
                // 还有其他 Rc 指向这个节点，不能释放，停止遍历
                break;
            }
        }
    }
}
```

这个 Drop 实现优雅地处理了共享场景：

- 如果链表的所有节点都是独占的（比如从未被共享过），整个链会被迭代释放，不会递归
- 如果遇到共享节点，立即停止——那个节点会在最后一个引用者 drop 时被释放

这是引用计数相比追踪式 GC 的一个优势：确定性析构。你知道一个值何时被释放，不需要等待 GC 扫描。

---

## Rc 不是线程安全的

如果我们想让持久化链表跨线程共享，直接用 `Rc` 是不行的。`Rc` 的引用计数更新不是原子的——两个线程同时 `clone` 同一个 `Rc` 可能导致计数更新丢失，进而造成 use-after-free。

Rust 的解决方案是 `Arc<T>`（Atomic Reference Counted）。`Arc` 和 `Rc` 的 API 完全相同，只是引用计数使用原子操作。把 `use std::rc::Rc` 换成 `use std::sync::Arc`，列表就线程安全了。

但 Rust 不会让你在编译期"意外"使用非线程安全的类型跨线程。这是通过两个标记 trait 实现的：

- **`Send`**：可以安全地移动到另一个线程
- **`Sync`**：可以安全地在多个线程间共享（即 `&T` 是 `Send`）

`Rc` 不是 `Sync` 的，因为它的引用计数使用普通 `Cell`（非原子操作）。`Arc` 是 `Sync` 的，因为它使用原子操作。Rust 的线程安全不是文档约定，而是**类型系统强制**的——如果你试图把 `Rc` 发送到另一个线程，编译器会直接报错。

这背后的原因与**内部可变性**（interior mutability）有关。到目前为止我们接触的都是**继承可变性**（inherited mutability）：一个值的可变性取决于它的容器是否可变。`Rc` 和 `Arc` 在内部使用 `Cell` 或原子类型来修改引用计数，即使外部只持有共享引用 `&Rc<T>`。这种"通过共享引用修改内部状态"的能力就是内部可变性。

关于 `Send`/`Sync` 的自动推导：如果类型的所有字段都是 `Send`/`Sync`，那么该类型自动实现 `Send`/`Sync`。这和 `Copy` 的推导规则类似。但内部可变性类型是特殊的——它们打破了默认规则，需要显式声明线程安全属性。

---

## 为什么 Rc 不能做"唯一性检查"

在函数式语言中，持久化数据结构通常有一个优化：如果某个节点在修改前是"唯一的"（只有一个引用），可以直接原地修改而不是创建新版本。这叫做**破坏性更新**（destructive update）或**唯一性优化**。

`Rc::try_unwrap` 看起来可以做这件事——它检查引用计数是否为 1。但问题是：

1. `try_unwrap` 消费 `Rc`，你只能用它做一次性检查
2. 在持久化链表的上下文中，你无法在 `prepend` 或 `tail` 时做这种检查，因为这些方法接收 `&self` 而不是 `self`
3. 即使引用计数为 1，你持有的是共享引用 `&self`，无法修改内部数据

所以，纯 `Rc` 方案无法做唯一性优化。如果我们想要这种优化，需要引入内部可变性（`RefCell`）或者 unsafe 代码。这也是下一章我们将探索的方向。

不过，`Rc::try_unwrap` 在 Drop 场景中非常有用——正如我们在 Drop 实现中看到的那样。这是 `Rc` 提供的"部分唯一性模拟"：你只能在拥有所有权（而非借用）的情况下尝试取出内部值。

---

## 完整代码

```rust
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {          // node: Rc<Node<T>>
            if let Ok(mut node) = Rc::try_unwrap(node) {  // node: Node<T>（遮蔽）
                // 内层 mut node 遮蔽了外层的 node，try_unwrap 消费 Rc 并返回内部值
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
```

---

## 关键教训

- **`Rc<T>` 提供共享所有权，通过引用计数管理生命周期**——`clone` 增加计数，drop 减少计数，计数归零时释放内存
- **持久化链表的核心是结构共享**——新版本和旧版本共享未修改的节点，`Rc` 让这种共享既安全又廉价
- **`Rc` 只提供不可变访问**——这是持久化数据结构的语义要求，也是 Rust 安全模型的体现
- **`Rc::try_unwrap` 可以部分模拟唯一性检查**——在 Drop 中用它避免递归析构，但在正常 API 中无法利用唯一性做原地修改
- **`Rc` 不是 `Sync`，不能跨线程共享**——需要线程安全时用 `Arc`，Rust 的 `Send`/`Sync` trait 在编译期阻止错误用法
- **`Option::and_then` 是处理 `Option` 嵌套的标准工具**——当闭包返回 `Option` 时，用 `and_then` 而不是 `map`

---

## 下一章

第三章的持久化链表是优雅的，但不可变性限制了它的实用性。第四章将引入 `unsafe` 代码，在保持安全抽象的同时实现一个**可变单端队列**——支持在尾部高效插入。这将是我们第一次踏入 Rust 的 unsafe 领域。

---

> 原文：[A Persistent Stack](https://rust-unofficial.github.io/too-many-lists/third.html) by Gankra
>
> 相关章节：
> - [所有权模型](../../ownership-lifetimes/ownership.md) — `Rc` 的 `Clone` 与共享所有权语义
> - [引用与借用](../../ownership-lifetimes/reference-borrow.md) — 不可变借用的多重性与 `Rc` 的共享访问
> - [智能指针](../../ownership-lifetimes/smart-pointer.md) — `Rc`/`Arc`/`RefCell` 的完整对比
