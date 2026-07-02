# 参与 Rust 贡献

> 一句话定位：为 `rust-lang/rust` 或其生态提交代码、文档和测试，是从"用 Rust"进阶到"懂 Rust"的最快路径。

---

## 关键差异（200 字）

与给普通 GitHub 项目提 PR 不同，Rust 是编译器项目，贡献前必须先建立对 rustc 工作方式的认知。开发环境通过 `./x.py`（简称 `./x`）管理，而不是裸 `cargo`；测试以 `compile-fail` / `ui` / `run-make` 等用例为主，而不是单元测试；issue 标签体系复杂，`E-easy`、`E-mentor`、`good first issue` 才是新人入口。此外，Rust 对提交历史、commit message 和稳定性有严格要求，新 API 通常需要 FCP（Final Comment Period）流程，不能随意合并。

---

## 对比：Rust 贡献工作流 vs 普通开源项目

| 维度 | Rust 贡献 | 普通 Node.js/TypeScript 项目 |
|---|---|---|
| 构建工具 | `./x setup` + `./x build` | `npm install` + `npm run build` |
| 本地测试 | `./x test tests/ui/...` | `npm test` / `vitest` |
| 找新手 issue | `E-easy` / `E-mentor` / `good first issue` | `good first issue` |
| 代码审查 | 多名维护者 + 子团队审批 | 维护者一人 review |
| API 变更 | 需稳定性评估与 FCP | 通常直接合并发版 |
| 文档位置 | `rustc-dev-guide` | `README.md` / `CONTRIBUTING.md` |

---

## 推荐入门路径

### 1. 阅读 rustc-dev-guide

`rustc-dev-guide` 是官方维护的"给 rustc 贡献者的说明书"。

```bash
git clone https://github.com/rust-lang/rustc-dev-guide
cd rustc-dev-guide
mdbook serve
```

必读章节：
- [Building and debugging rustc](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html)
- [The compiler source code](https://rustc-dev-guide.rust-lang.org/overview.html)
- [Testing the compiler](https://rustc-dev-guide.rust-lang.org/tests/intro.html)
- [Diagnostic codes](https://rustc-dev-guide.rust-lang.org/diagnostics/diagnostic-codes.html)

### 2. 找 good first issue

在 `rust-lang/rust` 仓库用标签过滤：

| 标签 | 含义 | 适合人群 |
|---|---|---|
| `E-easy` | 技术难度低 | 第一次贡献 |
| `E-mentor` | 有导师愿意带 | 希望有人指导 |
| `good first issue` | 官方推荐新手 | 完全没贡献过 |
| `A-diagnostics` | 改进编译器错误信息 | 熟悉 Rust 报错 |
| `T-libs` / `T-compiler` | 工作组分类 | 按兴趣选择 |

### 3. 搭建开发环境

```bash
# 1. Fork 并 clone 自己的 fork
git clone https://github.com/<your-username>/rust.git
cd rust

# 2. 配置开发环境（选择 compiler 配置）
./x setup

# 3. 构建标准库（较快，适合入门）
./x build library

# 4. 完整构建编译器（慢，按需执行）
# ./x build

# 5. 运行 UI 测试
./x test tests/ui/ --test-args your-test-name
```

### 4. 典型入门 PR 类型

| 类型 | 难度 | 示例 | 备注 |
|---|---|---|---|
| 改进诊断信息 | 低 | 让 `E0xxx` 错误提示更具体 | 改 `compiler/` 错误生成逻辑 |
| 修复 clippy lint | 低-中 | 修复误报或改进 lint | 仓库在 `rust-lang/rust-clippy` |
| 补充测试 | 低 | 为现有功能增加 `ui` 测试 | 只需写 `.rs` 和 `.stderr` |
| 改进文档 | 低 | 修复标准库示例或拼写 | 适合熟悉流程 |
| 标准库小 API | 中 | 新增稳定 trait 方法 | 通常需要 ACP 或 FCP |

---

## 踩坑清单

1. **不要直接 `cargo build` 整个 rustc**——必须用 `./x build`，它会处理 stage 0/1/2 的引导和 bootstrap 配置。
2. **不要跳过 `./x setup`**——初次贡献者常因缺少依赖或配置文件错误导致构建失败。
3. **测试不是普通 `cargo test`**——编译器用 `ui` 测试、增量测试、run-make 等；读 `rustc-dev-guide` 的测试章节。
4. **不要大改 API 作为第一个 PR**——先从小诊断信息或文档改起，熟悉流程再碰 T-libs/T-compiler。
5. **不要忽略 `rustfmt` 和 `tidy`**——提交前运行 `./x fmt` 和 `./x test tidy --bless`，避免 CI 因格式失败。

---

## 提交与 PR 流程

```bash
# 1. 从 master 切出功能分支
git checkout -b fix-diagnostic-e0xxx

# 2. 修改代码 + 测试
./x test tests/ui/error-codes/

# 3. 格式化并检查 tidy
./x fmt
./x test tidy --bless

# 4. 提交（参考项目惯例，首行简洁，正文说明 why）
git commit -m "Improve diagnostic for E0xxx: suggest using as_ref"

# 5. push 到 fork 并提 PR
git push origin fix-diagnostic-e0xxx
```

PR 描述建议包含：
- 关联的 issue（`Fixes #12345`）
- 修改了什么
- 为什么这样改
- 测试覆盖情况

---

## 快速建议

1. **从 rustc-dev-guide 开始，不要直接读 `compiler/` 源码。**
2. **第一个 PR 选"改进错误信息"或"补充测试"，最快拿到反馈。**
3. **让 CI 帮你检查，本地 `./x test` 能过最好。**
4. **不懂就问：** Rust Zulip 的 `#t-compiler/help` 或 issue 里 `@` 导师。

---

## 交叉链接

- [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/)
- [rust-lang/rust CONTRIBUTING.md](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md)
- [Rust Zulip #t-compiler/help](https://rust-lang.zulipchat.com/)
- [Crust of Rust 笔记](./crust-of-rust-notes.md) — 进阶前复习 Send / Sync / 'static
- [生命周期主题](../compiler-pitfalls/lifetime-theme/index.md) — 贡献编译器前先把生命周期踩坑踩熟

---

> 贡献 Rust 不是写完美代码，而是理解 rustc 如何接收、检查、合并你的代码。迈出第一小步比看懂整个编译器更重要。
