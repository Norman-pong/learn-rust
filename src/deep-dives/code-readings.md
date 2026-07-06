# 源码阅读

> **一句话**：Rust 源码是学习语言惯用法、API 设计与性能权衡的最佳教材；以 ripgrep 为例，从入口文件出发，沿着"参数解析 → 工作分发 → 核心搜索 → 输出渲染"的调用链逐步深入，并用"输入/输出/为什么这样设计"三要素记录笔记。

## 与 JS/TS 的关键差异

| 概念 | Rust 源码阅读 | TypeScript / Node.js 源码阅读 |
|------|---------------|-------------------------------|
| 技术栈一致性 | 从应用层到底层（libc/syscall）几乎都是 Rust | 高层 TS/JS，底层依赖 C/C++ 绑定（libuv、V8） |
| 工具链 | `cargo doc --open` 生成并跳转文档 | 依赖 TypeScript 语言服务或手写文档 |
| 类型安全 | 源码即契约，类型签名直接说明可变性/所有权 | 类型签名常被 `any` 或隐式转换弱化 |
| 性能细节 | 内存布局、零拷贝、SIMD 显式可见 | 通常在运行时或底层绑定中隐藏 |
| 调试入口 | `main.rs` / `lib.rs` 清晰，依赖图由 `Cargo.toml` 管理 | 入口可能分散在多个 `index.ts` 或构建产物中 |

**核心差异**：Rust 项目通常可以从 `main.rs` 一直追到操作系统接口，中间没有语言切换；Node.js 想追到底层，则需要同时阅读 TS 源码和 C++ 绑定，门槛更高。

## 代码对比表

### 工作流：打开一个 crate 后怎么做？

```rust
// 1. 先看 Cargo.toml：了解依赖、feature、入口
// [package]
// name = "ripgrep"
// version = "14.1.0"
// [[bin]]
// name = "rg"
// path = "crates/core/main.rs"

// 2. 生成文档并在浏览器中打开
// $ cargo doc --open -p ripgrep

// 3. 从 main.rs 入口开始追踪
fn main() -> ExitCode {
    let args = ArgScheme::parse();
    // 解析参数，构建搜索配置...
    run_search(args)
}
```

```typescript
// TypeScript 项目通常从 package.json 找入口
// "main": "dist/index.js" 或 "bin": "dist/cli.js"

// 阅读顺序：package.json → src/index.ts → 跟随 import 链
async function main(): Promise<void> {
    const args = parseArgs(process.argv.slice(2));
    await runSearch(args);
}

main();
```

### ripgrep 案例：关键文件与函数

```rust
// crates/core/main.rs
// 入口：解析命令行参数，调用 run 函数
fn main() {
    let cmd = app::app().get_matches();
    let result = args::Args::from_clap(&cmd).and_then(args::Args::to_config);
    // ...
}

// crates/core/args.rs
// 把 clap 解析结果转成内部配置 Config
impl Args {
    pub fn to_config(self) -> Result<Config> {
        // 构建 RegexMatcher、Printer、Searcher
    }
}

// crates/searcher/src/searcher/mod.rs
// 搜索器：根据文件类型选择 mmap、line-by-line 或 内存搜索
impl Searcher {
    pub fn search_path<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SError> {
        // 根据文件大小决定是否 mmap，兼顾速度与内存占用
    }
}

// crates/printer/src/printer.rs
// 输出渲染：高亮匹配、行号、上下文
impl Printer {
    fn print_match(&mut self, ...) -> Result<(), io::Error> {
        // 使用 termcolor 控制颜色与格式化
    }
}
```

### tokio 快速浏览：runtime → spawn → task 调度

```rust
// tokio/src/runtime/mod.rs
// Runtime 入口：选择当前线程或多线程调度器

// tokio/src/runtime/scheduler/multi_thread/mod.rs
// 多线程调度器：worker 窃取任务队列

// tokio/src/runtime/task/mod.rs
// Task 结构：包装 Future，管理状态机与唤醒

// tokio/src/task/spawn.rs
// spawn 函数：把 Future 加入队列，返回 JoinHandle

// 阅读建议：先理解 `Runtime::block_on`，再追踪 `spawn` 的调用链
```

```typescript
// Node.js 的 event loop 和任务调度在 libuv 层实现
// 阅读顺序：lib/internal/process/task_queues.js → libuv/src/unix/core.c
// 需要同时理解 JS 与 C++ 代码
```

### 笔记模板

阅读每个函数时记录以下三要素：

```markdown
## 函数名：`search_path`

- 输入：`&mut self`, `path: P`, 内部已配置 matcher/printer
- 输出：`Result<(), SError>`
- 为什么这样设计：
  - 用 `&mut self` 而不是 `&self`，因为搜索器会修改缓冲区状态；
  - 泛型 `P: AsRef<Path>` 让调用者可以传 `&str`、`PathBuf`、`Path` 多种类型；
  - 根据文件大小选择 mmap 或流式读取，是对内存与速度的经典权衡。
```

## 容易踩的坑

1. **一开始就读宏实现**：宏、proc-macro 会大幅增加阅读难度，先理解核心逻辑再回头读宏。
2. **忽略 `Cargo.toml` 的 features**：同一个 crate 在不同 feature 下代码路径差异很大。
3. **不看测试目录**：`tests/` 和 `examples/` 是最直观的 API 用法文档。
4. **在 nightly 或实验性 API 上钻牛角尖**：优先阅读稳定代码路径，避免被内部实现细节带偏。
5. **只读不运行**：对关键路径加 `println!` 或写最小复现，比反复阅读更有效。

## 交叉链接

- → [Crust of Rust 笔记](crust-of-rust-notes.md) — 配合视频源码阅读
- → [参与 Rust 贡献](contributing-to-rust.md) — 读懂 Rust 源码方法论
- → [生命周期进阶](../ownership-lifetimes/lifetime-advanced.md) — 阅读 tokio 源码时遇到的生命周期与 Pin
- → [自引用结构](../ownership-lifetimes/self-referential.md) — tokio 中 Future 自引用的实现
