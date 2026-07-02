# Trait 与泛型

> **一句话**：Trait 是 Rust 的接口抽象——定义共享行为；泛型是代码复用——一份代码支持多种类型；两者结合（trait bound）是 Rust 多态的核心。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 接口 | Trait（可含默认实现 + 关联类型） | `interface`（仅结构约束） |
| 泛型 | `fn foo<T: Trait>(x: T)` — 单态化（monomorphization），零运行时开销 | `function foo<T>(x: T)` — 类型擦除 |
| 多态 | trait bound（静态分发）或 `dyn Trait`（动态分发） | 鸭子类型 / 结构类型 |
| 运算符重载 | 通过 trait（`Add`, `Index`, `Deref` 等） | 不支持 |
| 继承 | 无类继承，trait 可以有超 trait（`trait B: A`） | class extends / implements |

**核心差异**：TypeScript 的接口是结构性（structural）的——只要形状匹配，就能传入；Rust 的 trait 是名义性（nominal）的——必须显式 `impl Trait for Type`，编译器才会建立关系。这种显式性让 Rust 的 trait 可以拥有默认实现、关联类型、超 trait 等更丰富的抽象能力。泛型在 Rust 中通过单态化生成具体代码，零运行时开销，但会增加二进制体积；TypeScript 的泛型在编译后被擦除。

## 代码对比表

### 基础 Trait + 泛型

```rust
trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn default_summary(&self) -> String {
        "(Read more...)".to_string()
    }
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        let end = self.title.len().min(20);
        format!("{}...", &self.title[..end])
    }
}

// 泛型 + trait bound
fn notify<T: Summary>(item: &T) {
    println!("Breaking: {}", item.summarize());
}

fn main() {
    let article = Article {
        title: "Rust 1.0 Released".to_string(),
        content: "...".to_string(),
    };
    notify(&article);
}
```

```typescript
interface Summary {
    summarize(): string;
}

class Article implements Summary {
    constructor(
        public title: string,
        public content: string,
    ) {}

    summarize(): string {
        return `${this.title.slice(0, 20)}...`;
    }
}

function notify<T extends Summary>(item: T): void {
    console.log(`Breaking: ${item.summarize()}`);
}

const article = new Article("Rust 1.0 Released", "...");
notify(article);
```

### 关联类型

```rust
trait Graph {
    type Node;
    type Edge;

    fn nodes(&self) -> Vec<Self::Node>;
    fn edges(&self) -> Vec<Self::Edge>;
}

struct SimpleGraph;

impl Graph for SimpleGraph {
    type Node = i32;
    type Edge = (i32, i32);

    fn nodes(&self) -> Vec<i32> {
        vec![1, 2, 3]
    }

    fn edges(&self) -> Vec<(i32, i32)> {
        vec![(1, 2), (2, 3)]
    }
}

fn main() {
    let g = SimpleGraph;
    println!("{:?}, {:?}", g.nodes(), g.edges());
}
```

```typescript
interface Graph<N, E> {
    nodes(): N[];
    edges(): E[];
}

class SimpleGraph implements Graph<number, [number, number]> {
    nodes(): number[] {
        return [1, 2, 3];
    }

    edges(): [number, number][] {
        return [[1, 2], [2, 3]];
    }
}

const g = new SimpleGraph();
console.log(g.nodes(), g.edges());
```

### `impl Trait`（RPIT）— 简化返回类型

```rust
// 返回一个闭包类型，但不暴露具体类型名
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let c = returns_closure();
    println!("{}", c(5)); // 6
}
```

```typescript
function returnsClosure(): (x: number) => number {
    return (x) => x + 1;
}

const c = returnsClosure();
console.log(c(5)); // 6
```

### `dyn Trait` — 动态分发

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) { println!("circle"); }
}

impl Drawable for Square {
    fn draw(&self) { println!("square"); }
}

// 静态分发：编译期为每个 T 生成单独代码
fn static_dispatch<T: Drawable>(item: &T) {
    item.draw();
}

// 动态分发：运行时通过 vtable 调用
fn dynamic_dispatch(item: &dyn Drawable) {
    item.draw();
}

fn main() {
    let circle = Circle;
    let square = Square;

    static_dispatch(&circle);
    dynamic_dispatch(&square);

    // 同类型集合
    let shapes: Vec<&dyn Drawable> = vec![&circle, &square];
    for s in shapes {
        s.draw();
    }
}
```

```typescript
interface Drawable {
    draw(): void;
}

class Circle implements Drawable {
    draw() { console.log("circle"); }
}

class Square implements Drawable {
    draw() { console.log("square"); }
}

function staticDispatch(item: Drawable): void {
    item.draw();
}

function dynamicDispatch(item: Drawable): void {
    item.draw();
}

const circle = new Circle();
const square = new Square();

staticDispatch(circle);
dynamicDispatch(square);

const shapes: Drawable[] = [circle, square];
for (const s of shapes) {
    s.draw();
}
```

### 常见标准 Trait 速查

| Trait | 作用 | derive? |
|-------|------|--------|
| `Debug` | `{:?}` 格式化输出 | ✅ |
| `Clone` | 显式深拷贝 `.clone()` | ✅ |
| `Copy` | 隐式按位复制 | ✅（仅简单类型） |
| `PartialEq`/`Eq` | `==` 和 `!=` | ✅ |
| `PartialOrd`/`Ord` | `<` `>` 排序 | ✅ |
| `Hash` | HashMap key | ✅ |
| `Default` | 默认值 `T::default()` | ✅ |
| `Display` | `{}` 用户面向输出 | ❌ 手动实现 |
| `From`/`Into` | 类型转换 | ❌ |
| `Deref` | `*` 解引用 | ❌ |
| `Drop` | 析构函数 | ❌ |

## 泛型单态化 (Monomorphization)

```rust
fn identity<T>(x: T) -> T { x }

fn main() {
    // 编译器为每个使用的具体类型生成一份代码
    let a = identity(42);     // → fn identity_i32(x: i32) -> i32
    let b = identity("hi");   // → fn identity_str(x: &str) -> &str

    println!("{a}, {b}");
}
// 零运行时开销，但会增加二进制大小
```

```typescript
function identity<T>(x: T): T {
    return x;
}

const a = identity(42);     // 运行时类型擦除
const b = identity("hi");   // 运行时类型擦除

console.log(`${a}, ${b}`);
```

## 容易踩的坑

1. **Orphan Rule（孤儿规则）**——不能为外部类型实现外部 trait（`impl Display for Vec<T>` ❌），只能为本地类型实现外部 trait，或为本地类型实现外部 trait。
2. **`impl Trait` 返回类型不透明**——调用者不知道具体类型，不能把返回值赋给具体类型变量，也不能用于关联类型位置。
3. **`dyn Trait` 有大小限制**——trait object 是 `!Sized`，需要 `Box<dyn Trait>` 或 `&dyn Trait` 才能使用。
4. **泛型膨胀**——`fn foo<T: Trait>` 为每种 T 生成代码，过多泛型参数会导致二进制变大、编译变慢。
5. **`Copy` 和 `Drop` 互斥**——一个类型实现了 `Drop`（需要析构），就不能同时实现 `Copy`，因为 Copy 语义与自定义析构冲突。

## 交叉链接

- → [结构体与枚举](struct-enum.md) — trait impl 的主体
- → [闭包](closure.md) — `Fn`/`FnMut`/`FnOnce` trait
- → [错误处理](error.md) — `thiserror` 用 derive 实现 `Error` trait
- → [函数](function.md) — 泛型函数与 trait bound 参数
