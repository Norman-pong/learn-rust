# 变量与绑定

> **一句话**：Rust 的变量默认不可变，可变性通过 `mut` 显式声明；`let` 创建绑定，`const` 声明编译期常量，shadowing 允许同名变量重新绑定。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 默认可变性 | 默认不可变：`let x = 1` 不可重新赋值 | 默认可变：`const` 仅限制重新绑定，对象内容仍可变 |
| 显式可变 | `let mut x = 1` | 无直接对应，对象属性天然可变 |
| 编译期常量 | `const MAX: u32 = 100` 必须标注类型 | `const MAX = 100`（类型可推断） |
| 重新绑定 | `let x = 1; let x = "a"` 允许 shadowing | 同一作用域 `const x` 重复声明会报错 |
| 解构赋值 | `let (a, b) = (1, 2)` 支持模式 | 数组/对象解构 `const [a, b] = [1, 2]` |
| 未初始化 | 编译器拒绝使用未初始化变量 | 允许声明未初始化变量（值为 `undefined`） |

**核心差异**：Rust 把"可变性"从类型系统里抽出来作为绑定属性。`let` 默认是"只读绑定"，而 TypeScript 的 `const` 更接近"变量不能再赋值"，但对象引用指向的内容仍然可被修改。Rust 的 `mut` 控制的是**绑定所指内存是否可变**，这使得数据竞争可以在编译期被捕获。

## 代码对比表

### 默认不可变 vs 可变

```rust
fn main() {
    let x = 5;
    // x = 6; // ❌ error: cannot assign twice to immutable variable

    let mut y = 5;
    y = 6; // ✅

    println!("x = {x}, y = {y}");
}
```

```typescript
const x = 5;
// x = 6; // ❌ TS 报错：无法分配到 'const'

let y = 5;
y = 6; // ✅

// 但 const 对象内部仍可变
const user = { name: "Alice" };
user.name = "Bob"; // ✅ TypeScript 不会阻止
```

### 常量与编译期计算

```rust
const MAX_USERS: u32 = 1000;
const PI: f64 = 3.14159;
// const 必须可编译期求值，不能是运行时计算结果
const DOUBLE_MAX: u32 = MAX_USERS * 2;

fn main() {
    println!("max users: {MAX_USERS}");
}
```

```typescript
const MAX_USERS = 1000;
const PI = 3.14159;
const DOUBLE_MAX = MAX_USERS * 2; // 运行时求值，但值固定

console.log(MAX_USERS);
```

### Shadowing（遮蔽）

```rust
fn main() {
    let x = 5;
    let x = x + 1; // 同名新绑定，类型可以不同
    let x = x * 2;

    let spaces = "    ";
    let spaces = spaces.len(); // 从 &str 变成 usize

    println!("x = {x}, spaces = {spaces}");
}
```

```typescript
function main() {
    const x = 5;
    const x2 = x + 1; // 必须换名
    const x3 = x2 * 2;

    let spaces = "    ";
    spaces = spaces.length; // 同一变量，类型可改变（any/number）

    console.log(`x = ${x3}, spaces = ${spaces}`);
}
```

### 解构赋值

```rust
fn main() {
    let (a, b, c) = (1, 2.0, "hello");

    let point = (10, 20);
    let (x, y) = point;

    let arr = [1, 2, 3];
    let [first, second, ..] = arr; // 忽略剩余元素

    println!("{a}, {b}, {c}, {x}, {y}, {first}");
}
```

```typescript
const [a, b, c] = [1, 2.0, "hello"];

const point: [number, number] = [10, 20];
const [x, y] = point;

const arr = [1, 2, 3];
const [first, second, ...rest] = arr;

console.log(`${a}, ${b}, ${c}, ${x}, ${y}, ${first}`);
```

## 容易踩的坑

1. **`let` 不是 `const` 的等价物**——`let` 变量本身不可重新赋值，但 shadowing 可以重新绑定；`const` 在 Rust 中是编译期常量，不能用于运行时值。
2. **`mut` 控制的是绑定，不是类型**——`let mut s = String::new()` 表示 `s` 这个绑定可变，可以重新指向另一个字符串，也可以修改字符串内容。
3. **Shadowing 与重新赋值的区别**——`let x = x + 1` 创建了新的绑定，旧绑定被遮蔽；`x = x + 1` 只是修改原值，要求 `mut`。
4. **忘记 `mut` 导致数组/Vec 无法 push**——`let v = vec![1]; v.push(2)` 会编译失败，需要 `let mut v`。
5. **常量必须大写且标注类型**——`const max = 100` 会报错；Rust 的 `const` 也不允许 `let` 的表达式结果。

## 交叉链接

- → [类型系统](type.md) — `let` 的类型推断与显式标注
- → [函数](function.md) — 函数参数中的 `mut` 绑定
- → [所有权模型](ownership-lifetimes/ownership.md) — `let` 绑定是所有权系统的起点
