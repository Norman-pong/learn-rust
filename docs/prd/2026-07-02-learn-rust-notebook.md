# PRD · Norman's Rust 笔记本

> 状态：**已归档** · 版本：v1.0 → v1.1+ 全部完成（2026-07-13）· 作者：Norman · 日期：2026-07-02
>
> **§5.2 v1.1+ 交付对照（2026-07-13 核实）**：
> - `basic/` 剩余 7 篇 → ✅ 全部完成（11/11）
> - `ownership-lifetimes/` 剩余 2 篇 → ✅ 全部完成（6/6）
> - `concurrency/` 剩余 3 篇 → ✅ 全部完成（6/6）
> - `deep-dives/` 剩余 4 章链表 → ✅ 全部完成（5 章，合并去重后）+ crust-of-rust 3 集
> - `compiler-pitfalls/` 剩余 4 主题 → ⬜ 已完成 2 主题（lifetime + move），剩余按需追加
> - `profiling/` 全部 3 章 → ✅ 全部完成
> - `projects/01-tools/` → ✅ log-grep 已实现
>
> **结论**：v1.1+ 计划范围内的核心内容已全部交付（36 篇内容单元），PRD 归档。后续 `compiler-pitfalls` 剩余主题、`crust-of-rust` 更多剧集按"按需触发"策略追加，不再设版本号目标。
>
> **设计依据**：调研与目录设计已归档至 [`docs/reference/sources.md`](../reference/sources.md)
> **下游文档**：[Architecture](../architecture/overview.md)（架构实现）· [Phase](../phase/2026-07-02-v1-foundation.md)（执行计划）
>
---

## §0 目标声明

> 溯源：← 核心边界再设定 · 8 条设计原则

构建一个 **面向 Norman 个人的 Rust 学习笔记本仓库**，以 mdbook 为主体、rustlings 风格练习为辅助、真实工作流项目为输出，用最简基建（justfile + mdbook + cargo test）在 6 周内交付 v1.0 可用的学习-练习-实战闭环。
> 本节的 8 条设计原则来自调研结论；评审意见已吸收。

**一句话**：不是公共教程，不是社区项目，是 Norman 自己学 Rust 的第二大脑。

---

## §1 背景与动机

### 1.1 用户画像

| 属性 | 值 |
|---|---|
| 身份 | Norman，全栈工程师 |
| 已有语言 | JS/TS、Python、C++ |
| Rust 水平 | 入门（读过 The Book 部分章节，写过少量代码） |
| 学习目标 | 系统掌握 Rust 核心概念（所有权/生命周期/并发/async），能在日常工作流中用 Rust 替代脚本、写高性能组件 |
| 可用时间 | 每周 5-6 小时 |

### 1.2 痛点

- 读教程"看完不会写"——缺少从概念到实战的桥梁
- 现有中文资源（sunface/rust-course）太庞大（170 章），个人时间无法覆盖
- rustlings 全部 94 题对已有编程基础的人前 10 章无挑战性
- 缺少个人进度的可视化锚点，容易中断

### 1.3 差异化定位

与现有开源项目的关系（非竞争，互补使用）：

| 项目 | 本仓库关系 |
|---|---|
| rust-lang/rustlings | 取其题库格式，自建单 crate 替代自研 CLI |
| sunface/rust-course | 取其编译错误章思路 + too-many-lists 内嵌做法 |
| too-many-lists | 取其核心 6 章内嵌为 deep-dives |
| comprehensive-rust | 取其 mdbook + 练习结合模式，拒绝其重基建 |
| The Book 中文版 | 不重复系统讲解，基础章用 TLDR 对比表 |

---

## §2 功能需求

### §2.1 内容系统（mdbook）

| 模块 | 章节数 | 粒度 | 说明 |
|---|---|---|---|
| `basic/` | 11 章 | S/M/L 三档（1-4 页/章） | 基础语法 TLDR，Rust vs JS/TS 对比表风格 |
| `ownership-lifetimes/` | 6 章 | 深挖 | ★ 核心：所有权 + 引用/借用 + 生命周期 + 智能指针 |
| `concurrency/` | 6 章 | 中深 | 线程 + channel + 同步原语 + Send/Sync + async/tokio |
| `deep-dives/` | 7 个专题 | 按需 | 含 3 个 P1 入口（contributing / crust-of-rust / code-readings）+ too-many-lists 内嵌 |
| `compiler-pitfalls/` | 15-20 案例 | 5 主题聚类 | ★ 杀手锏：编译错误实战案例（每个 case = 错码→报错→解释→修复） |
| `profiling/` | 3 章 | 按需 | 内存布局 + 性能调优 + Miri |

**每篇内容格式约定**（§6.5 模板）：
1. 一句话定位
2. 与 JS/TS/Go 关键差异（200 字）
3. 代码对比表（Rust vs TypeScript）
4. 容易踩的坑（5 行清单）
5. 交叉链接

### §2.2 练习系统（exercises/）

| 需求 | 实现 |
|---|---|
| 题库来源 | rustlings 原版 23 章题目 fork |
| 组织形式 | 单 crate（`exercises/Cargo.toml`），非多 sub-crate |
| 运行方式 | 每题 `#[test] #[ignore]`，`cargo test -- --ignored` 一行跑 |
| 进度策略 | 前 10 章快进（1h 内过），后 12 章深做 |
| 答案 | `solutions/` 只读注释文件夹，不含 Cargo.toml |

### §2.3 项目系统（projects/）

| 项目 | 优先级 | 形态 | 触发条件 |
|---|---|---|---|
| `01-tools/` | **P0 必做** | 多个独立 CLI crate，替代 sunniwell 工作流脚本 | 第一周即启动 |
| `02-component/` | P1 价值高 | Rust 组件服务现 server（napi/PyO3/microservice） | server 语言确定后 |
| `03-big/` | P2 按需 | wasm / axum / ripgrep-style 大项目 | 前两者完成后 |

每个项目必含 `DEVLOG.md`（撞墙 5 分钟记录"症状/错误/修复"）。

### §2.4 进度可视化

| 机制 | 位置 | 频率 |
|---|---|---|
| 周报 | `notes/weekly/YYYY-WNN.md` | 每周日 15 分钟 |
| 项目日志 | `projects/<n>/DEVLOG.md` | 每次撞墙时 |
| commit 约定 | `src:` / `exercises:` / `projects:` / `notes:` 前缀 | 每次提交 |

### §2.5 自由笔记区（notes/）

无强制结构，仅 `_index.md` 必填维护。子区：周报、源码阅读、设计模式、惯用法抄录、陷阱笔记、性能笔记、API 设计、版本发布记录。

---

## §3 非功能需求

### §3.1 基建约束

| 约束 | 决策 |
|---|---|
| 构建工具 | `justfile`（4 命令：serve / test / clean / weekly） |
| 内容引擎 | mdbook（默认主题，`playground.editable = true`） |
| 练习引擎 | 单 crate + `#[test] #[ignore]`，不用自研 CLI |
| CI | **不做**（本地 `cargo test` 即可） |
| 协作套件 | **不做**（CONTRIBUTING / ISSUE_TEMPLATE / CODE_OF_CONDUCT / dependabot 全免） |
| License | MIT |
| 多语言 | 不做（仅中文，面向自己） |

### §3.2 内容约束

| 约束 | 说明 |
|---|---|
| 写作风格 | TLDR 简洁（1-4 页），技术词命名章节，不玩双关 |
| 内容去重 | 编译错误→`compiler-pitfalls/`，概念→`ownership-lifetimes/`/`concurrency/`，实现→`deep-dives/`，运行时陷阱→`notes/gotchas.md` |
| solutions 隔离 | `solutions/` 不含 `Cargo.toml`，不参与编译，每文件头部固定 3 行注释 |

### §3.3 可持续性约束

| 约束 | 说明 |
|---|---|
| 放弃信号 | 章节 30 分钟写不出来→停，去 rustlings 练 30 分钟 |
| 卡住信号 | 项目卡 1 小时→停，写 DEVLOG 5 分钟 |
| 断更信号 | 连续 2 周没 commit→在 weekly 标注，强制回到"只练不写" |
| commit 节奏 | 频率 > 完美度 |

---

## §4 非目标（v1.0 明确不做）

- ~~面向公众读者~~（无协作套件、无 SEO、无多语言、无推广）
- ~~完整覆盖 Rust 所有知识点~~（基础章用 TLDR 对比表，不系统讲解）
- ~~自研 CLI 工具~~（用 `cargo test -- --ignored` 替代 rustlings watch）
- ~~mdbook 自研插件~~（不用 comprehensive-rust 的 exerciser/course 插件）
- ~~webdriver 集成测试~~（个人仓库不需要）
- ~~CI/CD pipeline~~（本地 `cargo test` 足够）
- ~~多语言 / i18n~~
- ~~在线练习站~~（不给 public 用）

---

## §5 交付范围

### §5.1 v1.0（6 周内必交付）

| 类别 | 数量 |
|---|---|
| `basic/` 基础 TLDR | 4 篇核心（variable / type / control-flow / function） |
| `ownership-lifetimes/` | 4 篇核心（ownership / lifetime-basic / reference-borrow / smart-pointer） |
| `concurrency/` | 3 篇核心（thread / async-await / tokio） |
| `deep-dives/` | 3 个 P1 入口 |
| `compiler-pitfalls/` | lifetime-theme 3-5 个案例 |
| `exercises/` | 23 章全量（前 10 快进 + 后 12 深做） |
| `projects/01-tools/` | 第一个 CLI crate 真用上 |
| `notes/weekly/` | W27-W32 共 6 篇 |
| 总量 | **~15-20 篇内容单元** |

### §5.2 v1.1+（v1.0 之后按需追加）

`basic/` 剩余 7 篇、`ownership-lifetimes/` 剩余 2 篇、`concurrency/` 剩余 3 篇、`deep-dives/` 剩余 4 章链表翻译 + 3 篇数据结构内部、`compiler-pitfalls/` 剩余 4 主题、`profiling/` 全部 3 章、`projects/01-tools/` 第二第三个 CLI、`projects/02-component/` 实质开发。

---

## §6 风险与缓解

| 风险 | 概率 | 缓解 |
|---|---|---|
| 每周 5-6h 不够写完计划内容 | 中 | v1.0 只交付 15-20 篇（非全部 46-51 篇），其余标注 `TODO: v1.1` |
| 中断超过 2 周 | 中 | weekly 机制 + 放弃信号兜底 |
| solutions 意外参与编译 | 低 | `solutions/` 不含 `Cargo.toml`，物理隔离 |
| 内容重复（同一概念在 3 个地方） | 中 | 内容去重策略（§3.2）在写作时强制执行 |
| project/02-component 方向不明 | 高 | 仅评估阶段，不实质开发；决策框架已预埋 |

---

## §7 引用

- 设计依据与外部资料：[`docs/reference/sources.md`](../reference/sources.md)（5 核心项目 + 3 补充 + 9 竞品 + 工具 + 外部文章）
- 评审意见：已吸收至本文档和 Architecture（P0 solutions 隔离 · P1 内容去重 · P1 内容量修正）
