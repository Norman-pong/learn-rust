# Reference · 参考文献与资料索引

> 本文件汇总 `learn-rust` 仓库设计中引用的所有外部资料。
> 调研依据：5 项目调研 + v5 设计建议（原始调研报告已归档，内容完整提取至本文档体系）· 2026-07-02

---

## 1. 核心调研项目（5 个）

### 1.1 rust-lang/rustlings

| 属性 | 值 |
|---|---|
| URL | https://github.com/rust-lang/rustlings |
| Stars | 63.4k（截至 2026-07-02） |
| 定位 | 交互式小练习（CLI 工具） |
| License | MIT |

**教学法**：编译器错误驱动 + `I AM NOT DONE` 标记 + 编号进度（90+ 题）。`rustlings watch` 文件一改就触发 `cargo test`。

**我们借鉴了什么**：
- 题库格式（23 章编号 + `#[test] #[ignore]`）→ `exercises/` 目录
- `solutions/` 只读答案镜像设计 → `solutions/` 编译隔离方案
- `I AM NOT DONE` 标记的进度感 → 练习文件标记格式

**被引用位置**：RESEARCH §2.1 · PRD §1.3 · Architecture ADR #2 · Phase §2

---

### 1.2 rust-lang/rust-by-example

| 属性 | 值 |
|---|---|
| URL | https://github.com/rust-lang/rust-by-example |
| Stars | 8k（截至 2026-07-02） |
| 定位 | 例子驱动的官方教程（mdbook） |
| License | MIT / Apache 2.0 双许可 |

**教学法**：每个概念一个最小可运行例子，mdbook + playground run 按钮。i18n 用 mdbook-gettext。

**我们借鉴了什么**：
- mdbook 作为内容引擎（默认主题 + `playground.editable = true`）
- 例子即文档的风格（每个 `.md` 独立、完整、可运行）

**被引用位置**：RESEARCH §2.2 · Architecture §2（技术选型对比）

---

### 1.3 rust-unofficial/too-many-lists

| 属性 | 值 |
|---|---|
| URL | https://github.com/rust-unofficial/too-many-lists |
| Stars | 3.6k（截至 2026-07-02） |
| 定位 | 单一主题深度探索（mdbook + 真 cargo crate） |
| License | MIT |

**教学法**：选"链表"一个题目，故意写错 6 种实现，每次演化揭示一个 Rust 核心概念。先写错 → 编译器报 → 读错误 → 讲概念 → 改对。

**我们借鉴了什么**：
- 核心 6 章内嵌为 `src/deep-dives/linked-lists/`
- 错误驱动的诚实教学法（"先错后改"）
- 每章末尾 `-final.md` 完整代码快照

**被引用位置**：RESEARCH §2.3 · §4（教学哲学）· PRD §1.3 · Phase §4（第 4 周翻译）

---

### 1.4 sunface/rust-course

| 属性 | 值 |
|---|---|
| URL | https://github.com/sunface/rust-course · https://course.rs |
| Stars | 30.6k（截至 2026-07-02） |
| 定位 | 中文 Rust 学习圣经（mdbook 深度系统化，170+ 章 / 110 万字） |
| License | 自定义（No License，可自由 fork 读但禁止私下修改后包装分发） |

**教学法**：重新消化 The Book / Nomicon / Async Book / Cargo Book，加例子、图解、练习。讲故事 + 吐槽 + 价值观。

**我们借鉴了什么**：
- `compiler/` 杀手锏章思路 → `src/compiler-pitfalls/`（分类整理编译错误）
- too-many-lists 内嵌做法 → `deep-dives/linked-lists/`
- rustlings-zh fork 内嵌做法 → 我们自建 exercises/ 替代
- 基础章节 TLDR 模板 → PRD §2.1 的 5 段格式模板
- 8 条设计启示（RESEARCH §2.4.1）→ 写入 PRD §0 设计原则

**被引用位置**：RESEARCH §2.4 · §2.4.1 · PRD §1.3 · Architecture §2 · Phase §3-4

---

### 1.5 google/comprehensive-rust

| 属性 | 值 |
|---|---|
| URL | https://github.com/google/comprehensive-rust |
| Stars | 33.2k（截至 2026-07-02） |
| 定位 | Google Android 团队内部培训课程（mdbook + 自研插件） |
| License | Apache 2.0 + CC-BY 4.0 |

**教学法**：课堂用，mdbook + speaker notes + 自研 `mdbook-exerciser`（文档内代码自动提取测试）+ `mdbook-course`（演讲者/学生/打印三模式）。

**我们借鉴了什么**：
- mdbook + 练习结合的模式 → 确认技术方向
- 明确拒绝其重基建（自研插件 / xtask / webdriver CI / Bazel）

**被引用位置**：RESEARCH §2.5 · Architecture §2（拒绝理由）· Architecture ADR #5

---

## 2. 补充参考（3 个）

| # | 项目 | URL | 定位 | 我们如何使用 |
|---|---|---|---|---|
| 1 | LukeMathWalker/zero-to-production | https://github.com/LukeMathWalker/zero-to-production | 单项目 6 章演化型教程（分支做快照） | 项目驱动型教学法参考（模式分类 D 型）；未被直接采用但确认了"项目驱动"方向 |
| 2 | ctjhoa/rust-learning | https://github.com/ctjhoa/rust-learning | 链接型 awesome list（课程地图） | 路线图参考（模式分类 E 型）；未被直接采用 |
| 3 | exercism/rust | https://exercism.org/tracks/rust | mentor 模式 + 测试驱动练习（外部平台） | 题库补料来源（RESEARCH §8）；作为 `exercises/` 后续扩展选项 |

**被引用位置**：RESEARCH §1（补充参考）· §3（模式分类）

---

## 3. 竞品与生态参考（9 个，来自 RESEARCH §8）

| # | 参考 | URL | 我们的应用点 | 关联 SDD 文档 |
|---|---|---|---|---|
| 1 | rust-lang-cn/book-cn | https://github.com/rust-lang-cn/book-cn | 官方 The Book 中文翻译。**不重叠**：book-cn 是系统入门，本仓库基础章是 TLDR 对比表 | PRD §1.3 |
| 2 | rust-lang/rust `tests/ui/` | https://github.com/rust-lang/rust/tree/master/tests/ui | `compiler-pitfalls/` 的现成案例库（几百个真实编译错误） | Architecture §3.2 |
| 3 | BurntSushi/ripgrep | https://github.com/BurntSushi/ripgrep | "生产级代码教 Rust"范本，`notes/code-readings/` 第一案 | Phase §5 |
| 4 | tokio-rs/tokio `examples/` | https://github.com/tokio-rs/tokio/tree/master/examples | 并发实战分层参考（spawned task / shared state / messaging） | Architecture §2 |
| 5 | rust-unofficial/awesome-rust | https://github.com/rust-unofficial/awesome-rust | `usecases/` 种子清单 | — |
| 6 | mre/idiomatic-rust | https://github.com/mre/idiomatic-rust | `notes/idiomatic-rust.md` 抄录来源 | Phase §8 |
| 7 | rust-lang-nursery/rust-cookbook | https://github.com/rust-lang-nursery/rust-cookbook | 官方 cookbook（偏配方），与 `notes/patterns/` 定位互补 | — |
| 8 | exercism.org/rust track | https://exercism.org/tracks/rust | 100+ mentor-mode 题库，`exercises/` 补料 | Phase §8 |
| 9 | rust-lang/rustc-dev-guide | https://github.com/rust-lang/rustc-dev-guide | 配合 `deep-dives/contributing-to-rust.md` 入口 | Phase §5 |

---

## 4. 工具与基础设施

| 工具 | URL / 安装方式 | 用途 | 备注 |
|---|---|---|---|
| mdbook | `cargo install mdbook` | 内容引擎 | Rust 圈标准文档工具 |
| just | `cargo install just` | 任务运行器 | 替代 Makefile，4 命令 |
| rustfmt | Rust 内置 | 代码格式化 | `rustfmt.toml` 标准配置 |
| clippy | Rust 内置 | Lint 检查 | `clippy.toml` 教学宽松配置 |
| mdbook-svgbob | `cargo install mdbook-svgbob` | ASCII → SVG 图示 | comprehensive-rust 用，本仓库暂不需要 |
| mdbook-i18n-helpers | `cargo install mdbook-i18n-helpers` | 多语言翻译 | 本仓库不做 i18n |
| mdbook-exerciser | comprehensive-rust 自研 | mdbook 内嵌可验证练习 | **拒绝**：太重，用 `#[test] #[ignore]` 替代 |
| mdbook-course | comprehensive-rust 自研 | 演讲者/学生模式 | **拒绝**：个人仓库无课堂场景 |
| mdbook-linkcheck2 | `cargo install mdbook-linkcheck2` | 死链检查 | 可选，v1.0 手动抽查 |
| cargo xtask | comprehensive-rust 模式 | 一站式任务 runner | **拒绝**：justfile 足够 |
| insta | crates.io | Snapshot test | comprehensive-rust 用，本仓库暂不需要 |

**被引用位置**：RESEARCH §5 · Architecture §2（技术选型对比表）

---

## 5. 外部文章与社区资源

| # | 标题 / 来源 | URL | 引用位置 | 摘要 |
|---|---|---|---|---|
| 1 | Carol Nichols 设计哲学 | https://github.com/carols10cents | RESEARCH §4 | rustlings 原作者："先改编译，再讲理论" |
| 2 | 知乎《Too Many Lists》读书笔记 | https://zhuanlan.zhihu.com/p/83776098 | RESEARCH §4 | 描述 too-many-lists 读者体验："像身边有亲密好朋友手把手教" |
| 3 | CSDN rustlings 分析 | https://www.cnblogs.com/xiao987334176/p/19199209 | RESEARCH 附录 B | rustlings 使用分析 |
| 4 | CSDN too-many-lists 用户指南 | https://blog.csdn.net/gitblog_00702/article/details/154270122 | RESEARCH 附录 B | too-many-lists 使用指南 |
| 5 | Google Security Blog: Scaling Rust Adoption Through Training | https://security.googleblog.com/2023/09/scaling-rust-adoption-through-training.html | RESEARCH 附录 B | comprehensive-rust 背景介绍 |
| 6 | Rust 语言圣经在线站 | https://course.rs | RESEARCH §2.4 | sunface/rust-course 主站，含在线 playground |

---

## 6. 设计哲学来源

| 来源 | 核心理念 | 如何体现在本仓库 |
|---|---|---|
| rustlings（Carol Nichols） | "先改编译，再讲理论" | exercises/ 错误驱动练习 |
| too-many-lists | "先写错的代码 → 让编译器报 → 读错误 → 讲概念 → 改对" | compiler-pitfalls/ 4 段格式（错码→报错→解释→修复） |
| sunface/rust-course | "教程 + 实战 + 编译错误攻关 + 性能优化"全栈覆盖 | 四层分离架构（内容/练习/项目/笔记） |
| comprehensive-rust | "课程 + speaker notes + 测试 + CI 标准化" | 借鉴其文档工程化思路，拒绝其基建复杂度 |
| Rust 官方（The Book / Nomicon / Async Book / Cargo Book） | 系统化知识体系 | 基础章 TLDR 对比表（不重复系统讲解），核心章深挖 |

---

## 7. 引用速查（按 SDD 文档反查）

| SDD 文档 | 依赖的外部资料（编号） |
|---|---|
| PRD §0 | 核心调研 1.4（sunface 8 条启示）· 设计哲学 3 |
| PRD §1 | 核心调研 1.1-1.5（全部 5 项目）· 竞品 3.1（book-cn） |
| PRD §2 | 工具 4（mdbook / just / cargo test）· 核心调研 1.1（rustlings 题库） |
| Architecture §2 | 工具 4（全部技术选型对比）· 核心调研 1.5（comprehensive-rust 拒绝理由） |
| Architecture §3 | 核心调研 1.3（too-many-lists 目录设计）· 核心调研 1.4（sunface 目录设计） |
| Architecture §5 | 核心调研全部 + 竞品 3.2（tests/ui/）· 评审意见 |
| Architecture §6 | 评审意见（内容去重 P1） |
| Phase §1 | 工具 4（mdbook + just 安装）· Architecture ADR #1/#8 |
| Phase §2 | 核心调研 1.1（rustlings 题库 fork）· Architecture ADR #2/#3 |
| Phase §3 | 核心调研 1.4（sunface 写作模板）· Architecture ADR #4 |
| Phase §4 | 核心调研 1.3（too-many-lists 翻译）· 竞品 3.2（tests/ui/ 案例） |
| Phase §5 | 竞品 3.3（ripgrep）· 竞品 3.6（idiomatic-rust）· 竞品 3.9（rustc-dev-guide） |
| Phase §6 | 核心调研 1.4（sunface trait / error 章参考） |
