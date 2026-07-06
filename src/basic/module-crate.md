# 模块与 Crate

> **一句话**：Rust 用 `mod` 组织文件内外的命名空间，用 `use` 引入路径，用 `pub` 控制可见性；`crate` 是编译的最小单元，模块只是 crate 内部的可见性边界，不改变所有权规则。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript / ES Modules |
|------|------|------------------------|
| 模块声明 | `mod foo;` 显式声明文件/目录模块 | 文件名即路径，`import` 自动解析 |
| 路径引入 | `use crate::a::b::C` | `import { C } from "./a/b"` |
| 可见性 | 默认私有，需要 `pub` 公开 | 默认导出即公开（`export`） |
| 上/当前/根路径 | `super::`、`self::`、`crate::` | `../`、`./`、package alias |
| 可执行文件 | `bin` crate（`main.rs`） | `package.json` 的 `bin` 字段 |
| 库 | `lib` crate（`lib.rs`） | `package.json` 的 `main` / `exports` |
| 多包仓库 | `[workspace]` | monorepo（pnpm workspace、Turborepo） |

**核心差异**：在 TypeScript 中，一个文件通常就是一个模块，路径和文件系统一一对应；Rust 的 `mod` 是显式声明，文件系统映射只是约定。这意味着你可以把多个 `mod` 写在同一个 `.rs` 文件里，也可以让 `mod xxx` 指向 `xxx.rs` 或 `xxx/mod.rs`。`pub` 控制的是**可见性**，而不是“是否被导出为模块”。

## 代码对比表

### 单文件内的 mod

```rust
mod player {
    pub struct Player {
        pub name: String,
        score: u32,             // 默认私有
    }

    impl Player {
        pub fn new(name: &str) -> Self {
            Player { name: name.to_string(), score: 0 }
        }

        pub fn score(&self) -> u32 {
            self.score
        }
    }
}

mod game {
    use super::player::Player;  // 使用父模块的路径

    pub fn start() -> Player {
        Player::new("Alice")
    }
}

fn main() {
    let p = game::start();
    println!("{}, score {}", p.name, p.score());
}
```

```typescript
// 单文件内模拟命名空间
namespace Player {
    export class Player {
        constructor(
            public name: string,
            private score: number = 0,
        ) {}

        score(): number {
            return this.score;
        }
    }
}

namespace Game {
    import { Player } from "./player"; // 真实 TS 依赖文件路径

    export function start(): Player {
        return new Player("Alice");
    }
}

function main() {
    const p = Game.start();
    console.log(`${p.name}, score ${p.score()}`);
}
```

### 文件系统映射

```rust
// src/main.rs
mod player;     // 引入 src/player.rs
mod game;       // 引入 src/game.rs

fn main() {
    let p = game::start();
    println!("{}", p.name);
}

// src/player.rs
pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player { name: name.to_string() }
    }
}

// src/game.rs
use crate::player::Player;  // crate:: 表示当前 crate 根

pub fn start() -> Player {
    Player::new("Alice")
}
```

```typescript
// src/main.ts
import { start } from "./game";

function main() {
    const p = start();
    console.log(p.name);
}

// src/player.ts
export class Player {
    constructor(public name: string) {}
}

// src/game.ts
import { Player } from "./player";

export function start(): Player {
    return new Player("Alice");
}
```

### 目录模块（mod.rs / 2021 edition 的 xxx.rs）

```rust
// src/network/mod.rs（传统）
pub mod tcp;
pub mod udp;

// 或 src/network.rs（Rust 2018+ 推荐）
pub mod tcp;
pub mod udp;

// src/network/tcp.rs
pub fn connect() {}

// src/main.rs
mod network;

fn main() {
    network::tcp::connect();
}
```

```typescript
// src/network/index.ts
export * as tcp from "./tcp";
export * as udp from "./udp";

// src/network/tcp.ts
export function connect() {}

// src/main.ts
import * as network from "./network";

function main() {
    network.tcp.connect();
}
```

### pub 与可见性

```rust
// crate::outer::inner::Thing
// 只有 `inner` 和 `Thing` 都是 pub，外部才能访问到 Thing

mod outer {
    pub mod inner {
        pub struct Thing;

        // pub(in crate::outer) 表示只在 outer 模块内可见
        pub(in crate::outer) struct Secret;

        // pub(crate) 表示整个 crate 可见
        pub(crate) struct CrateVisible;
    }
}

fn main() {
    let _ = outer::inner::Thing;
}
```

```typescript
// TypeScript 没有路径可见性层，只有 export / 不 export

export function publicApi() {}

function internalHelper() {} // 文件级私有
```

## 容易踩的坑

1. **`mod` 不写就等于不存在**——即使文件叫 `player.rs`，主文件没写 `mod player;`，它也不会被编译进 crate。
2. **默认私有**——`struct` 即使声明为 `pub`，字段也默认私有；字段需要公开必须逐个写 `pub`。
3. **路径忘记了 `crate::` 或 `super::`**——在深层模块里引用根模块或兄弟模块时，必须用 `crate::` / `super::`，不能写相对路径字符串。
4. **重复引入 `mod`**——如果 `main.rs` 写 `mod player;`、`game.rs` 又写 `mod player;`，同一个模块会被编译两次，产生错误。
5. **模块不影响所有权**——把值传入另一个模块的函数，照样遵循 move/借用规则；`pub` 只是把名字公开，不复制数据。

## 交叉链接

- → [Cargo 入门](cargo.md) — crate 的创建、构建与发布
- → [所有权模型](../ownership-lifetimes/ownership.md) — 跨模块传递值依然遵守所有权规则
- → [结构体与枚举](struct-enum.md) — 模块中公开自定义类型的字段与方法
