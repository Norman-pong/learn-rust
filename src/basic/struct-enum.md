# 结构体与枚举

> **一句话**：`struct` 把数据打包成具名形状，`enum` 让同一个类型拥有互斥的变体；`impl` 块把方法绑定到类型上，Rust 用这套组合替代了 TypeScript 中 `interface` + `class` + `discriminated union` 的混合模式。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 自定义数据形状 | `struct`（具名字段 / 元组 / 单元） | `interface` / `type` / `class` |
| 枚举 | `enum` 支持带数据的变体，原生 discriminated union | 手动构造 `{ kind: "..." }` 联合类型 |
| 方法绑定 | `impl` 块，与数据定义分离 | `class` 内部写方法；或用对象+函数 |
| 空值 | `Option<T>` / `Result<T, E>` 强制处理 | `T \| undefined` / `throw` + try/catch |
| 匹配分支 | `match` 必须穷尽所有 `enum` 变体 | `switch` 默认不穷尽，需手动兜底 |
| 复制语义 | 默认 move，可 `derive(Clone, Copy)` | 对象/数组默认引用，浅拷贝即可复用 |

**核心差异**：TypeScript 的类型只是设计时的注解，运行时依然是 JavaScript 对象；Rust 的 `struct`/`enum` 直接决定内存布局，方法、trait、所有权都围绕它展开。最实用的感受是：**Rust 的 `enum` 可以自带数据，且 `match` 编译器会检查你是否漏了分支**。

## 代码对比表

### 三种 struct

```rust
struct User {
    id: u64,
    name: String,
}

struct Point(f64, f64);          // tuple struct
struct Unit;                     // unit struct

fn main() {
    let u = User { id: 1, name: String::from("Alice") };
    let p = Point(3.0, 4.0);
    let _ = Unit;

    println!("{}, ({}, {})", u.name, p.0, p.1);
}
```

```typescript
interface User {
    id: number;
    name: string;
}

type Point = [number, number];   // 元组类型
const Unit = Symbol("Unit");     // 无数据单例

function main() {
    const u: User = { id: 1, name: "Alice" };
    const p: Point = [3.0, 4.0];
    const _ = Unit;

    console.log(u.name, p[0], p[1]);
}
```

### 带数据的 enum

```rust
enum Message {
    Quit,                        // 无数据
    Move { x: i32, y: i32 },     // 具名字段
    Write(String),               // 单字段
    ChangeColor(u8, u8, u8),     // 多字段
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to {x}, {y}"),
        Message::Write(text) => println!("text: {text}"),
        Message::ChangeColor(r, g, b) => println!("rgb({r},{g},{b})"),
    }
}
```

```typescript
type Message =
    | { kind: "quit" }
    | { kind: "move"; x: number; y: number }
    | { kind: "write"; text: string }
    | { kind: "changeColor"; rgb: [number, number, number] };

function main() {
    const msg: Message = { kind: "move", x: 10, y: 20 };

    switch (msg.kind) {
        case "quit":
            console.log("quit");
            break;
        case "move":
            console.log(`move to ${msg.x}, ${msg.y}`);
            break;
        case "write":
            console.log(`text: ${msg.text}`);
            break;
        case "changeColor":
            const [r, g, b] = msg.rgb;
            console.log(`rgb(${r},${g},${b})`);
            break;
    }
}
```

### impl 与方法

```rust
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Rect { width: size, height: size }
    }
}

fn main() {
    let r1 = Rect { width: 30, height: 50 };
    let r2 = Rect { width: 10, height: 20 };

    println!("area = {}", r1.area());
    println!("can hold? {}", r1.can_hold(&r2));
    println!("square = {:?}", Rect::square(10));
}
```

```typescript
interface Rect {
    width: number;
    height: number;
}

function area(rect: Rect): number {
    return rect.width * rect.height;
}

function canHold(a: Rect, b: Rect): boolean {
    return a.width > b.width && a.height > b.height;
}

function square(size: number): Rect {
    return { width: size, height: size };
}

function main() {
    const r1: Rect = { width: 30, height: 50 };
    const r2: Rect = { width: 10, height: 20 };

    console.log("area =", area(r1));
    console.log("can hold?", canHold(r1, r2));
    console.log("square =", square(10));
}
```

### Option 与 Result

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let x = divide(10.0, 2.0);
    match x {
        Some(v) => println!("result: {v}"),
        None => println!("cannot divide by zero"),
    }

    // 只处理 Some 的快捷写法
    if let Some(v) = divide(10.0, 0.0) {
        println!("got {v}");
    } else {
        println!("skipped");
    }
}
```

```typescript
function divide(a: number, b: number): number | undefined {
    return b === 0 ? undefined : a / b;
}

function main() {
    const x = divide(10.0, 2.0);
    if (x !== undefined) {
        console.log(`result: ${x}`);
    } else {
        console.log("cannot divide by zero");
    }

    // 等价于 Rust 的 if let
    if (x !== undefined) {
        console.log(`got ${x}`);
    } else {
        console.log("skipped");
    }
}
```

## 容易踩的坑

1. **`move` 与字段所有权**——`let name = user.name` 会把字段 move 走，之后 `user` 不再完整；要保留原结构体需 `clone()` 或借用 `&user.name`。
2. **`match` 必须穷尽**——如果 `enum` 新增一个变体而忘记更新所有 `match`，编译器会直接报错，这是好事，但新手会以为编译器在刁难。
3. **`&self` / `&mut self` / `self` 选错**——只读方法用 `&self`，需要修改用 `&mut self`，想把所有权转移进方法用 `self`；选错会触发借用检查错误。
4. **`Option<T>` 不是可空指针**——`None` 和 `Some(T)` 大小相同，不能把它当成 `null` 随便解包；未处理的 `Option` 会被编译器警告（lint 级别）。
5. **`derive` 能省但不能乱省**——`#[derive(Debug, Clone, PartialEq)]` 很方便，但 `Copy` 只能给完全由 `Copy` 类型组成的 struct；包含 `String` 或 `Vec` 的 struct 不能 `Copy`。

## 交叉链接

- → [模式匹配](pattern-matching.md) — `match` 与 `if let` 的深入用法
- → [所有权模型](../ownership-lifetimes/ownership.md) — struct 字段与 enum 变体的 move 规则
- → [错误处理](error.md) — `Result<T, E>` 的实战与 `?` 传播
- → [Trait 与泛型](trait-generic.md) — `derive` 背后的宏与 trait 系统
