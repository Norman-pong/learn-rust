# 链表实现 · 第五章 · 生产级双向队列

> 译自 [too-many-lists](https://rust-unofficial.github.io/too-many-lists/sixth.html)。本章是 too-many-lists 系列的终极挑战：用 unsafe 实现一个生产级双向链表（Doubly-Linked Deque），对标 `std::collections::LinkedList`。我们将引入 `NonNull`、PhantomData、cursor API，以及 `Send`/`Sync` 的手工验证。

---

## 为什么双向链表是终极挑战

回顾前四章，我们逐步升级了链表的能力：

| 章节 | 数据结构 | 核心工具 | 难度 |
|------|---------|---------|------|
| 第一章 | 不可变栈（Enum） | `Box` | ⭐ |
| 第二章 | 可变栈 | `Option::take()` | ⭐⭐ |
| 第三章 | 持久化栈 | `Rc` | ⭐⭐ |
| 第四章 | 不安全单链表队列 | `*mut Node<T>` | ⭐⭐⭐⭐ |
| **第五章** | **生产级双向 Deque** | `NonNull<T>` + `PhantomData` | ⭐⭐⭐⭐⭐ |
>
> **译注**：本系列跳过了原书第四章（`Rc<RefCell<Node>>` 安全双向 Deque），因为其核心概念已在第三章 `Rc` 和[智能指针](../../ownership-lifetimes/smart-pointer.md)章节中覆盖。如果你还没读过第四章的不安全队列实现，强烈建议先回顾——它引入了裸指针和 `Box::into_raw`/`from_raw` 的所有权转移技巧，本章将把这些概念推向极致。

双向链表之所以是终极挑战，是因为它在 Rust 中同时触发了所有权系统的所有痛点：

1. **每个节点有两个指针**（front 和 back），形成图结构而非树结构
2. **头尾指针需要自引用**——链表结构体持有指向内部节点的指针
3. **迭代器需要同时遍历两个方向**，且必须保证 aliasing 规则不被破坏
4. **Cursor API** 允许在链表中间插入、删除、分裂、拼接——这是 `std::collections::LinkedList` 的标志性能力
5. **Send/Sync 标记**需要手工推导和验证


---

## 布局设计：传统方案 vs 哑节点方案

双向链表在概念上很简单：每个节点多一个 prev 指针。但"简单"正是它欺骗你的方式。

### 传统布局

```
[front, back] <-> (ptr, A, ptr) <-> (ptr, B, ptr) <-> (ptr, C, ptr)
```

链表结构体保存 `front` 和 `back` 两个裸指针，分别指向头节点和尾节点。每个节点保存 `front`（前一个节点）、`back`（后一个节点）和 `elem`（数据）。

### 哑节点（Dummy Node）布局

另一种方案是引入一个不存储数据的哑节点，把链表的两端连成一个环：

```
[ptr] -> (ptr, DUMMY, ptr) <-> (ptr, A, ptr) <-> (ptr, B, ptr)
         ^                                               ^
         +-----------------------------------------------+
```

哑节点的优点是消除了空链表、单节点链表等特殊边界情况——每个节点永远有真实的 prev 和 next。但它在 Rust 中有几个致命问题：

- **额外的分配**：空链表也必须有一个哑节点
- **哑节点的 `elem` 字段怎么初始化？** 如果 `T` 是 `Box<str>` 或自定义类型，可能无法构造默认值
- 解决方案（`Option<T>`、`MaybeUninit<T>`、类型双关）都过于复杂

因此，too-many-lists 选择传统布局——这也是 `std::collections::LinkedList` 的方案。

---

## 基础结构

```rust
use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,   // 前一个节点
    back: Link<T>,    // 后一个节点
    elem: T,
}
```

### `NonNull<T>`：有态度的裸指针

`NonNull<T>` 是对 `*mut T` 的包装，保证指针**永远不会是 null**。它的核心优势：

- **与 `Option` 组合**：`Option<NonNull<T>>` 可以用 null pointer optimization，使得 `None` 不占用额外空间
- **协变（covariant）**：`NonNull<&'static T>` 可以自动转换为 `NonNull<&'a T>`，这对泛型容器至关重要
- **语义清晰**：它告诉读者"这个指针总是有效的（如果存在的话）"

> **译注**：`PhantomData<T>` 是一个零大小类型（ZST），它的存在告诉编译器"`LinkedList<T>` 逻辑上拥有 `T` 类型的数据"。这影响 drop check 和 auto trait 推导。没有它，编译器可能错误地认为 `LinkedList<T>` 不拥有 `T`，导致 `Drop` 顺序或 `Send`/`Sync` 推导出错。

---

## push_front：在头部插入

```rust
impl<T> LinkedList<T> {
    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(old) = self.front {
                // 旧头节点的前指针指向新节点
                (*old.as_ptr()).front = Some(new);
                // 新节点的后指针指向旧头节点
                (*new.as_ptr()).back = Some(old);
            } else {
                // 空链表，新节点也是尾节点
                self.back = Some(new);
            }

            self.front = Some(new);
            self.len += 1;
        }
    }
}
```

关键点：

1. `Box::into_raw(Box::new(Node { ... }))` 在堆上分配节点并返回 `*mut Node<T>`
2. `NonNull::new_unchecked(...)` 把裸指针包装成 `NonNull`——我们确信 `Box::into_raw` 永远不会返回 null
3. 通过 `old.as_ptr()` 获取 `*mut Node<T>`，然后用 `(*ptr).field` 解引用修改字段
4. 所有裸指针解引用都在 `unsafe` 块中

> **译注**：`NonNull::new_unchecked` 是 unsafe 的，因为如果你传了 null 指针，后续的所有操作都是未定义行为。但 `Box::into_raw` 的契约保证返回非 null，所以这个 `unsafe` 块是 sound 的。这种"依赖底层 API 的契约来保证 safety"的模式在 Rust 中非常常见。

---

## pop_front：从头部弹出

```rust
impl<T> LinkedList<T> {
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|node| {
                // 把裸指针重新包装成 Box，这样 Rust 会正确 drop 它
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                // 新头节点是旧头节点的 back
                self.front = boxed_node.back;

                if let Some(new) = self.front {
                    // 新头节点的 front 指针置空
                    (*new.as_ptr()).front = None;
                } else {
                    // 链表已空，back 也要置空
                    self.back = None;
                }

                self.len -= 1;
                result
                // boxed_node 在这里离开作用域，内存被释放
            })
        }
    }
}
```

`Box::from_raw` 是 `Box::into_raw` 的逆操作。它重新获得堆内存的所有权，当 `boxed_node` 离开作用域时，`Box` 的 `Drop` 实现会释放内存。

> **译注**：注意 `self.front = boxed_node.back` 这行。`boxed_node` 是一个 `Box<Node<T>>`，它的 `back` 字段是 `Option<NonNull<Node<T>>>`。我们在把 `boxed_node` 移出字段的同时，`Box` 本身会在函数末尾被 drop。这意味着节点的内存被释放，但 `back` 指针（指向链表中的下一个节点）被安全地转移到了 `self.front`。这是裸指针链表的核心技巧：所有权和指针分离管理。

---

## 迭代器设计

双向链表需要三种迭代器：`Iter`（不可变）、`IterMut`（可变）、`IntoIter`（消耗所有权）。

### Iter（不可变借用）

```rust
pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}
```

`DoubleEndedIterator` 允许从两端同时遍历——这是双向链表的标志性能力。

### IterMut（可变借用）

```rust
pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

/// 消耗所有权的迭代器
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}
```

> **译注**：`IterMut` 的 `_boo: PhantomData<&'a mut T>` 与 `Iter` 的 `_boo: PhantomData<&'a T>` 有本质区别。`&'a mut T` 对 `T` 是**不变（invariant）**的，对 `'a` 是协变的。这意味着编译器不会允许 `IterMut<'static, &'static T>` 转换为 `IterMut<'a, &'a T>`。这是必要的，因为可变迭代器必须保证在迭代期间链表不会被修改。详见 [生命周期进阶](../../ownership-lifetimes/lifetime-advanced.md) 中型变的讨论。

> **译注**：为什么本章的迭代器改用 `Option<NonNull<Node<T>>>` 而非前几章的 `Option<&'a Node<T>>`？因为双向链表需要 `DoubleEndedIterator`，必须同时维护 `front` 和 `back` 两个指针；而 `NonNull` 的协变特性也让泛型容器的型变推导更自然。这是从"引用型"到"裸指针型"的合理演进。

---

## CursorMut：链表的灵魂

Cursor（游标）是双向链表最强大的 API。它允许你在链表中间移动、查看、插入、删除、分裂和拼接——这是数组或 Vec 无法高效做到的。

```rust
pub struct CursorMut<'a, T> {
    list: &'a mut LinkedList<T>,
    cur: Link<T>,       // 当前指向的节点
    index: Option<usize>, // 当前索引（None 表示在"幽灵位置"）
}
```

### 工厂方法

```rust
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut {
            list: self,
            cur: None,
            index: None,
        }
    }
}
```

### 移动

```rust
impl<'a, T> CursorMut<'a, T> {
    pub fn move_next(&mut self) {
        if let Some(cur) = self.cur {
            unsafe {
                self.cur = (*cur.as_ptr()).back;
                if self.cur.is_some() {
                    *self.index.as_mut().unwrap() += 1;
                } else {
                    self.index = None; // 走到幽灵位置
                }
            }
        } else if !self.list.is_empty() {
            // 从幽灵位置移到第一个真实节点
            self.cur = self.list.front;
            self.index = Some(0);
        }
    }

    pub fn move_prev(&mut self) {
        if let Some(cur) = self.cur {
            unsafe {
                self.cur = (*cur.as_ptr()).front;
                if self.cur.is_some() {
                    *self.index.as_mut().unwrap() -= 1;
                } else {
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // 从幽灵位置移到最后一个真实节点
            self.cur = self.list.back;
            self.index = Some(self.list.len - 1);
        }
    }
}
```

"幽灵位置"（ghost element）是 cursor 设计的关键概念。当 cursor 不在任何真实节点上时（`cur: None`），它位于头节点之前或尾节点之后。这让你可以在链表头部之前插入元素——一个非常有用的边界情况。

### split_before：在游标处分裂链表

```rust
impl<'a, T> CursorMut<'a, T> {
    pub fn split_before(&mut self) -> LinkedList<T> {
        if let Some(cur) = self.cur {
            unsafe {
                let old_len = self.list.len;
                let old_idx = self.index.unwrap();
                let prev = (*cur.as_ptr()).front;

                // 新链表（右侧）
                let new_len = old_len - old_idx;
                let new_front = self.cur;
                let new_back = self.list.back;

                // 输出链表（左侧）
                let output_len = old_len - new_len;
                let output_front = self.list.front;
                let output_back = prev;

                // 断开 cur 和 prev 之间的链接
                if let Some(prev) = prev {
                    (*cur.as_ptr()).front = None;
                    (*prev.as_ptr()).back = None;
                }

                // 更新原链表
                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                self.index = Some(0);

                LinkedList {
                    front: output_front,
                    back: output_back,
                    len: output_len,
                    _boo: PhantomData,
                }
            }
        } else {
            // 游标在幽灵位置，直接替换整个链表
            std::mem::replace(self.list, LinkedList::new())
        }
    }
}
```

> **译注**：`split_before` 是双向链表最复杂的操作之一。它把链表分成两部分：游标左侧的所有节点返回为新链表，右侧保留在原链表中。关键是不变量维护——分裂后两个链表的头尾指针、长度、索引都必须正确。`std::mem::replace` 在幽灵位置的情况下非常优雅：它把空链表塞给 `self.list`，同时返回原来的完整链表。

### splice_before：在游标前拼接链表

```rust
impl<'a, T> CursorMut<'a, T> {
    pub fn splice_before(&mut self, mut input: LinkedList<T>) {
        unsafe {
            if input.is_empty() {
                return;
            }

            if let Some(cur) = self.cur {
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                if let Some(prev) = (*cur.as_ptr()).front {
                    // 一般情况：在内部插入
                    (*prev.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(prev);
                    (*cur.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cur);
                } else {
                    // 在头部插入
                    (*cur.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cur);
                    self.list.front = Some(in_front);
                }

                *self.index.as_mut().unwrap() += input.len;
            } else if let Some(back) = self.list.back {
                // 游标在幽灵位置，但链表非空：拼接到尾部
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();
                (*back.as_ptr()).back = Some(in_front);
                (*in_front.as_ptr()).front = Some(back);
                self.list.back = Some(in_back);
            } else {
                // 空链表：直接替换
                std::mem::swap(self.list, &mut input);
            }

            self.list.len += input.len;
            input.len = 0; // 礼貌地清空 input
        }
    }
}
```

---

## Send 与 Sync：手工验证

`LinkedList<T>` 默认不会自动实现 `Send` 和 `Sync`，因为它包含裸指针。但如果我们能证明它在多线程环境下是安全的，就可以手动实现：

```rust
unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}
```

### 为什么这些实现是 sound 的？

- **`LinkedList<T>: Send` 当 `T: Send`**：链表本身只是一些指针和计数器。如果 `T` 可以安全地跨线程移动，那么把链表从一个线程移到另一个线程不会导致数据竞争或悬垂指针。
- **`LinkedList<T>: Sync` 当 `T: Sync`**：如果多个线程同时持有 `&LinkedList<T>`，它们只能通过 `front()`/`back()`/`iter()` 等方法读取数据。这些方法返回 `&T`，而 `T: Sync` 保证 `&T` 可以安全共享。
- **`IterMut` 不是 `Sync`**：可变迭代器持有内部状态（`front`、`back`），多线程同时访问会导致数据竞争。但 `IterMut<'a, T>: Send` 当 `T: Send` 是安全的，因为迭代器本身不共享——它只是一个独立的状态机。

> **译注**：`unsafe impl` 是 Rust 中最严肃的承诺之一。你在告诉编译器"我用人格担保这个类型满足 Send/Sync 的契约"。如果错了，就是未定义行为。标准库的 `LinkedList` 也做了完全相同的推导，所以我们可以有信心。但如果你自己设计一个全新的数据结构，务必用 [Miri](https://github.com/rust-lang/miri) 验证。

---

## Drop 与 panic safety

```rust
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
```

复用 `pop_front` 是最简洁的方案。但如果 `T::drop` panic 了怎么办？

在 Rust 中，如果析构函数 panic，程序会 abort（除非在 `catch_unwind` 中）。但对于链表来说，一个节点的 `drop` panic 不应该导致其他节点泄漏。`pop_front` 在提取 `elem` 之后才释放节点，所以即使 `elem` 的 `drop` panic，节点本身已经被从链表上摘下来了——不会导致双重释放或泄漏。

> **译注**：panic safety 是 unsafe 代码的进阶话题。简单来说，你要确保即使在最糟糕的 panic 场景下，数据结构的不变量仍然保持。对于链表，最关键的不变量是：每个节点要么在链表上（被某个指针指向），要么已经被释放。`pop_front` 先修改链表指针、再提取数据、最后让 `Box` 离开作用域——这个顺序保证了即使中间 panic，链表仍然是有效的。

---

## 与原书 `Rc<RefCell>` 方案的终极对比

> **译注**：本系列跳过了原书第四章的 `Rc<RefCell<Node>>` 安全双向 Deque 实现。下表将本章的 unsafe 方案与原书第四章的 safe 方案做对比，帮助读者理解两条路线的取舍。原书第四章详见[这里](https://rust-unofficial.github.io/too-many-lists/fourth.html)。

| 特性 | 原书第四章（`Rc<RefCell>`） | 本章（Production Unsafe） |
|------|---------------------|--------------------------|
| 运行时开销 | 引用计数 + 借用检查 | 无 |
| 内存布局 | 每个节点额外 2-3 个 usize | 每个节点 2 个指针 |
| 循环引用风险 | 存在（相邻节点互相持有 `Rc`，需 `Weak` 打破） | 不存在（裸指针无引用计数） |
| Cursor API | 极难实现（`RefCell` 借用冲突） | 自然支持 |
| 迭代器 | 简单但性能差 | 高效，支持 DoubleEnded |
| Send/Sync | 天然不支持 | 手工实现，条件更精确 |
| 代码复杂度 | 冗长，嵌套 borrow_mut() | 需要 unsafe，但逻辑清晰 |
| Miri 验证 | 不需要 | 强烈推荐 |
| 生产可用性 | ❌ 不推荐 | ✅ 对标 std |

> **译注**：第四章的 `Rc<RefCell<Node>>` 方案在语义上是"safe"的——它不会触发未定义行为。但它在工程上是"bad"的：运行时开销高、API 受限、代码难以维护。本章的 unsafe 方案在工程上是"good"的：零开销、完整 API、与标准库同等质量。这就是 Rust 的设计哲学：**安全抽象无法表达时，用 unsafe 构建安全的外壳，而不是牺牲性能和功能**。

---

## 为什么"safe"也可能是"bad"——明确边界与取舍

too-many-lists 这个系列的名字本身就带有讽刺意味。链表在大多数场景下都不是最佳选择：

- **缓存局部性差**：节点分散在堆上，遍历时的缓存未命中比数组高得多
- **分配开销**：每个节点都是独立的堆分配
- **预取不友好**：CPU 无法预测下一个节点的地址

对于绝大多数 Rust 程序，`Vec`、`VecDeque` 或 `HashMap` 都比链表更快、更省内存、更 cache-friendly。`std::collections::LinkedList` 的存在是为了那些真正需要 O(1) 中间插入/删除的场景——比如实现 LRU Cache、内存分配器、或某些特定的图算法。

但 too-many-lists 的真正目的不是教你写链表，而是教你理解 Rust 的内存模型：

1. **所有权和借用规则不是限制，而是保护**——当你试图绕过它们时，你会更深刻地理解为什么它们存在
2. **`unsafe` 不是洪水猛兽**——它是 Rust 提供的精确工具，用于在编译器无法验证的地方由程序员提供保证
3. **`Rc<RefCell>` 是逃生舱，不是目的地**——它让你快速原型，但不适合生产代码
4. **自引用结构需要特殊处理**——裸指针、`Pin`、`PhantomPinned` 各有适用场景，详见 [自引用结构](../../ownership-lifetimes/self-referential.md)

---

## 系列小结：unsafe 边界反思

五章下来，我们从最天真的 `enum List` 走到了生产级的 `LinkedList<T>`。这条路径上的每一个坑，都是 Rust 所有权系统的教学案例（本系列跳过了原书第四章 `Rc<RefCell>` 双向 Deque，其核心概念已在第三章和智能指针章节覆盖）：

| 章节 | 核心教训 |
|------|---------|
| 第一章 | 递归类型需要指针打破无限大小 |
| 第二章 | `Option::take()` 是从可变引用转移所有权的钥匙 |
| 第三章 | `Rc` 让共享所有权成为可能，但代价是不可变性 |
| 第四章 | 裸指针是处理自引用和 O(1) 尾操作的最终手段 |
| 第五章 | `NonNull` + `PhantomData` + 手工 `Send`/`Sync` = 生产级 unsafe 抽象 |

Rust 的 unsafe 边界不是"安全 vs 不安全"的二元对立，而是一个**频谱**：

- **完全 safe**：编译器验证一切（第二章的栈）
- **safe 但 bad**：编译器通过，但运行时开销高、API 受限（原书第四章的 `Rc<RefCell>` deque，本系列跳过）
- **unsafe 但 sound**：程序员验证不变量，对外暴露 safe API（第四章的队列、第五章的 deque）
- **unsafe 且 unsound**：程序员犯错，导致未定义行为（我们要避免的）

作为 Rust 开发者，你的目标不是"避免 unsafe"，而是"在正确的抽象层次使用正确的工具"。当你需要链表时，先用 `std::collections::LinkedList`；当你需要理解它时，回来看 too-many-lists；当你需要构建自己的 unsafe 抽象时，记得用 Miri 验证、写文档说明不变量、并请求代码审查。

> **译注**：本系列到此结束。如果你想继续深入，推荐阅读 `std::collections::LinkedList` 的源码（它比我们这里的实现更复杂，处理了自定义分配器、更精细的 trait 实现等）。另外，[自引用结构](../../ownership-lifetimes/self-referential.md) 和 [智能指针](../../ownership-lifetimes/smart-pointer.md) 是理解本章的两个关键前置知识，建议交叉阅读。

---

> 原文：[A Production Unsafe Doubly-Linked Deque](https://rust-unofficial.github.io/too-many-lists/sixth.html) by Gankra
>
> 系列终章。感谢 Gankra 写了这本让无数 Rustacean 又爱又恨的教程。
