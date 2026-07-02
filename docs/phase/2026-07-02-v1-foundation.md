# Phase · v1.0 基础骨架（6 周）

> 状态：已完成 · 日期：2026-07-02
>
> **设计依据**：调研已归档至 [`docs/reference/sources.md`](../reference/sources.md)
> **上游 PRD**：[learn-rust-notebook](../prd/2026-07-02-learn-rust-notebook.md)（§5 交付范围 → 任务分解）
> **上游 Architecture**：[overview](../architecture/overview.md)（§5 ADR 索引 → 技术约束）
> **评审输入**：已吸收至本文档（P1 内容量修正 → v1.0 缩减为 15-20 篇）
---

## §0 前置条件（边界信号）

在开始任何任务之前，确认以下**放弃/止损信号**已建立：

- [ ] 任何章节 30 分钟写不出来 → 当天停，去 rustlings 练 30 分钟
- [ ] 项目卡 1 小时没动 → 当天停，写 DEVLOG.md 5 分钟，明天继续
- [ ] 连续 2 周没 commit → 在 weekly 标注，强制回到"只练不写"模式

---

## §1 第 1 周 · 基础设施（5-6h）

**交付物**：可运行的 mdbook 空壳 + justfile + 第一份周报
> 溯源：← [PRD §3.1](../prd/2026-07-02-learn-rust-notebook.md)（基建约束）+ [Architecture ADR #1/#8](../architecture/overview.md)（mdbook + justfile）

| # | 任务 | 验收标准 | 预估 |
|---|---|---|---|
| 1.1 | 安装工具链：`cargo install mdbook` `cargo install just` | `mdbook --version` 正常 | 15min |
| 1.2 | `mdbook init ./` 初始化，删 example | `mdbook serve` 看到默认首页 | 15min |
| 1.3 | 写 `book.toml`（playground.editable=true + 中文 title） | `mdbook serve` 渲染正确 | 30min |
| 1.4 | 写 `justfile`（serve / test / clean / weekly 四个命令） | `just serve` / `just test` / `just clean` / `just weekly` 均可执行 | 30min |
| 1.5 | 写 `README.md`（10 行以内） | 他人 30 秒看懂仓库用途 | 15min |
| 1.6 | 添加 LICENSE（MIT） | 文件存在 | 5min |
| 1.7 | 写第一份 weekly：`notes/weekly/2026-W27.md` | 含"本周完成/卡住/下周计划" | 15min |
| 1.8 | 初始化 `notes/_index.md` | 一句描述 + 本文件存在 | 10min |

---

## §2 第 2 周 · rustlings 快进 + 起步所有权（5-6h）

**交付物**：exercises 单 crate 可用 + solutions 就位 + ownership.md 初稿
> 溯源：← [PRD §2.2](../prd/2026-07-02-learn-rust-notebook.md)（练习系统）+ [Architecture ADR #2/#3](../architecture/overview.md)（单 crate + solutions 隔离）+ RESEARCH §2.1（rustlings 分析）

| # | 任务 | 验收标准 | 预估 |
|---|---|---|---|
| 2.1 | `cargo new --lib exercises` 创建练习 crate | `cargo test` 通过（空骨架） | 15min |
| 2.2 | Fork rustlings 00-10 章题目写入 `exercises/src/` | 每题 `#[test] #[ignore]` 格式正确，`cargo test -- --ignored` 全部 FAIL（符合预期） | 1.5h |
| 2.3 | **快进完成**前 11 章节（1.5h 内过） | 每题编译通过，测试 green | 1.5h |
| 2.4 | Fork 章节 11-15（hashmaps/options/error/generics/traits） | 题目就位，`#[ignore]` | 30min |
| 2.5 | 同步答案到 `solutions/`（P0 编译隔离：不含 Cargo.toml，每文件头部 3 行注释） | `cat solutions/variables.rs` 可见完整答案 | 30min |
| 2.6 | 写 `src/ownership-lifetimes/ownership.md` 初稿 | 含 Move 语义 + 所有权规则 + 代码示例 | 1h |
| 2.7 | 写 `exercises/README.md`（章节 ↔ src/ 映射表） | 23 行映射表 + 快进/深做标注 | 15min |
| 2.8 | weekly 更新 | `notes/weekly/2026-W28.md` 存在 | 15min |

---

## §3 第 3 周 · Rust 灵魂 + 基础 TLDR（5-6h）

**交付物**：lifetime-basic + smart-pointer + 4 篇 basic TLDR
> 溯源：← [PRD §2.1](../prd/2026-07-02-learn-rust-notebook.md)（内容系统 · ownership-lifetimes + basic）+ [Architecture ADR #4](../architecture/overview.md)（TLDR 对比表）+ RESEARCH §6.5（写作模板）

| # | 任务 | 验收标准 | 预估 |
|---|---|---|---|
| 3.1 | 写 `src/ownership-lifetimes/lifetime-basic.md` | 周期标注 + 省略规则完整，含代码示例 | 1.5h |
| 3.2 | 写 `src/ownership-lifetimes/reference-borrow.md` | `&` / `&mut` / NLL 讲解 | 30min |
| 3.3 | 写 `src/ownership-lifetimes/smart-pointer.md` | Box/Rc/Arc/RefCell/Cell 概念 + 对比 | 45min |
| 3.4 | 写 `src/basic/variable.md`（按 §6.5 模板，S 档） | 一句话 + 差异 + 对比表 + 踩坑 + 链接 | 30min |
| 3.5 | 写 `src/basic/type.md`（M 档） | &str/String/str + HashMap + derive 五件套 | 45min |
| 3.6 | 写 `src/basic/control-flow.md`（S 档） | if/loop/match/while let/let-else | 30min |
| 3.7 | 写 `src/basic/function.md`（M 档） | 函数 + 高阶函数（不含闭包） | 30min |
| 3.8 | rustlings 16_lifetimes / 19_smart_pointers 深做 | 每题通过，理解每个错误原因 | 45min |
| 3.9 | weekly 更新 | `notes/weekly/2026-W29.md` | 15min |

---

## §4 第 4 周 · 并发 + 链表深挖 + 编译错误（5-6h）

**交付物**：3 篇并发 + 2 章 too-many-lists 翻译 + 3-5 个 lifetime 编译错误案例
> 溯源：← [PRD §2.1](../prd/2026-07-02-learn-rust-notebook.md)（concurrency + deep-dives + compiler-pitfalls）+ [Architecture §6](../architecture/overview.md)（内容去重：概念 vs 错误案例边界）+ RESEARCH §2.3（too-many-lists 教学法）

| # | 任务 | 验收标准 | 预估 |
|---|---|---|---|
| 4.1 | 写 `src/concurrency/thread.md` | std::thread + spawn + join + move 闭包 | 45min |
| 4.2 | 写 `src/concurrency/async-await.md` | async/await + Future 模型基础 | 1h |
| 4.3 | 写 `src/concurrency/tokio.md` | runtime / spawn / 共享状态初探 | 45min |
| 4.4 | 内嵌 too-many-lists first 章翻译 | 完整代码块 + 中文讲解，放在 `src/deep-dives/linked-lists/first.md` | 1h |
| 4.5 | 内嵌 too-many-lists second 章翻译 | 同上，`second.md` | 45min |
| 4.6 | `src/compiler-pitfalls/lifetime-theme/` 写 3-5 个案例 | 每个 case 含：错码→报错→解释→修复，4 段格式 | 1h |
| 4.7 | rustlings 20_threads 深做 | 每题通过 | 30min |
| 4.8 | weekly 更新（含首次月度回顾：卡在哪、调整什么） | `notes/weekly/2026-W30.md` | 15min |

---

## §5 第 5 周 · 实战 P0 + deep-dive 入口（5-6h）

**交付物**：第一个 CLI crate 能用 + 3 个 P1 deep-dive 入口
> 溯源：← [PRD §2.3](../prd/2026-07-02-learn-rust-notebook.md)（项目系统 P0）+ [PRD §2.1](../prd/2026-07-02-learn-rust-notebook.md)（deep-dives 3 入口）+ [Architecture §3.4](../architecture/overview.md)（项目层设计）

| # | 任务 | 验收标准 | 预估 |
|---|---|---|---|
| 5.1 | 选定替代目标：从 sunniwell 工作流挑一个脚本 | 明确"这个脚本做什么、Rust 版怎么替代" | 30min |
| 5.2 | 创建 `projects/01-tools/tools/<name>/` crate | `cargo new` 成功，Cargo.toml 配好 | 15min |
| 5.3 | 实现 CLI 核心逻辑 | 功能等价于原脚本，`cargo run` 可执行 | 2h |
| 5.4 | 写 `projects/01-tools/DEVLOG.md` 第一案 | 第一个卡点已记录（症状/错误/修复） | 15min |
| 5.5 | 写 `src/deep-dives/contributing-to-rust.md` | 含 rust-lang/rust good first issue 挑选指南 + rustc-dev-guide 入口 | 30min |
| 5.6 | 看 Crust of Rust 第一集（Held by a Thread），写 `src/deep-dives/crust-of-rust-notes.md` 第一段 | ≥ 200 字笔记 | 1h |
| 5.7 | 初始化 `notes/code-readings/_index.md` | 一句描述 + 链接 | 10min |
| 5.8 | weekly 更新 | `notes/weekly/2026-W31.md` | 15min |

---

## §6 第 6 周 · 实战 P0 收尾 + P1 评估 + 收尾回顾（5-6h）

**交付物**：第一个 CLI 真用上 + P1 方向确定 + 6 周回顾
> 溯源：← [PRD §5.1](../prd/2026-07-02-learn-rust-notebook.md)（v1.0 必交付）+ [PRD §2.3](../prd/2026-07-02-learn-rust-notebook.md)（P1 评估）+ [Architecture §3.4](../architecture/overview.md)（02-component 决策框架）

| # | 任务 | 验收标准 | 预估 |
|---|---|---|---|
| 6.1 | 第一个 CLI 工具在生产环境用起来 | 替代原脚本 ≥ 1 次实际调用 | 1h |
| 6.2 | 写 `projects/01-tools/README.md` | 工具列表 + 替代目标 + 使用说明 | 30min |
| 6.3 | 评估现 server 语言，用决策框架确定 `02-component/` 方向 | 输出一行结论（napi / PyO3 / microservice）写入 DEVLOG | 30min |
| 6.4 | rustlings 22_clippy / 23_conversions 深做 | 每题通过 | 45min |
| 6.5 | 写 `src/basic/trait-generic.md`（L 档） | RPIT/RPITIT + async fn in trait，多对比表 | 1h |
| 6.6 | 写 `src/basic/error.md`（L 档） | Result/?/thiserror/anyhow + 错误处理哲学 | 1h |
| 6.7 | **6 周末回顾**：写 `notes/weekly/2026-W32.md` | 含：学到什么 / 卡住什么 / 未来 6 周计划 | 30min |
| 6.8 | 全局 `cargo test` 验证 + mdbook 链接检查 | 练习全绿，mdbook 无死链 | 15min |

---

## §7 v1.0 验收清单

完成以下全部条目即 v1.0 交付：

- [ ] `mdbook serve` 可浏览全部已写内容（≥ 15 篇内容单元）
- [ ] `cargo test -- --ignored` 全部练习题通过（23 章，前 10 快进 + 后 12 深做）
- [ ] `just test` 一键跑通
- [ ] `projects/01-tools/` 至少 1 个 CLI crate 在生产中替代了原脚本
- [ ] `notes/weekly/` 含 W27-W32 共 6 篇周报
- [ ] 每个项目目录含 `DEVLOG.md`（至少 01-tools 有内容）
- [ ] `solutions/` 不含 Cargo.toml，`cargo build` 不会误编译
- [ ] `notes/_index.md` 链接完整
- [ ] `src/SUMMARY.md` 章节导航完整
- [ ] 无死链（手动抽查）

---

## §8 v1.1+ 预留任务（v1.0 完成后按需启动）

| 类别 | 任务 | 触发条件 |
|---|---|---|
| basic/ | 写剩余 7 篇（pattern-matching / closure / struct-enum / trait-generic / module-crate / error / cargo） | 基础章用到对应概念时 |
| ownership-lifetimes/ | 写 self-referential / lifetime-advanced | async 实战撞到 Pin/HRTB 时 |
| concurrency/ | 写 message-passing / sync-primitives / send-sync | 项目需要 channel/Mutex 时 |
| deep-dives/ | 翻译 too-many-lists 剩余 4 章 + 写 3 篇数据结构内部 | 链表章做完 first/second 后 |
| compiler-pitfalls/ | 补完 borrow / trait-bound / move / type-inference 四主题 | 遇到对应编译错误时 |
| profiling/ | 写 memory-layout / perf-tuning / miri | 真的遇到性能问题时 |
| projects/01-tools/ | 第二个、第三个 CLI 子项目 | 第一个 CLI 稳定后用 |
| projects/02-component/ | 实质开发 Rust 组件 | server 语言确定后 |
| notes/ | code-readings/ 持续 + patterns/ 边写边沉淀 + release-notes/ 每周摘要 | 持续 |
