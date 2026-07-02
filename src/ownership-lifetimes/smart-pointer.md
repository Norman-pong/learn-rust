# 智能指针

> **一句话**：智能指针是包装了额外元数据和行为的指针类型——`Box` 分配堆内存，`Rc` 共享所有权，`Arc` 跨线程共享，`RefCell` 提供运行时借用检查，`Cell` 提供 Copy 类型内部可变性。

## 与 JS/TS 的关键差异

JS/TS 中所有引用本质上都是"智能"的——由 GC 追踪、自动释放，没有所有权/可变性约束。Rust 的智能指针在保持编译期安全的前提下，提供类似动态语言的灵活性，但每种指针都有明确的代价模型和适用场景。

| 智能指针 | 解决的问题 | 开销 | JS/TS 对应 |
|----------|-----------|------|-----------|
| `Box<T>` | 堆分配 + 递归类型 | 一次堆分配 | 基本引用（无额外开销版本） |
| `Rc<T>` | 单线程多所有者共享数据 | 引用计数（非原子） | 引用计数 GC 对象 |
| `Arc<T>` | 多线程共享数据 | 原子引用计数 | 多线程安全的引用计数 |
| `RefCell<T>` | 编译期借用检查→运行时 | 运行时 borrow 计数 | `Proxy` 或 getter/setter |
| `Cell<T>` | 内部可变性（Copy 类型） | 无（值替换，非借用） | 无直接对应 |

## 代码对比表

### `Box<T>` — 堆分配

```rust
// 递归类型：需要 Box 打破无限嵌套
enum List {
    Cons(i32, Box<List>), // Box 使得 List 的大小在编译期可知
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 普通堆值
    let boxed = Box::new(5);
    println!("{boxed}"); // 自动解引用
}
```

```typescript
// TypeScript — 引用天然是堆分配
type List = { value: number; next: List | null };

const list: List = {
    value: 1,
    next: { value: 2, next: { value: 3, next: null } },
};

const boxed = 5; // 没有显式 Box 概念
console.log(boxed);
```

### `Rc<T>` — 共享所有权

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a); // 不克隆数据，只增加引用计数
    let c = Rc::clone(&a);

    println!("count: {}", Rc::strong_count(&a)); // count: 3
    // a, b, c 共享同一块堆数据
}
```

```typescript
class SharedRef<T> {
    private count = 1;
    constructor(public value: T) {}
    clone() { this.count++; return this; }
    strongCount() { return this.count; }
}

function main() {
    const a = new SharedRef("shared");
    const b = a.clone();
    const c = a.clone();
    console.log(`count: ${a.strongCount()}`); // 3
}
```

### `RefCell<T>` — 内部可变性

```rust
use std::cell::RefCell;

fn main() {
    // 外部不可变，内部可修改
    let data = RefCell::new(42);
    *data.borrow_mut() = 100; // 运行时检查：当前没有不可变借用
    assert_eq!(*data.borrow(), 100); // 运行时检查

    // 运行时 panic：同时持有可变和不可变借用
    // let mut_ref = data.borrow_mut();
    // let imm_ref = data.borrow(); // panic!
}
```

```typescript
class RefCell<T> {
    private borrowed = false;
    constructor(private value: T) {}

    borrow() { return this.value; }
    borrowMut(): { value: T; done: () => void } {
        if (this.borrowed) throw new Error("already borrowed");
        this.borrowed = true;
        return {
            value: this.value,
            done: () => { this.borrowed = false; },
        };
    }
}

function main() {
    const data = new RefCell(42);
    const mutRef = data.borrowMut();
    mutRef.value = 100;
    mutRef.done();
    console.log(data.borrow());
}
```

### `Rc<RefCell<T>>` — 共享可变数据

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);

    clone1.borrow_mut().push(4); // 通过任一 Rc 修改内部 Vec
    clone2.borrow_mut().push(5);

    assert_eq!(*shared.borrow(), vec![1, 2, 3, 4, 5]);
}
```

```typescript
class SharedRef<T> {
    private count = 1;
    constructor(public value: T) {}
    clone() { this.count++; return this; }
}

function main() {
    const shared = new SharedRef({ data: [1, 2, 3] });
    const clone1 = shared.clone();
    const clone2 = shared.clone();

    clone1.value.data.push(4);
    clone2.value.data.push(5);

    console.log(shared.value.data); // [1, 2, 3, 4, 5]
}
```

### `Deref` 强制转换

```rust
use std::rc::Rc;

fn main() {
    // Box<T> 自动解引用为 T
    let x = Box::new(5);
    assert_eq!(*x, 5); // *x 通过 Deref 解引用

    // Rc<T> 也可以自动解引用
    let s = Rc::new(String::from("hello"));
    fn greet(name: &str) { println!("Hello, {name}"); }
    greet(&s); // &Rc<String> → &String → &str（Deref 链）
}
```

```typescript
function greet(name: string) {
    console.log(`Hello, ${name}`);
}

function main() {
    const s = "hello";
    greet(s); // 无 Deref 链，统一按引用传递
}
```

## 容易踩的坑

1. **`Rc::clone` vs `.clone()`**——`Rc::clone(&a)` 只加引用计数不深拷贝；`a.clone()` 对 `Rc<T>` 效果相同，但对 `T` 本身会深拷贝。
2. **RefCell 的运行时惩罚**——违反借用规则不会编译错误，而是运行时 panic，因此只应在单线程且确定借用不冲突时使用。
3. **`Rc` 不是 `Send`**——不能跨线程，跨线程共享应改用 `Arc`；`RefCell` 不是 `Sync`，所以 `Arc<RefCell<T>>` 不能多线程读写。
4. **引用循环**——`Rc` 不会自动处理循环引用（`a → b → a`），会造成内存泄漏，必要时用 `Weak`。
5. **`Box<dyn Trait>` vs 泛型**——trait object 有动态分发开销，泛型是静态分发；`dyn` 会擦除类型，调用者无法知道具体类型。

## 交叉链接

- → [所有权模型](ownership.md) — 智能指针扩展了所有权系统的能力
- → [引用与借用](reference-borrow.md) — RefCell 是借用规则的"运行时软版本"
- → [并发](concurrency/thread.md) — `Arc` 替代 `Rc` 实现线程安全共享
- → [Trait 与泛型](trait-generic.md) — `Deref` 是一个标准 trait
