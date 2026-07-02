# DEVLOG · 01-tools

> 撞墙记录——症状、错误、修复。每次卡住超过 5 分钟就记一笔。

---

## 2026-07-02 · 项目初始化

### 决策：第一个替代目标待选定

**症状**：Phase 计划要求选定 sunniwell 工作流中的一个脚本作为 CLI 替代目标。

**分析**：
- 当前可选项未明确，需要在实际工作中观察哪些脚本调用频繁或性能不足
- 脚手架代码（`hello-rust`）已就位，可随时替换业务逻辑

**行动**：在实际工作流的下一周中注意记录执行频率高的脚本，选一个耗时最长或用着最不舒服的。

---

## 2026-07-02 · log-grep 实现

### 卡点 1: clap derive 参数冲突

**症状**：`-p` / `-c` / `-v` 三个参数互相独立，但 clap 的 `required = true` 在 `-c` 模式下必须要求 `-p`。

**理解**：clap 的 `#[arg]` 宏中 `required = true` 会导致无该参数时报错。`-c` 和 `-v` 是可选标记，仅 `-p` 为必填。

**修复**：
```rust
#[arg(short, long, required = true)]
pattern: String,  // -p 必填
#[arg(short, long, default_value_t = false)]
count: bool,       // -c 可选
#[arg(short = 'v', long, default_value_t = false)]  
invert: bool,      // -v 可选
```

### 卡点 2: stdin vs 文件读取的 BufRead trait

**症状**：`File::open(path)` 和 `stdin()` 返回不同类型，不能直接赋值给同一个 `Box<dyn BufRead>`。

**理解**：Rust 的 `File` 和 `Stdin` 都实现了 `BufRead` trait，但需显式 boxing。

**修复**：用 `match path { Some(p) => Box::new(BufReader::new(File::open(p)?)), None => Box::new(stdin().lock()) }`。

### 卡点 3: regex crate 的 capture group

**症状**：最初计划支持 `-r` 替换模式，但 regex 的 `replace_all` 语法较复杂且需求场景不明确。

**理解**：premature optimization。命令行工具应先实现最小可用功能。

**修复**：去掉 `-r` 替换功能，保持 `grep + count + invert` 三个核心操作。

---

## 2026-07-02 · reset 脚本开发

### 卡点: 正则匹配 vs 行级 brace 计数

**症状**：Python 基于 regex 的 reset 脚本无法正确处理嵌套大括号（`if {} else {}`、结构体定义等），导致函数体替换后残留代码。

**理解**：Rust 的语法需要 brace-depth 感知。简单的 `fn xxx() { ... }` 匹配无法处理任意嵌套。

**修复**：改用 JavaScript 按行读取 + brace depth 计数器。从最后一个 test 函数往前处理，避免 offset 偏移问题。最终 93/93 测试桩全部正确。

