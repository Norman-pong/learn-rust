# 控制流

> **一句话**：Rust 的控制流是表达式导向的——`if`/`match`/`loop` 都可以返回值，配合 `if let` 和 `let-else` 可以简洁地处理 `Option`/`Result` 等枚举类型。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| `if` 表达式 | `if` 返回同类型值，分支必须一致 | `if` 是语句，返回 `undefined` 或最后一行表达式 |
| 条件括号 | 不需要：`if x > 0` | 需要：`if (x > 0)` |
| 模式匹配 | `match` 穷尽所有分支，编译器检查 | 无原生 match，用 `switch` + `case` |
| 循环返回值 | `loop` 可 `break value` | `while`/`for` 只能 `break` |
| 解构条件 | `if let Some(v) = opt` | `if (opt !== undefined)` 或可选链 |
| let-else | `let Some(v) = opt else { return; }` | 无对应，需要 if-return 或 throw |

**核心差异**：Rust 把控制流视为表达式。`if` 的两个分支必须返回相同类型，因此整个 `if` 可以赋值给变量。`match` 更进一步，要求穷尽所有模式，这使得"漏掉分支"在编译期就被捕获。TypeScript 的控制流更偏向语句，类型收窄依赖 CFA（Control Flow Analysis），但运行时不强制穷尽。

## 代码对比表

### `if` 表达式

```rust
fn main() {
    let n = 5;

    let msg = if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    };

    println!("{msg}");
}
```

```typescript
function main() {
    const n = 5;

    let msg: string;
    if (n > 0) {
        msg = "positive";
    } else if (n < 0) {
        msg = "negative";
    } else {
        msg = "zero";
    }

    console.log(msg);
}
```

### `match` 穷尽匹配

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // 所属州
}

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {state}");
            25
        }
    }
}
```

```typescript
type Coin =
    | { kind: "penny" }
    | { kind: "nickel" }
    | { kind: "dime" }
    | { kind: "quarter"; state: string };

function value(coin: Coin): number {
    switch (coin.kind) {
        case "penny": return 1;
        case "nickel": return 5;
        case "dime": return 10;
        case "quarter":
            console.log(`来自 ${coin.state}`);
            return 25;
    }
}
```

### `if let` 与 `let-else`

```rust
fn greet(maybe: Option<&str>) {
    // if let 只处理 Some，忽略 None
    if let Some(name) = maybe {
        println!("Hello, {name}");
    }

    // let-else：Some 继续，None 提前返回
    let Some(name) = maybe else {
        println!("no name");
        return;
    };

    println!("Hello again, {name}");
}

fn main() {
    greet(Some("Alice"));
    greet(None);
}
```

```typescript
function greet(maybe: string | undefined) {
    if (maybe !== undefined) {
        console.log(`Hello, ${maybe}`);
    }

    if (maybe === undefined) {
        console.log("no name");
        return;
    }

    console.log(`Hello again, ${maybe}`);
}
```

### `loop` 与 `while let`

```rust
fn main() {
    let mut i = 0;

    // loop 可以返回值
    let result = loop {
        i += 1;
        if i == 10 {
            break i * 2; // break 带值
        }
    };
    println!("result: {result}"); // 20

    // while let 循环解构迭代器
    let nums = vec![1, 2, 3];
    let mut iter = nums.iter();
    while let Some(v) = iter.next() {
        println!("{v}");
    }
}
```

```typescript
function main() {
    let i = 0;

    let result = undefined;
    while (true) {
        i += 1;
        if (i === 10) {
            result = i * 2;
            break;
        }
    }
    console.log(`result: ${result}`); // 20

    const arr = [1, 2, 3];
    for (const v of arr) {
        console.log(v);
    }
}
```

## 容易踩的坑

1. **`if` 分支类型不一致**——`if cond { 1 } else { "a" }` 会编译失败，必须两边同类型。
2. **`match` 漏分支**——非穷尽 `match` 编译失败，尤其是枚举新增变体后，所有 match 点都会被检查。
3. **忘记 `if` 条件不加括号**——`if (x > 0)` 在 Rust 里合法但冗余，常让新手困惑。
4. **`let-else` 的 `else` 块必须发散**——`let Some(v) = opt else { println!("x") };` 会报错，因为 else 必须返回 `!`（如 return/continue/break/panic）。
5. **`match` 变量遮蔽**——`match coin { Coin::Quarter(state) => ... }` 中 `state` 是新绑定，不要与外层同名变量混淆。

## 交叉链接

- → [变量与绑定](variable.md) — `let` 与模式解构
- → [类型系统](type.md) — `Option<T>` 和枚举类型
- → [错误处理](error.md) — `Result` 与 `?` 配合 `if let`
- → [模式匹配](pattern-matching.md) — match 模式的完整语法
