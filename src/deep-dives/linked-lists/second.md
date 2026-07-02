# 链表实现 · 第二章

> 译自 [too-many-lists](https://rust-unofficial.github.io/too-many-lists/second.html)。

---

## 改进目标

上一章的 `Enum List` 有两个核心问题：
1. 每次都消费 `self`，不实用
2. `Empty` 变体浪费内存

本章使用 `Option<Box<Node>>` 替代枚举，引入 `&mut self` 方法实现**可变的数据结构**。

## 新结构：Option-based

```rust
struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}
```

- `List` 是公开类型，包装 `head` 指针
- `Node` 是内部实现细节
- `Link` 是类型别名：要么是某个节点（`Some(Box<Node>)`），要么是空（`None`）

## 完整实现

```rust
struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    fn new() -> Self {
        List { head: None }
    }

    // 在头部插入
    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),  // take() 取走值，留下 None
        });
        self.head = Some(new_node);
    }

    // 从头部弹出
    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}
```

### 关键点：`take()` 方法

```rust
// Option::take 的作用：
let mut x = Some(42);
let y = x.take();  // x 变成 None，y = Some(42)
assert_eq!(x, None);
assert_eq!(y, Some(42));
```

`take()` 让我们从 `&mut self` 中"偷走"所有权，而不需要消费整个 `self`。这是 Rust 中可变数据结构的基本模式。

## 与第一章的对比

| 特性 | 第一章（Enum List） | 第二章（Option + Box） |
|------|-------------------|----------------------|
| 方法签名 | `fn push(self, ...) -> List` | `fn push(&mut self, ...)` |
| 使用方式 | `let list = list.push(1)` | `list.push(1)` |
| 空节点 | `Empty` 变体（1 字节标记） | `None`（更紧凑） |
| 所有权 | 消费/重建 | 可变借用 |
| 性能 | O(n) push（重建整个列表） | O(1) push |

## Drop 的问题

我们的链表可能非常长（成千上万个节点）。默认的递归 Drop 可能导致栈溢出：

```rust
// 默认 Drop 行为（递归）：
// drop(node) → drop(node.next) → drop(node.next.next) → ...
// 10000 个节点 → 10000 层递归 → 栈溢出！
```

### 解决方案：迭代 Drop

```rust
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node 在这里离开作用域，被 drop
            // 但 next 已经被 take 走了，不会触发递归 drop
        }
    }
}
```

## 使用示例

```rust
let mut list = List::new();

list.push(1);
list.push(2);
list.push(3);  // 3 → 2 → 1

assert_eq!(list.pop(), Some(3));
assert_eq!(list.pop(), Some(2));

list.push(4);
list.push(5);  // 5 → 4 → 1

assert_eq!(list.pop(), Some(5));
assert_eq!(list.pop(), Some(4));
assert_eq!(list.pop(), Some(1));
assert_eq!(list.pop(), None);
```

## 关键教训

- **`Option::take()` 是从可变引用中转移所有权的核心技巧**
- **递归 Drop 可能栈溢出**——数据结构必须手动实现迭代式 Drop
- **`Option<Box<Node>>` 比 Enum 更紧凑、更实用**
- **可变数据结构的内部细节（`Node`, `Link`）保持私有**

## 下一章

第三章将实现不可变链表的持久化版本，引入 `Rc`。

---

> 原文：[An Ok Stack](https://rust-unofficial.github.io/too-many-lists/second.html) by Gankra
