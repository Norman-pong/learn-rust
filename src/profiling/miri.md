# Miri 入门

> **一句话**：Miri 是 Rust 的未定义行为（UB）检测器——在 MIR（中级中间表示）层面解释执行代码，捕获编译器无法静态检查的运行时 UB：悬垂指针、数据竞争、未初始化读取等。`cargo miri test` 一行命令即可运行。

## 与 JS/TS 的关键差异

| 概念 | Rust + Miri | TypeScript (Node.js) |
|------|-------------|---------------------|
| UB 检测 | Miri 在 MIR 层逐条解释，捕获 use-after-free / data race / invalid pointer | 不存在 UB 概念——JS 引擎保证内存安全（GC + 无指针） |
| 安装 | `rustup +nightly component add miri` | `npm i -D` 即可 |
| 运行方式 | `cargo miri test` / `cargo miri run` | Node.js 内置 `--inspect` + Chrome DevTools |
| 覆盖范围 | safe + unsafe 代码均检测，但仅限被执行的路径 | 不适用（无 unsafe 概念） |
| 性能 | 解释执行，比本机编译慢 10-100× | Node.js JIT 全速 |

**核心差异**：JS/TS 用垃圾回收和运行时类型检查保证内存安全，不存在"编译通过但运行 UB"的场景。Rust 的 `unsafe` 块允许手动管理内存，Miri 填补了编译器静态检查与运行时 crash 之间的空隙——在测试阶段提前发现 UB，而不是在生产环境 segfault。

## 代码对比表

### 案例 1：use-after-free（悬垂指针写入）

```rust,ignore
fn main() {
    let mut data = vec![1, 2, 3];
    let ptr = data.as_mut_ptr();   // 指向 Vec 堆内存的裸指针
    drop(data);                     // Vec 被释放
    unsafe {
        *ptr = 42;                 // UB: 写入已释放的内存
    }
}
```

用 `cargo miri run` 运行会得到：

```text
error: Undefined Behavior: Memory was deallocated but a pointer
       derived from it is still live
  --> src/main.rs:6:9
   |
6  |         *ptr = 42;
   |         ^^^^^^^^^ Memory was deallocated... (use-after-free)
```

对比 TypeScript——JS 引擎的 GC 保证只有在**没有任何引用**时才释放内存，所以不存在"手动 drop 后继续用指针"的场景。但 Rust 给了你 `unsafe` 的枪，Miri 是靶场的教官。

### 案例 2：data race（多线程写同一内存，无同步）

```rust,ignore
use std::thread;

fn main() {
    let mut data = 0;
    let ptr = &raw mut data;  // 获取裸指针

    thread::scope(|s| {
        s.spawn(|| unsafe { *ptr = 1; });  // 线程 A 写
        s.spawn(|| unsafe { *ptr = 2; });  // 线程 B 写——无同步，data race
    });
}
```

```text
error: Undefined Behavior: Data race detected
  --> src/main.rs:8:30
   |
8  |         s.spawn(|| unsafe { *ptr = 2; });
   |                              ^^^^^^^^ Data race detected
```

Rust 的安全子集（safe Rust）在**编译期**通过 `Send`/`Sync` trait 阻止 data race。Miri 则是在 `unsafe` 里做**运行时**检测——即使你绕过了编译器的 trait 检查，Miri 也能在测试阶段捕获。

在 Node.js 中，Worker Threads 通过 `SharedArrayBuffer` + `Atomics` 共享内存，JS 引擎不做 data race 检测（Atomics 保证原子性但不保证无竞争），出问题时是静默的逻辑错误，没有 Miri 这样的工具。

### 案例 3：uninitialized read（读未初始化内存）

```rust,ignore
fn main() {
    let x: i32;
    unsafe {
        // 编译器允许裸指针操作跳过初始化检查
        let ptr = &raw const x;
        println!("{}", *ptr);  // UB: x 未初始化，读到的值是未定义的
    }
    x = 42;  // 编译器报错：赋值给已移动的值（Miri 已经先捕获了 UB）
}
```

```text
error: Undefined Behavior: type validation failed: encountered uninitialized bytes
  --> src/main.rs:6:23
   |
6  |         println!("{}", *ptr);
   |                        ^^^^ uninitialized bytes
```

JS/TS 中变量默认初始化为 `undefined`，不存在"未初始化"状态。这是 Rust 的性能取舍：不为每个栈变量做零初始化开销，而是由编译器跟踪初始化状态——`unsafe` 绕过了这个跟踪，Miri 负责兜底。

## 编译器 vs Miri：能/不能检测什么

| UB 类型 | `rustc` 编译期 | `cargo miri` 运行时 | 备注 |
|---------|:-----------:|:---------------:|------|
| use-after-free | ❌ | ✅ | 编译器不追踪裸指针生命周期 |
| data race | ❌（safe Rust 编译期阻止，unsafe 不管） | ✅ | Miri 通过向量时钟检测 |
| 未初始化读 | ❌（safe Rust 编译期阻止，unsafe 不管） | ✅ | |
| null pointer deref | ❌ | ✅ | |
| 对齐违规 | ❌ | ✅ | `*(p as *const u64)` 在不对齐地址 |
| 越界访问 | ❌（safe Rust 有 bound check panic，unsafe 不检查） | ✅ | |
| **无效枚举值** | ❌ | ✅ | 如把 `3u8` transmute 成 `bool` |
| **整数溢出（release 模式）** | ❌ | ✅ | debug 有 panic，release wrap |
| **死锁** | ❌ | ❌ | Miri 不检测死锁 |

> Miri **不能**检测：死锁、FFI 调用内部的 UB、SIMD 指令、文件系统操作、网络 I/O。`cargo miri` 只执行**被测试覆盖**的代码路径。

## 容易踩的坑

1. Miri 需要 nightly 工具链——`rustup +nightly component add miri`，但你的项目代码不需要 nightly，只需跑 Miri 时用 `+nightly`
2. `cargo miri test` 而非 `cargo miri run`——Miri 把自己注入到 `#[test]` 函数里解释执行，裸 `cargo miri run` 只跑 main
3. Miri 解释执行极慢（比本机慢 10-100×），不要在大型 benchmark 上跑——只跑涉及 unsafe 的测试
4. Miri 不模拟系统调用——用到 `std::fs` 或网络操作的测试会返回 `unsupported operation`，需要 `#[cfg_attr(miri, ignore)]` 跳过
5. Miri 不能检测**未被覆盖的代码路径**——如果你的 unsafe 代码在某个分支里但测试没跑那个分支，Miri 也看不到 UB

## 交叉链接

- → [内存布局](memory-layout.md) — `size_of`/`align_of`、struct 排布、enum niche 优化
- → [性能调优](perf-tuning.md) — `criterion` benchmark + flamegraph（不同于 Miri 的 UB 检测，这些是 wall-clock profiling）
- 概念层：Rust 所有权模型保证 safe Rust 无需 Miri，Miri 只在 `unsafe` 里有意义
- 外部参考：[Miri 官方文档](https://github.com/rust-lang/miri) · [Rustonomicon: 未定义行为列表](https://doc.rust-lang.org/nomicon/what-unsafe-does.html)
