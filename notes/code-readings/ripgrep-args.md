# ripgrep 的 clap derive args parsing 源码阅读

来源：BurntSushi/ripgrep @ [`4519153e`](https://github.com/BurntSushi/ripgrep/tree/4519153e/crates/core/flags)。

## 1. 文件结构：两层 Args

ripgrep 没有用一个巨大的 clap derive `Args` 包揽所有事，而是拆成两层：

- `LowArgs`（`lowargs.rs`）：贴近 CLI flag 的原始、可叠加状态。
- `HiArgs`（`hiargs.rs`）：在解析完成后，把低级状态转换成真正可用的业务对象，比如 `ignore::overrides::Override`、`grep::searcher::MmapChoice` 等。

```rust
// crates/core/flags/lowargs.rs
#[derive(Debug, Default)]
pub(crate) struct LowArgs {
    pub(crate) special: Option<SpecialMode>,
    pub(crate) mode: Mode,
    pub(crate) positional: Vec<OsString>,
    pub(crate) patterns: Vec<PatternSource>,
    pub(crate) binary: BinaryMode,
    pub(crate) buffer: BufferMode,
    pub(crate) case: CaseMode,
    pub(crate) color: ColorChoice,
    pub(crate) context: ContextMode,
    pub(crate) engine: EngineChoice,
    pub(crate) logging: Option<LoggingMode>,
    // ... 其余字段
}
```

与 TypeScript 的 `process.argv` + `yargs/minimist` 手动解析不同：Rust 侧没有反射地扫描 struct 字段来生成 CLI，而是显式地给每个 flag 实现一个 `Flag` trait，由 `parse.rs` 把它们注册进一个全局 trie，再按顺序 `update` 进 `LowArgs`。也就是说，**clap derive 的“声明即解析”被替换成了“手工声明 + 手工 update”**，换来了对 override 语义和 negated flag 的细粒度控制。

## 2. Flag 的分类

在 `defs.rs` 中，所有 flag 被统一放进 `const FLAGS: &[&dyn Flag]`，并按 `Category` 分组。主要类别包括：

| Category | 代表 flag | 作用 |
|---|---|---|
| `Input` | `-e/--regexp`, `-f/--file`, `--pre`, `--search-zip` | 模式来源与输入预处理 |
| `Search` | `-s/--case-sensitive`, `-i/--ignore-case`, `-S/--smart-case`, `-P/--pcre2`, `--engine` | 匹配行为与引擎选择 |
| `Filter` | `--glob`, `--iglob`, `--type`, `--hidden`, `--no-ignore-vcs`, `--one-file-system` | 文件过滤与忽略规则 |
| `Output` | `-n/--line-number`, `--column`, `-b/--byte-offset`, `-A/-B/-C`, `--pretty`, `--json`, `--vimgrep` | 输出格式 |
| `OutputModes` | `-l/--files-with-matches`, `-L/--files-without-match`, `-c/--count`, `--count-matches`, `--files`, `--generate` | 运行模式（与 `Mode` enum 对应） |
| `Logging` | `--debug`, `--trace`, `--no-messages`, `--stats` | 日志与诊断 |
| `OtherBehaviors` | `--help`, `--version`, `--pcre2-version`, `--no-config` | 特殊短路行为 |

一个 flag 的声明长这样（以 `-A/--after-context` 为例）：

```rust
// crates/core/flags/defs.rs
struct AfterContext;

impl Flag for AfterContext {
    fn is_switch(&self) -> bool { false }
    fn name_short(&self) -> Option<u8> { Some(b'A') }
    fn name_long(&self) -> &'static str { "after-context" }
    fn doc_variable(&self) -> Option<&'static str> { Some("NUM") }
    fn doc_category(&self) -> Category { Category::Output }
    fn doc_short(&self) -> &'static str { "Show NUM lines after each match." }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.context.set_after(convert::usize(&v.unwrap_value())?);
        Ok(())
    }
}
```

这里 `Flag` 不是 derive macro，而是手写 trait。每个 flag 自己决定 short/long/negated/alias，以及如何把 `FlagValue` 写进 `LowArgs`。对比 TypeScript：

```ts
// TS 常见做法：类型即 schema
const args = yargs(process.argv.slice(2))
  .option('after-context', { alias: 'A', type: 'number' })
  .argv;
```

Rust 的 `Flag` trait 更底层，像“为每个 CLI 选项写一个小型 reducer”，没有全局 schema，但语义极明确。

## 3. clap derive 模式与 `default_value_t`

严格说 ripgrep 这里**没有使用 clap derive**。它用的是 `lexopt` + 自定义 `Flag` trait。许多 Rust 项目会把 clap 的 `derive` 作为入口，例如：

```rust
#[derive(Parser)]
struct Args {
    #[arg(short = 'A', long, default_value_t = 0)]
    after_context: usize,
}
```

`default_value_t` 是 clap derive 的语法糖，表示“用该类型的 `Default::default()` 作为缺省值”。ripgrep 的等价做法是给 `LowArgs` 派生 `Default`：

```rust
#[derive(Debug, Default)]
pub(crate) struct LowArgs { ... }
```

并且让各子类型如 `CaseMode::Sensitive`、`BufferMode::Auto`、`ContextMode::Limited(...)` 也都实现 `Default`。这样 `LowArgs::default()` 一调用出来就是所有 flag 的缺省状态，每个 flag 的 `update` 只负责覆盖。

TypeScript 的等价物更接近：

```ts
const args = yargs().default('after-context', 0).argv;
```

Rust 的 enum 还能带数据（如 `Mode::Generate(GenerateMode)`、`BinaryMode::SearchAndSuppress`），而 TypeScript 的 string union 通常只能写 `type BinaryMode = 'auto' | 'search-and-suppress' | 'as-text'`。如果要附加数据，TS 必须借助 tagged union；Rust 则原生支持：

```rust
pub(crate) enum Mode {
    Search(SearchMode),
    Files,
    Types,
    Generate(GenerateMode),
}
```

## 4. 冲突检测与 override 语义

ripgrep 没有让解析器自动做“互斥组报错”，而是让后出现的 flag 直接覆盖先出现的 flag。这种“最后一个赢”的语义在 `update` 方法里显式实现。

### 4.1 `--case-sensitive` 覆盖 `--ignore-case` 和 `--smart-case`

```rust
// -s/--case-sensitive
impl Flag for CaseSensitive {
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "flag has no negation");
        args.case = CaseMode::Sensitive;
        Ok(())
    }
}

// -i/--ignore-case
impl Flag for IgnoreCase {
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "flag has no negation");
        args.case = CaseMode::Insensitive;
        Ok(())
    }
}
```

因此 `-i -s` 得到 `Sensitive`，`-s -i` 得到 `Insensitive`。这不是解析时报错，而是按顺序覆盖。

### 4.2 `--binary` vs `-a/--text`

```rust
impl Flag for Binary {
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.binary = if v.unwrap_switch() {
            BinaryMode::SearchAndSuppress
        } else {
            BinaryMode::Auto
        };
        Ok(())
    }
}

impl Flag for Text {
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.binary = if v.unwrap_switch() {
            BinaryMode::AsText
        } else {
            BinaryMode::Auto
        };
        Ok(())
    }
}
```

测试用例说明了覆盖顺序：

```rust
let args = parse_low_raw(["-a", "--binary"]).unwrap();
assert_eq!(BinaryMode::SearchAndSuppress, args.binary);

let args = parse_low_raw(["--binary", "-a"]).unwrap();
assert_eq!(BinaryMode::AsText, args.binary);
```

### 4.3 `--block-buffered` vs `--line-buffered`

```rust
impl Flag for BlockBuffered {
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.buffer = if v.unwrap_switch() {
            BufferMode::Block
        } else {
            BufferMode::Auto
        };
        Ok(())
    }
}

// test
let args = parse_low_raw(["--block-buffered", "--line-buffered"]).unwrap();
assert_eq!(BufferMode::Line, args.buffer);
```

### 4.4 `Mode::update` 的跨组覆盖规则

`Mode` 的 override 比单个字段更复杂，因为 `-l/--files-with-matches`、`-c/--count`、`-F/--files` 等属于“运行模式”，彼此互斥，但 `-q/--quiet` 不应被它们覆盖：

```rust
// crates/core/flags/lowargs.rs
impl Mode {
    pub(crate) fn update(&mut self, new: Mode) {
        match *self {
            // 任何非 search 模式都可以覆盖 search 模式
            Mode::Search(_) => *self = new,
            _ => {
                // 已经处于非 search 模式时，只有新的非 search 模式能覆盖它
                if !matches!(new, Mode::Search(_)) {
                    *self = new;
                }
            }
        }
    }
}
```

这保证了：

- `-l --files` 最终是 `Files`（非 search 之间覆盖）。
- `-q --files` 仍然是 `Files`（`--files` 是新的非 search，覆盖 `Quiet` 这个 search 子状态）。
- `--files -l` 保持 `Files`（新的 search 不能覆盖旧的非 search）。

## 5. 从 LowArgs 到 HiArgs 的再加工

`parse.rs` 解析完 `LowArgs` 后，调用 `HiArgs::from_low_args`。这一步才真正做“业务级”决策：

```rust
// crates/core/flags/hiargs.rs
impl HiArgs {
    pub(crate) fn from_low_args(mut low: LowArgs) -> anyhow::Result<HiArgs> {
        if let Some(ref sort) = low.sort { sort.supported()?; }

        match low.mode {
            Mode::Search(ref mut mode) => match *mode {
                SearchMode::CountMatches if low.invert_match => {
                    *mode = SearchMode::Count;
                }
                SearchMode::Count if low.only_matching => {
                    *mode = SearchMode::CountMatches;
                }
                _ => {}
            },
            _ => {}
        }

        let patterns = Patterns::from_low_args(&mut state, &mut low)?;
        let paths = Paths::from_low_args(&mut state, &patterns, &mut low)?;
        let globs = globs(&state, &low)?;
        let types = types(&low)?;
        let mmap_choice = /* ... */;
        // ...
        Ok(HiArgs { /* ... */ })
    }
}
```

这里体现了 ripgrep 的设计哲学：

- **低级阶段只验证用户输入，不依赖环境**。例如不做 hostname 查询、不读文件系统。
- **高级阶段才做组合与环境判断**。例如根据 `low.threads`、是否单文件、是否排序决定线程数；根据 `paths.is_one_file` 决定默认 `with_filename`；根据 `state.is_terminal_stdout` 调整 `color`。

对 TypeScript 开发者来说，这就像把 CLI 解析拆成两层：

```ts
// 第一层：纯 reducer，纯类型化 raw args
const low = parseLow(process.argv.slice(2));
// 第二层：业务初始化，可能 throw，依赖 env/fs
const hi = buildHiArgs(low);
```

## 6. 小结

- ripgrep 的 `Args` 不是单个 clap derive struct，而是 **LowArgs + HiArgs 双层结构**。
- 每个 flag 通过实现 `Flag` trait 手写注册，语义由 `update` 自己控制，因此 override 和 negated flag 非常清晰。
- `default_value_t` 的等价物是 `#[derive(Default)]` 加上各 enum 的 `#[default]` 变体。
- Rust 的 enum variant 可以携带数据，比 TS 的 string union 更适合表达 `Mode::Search(SearchMode)` 这类状态。
- 解析器只产生 `LowArgs`，真正业务对象在 `HiArgs::from_low_args` 中构建，实现了**输入验证与业务初始化**的分离。
