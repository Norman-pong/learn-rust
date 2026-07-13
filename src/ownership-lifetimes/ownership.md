# 所有权模型

> **一句话**：Rust 的所有权模型在编译期管理内存——每个值有且只有一个所有者，所有者离开作用域时自动释放；通过 Move、Clone 和 Copy 三种语义精确控制值的转移与复制行为。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 内存管理 | 编译期所有权 + 析构，无 GC | 垃圾回收（GC），自动回收不可达对象 |
| 赋值语义 | 默认 Move（非 Copy 类型） | 引用复制，对象共享 |
| 深拷贝 | 显式 `.clone()` | 依赖 `structuredClone` 或手动实现 |
| 按位复制 | `Copy` trait（简单标量/不可变引用） | 无对应概念，基本类型按值传递 |
| 所有权转移 | 函数传参/返回/赋值会转移所有权 | 总是共享引用 |

**核心差异**：在 TypeScript 中，把对象赋值给新变量只是多了一个引用；在 Rust 中，把 `String` 或 `Vec` 赋值给新变量会**移动**所有权，原变量此后无效。这个规则让 Rust 在编译期就能知道何时释放内存，不需要 GC，也避免了双重释放和悬空指针。只有实现了 `Copy` 的简单类型（如整数、浮点、布尔、引用）才会按位复制。

## 代码对比表

### Move 语义

```rust,should-compile
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Move：s1 的所有权转移到 s2

    // println!("{s1}"); // ❌ borrow of moved value: `s1`
    println!("{s2}"); // ✅

    take_ownership(s2);
    // s2 此时也无效了
}

fn take_ownership(s: String) {
    println!("took: {s}");
} // s 在这里 drop
```

```typescript
function main() {
    const s1 = "hello";
    const s2 = s1; // s1 仍然有效，引用复制

    console.log(s1); // ✅
    console.log(s2); // ✅

    takeOwnership(s2);
    console.log(s2); // ✅ 仍然有效
}

function takeOwnership(s: string) {
    console.log(`took: ${s}`);
}
```

### Clone 显式深拷贝

```rust,should-compile
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝，s1 和 s2 各自拥有独立堆数据

    println!("s1 = {s1}, s2 = {s2}"); // 两者都有效
}
```

```typescript
function main() {
    const s1 = { text: "hello" };
    const s2 = structuredClone(s1); // 深拷贝

    console.log(`s1 = ${s1.text}, s2 = ${s2.text}`); // 两者都有效
}
```

### Copy 类型

```rust,should-compile
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // Copy：按位复制，p1 仍然有效

    println!("p1 = {:?}, p2 = {:?}", p1, p2);

    let a = 42;
    let b = a; // i32 是 Copy
    println!("a = {a}, b = {b}");
}
```

```typescript
interface Point {
    x: number;
    y: number;
}

function main() {
    const p1: Point = { x: 1, y: 2 };
    const p2 = { ...p1 }; // 浅拷贝，p1 仍然有效

    console.log(`p1 = ${JSON.stringify(p1)}, p2 = ${JSON.stringify(p2)}`);

    const a = 42;
    const b = a; // 按值复制
    console.log(`a = ${a}, b = ${b}`);
}
```

### 所有权规则

```rust,should-compile
fn main() {
    // 规则 1：每个值有且只有一个所有者
    let s = String::from("owner");
    let t = s; // 所有者从 s 转移到 t

    // 规则 2：当所有者离开作用域，值被 drop
    {
        let inner = String::from("inner");
        println!("{inner}");
    } // inner 在这里被释放

    // 规则 3：所有权可以转移给函数或返回给调用者
    let returned = gives_ownership();
    println!("{returned}");
}

fn gives_ownership() -> String {
    String::from("from function")
}
```

```typescript
function main() {
    // 规则 1 在 TS 中不存在：多个引用共享同一对象
    const s = { text: "owner" };
    const t = s;

    {
        const inner = { text: "inner" };
        console.log(inner.text);
    } // 超出作用域，但 GC 不一定立即回收

    const returned = givesOwnership();
    console.log(returned);
}

function givesOwnership(): string {
    return "from function";
}
```

## 容易踩的坑

1. **赋值后继续使用原变量**——`let s2 = s1; println!("{s1}");` 对非 Copy 类型会编译失败，这是 Move 不是 Copy。
2. **`Copy` 与 `Clone` 混淆**——`Copy` 是隐式按位复制，`Clone` 是显式深拷贝；实现 `Copy` 必须同时实现 `Clone`（惯用 `#[derive(Copy, Clone)]`），且 `Drop` 与 `Copy` 互斥。
3. **在循环里 move 值**——`for item in vec { ... }` 会消耗 Vec，如果需要保留，应使用 `&vec` 或 `vec.iter()`。
4. **函数返回所有权**——`fn f(s: String) -> String` 把所有权返回给调用者，调用者必须接住，否则编译器会警告未使用的返回值。
5. **Move 与 `Rc`/`Arc` 的选择**——多所有者场景不要硬拼 `clone()`，应使用 `Rc`（单线程）或 `Arc`（多线程）共享所有权。

## 交叉链接

- → [引用与借用](reference-borrow.md) — 不转移所有权的情况下临时使用值
- → [生命周期基础](lifetime-basic.md) — 借用必须保证引用有效
- → [智能指针](smart-pointer.md) — `Rc`/`Arc` 处理多所有者，`Box` 处理堆分配
- → [变量与绑定](../basic/variable.md) — `let` 绑定是所有权的起点
