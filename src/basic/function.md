# 函数

> **一句话**：Rust 函数显式声明参数类型和返回类型，函数体是表达式块；区分表达式与语句，支持高阶函数、闭包参数和返回 impl trait，编译期类型安全优先。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 参数类型 | 必须显式声明：`fn add(a: i32, b: i32)` | 可选：`function add(a: number, b: number)` |
| 返回类型 | 必须显式声明：`-> i32` | 可选，可推断 `=> number` |
| 语句 vs 表达式 | 函数体最后一个表达式是返回值，分号使其成为语句 | `return` 显式返回，或函数体隐式返回 `undefined` |
| 高阶函数 | `fn call_fn(f: impl Fn(i32) -> i32)` | `function callFn(f: (x: number) => number)` |
| 泛型函数 | `fn identity<T>(x: T) -> T` | `function identity<T>(x: T): T` |
| 默认参数 | 不支持，用函数重载或 builder 模式 | 支持 `function f(a = 1)` |
| 可变参数 | 不支持，用宏或迭代器 | 支持 rest params `...args` |

**核心差异**：Rust 函数签名是契约，调用者与被调用者都在编译期受约束。函数体末尾的表达式自动成为返回值，这消除了"忘记 return"的 bug。高阶函数通过 trait（`Fn`/`FnMut`/`FnOnce`）而不是函数类型表达，这让编译器可以区分闭包对环境的捕获方式。

## 代码对比表

### 基础函数：表达式与语句

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // 表达式，没有分号，作为返回值
}

fn subtract(a: i32, b: i32) -> i32 {
    let result = a - b; // 语句
    result // 表达式返回
}

fn main() {
    println!("add: {}", add(2, 3));
    println!("subtract: {}", subtract(5, 2));
}
```

```typescript
function add(a: number, b: number): number {
    return a + b;
}

function subtract(a: number, b: number): number {
    const result = a - b;
    return result;
}

console.log(`add: ${add(2, 3)}`);
console.log(`subtract: ${subtract(5, 2)}`);
```

### 高阶函数

```rust
fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    let result = apply_twice(|x| x + 1, 5);
    println!("{result}"); // 7
}
```

```typescript
function applyTwice(f: (x: number) => number, x: number): number {
    return f(f(x));
}

const result = applyTwice((x) => x + 1, 5);
console.log(result); // 7
```

### 返回函数（impl Trait）

```rust
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor // 闭包捕获 factor，move 取得所有权
}

fn main() {
    let triple = make_multiplier(3);
    println!("{}", triple(4)); // 12
}
```

```typescript
function makeMultiplier(factor: number): (x: number) => number {
    return (x) => x * factor;
}

const triple = makeMultiplier(3);
console.log(triple(4)); // 12
```

### 泛型函数与 trait bound

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let nums = [3, 1, 4, 1, 5];
    println!("largest: {}", largest(&nums));
}
```

```typescript
function largest(list: number[]): number {
    let max = list[0];
    for (const item of list) {
        if (item > max) {
            max = item;
        }
    }
    return max;
}

const nums = [3, 1, 4, 1, 5];
console.log(`largest: ${largest(nums)}`);
```

### 发散函数

```rust
fn exit() -> ! {
    panic!("unrecoverable");
}

fn main() {
    let guess: u32 = match "42".parse() {
        Ok(n) => n,
        Err(_) => exit(), // ! 可以兼容任何类型
    };
    println!("{guess}");
}
```

```typescript
function exit(): never {
    throw new Error("unrecoverable");
}

function main() {
    const guess: number = (() => {
        const parsed = Number.parseInt("42", 10);
        if (Number.isNaN(parsed)) {
            exit();
        }
        return parsed;
    })();
    console.log(guess);
}
```

## 容易踩的坑

1. **末尾分号吞掉返回值**——`fn f() { 1 + 2; }` 返回 `()`（表达式结果被丢弃）；若声明了 `-> i32` 则编译失败（类型不匹配）。正确写法：去掉分号 `1 + 2` 或显式 `return 1 + 2;`。
2. **函数参数默认不可变**——`fn change(s: String)` 不能修改 `s`，需要 `fn change(mut s: String)` 或 `fn change(s: &mut String)`。
3. **返回值转移所有权**——`fn create() -> String { String::from("hi") }` 返回后调用者拥有该字符串。
4. **高阶函数 trait 选择**——`Fn` 只读捕获，`FnMut` 可变捕获，`FnOnce` 消费所有权；选错会导致编译失败。
5. **`impl Trait` 在返回位置不透明**——`fn f() -> impl Trait` 调用者无法把返回值赋给具体类型变量，只能作为 trait 对象使用或继续传参。

## 交叉链接

- → [变量与绑定](variable.md) — 函数参数绑定与 `mut`
- → [Trait 与泛型](trait-generic.md) — trait bound 与 impl Trait 的完整展开
- → [闭包](closure.md) — `Fn`/`FnMut`/`FnOnce` 的捕获语义
- → [错误处理](error.md) — `Result` 作为函数返回类型
