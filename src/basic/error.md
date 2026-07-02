# 错误处理

> **一句话**：Rust 没有异常——用 `Result<T, E>` 和 `?` 运算符处理可恢复错误，用 `panic!` 处理不可恢复错误；`thiserror` 适合库，`anyhow` 适合应用。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 可恢复错误 | `Result<T, E>` — 强制处理（`#[must_use]`） | `try/catch` 或 `Promise` — 可选，容易忘记 catch |
| 错误传播 | `?` 运算符（自动 `From` 转换） | `throw` / `Promise.reject` |
| 不可恢复错误 | `panic!`（默认 unwind，可配置 abort） | `throw`（语义上是可恢复错误，但可错用） |
| 错误类型 | 枚举（`io::Error`, `ParseIntError` 等） | `Error` 类继承或 `any` |
| 自定义错误 | `thiserror`（derive Error） | 自定义 `Error` class |
| 动态错误 | `anyhow`（应用层快速组合） | `Error` 或第三方 error aggregator |
| null/undefined | 无 null，用 `Option<T>` | `null`/`undefined` |

**核心差异**：Rust 把错误作为返回值类型的一部分，而不是一种控制流副作用。调用 `Result` 返回的函数时，必须显式处理 `Ok`/`Err`，否则编译器会发出 `#[must_use]` 警告。`?` 运算符把这种显式处理变成了语法糖，但本质仍然是类型驱动。TypeScript 的 `try/catch` 是控制流，错误类型容易丢失，调用者可以忽略 catch。

## 代码对比表

### `Result` 基础

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;   // ? 传播错误
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("config.toml") {
        Ok(content) => println!("{content}"),
        Err(e) => eprintln!("错误: {e}"),
    }
}
```

```typescript
import * as fs from "node:fs";

function readFile(path: string): string {
    try {
        return fs.readFileSync(path, "utf-8");
    } catch (e) {
        throw new Error(`Failed to read ${path}: ${String(e)}`);
    }
}

function main() {
    try {
        const content = readFile("config.toml");
        console.log(content);
    } catch (e) {
        console.error(`错误: ${e}`);
    }
}
```

### `?` 运算符与类型转换

```rust
fn complex_operation() -> Result<i32, Box<dyn std::error::Error>> {
    // ? 等价于：
    // match result {
    //     Ok(v) => v,
    //     Err(e) => return Err(e.into()),
    // }

    let config = std::fs::read_to_string("config.toml")?;     // io::Error → Box<dyn Error>
    let value: i32 = config.trim().parse()?;                  // ParseIntError → Box<dyn Error>
    Ok(value * 2)
}

fn main() {
    match complex_operation() {
        Ok(v) => println!("{v}"),
        Err(e) => eprintln!("{e}"),
    }
}
```

```typescript
import { readFileSync } from "node:fs";

function complexOperation(): number {
    const config = readFileSync("config.toml", "utf-8");
    const value = Number.parseInt(config.trim(), 10);
    if (Number.isNaN(value)) {
        throw new Error("parse failed");
    }
    return value * 2;
}

function main() {
    try {
        console.log(complexOperation());
    } catch (e) {
        console.error(e);
    }
}
```

### `thiserror` — 自定义错误类型

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("解析失败: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("配置缺失: {0}")]
    Config(String),
}

fn load() -> Result<i32, AppError> {
    let content = std::fs::read_to_string("config.toml")?; // io::Error → AppError
    let val: i32 = content.trim().parse()?;                 // ParseIntError → AppError
    Ok(val)
}

fn main() {
    match load() {
        Ok(v) => println!("{v}"),
        Err(e) => eprintln!("{e}"),
    }
}
```

```typescript
class AppError extends Error {
    constructor(
        public readonly kind: "io" | "parse" | "config",
        message: string,
        public readonly cause?: unknown,
    ) {
        super(message);
    }
}

import { readFileSync } from "node:fs";

function load(): number {
    try {
        const content = readFileSync("config.toml", "utf-8");
        const val = Number.parseInt(content.trim(), 10);
        if (Number.isNaN(val)) {
            throw new AppError("parse", "解析失败");
        }
        return val;
    } catch (e) {
        if (e instanceof AppError) throw e;
        throw new AppError("io", "IO error", e);
    }
}

function main() {
    try {
        console.log(load());
    } catch (e) {
        console.error(e);
    }
}
```

### `anyhow` — 应用层动态错误

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .context("无法读取配置文件")?;

    // 或者用 bail! 提前返回错误
    // anyhow::bail!("config not found");
}

fn main() -> Result<()> {
    let config = load_config()?;
    println!("{config}");
    Ok(())
}
```

```typescript
function loadConfig(): string {
    try {
        return readFileSync("config.toml", "utf-8");
    } catch (e) {
        throw new Error(`无法读取配置文件: ${e}`);
    }
}

function main() {
    try {
        const config = loadConfig();
        console.log(config);
    } catch (e) {
        console.error(e);
        process.exit(1);
    }
}
```

### `panic!` — 不可恢复错误

```rust
fn main() {
    let v = vec![1, 2, 3];
    // v[99]; // 索引越界 → panic!（不可恢复）

    // 手动 panic
    let n = divide(10, 0);
    println!("{n}");
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除数不能为零！"); // 库代码应返回 Err，而不是 panic
    }
    a / b
}
```

```typescript
function divide(a: number, b: number): number {
    if (b === 0) {
        throw new Error("除数不能为零！"); // 语义上是可恢复错误
    }
    return a / b;
}

function main() {
    const v = [1, 2, 3];
    // v[99]; // undefined，不会 panic

    try {
        console.log(divide(10, 0));
    } catch (e) {
        console.error(e);
    }
}
```

## 错误处理哲学

| 场景 | 推荐 | 理由 |
|------|------|------|
| 库代码 | `thiserror` + 自定义 `enum Error` | 调用者可以精确匹配错误变体 |
| 应用代码 | `anyhow` + `Result<T>` | 错误类型不固定，快速原型 |
| 不可恢复 | `panic!` / `unwrap` / `expect` | 继续运行没有意义 |
| 可恢复 | `Result<T, E>` + `?` | 调用者可以选择恢复策略 |
| 原型阶段 | `unwrap` / `expect` | 快速迭代，后续替换为 `?` |

## 容易踩的坑

1. **忽略 `Result`**——Rust 会 warn（`#[must_use]`），但末尾 `;` 可以吞掉 `Result`，使其不参与返回值。
2. **`Box<dyn Error>` 丢失类型信息**——调用者无法 match 具体错误类型，应用层可用，库代码应避免。
3. **`?` 的类型转换**——`?` 自动调用 `.into()`，需要源错误类型实现 `From` trait 或 `Into` 目标错误。
4. **`main` 函数返回 `Result`**——`fn main() -> Result<(), Box<dyn Error>>` 可以直接返回错误，失败时打印并返回非零退出码。
5. **`unwrap` 在生产代码中**——`unwrap` 等于隐式 panic，除非是测试或确实不可能发生，应改用 `?` 或 `expect("为什么这里不会失败")`。

## 交叉链接

- → [Trait 与泛型](trait-generic.md) — `From` trait 是 `?` 自动转换的基础
- → [控制流](control-flow.md) — `if let` / `match` 处理 Result 和 Option
- → [类型系统](type.md) — `Option<T>` 替代 null/undefined
