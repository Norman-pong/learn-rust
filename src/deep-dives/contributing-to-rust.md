# 贡献 Rust 本身

> 给 `rust-lang/rust` 贡献代码的入门指南——从找 issue 到提 PR 的实操步骤。

## 为什么贡献 Rust

1. **深入理解语言**——编译器代码是最好的 Rust 教材
2. **提升 Rust 技能**——大型代码库的协作经验
3. **回馈社区**——你用的每个功能都是社区贡献的

## 入门路径

### Step 1: 读 rustc-dev-guide

```bash
git clone https://github.com/rust-lang/rustc-dev-guide
cd rustc-dev-guide
mdbook serve  # 本地浏览
```

核心章节：
- [Building and debugging rustc](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html)
- [The compiler source code](https://rustc-dev-guide.rust-lang.org/overview.html)
- [Testing the compiler](https://rustc-dev-guide.rust-lang.org/tests/intro.html)

### Step 2: 找 good first issue

标签过滤：
- [`E-easy`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+label%3AE-easy) — 简单修复
- [`E-mentor`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+label%3AE-mentor) — 有导师带
- [`good first issue`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

### Step 3: 典型入门 PR 类型

| PR 类型 | 难度 | 示例 |
|---------|------|------|
| 改进诊断信息 | 低 | 添加更详细的错误提示 |
| 修复 clippy lint | 低-中 | 调整 lint 规则或修复误报 |
| 补充测试 | 低 | 为已有功能增加测试用例 |
| 改进文档 | 低 | 修复标准库文档的错别字或示例 |
| 实现小功能 | 中 | 为标准库添加小的 API |

### Step 4: 提交流程

```bash
# 1. Fork rust-lang/rust
# 2. Clone 你的 fork
git clone https://github.com/<your-username>/rust.git
cd rust

# 3. 配置开发环境
./x setup  # 选择 compiler 配置
./x build library  # 只构建标准库（快）
# ./x build         # 构建整个编译器（慢，可选）

# 4. 修改 + 测试
./x test tests/ui/your-test-dir/

# 5. 提交
git commit -m "Improve diagnostic for E0XXX: suggest ..."
```

## 快速上手建议

1. **从 `rustc-dev-guide` 开始**，不要直接看编译器代码
2. **从小 PR 开始**——改进错误信息是最佳入门
3. **让 CI 帮你检查**——push 后 rust-lang 的 CI 很全面
4. **在 Zulip 或 Discord 提问**——社区很友好

## 相关资源

- [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/)
- [rust-lang/rust CONTRIBUTING.md](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md)
- [Rust Zulip](https://rust-lang.zulipchat.com/) — #t-compiler/help 频道
