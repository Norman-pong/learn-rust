# Cargo 入门

> **一句话**：Cargo 是 Rust 的官方构建系统和包管理器，一个 `Cargo.toml` 同时扮演 `package.json`、`webpack.config.js` 和任务运行器的角色，依赖解析、构建、测试、文档、发布全部统一在一条命令流里。

## 与 JS/TS 的关键差异

| 概念 | Rust / Cargo | TypeScript / npm 生态 |
|------|--------------|----------------------|
| 项目描述文件 | `Cargo.toml` | `package.json` |
| 锁文件 | `Cargo.lock` | `package-lock.json` / `pnpm-lock.yaml` / `yarn.lock` |
| 包仓库 | crates.io | npm registry |
| 依赖版本语义 | 语义版本 `^` / `~` / `>=` 写在 `Cargo.toml` | `package.json` 的 `dependencies` 版本范围 |
| 条件特性 | `[features]` + `default` 显式开关 | `optionalDependencies` / `peerDependencies` 组合 |
| 开发依赖 | `[dev-dependencies]` | `devDependencies` |
| 任务脚本 | `[[bin]]`、`[workspace]`、`build.rs` | `package.json` 的 `scripts` |
| 构建产物 | 原生二进制 / 静态/动态库 | JS bundle（经 webpack、vite 等工具） |

**核心差异**：Cargo 是 Rust 编译链的**一等公民**，不像 npm 那样把构建、打包、测试交给多个工具拼起来。`cargo build` 直接调用 `rustc` 做增量编译，`cargo test` 内建测试运行器，`cargo doc` 生成文档站点，`cargo publish` 推送到 crates.io。对 TS 开发者来说，可以把 Cargo 理解成 **npm + tsc + vite + vitest 的 Rust 原生整合版**。

## 代码对比表

### `Cargo.toml` 结构

```rust
// 这不是 Rust 代码，仅用于展示 TOML 结构；
// 在真实的 Cargo.toml 中不能有 Rust 语法注释。
/*
[package]
name = "rusty-app"
version = "0.1.0"
edition = "2021"
authors = ["you@example.com"]
description = "A Rust TLDR demo"
license = "MIT"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"

[dev-dependencies]
reqwest = "0.12"
*/
```

```typescript
// package.json 等价结构
{
  "name": "ts-app",
  "version": "0.1.0",
  "description": "A TypeScript demo",
  "license": "MIT",
  "dependencies": {
    "fastify": "^4.28",
    "zod": "^3.23"
  },
  "devDependencies": {
    "vitest": "^2.0",
    "typescript": "^5.5"
  }
}
```

### 常用 Cargo 命令

```bash
# 创建新项目（默认 bin）
cargo new my-app
cd my-app

# 构建
cargo build              # 调试版（target/debug）
cargo build --release    # 优化发布版（target/release）

# 运行（默认 bin）
cargo run

# 检查（比 build 快，只跑类型检查不生成产物）
cargo check

# 测试
cargo test

# 文档
cargo doc --open

# 添加依赖
cargo add serde --features derive
cargo add tokio --features full

# 发布到 crates.io
cargo publish
```

```bash
# 对应 TypeScript / pnpm 工作流
pnpm create vite ts-app --template vanilla-ts
pnpm install
pnpm run build
pnpm dev
pnpm test
pnpm add fastify zod
pnpm add -D vitest typescript
pnpm publish
```

### 依赖版本与 features

```rust
// Cargo.toml 片段
/*
[dependencies]
serde = "1.0"                         # 等价于 >=1.0.0, <2.0.0
serde_json = "=1.0.117"               # 精确锁定
regex = ">=1.10, <2.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }

[features]
default = ["std"]
std = []
async = ["tokio"]
*/
```

```typescript
// package.json 等价片段
{
  "dependencies": {
    "fastify": "^4.28.0",
    "zod": "~3.23.0",
    "lodash": "4.17.21"
  }
}

// TypeScript 没有内建 feature 机制，
// 通常通过子路径导出或多包拆分实现：
// import { ... } from "lodash/core";
```

## 容易踩的坑

1. **不加 `--release` 的性能幻觉**——`cargo build` 默认是未优化调试版，运行速度可能慢 10 倍以上；上线前必须用 `cargo build --release`。
2. **`cargo check` 与 `cargo build` 的区别**——`check` 只做类型检查不生成二进制，适合写代码时快速迭代；`build` 才会产出可执行文件。
3. **`Cargo.lock` 的提交策略**——二进制项目（bin）应把 `Cargo.lock` 提交到版本库，保证构建可复现；库项目（lib）通常不提交，让调用方自由解析。
4. **features 默认即生效**——`[features]` 里写了 `default = [...]` 的 feature 会默认启用，要禁用需用 `--no-default-features`。
5. **版本语义 `^` 与 `~` 的微妙差异**——`serde = "1.0"` 等价 `^1.0`（允许 `1.x` 但不到 `2.0`）；`=1.0.117` 才是精确锁定，和 `pnpm-lock.yaml` 的职责不一样。

## 交叉链接

- → [模块与 Crate](module-crate.md) — Cargo 项目的目录结构与 crate 边界
- → [所有权模型](../ownership-lifetimes/ownership.md) — 二进制与库 crate 共同遵守所有权规则
- → [Trait 与泛型](trait-generic.md) — 库 crate 中 trait 的公共接口设计
