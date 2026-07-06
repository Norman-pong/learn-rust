# 性能调优

> **一句话**：Rust 的性能调优工具链——`criterion` 做微基准测试告诉你"改完快了多少"，`cargo flamegraph` 做热点分析告诉你"时间花在哪了"，`perf stat` 做硬件计数器告诉你"CPU 在等什么"。三者配合：benchmark 定基线 → flamegraph 找热点 → 硬件计数器解释根因 → 优化代码。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript (Node.js) |
|------|------|---------------------|
| 基准测试 | `criterion` — 统计稳健，自动检测 outlier + 回归 | `benchmark.js` — 统计简单，易受 GC 干扰 |
| 热点分析 | `cargo flamegraph` / `perf record` — 原生栈帧，符号精确 | `node --prof` + `flamebearer` — JS→C++ 边界模糊 |
| 优化手段 | 编译期单态化 + LLVM 优化 + PGO/BOLT | JIT 热点编译 + hidden class 内联缓存 |
| Cache 感知 | `#[repr(C)]` 控制布局 + `size_of` 可观测 | 无——V8 内部决策，开发者不可控 |
| 确定性 | 编译期优化结果可复现（同一版本 rustc） | JIT 受运行时状态影响，两次运行可能不同 |

**核心差异**：Rust 的优化在编译期完成——你改一个 `collect` 为 `fold`、或把 `Vec<T>` 换 `Box<[T]>`，编译后的机器码就不同了，`criterion` 能精确测出差异。JS 的优化在运行时——V8 的 JIT 根据实际调用模式内联函数、转换 hidden class，同一个函数在不同调用点可能走不同优化路径。Rust 的 `criterion` 给你**因果性**；JS 的 benchmark 给你**相关性**。

## 代码对比表

### 基准测试：criterion vs benchmark.js

Rust 用 `criterion` 做微基准测试——它自带统计分析、异常值检测、回归对比：

```rust,ignore
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_collect(c: &mut Criterion) {
    let data: Vec<i32> = (0..10_000).collect();

    c.bench_function("collect to Vec", |b| {
        b.iter(|| {
            let result: Vec<_> = data.iter().map(|x| x * 2).collect();
            black_box(result);
        })
    });
}

criterion_group!(benches, bench_collect);
criterion_main!(benches);
```

```bash
# 运行全部 benchmark
cargo bench

# 输出示例：
# collect to Vec   time: [4.23 µs 4.31 µs 4.38 µs]
#                  change: [-0.5% +0.3% +1.1%] (p = 0.42 > 0.05)
#                  No change in performance detected.
```

> `black_box` 阻止编译器优化掉未使用的结果——不加的话 LLVM 可能发现 `result` 没被读取而直接删掉整个循环。

TypeScript 侧——`benchmark.js` 的 API 简单但缺乏统计分析：

```typescript
import Benchmark from 'benchmark';

const suite = new Benchmark.Suite();
const data = Array.from({ length: 10_000 }, (_, i) => i);

suite
  .add('map to array', () => {
    const result = data.map(x => x * 2);
    // 无法阻止 V8 的 DCE——如果 result 没被用，V8 可能优化掉
  })
  .on('cycle', (event: any) => console.log(String(event.target)))
  .run();
```

**关键差别**：`criterion` 测量的是编译后的机器码时间，不受 GC 干扰；`benchmark.js` 测量的是 V8 JIT + GC 的混合时间，统计方差通常比 Rust 大 10-100 倍。

### 火焰图：找到热点

`cargo flamegraph` 一行命令生成火焰图，栈帧宽度 = CPU 时间占比：

```bash
# 安装
cargo install flamegraph

# 在 release 模式下跑 benchmark 并生成火焰图
cargo flamegraph --bench my_benchmark -- --bench

# 或者对一个 binary
cargo flamegraph --bin my_app
```

输出是 SVG 火焰图——水平轴按时间排列，纵向是调用栈，宽度 = CPU 占用。典型阅读方法：

```text
                main (100%)
         ┌───────┴───────┐
     init (5%)       compute (95%)
                  ┌──────┴──────┐
              parse (60%)   hash (35%)
```

> 火焰图的宽度告诉你"时间花在哪"，高度告诉你"调用有多深"。**宽度才是瓶颈**——一个宽但矮的 `memcpy` 比一个窄而高的递归更值得优化。

Node.js 侧——`node --prof` 生成 v8.log 然后用 `flamebearer` 转换：

```bash
node --prof my_script.js          # 生成 isolate-0x...-v8.log
node --prof-process isolate-*.log | flamebearer  # 转为火焰图
```

Rust 的火焰图比 Node.js 的**更干净**：没有 V8 内部函数（`Builtins_KeyedLoadIC` 等）噪音，栈帧直接对应源码函数名（前提是编译时保留 debug symbol：`debug = true` 即使 release）。

### 硬件计数器：CPU 在等什么

`perf stat`（Linux）/ Instruments（macOS）告诉你 CPU 在做什么——以及更重要的——在等什么：

```bash
# 统计一次 benchmark 运行的硬件事件
perf stat -e cycles,instructions,cache-references,cache-misses,branch-misses \
    cargo bench --bench my_benchmark

# 输出示例：
#     15,234,567,890  cycles
#     12,100,234,567  instructions       # IPC = 0.79 (偏低)
#        234,567,890  cache-references
#         12,345,678  cache-misses        # 5.3% miss rate (偏高)
#          4,567,890  branch-misses       # 1.2% mispredict rate
```

| 指标 | 正常值 | ❌ 警报 | 可能原因 |
|------|--------|--------|---------|
| IPC（instructions per cycle） | 1.5-3.0 | < 1.0 | 大量 cache miss 或分支预测失败 |
| cache miss rate | < 2% | > 5% | struct 过大 / 内存访问模式跳跃 / `Vec<Box<T>>` |
| branch miss rate | < 1% | > 3% | 不可预测的分支（如二分搜索在已排序数据上） |

Node.js 没有直接的硬件计数器 API——`node --prof` 是软件采样，不含 PMU（Performance Monitoring Unit）数据。

## 三种优化模式（before / after）

### 模式 1：`collect::<Vec<_>>` → `fold`

当中间容器只是"收集然后消费"时，`fold` 消除一次分配：

```rust
// Before: 中间 Vec 分配 + 二次遍历
fn sum_squares_collect(v: &[i32]) -> i32 {
    let squares: Vec<i32> = v.iter().map(|x| x * x).collect();
    squares.iter().sum()                       // 额外遍历
}

// After: fold 单次遍历，零额外分配
fn sum_squares_fold(v: &[i32]) -> i32 {
    v.iter().fold(0, |acc, x| acc + x * x)    // 一次遍历
}
```

> `criterion` benchmark 典型结果：10,000 元素 → `fold` 比 `collect` 快 1.5-2×，内存分配量减少 40 KB。

### 模式 2：`Vec<T>` → `Box<[T]>`

如果 Vec 的长度固定不再变化，`Box<[T]>` 省掉 capacity 字段（8 字节）且不会再触 realloc：

```rust
// Before: Vec 保留 capacity，即使不再增长
fn to_owned_slice(v: &[i32]) -> Vec<i32> {
    v.to_vec()          // capacity = v.len(), 不浪费但保留结构
}

// After: Box<[T]> 仅存 ptr + len（16 字节 vs Vec 的 24 字节）
fn to_owned_box(v: &[i32]) -> Box<[i32]> {
    v.to_vec().into_boxed_slice()  // 释放多余 capacity
}
```

> `size_of::<Vec<i32>>()` = 24 字节，`size_of::<Box<[i32]>>()` = 16 字节。对大量小切片（如解析器的 token 数组），差距可以累积到 MB 级。

### 模式 3：`Cow<str>` 延迟分配

当你**通常**不需要修改字符串但**偶尔**需要时，`Cow`（Copy-on-Write）避免不必要的 clone：

```rust
use std::borrow::Cow;

// Before: 总是 clone，即使不需要修改
fn normalize(input: &str) -> String {
    if input.contains('_') {
        input.replace('_', "-")   // 只有有下划线才需要分配
    } else {
        input.to_string()         // 无下划线也 clone 了——浪费
    }
}

// After: Cow 只在需要时分配
fn normalize_cow(input: &str) -> Cow<'_, str> {
    if input.contains('_') {
        Cow::Owned(input.replace('_', "-"))  // 有下划线 → 分配新 String
    } else {
        Cow::Borrowed(input)                  // 无下划线 → 零拷贝
    }
}
```

> 对高频调用（如 HTTP 路由规范化、配置文件 key 统一化），Cow 可以消除 80%+ 的字符串分配。JS/TS 没有等价概念——`toLowerCase()` 总是分配新字符串。

## 容易踩的坑

1. `criterion` 的 `black_box` 是必需的 —— 不加的话 LLVM 经常优化掉基准测试的目标代码，你的 benchmark 测的是"空循环"
2. 火焰图必须在 release 模式下生成 —— debug 模式的栈帧不反映真实性能，但 release 需要 `debug = true` 才有函数名
3. `cargo flamegraph` 需要 `perf`（Linux）或 `dtrace`（macOS）系统权限 —— macOS 上可能需要在 Recovery 模式降低 SIP 保护，或用 Instruments 替代
4. `Cow` 不是免费的 —— 每次匹配 `Cow::Borrowed` vs `Cow::Owned` 有一个分支，只在"通常不需要修改"的场景下有利
5. PGO（Profile-Guided Optimization）和 BOLT 是高级优化 —— 对大多数 codebase 收益 < 10%，在确认常规手段用尽后再考虑

## 交叉链接

- → [内存布局](memory-layout.md) — struct 排布影响 cache miss rate，是性能调优的基础
- → [Miri 入门](miri.md) — Miri 检测 UB，不测性能——不要混淆"正确性工具"和"性能工具"
- 概念层：`Iterator` trait 的惰性设计（`map` / `filter` 只在 `collect` 或 `fold` 时执行）
- 外部参考：[Criterion.rs 用户指南](https://bheisler.github.io/criterion.rs/book/) · [perf wiki](https://perf.wiki.kernel.org/) · [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
