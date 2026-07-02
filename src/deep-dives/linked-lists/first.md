# 链表实现 · 第一章

> 译自 [too-many-lists](https://rust-unofficial.github.io/too-many-lists/first.html)，这是 Rust 入门者必读的链表教学。本章实现最"天真"的单向链表，揭示所有权系统的第一个 trade-off。

---

## 目标：实现一个栈式链表

```rust
// 我们要实现：
let mut list = List::new();
list.push(1);
list.push(2);
list.push(3);
assert_eq!(list.pop(), Some(3));
assert_eq!(list.pop(), Some(2));
assert_eq!(list.pop(), Some(1));
assert_eq!(list.pop(), None);
```

---

## 第一次尝试：朴素枚举

```rust
enum List {
    Empty,
    Elem(i32, List),  // 编译错误！
}
```

**编译器报错**：`recursive type has infinite size`

问题是：`List` 包含自身，编译器无法计算它的大小。`List` 的大小 = `i32` + `List` 的大小 = 无穷。

## 用 Box 打破递归

```rust
enum List {
    Empty,
    Elem(i32, Box<List>),  // Box 是固定大小的指针（usize 大小）
}
```

`Box<List>` 在栈上只是一个指针，实际数据在堆上。`List` 的大小现在是 `i32 + usize`，可计算。

## 完整实现

```rust
use List::{Empty, Elem};

enum List {
    Empty,
    Elem(i32, Box<List>),
}

impl List {
    fn new() -> Self {
        Empty
    }

    fn push(self, elem: i32) -> List {
        Elem(elem, Box::new(self))
    }

    fn pop(self) -> Option<(i32, List)> {
        match self {
            Empty => None,
            Elem(head, tail) => Some((head, *tail)),
        }
    }
}
```

### 使用方法

```rust
let list = List::new();           // Empty
let list = list.push(1);          // Elem(1, Empty)
let list = list.push(2);          // Elem(2, Elem(1, Empty))
let list = list.push(3);          // Elem(3, Elem(2, Elem(1, Empty)))

let (val, list) = list.pop().unwrap();  // val = 3
let (val, list) = list.pop().unwrap();  // val = 2
```

## 这个实现的问题

1. **每次 `push`/`pop` 都消费 `self` 返回新的 `List`**——调用者必须重新绑定 `let list = list.push(1)`
2. **空节点 `Empty` 浪费内存**——每个节点末尾都有一个 `Empty` 变体（额外内存开销）
3. **无法共享尾部**——push 在头部，pop 也在头部（栈式），但整个列表在每次操作时被重建
4. **没有可变引用**——无法迭代、无法修改中间节点

## 关键教训

- **递归类型必须用指针包装**（`Box`, `Rc` 等），让编译器能确定大小
- **所有权在数据结构设计中无处不在**——`push(self, ...)` 消费 self 是因为我们选择"新列表 = 新节点"模式
- **枚举 + 所有权 = 函数式数据结构**——这种风格类似 Lisp 的 cons cell，适合不可变数据

## 下一章

第二章将用 `Option<Box<Node>>` 替代 `Enum`，引入可变引用，实现更实用的链表。

---

> 原文：[A Bad Stack](https://rust-unofficial.github.io/too-many-lists/first.html) by Gankra
