# 错误处理

> **一句话**：Rust 没有异常——用 `Result<T, E>` 和 `?` 运算符处理可恢复错误，用 `panic!` 处理不可恢复错误。这是 Rust 可靠性的基石。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 可恢复错误 | `Result<T, E>` — 强制处理 | `try/catch` — 可选，可以忘记 catch |
| 错误传播 | `?` 运算符（语法糖） | `throw` 或 `Promise.reject` |
| 不可恢复 | `panic!`（默认 unwind，也可 abort） | `throw`（不可 catch 的 Error） |
| 错误类型 | 枚举（`io::Error`, `ParseIntError` 等） | `Error` 类继承 |
| 组合 | `thiserror`（derive Error）或 `anyhow`（动态错误） | 自定义 Error class |
| null/undefined | 无 null，用 `Option<T>` | `null`/`undefined` |

## 代码对比表

### Result 基础

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;   // ? 传播错误
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 使用
match read_file("config.toml") {
    Ok(content) => println!("{content}"),
    Err(e) => eprintln!("错误: {e}"),
}
```

```typescript
// TypeScript — try/catch
function readFile(path: string): string {
    try {
        return fs.readFileSync(path, 'utf-8');
    } catch (e) {
        throw new Error(`Failed to read ${path}: ${e}`);
    }
}
```

### `?` 运算符

```rust
// ? 等价于：
// match result {
//     Ok(v) => v,
//     Err(e) => return Err(e.into()),
// }

fn complex_operation() -> Result<i32, Box<dyn std::error::Error>> {
    let config = read_file("config.toml")?;         // io::Error → Box<dyn Error>
    let value: i32 = config.trim().parse()?;        // ParseIntError → Box<dyn Error>
    Ok(value * 2)
}
```

### thiserror — 自定义错误类型

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),       // #[from] 自动实现 From

    #[error("解析失败: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("配置缺失: {0}")]
    Config(String),
}

fn load() -> Result<i32, AppError> {
    let content = std::fs::read_to_string("config.toml")?;  // io::Error → AppError
    let val: i32 = content.trim().parse()?;                  // ParseIntError → AppError
    Ok(val)
}
```

### anyhow — 动态错误（适合应用层）

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .context("无法读取配置文件")  // 添加上下文信息
}

fn main() -> Result<()> {
    let config = load_config()?;
    println!("{config}");
    Ok(())
}
```

### panic! — 不可恢复错误

```rust
// panic! 用于不应该发生的情况
let v = vec![1, 2, 3];
v[99];  // 索引越界 → panic!（不可恢复）

// 手动 panic
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除数不能为零！");  // 库代码应返回 Err，而不是 panic
    }
    a / b
}
```

## 错误处理哲学

| 场景 | 推荐 | 理由 |
|------|------|------|
| 库代码 | `thiserror` + 自定义 `enum Error` | 调用者可以精确匹配错误 |
| 应用代码 | `anyhow` + `Result<T>` | 错误类型不固定，快速原型 |
| 不可恢复 | `panic!` / `unwrap` / `expect` | 继续运行没有意义 |
| 可恢复 | `Result<T, E>` + `?` | 调用者可以选择恢复策略 |
| 原型阶段 | `unwrap` / `expect` | 快速迭代，后续替换为 `?` |

## 容易踩的坑

1. **忽略 `Result`**——Rust 会 warn（`#[must_use]`），但 `;` 分号可以吞掉
2. **`Box<dyn Error>` 丢失类型信息**——调用者无法匹配具体错误类型
3. **`?` 的类型转换**——`?` 自动调用 `.into()`，需要 `From` trait 实现
4. **`main` 函数的返回值**——`fn main() -> Result<(), Box<dyn Error>>` 可以直接返回错误
5. **`unwrap` 在生产代码中**——`unwrap` 等于隐式 panic，应该用 `?` 或 `expect("why")`

## 交叉链接

- → [Trait 与泛型](trait-generic.md) — `From` trait 是 `?` 自动转换的基础
- → [控制流](control-flow.md) — `if let` / `match` 处理 Result 和 Option
- → [类型系统](type.md) — `Option<T>` 替代 null
