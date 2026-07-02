# 模式匹配

> **一句话**：Rust 的 `match` 是表达式，必须穷尽所有可能；通过 `if let` / `while let` / `matches!` 可以针对特定模式写轻量分支，避免为单一分支大动干戈。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 主要结构 | `match value { A => ..., B => ... }` | `switch(value) { case A: ... }` |
| 穷尽性 | 编译器强制检查所有模式都被覆盖 | `switch` 可以漏 `case`， fall-through 容易出错 |
| 解构能力 | 原生支持解构 enum、struct、tuple、slice、嵌套 | 需手动用对象/数组解构或 `if (x.kind === ...)` 判断 |
| 绑定与守卫 | 模式内部可绑定变量、使用 `if` 守卫 | 需额外 `if` 语句或嵌套判断 |
| 轻量分支 | `if let`、`while let`、`matches!` 宏 | 只能用 `if` 或 `switch` 写条件分支 |
| 可变绑定 | `mut` 与 `ref` / `ref mut` 修饰模式 | 没有对应概念，可变由 `let` 与 `const` 决定 |

**核心差异**：TypeScript 的 `switch` 是控制语句，默认贯穿且不做穷尽检查；Rust 的 `match` 是表达式，必须处理所有变体，并把解构、绑定、守卫统一在一套语法里。`if let` 与 `while let` 是 `match` 的语法糖，只关心一个模式时比写完整 `match` 更清爽。

## 代码对比表

### 基础 match：穷尽性与分支

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn describe(d: Direction) -> &'static str {
    match d {
        Direction::North => "up",
        Direction::South => "down",
        Direction::East => "right",
        Direction::West => "left",
    }
}

fn main() {
    println!("{}", describe(Direction::East)); // right
}
```

```typescript
enum Direction {
    North,
    South,
    East,
    West,
}

function describe(d: Direction): string {
    switch (d) {
        case Direction.North: return "up";
        case Direction.South: return "down";
        case Direction.East: return "right";
        case Direction.West: return "left";
    }
    // TS 不会强制穷尽，这里经常漏写 default
}

console.log(describe(Direction.East)); // right
```

### 解构枚举与绑定

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn handle(msg: Message) -> String {
    match msg {
        Message::Quit => String::from("quit"),
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(text) => format!("write: {text}"),
        Message::ChangeColor(r, g, b) => format!("rgb({r}, {g}, {b})"),
    }
}

fn main() {
    println!("{}", handle(Message::Move { x: 10, y: 20 }));
}
```

```typescript
type Message =
    | { kind: "quit" }
    | { kind: "move"; x: number; y: number }
    | { kind: "write"; text: string }
    | { kind: "changeColor"; rgb: [number, number, number] };

function handle(msg: Message): string {
    switch (msg.kind) {
        case "quit": return "quit";
        case "move": return `move to (${msg.x}, ${msg.y})`;
        case "write": return `write: ${msg.text}`;
        case "changeColor": return `rgb(${msg.rgb.join(", ")})`;
    }
}

console.log(handle({ kind: "move", x: 10, y: 20 }));
```

### 解构 struct 与 tuple

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let Point { x, y } = p;
    println!("{x}, {y}"); // 1, 2

    let pair = (1, 2, 3);
    let (a, b, c) = pair;
    println!("{a}, {b}, {c}"); // 1, 2, 3

    let nested = (Point { x: 1, y: 2 }, 3);
    let (Point { x: nx, .. }, z) = nested;
    println!("{nx}, {z}"); // 1, 3
}
```

```typescript
interface Point {
    x: number;
    y: number;
}

function main() {
    const p: Point = { x: 1, y: 2 };
    const { x, y } = p;
    console.log(`${x}, ${y}`); // 1, 2

    const pair: [number, number, number] = [1, 2, 3];
    const [a, b, c] = pair;
    console.log(`${a}, ${b}, ${c}`); // 1, 2, 3

    const nested: [Point, number] = [{ x: 1, y: 2 }, 3];
    const [{ x: nx }, z] = nested;
    console.log(`${nx}, ${z}`); // 1, 3
}
```

### if let / while let 轻量分支

```rust
fn main() {
    let maybe = Some(5);

    if let Some(n) = maybe {
        println!("got {n}"); // got 5
    }

    let mut count = 0;
    let mut stack = vec![Some(1), None, Some(3)];
    while let Some(item) = stack.pop() {
        if let Some(n) = item {
            count += n;
        }
    }
    println!("{count}"); // 4
}
```

```typescript
function main() {
    const maybe = 5;

    if (maybe !== null && maybe !== undefined) {
        console.log(`got ${maybe}`); // got 5
    }

    let count = 0;
    const stack: (number | null)[] = [1, null, 3];
    while (stack.length > 0) {
        const item = stack.pop();
        if (item != null) {
            count += item;
        }
    }
    console.log(count); // 4
}
```

### matches! 宏与模式守卫

```rust
enum State {
    Idle,
    Running { pid: u32 },
    Error(String),
}

fn is_running(state: &State) -> bool {
    matches!(state, State::Running { .. })
}

fn is_critical_error(state: &State) -> bool {
    matches!(state, State::Error(msg) if msg.len() > 10)
}

fn main() {
    let s = State::Running { pid: 42 };
    println!("{}", is_running(&s)); // true

    let e = State::Error(String::from("disk full: cannot write"));
    println!("{}", is_critical_error(&e)); // true
}
```

```typescript
type State =
    | { kind: "idle" }
    | { kind: "running"; pid: number }
    | { kind: "error"; msg: string };

function isRunning(state: State): boolean {
    return state.kind === "running";
}

function isCriticalError(state: State): boolean {
    return state.kind === "error" && state.msg.length > 10;
}

function main() {
    const s: State = { kind: "running", pid: 42 };
    console.log(isRunning(s)); // true

    const e: State = { kind: "error", msg: "disk full: cannot write" };
    console.log(isCriticalError(e)); // true
}
```

### ref 与 ref mut

```rust
fn main() {
    let mut v = 42;

    // ref 在模式里创建引用，而不是按值移动
    let ref r = v;
    println!("{r}"); // 42

    let mut s = String::from("hello");
    match s {
        ref mut inner => {
            inner.push_str(" world");
        }
    }
    println!("{s}"); // hello world
}
```

```typescript
function main() {
    const v = 42;
    const r = v; // 按值复制，不存在 ref 模式
    console.log(r); // 42

    let s = "hello";
    // TypeScript 没有 ref 模式，字符串也是不可变的
    s += " world";
    console.log(s); // hello world
}
```

## 容易踩的坑

1. **match 必须穷尽**——漏掉 enum 变体会报 `non-exhaustive patterns`，用 `_` 通配符兜底即可，但不要无意义滥用 `_` 掩盖新增变体。
2. **模式匹配是按值移动默认**——`match s { ... }` 如果 `s` 未实现 `Copy`，某些分支会转移所有权；不想移动时用 `ref` 或 `&` 匹配。
3. **嵌套解构容易忘记所有字段**——`struct` 模式必须列出字段或用 `..` 忽略剩余字段，否则报 `missing fields`。
4. **if let 只能匹配一个模式**——如果后面需要 `else if let` 或额外分支，考虑换回 `match` 以保持穷尽检查。
5. **ref mut 与 &mut 含义不同**——`ref mut x` 表示“在模式里绑定一个可变引用”，而不是“匹配一个可变引用”；初学者常在 `match` 里写反。

## 交叉链接

- → [结构体与枚举](struct-enum.md) — enum 与 struct 的定义和变体
- → [控制流](control-flow.md) — `match` 作为表达式与 `if` 的关系
- → [变量与绑定](variable.md) — `let` 解构与 `mut` 语义
- → [错误处理](error.md) — `Result` / `Option` 与 `match`、`if let` 的配合
