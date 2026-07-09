# Crust of Rust 笔记

> 对 Jon Gjengset 的 [Crust of Rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) 系列视频的学习笔记。

---

## Ep 1: Held by a Thread

**核心问题**：`std::thread::spawn` 如何把数据传进线程？`Send` trait 如何在编译期保证线程安全？

### 关键点

1. **`thread::spawn` 的签名**：
   ```rust
   pub fn spawn<F, T>(f: F) -> JoinHandle<T>
   where
       F: FnOnce() -> T + Send + 'static,
       T: Send + 'static,
   ```

2. **`Send`**：类型的所有权可以安全地转移到另一个线程
   - `i32`, `String`, `Vec<T: Send>`, `Arc<T: Send>` → `Send`
   - `Rc<T>`, `*const T`, `*mut T` → 不是 `Send`
   - `RefCell<T>`：`Send if T: Send`

3. **`'static`**：类型不包含非 `'static` 的引用
   - 闭包必须不借用任何局部变量（或借用的是 `'static` 数据）
   - 解决：用 `move` 将数据所有权移入闭包

4. **为什么 `Rc<T>` 不是 `Send`**：
   - `Rc` 的引用计数不是原子操作
   - 两个线程同时 clone/drop `Rc` 会产生数据竞争
   - 用 `Arc<T>`（Atomic Reference Count）代替

5. **`Sync`**：类型的引用可以安全地在多个线程间共享
   - `Mutex<T: Send>` → `Sync`
   - `RefCell<T>` → 不是 `Sync`（内部可变性不是线程安全的）

### 收获

- `Send`/`Sync` 是 Rust 线程安全的核心抽象——它们通过类型系统在编译期消除数据竞争
- `move` 闭包不是"拷贝数据"，而是转移所有权——编译器保证原始数据不再被使用
- `Arc` vs `Rc` 的选择不是性能问题，而是正确性问题（编译器强制执行）

---

## Ep 2: Declarative Macros

> **一句话**：`macro_rules!` 不是函数，而是一种**在编译期匹配 Rust 语法片段并展开成新代码**的模板系统；它让我们在保持类型安全的同时写出 `vec![]`、`println!` 这类可变长、零开销的便捷宏。

### 核心概念

以下内容基于 Rust 2021 edition（Rust ≥1.56），其中 `$(,)?` 等重复语法需要 Rust ≥1.32。

### 1. 宏与函数的根本区别

函数调用发生在运行期，实参被求值后压入栈帧；宏调用发生在编译期，Rust 先把宏**展开成普通代码**，然后再做类型检查。

| 特性 | 函数 | 声明宏 `macro_rules!` |
|------|------|----------------------|
| 调用时机 | 运行期 | 编译期（展开后编译） |
| 参数类型 | 必须预先声明类型 | 按语法片段匹配（如 `expr`, `ty`, `ident`） |
| 可变参数 | 依赖数组/切片/vec | 原生支持重复匹配 `$(...)*` |
| 类型检查 | 签名即约束 | 展开后的代码决定类型检查 |
| 返回位置 | 语句或表达式 | 表达式、语句、类型、模式、item 均可 |

这意味着宏展开后生成的代码中的**类型错误和借用检查错误**要到展开后才暴露，调试时需要用 `cargo expand`（需 `cargo install cargo-expand`）或 `rustc -Zunpretty=expanded` 查看展开结果。宏定义本身的语法错误（如括号不匹配、fragment specifier 非法、重复次数不一致）会在定义处直接报错。

### 2. 最简单的宏：`vec!` 的朴素实现

我们先从 `vec!` 宏开始，因为它最直观：把输入的若干表达式收集成 `Vec`。

```rust
macro_rules! my_vec {
    // 空调用：vec![] -> Vec::new()
    () => {
        Vec::new()
    };
    // 非空调用：vec![1, 2, 3]
    ($($x:expr),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
}
```

#### 关键点解析

- `$(...)`: 重复组；`$x:expr` 表示匹配一个表达式片段。
- `,+`: 逗号分隔，至少出现一次（`+` 等价正则的 `+`）。
- `$(,)?`: 可选的尾随逗号，保持与标准库一致的用户体验。
- 展开时 `$x` 与 `+` 的重复数量必须一致——不能在匹配时用 `,+` 却在展开侧用 `*`。

### 3. 重复模式：`*`, `+`, `?`

| 操作符 | 含义 | 示例匹配 |
|--------|------|----------|
| `*` | 零次或多次 | `vec![]`, `vec![1]`, `vec![1, 2, 3]` |
| `+` | 一次或多次 | `vec![1]`, `vec![1, 2, 3]`（不匹配空） |
| `?` | 零次或一次 | 可选片段 |

如果要同时支持空与非空，最简洁的规则是单条 `$(...)*`，但内部展开需要构造与重复数量对应的代码：

```rust
macro_rules! vec_literal {
    ($($x:expr),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}
```

注意：展开侧 `$(v.push($x);)*` 中的 `*` 与匹配侧数量一致；如果匹配侧有多个 metavariable 处于同一重复组，它们必须被重复**相同次数**，否则编译器会报错。

### 4. 卫生性（Hygiene）：局部变量与标签不会污染外部作用域

Rust 的声明宏是**部分卫生（hygienic）**的：宏内部引入的局部变量和标签不会与外部冲突。

> 注意：生命周期名在 `macro_rules!` 展开体中**不是 hygienic**——如果调用点存在同名生命周期（如 `'a`），展开体中的 `'a` 会按调用点解析，可能产生意外绑定。需要隔离生命周期时，应使用新版声明宏 `macro`（`macro_rules!` 的继任者）或在宏内部用唯一前缀命名。

```rust
macro_rules! double {
    ($x:expr) => {
        {
            let n = $x;  // 这个 `n` 与外部 `n` 不是同一个标识符
            n + n
        }
    };
}

fn main() {
    let n = 10;
    let m = double!(n);  // 展开后内部的 `n` 不会覆盖这里的 `n`
    assert_eq!(m, 20);
    assert_eq!(n, 10);   // 外部 `n` 仍是 10
}
```

这是 `macro_rules!` 相比 C 宏的巨大优势：宏里的 `let n` 不会意外捕获或污染调用者的 `n`。但卫生性只覆盖局部变量和标签，**宏外部引用的路径**（例如 `Vec::new()`）会按调用点作用域解析，所以跨 crate 调用时可能遇到路径问题。

### 5. 跨 crate 引用：使用 `$crate`

`$crate` 是宏展开时的特殊标识符，始终指向定义该宏的 crate 根。它不是 hygiene 机制——它解决的是「我的 crate 叫什么名字」这个路径问题，而非标识符冲突问题。

当宏定义在一个 crate 里，调用方却在另一个 crate 时，宏内部直接写 `Vec::new()` 没问题，但如果写你的 crate 里的私有 helper，就必须用 `$crate`：

```rust
// 假设这是 mylib crate 的宏
pub fn make_bytes() -> Vec<u8> {
    vec![0, 1, 2]
}

macro_rules! bytes {
    () => {
        $crate::make_bytes()  // 正确：无论调用方 crate 路径是什么都指向 mylib
    };
}
```

**陷阱**：`$crate::make_bytes()` 只解决路径问题，不改变可见性。如果 `make_bytes` 不是 `pub`，外部 crate 仍然无法调用。

### 6. 宏的导入与导出

宏默认只在定义它的模块内可见。要让其他模块或 crate 使用，需要声明 `#[macro_export]`：

```rust
#[macro_export]
macro_rules! say_hello {
    () => {
        println!("hello from macro!");
    };
}
```

被 `#[macro_export]` 导出的宏会被提升到 crate 根，因此通过 `use mycrate::say_hello;` 即可使用。注意：它同时也脱离了原始模块的路径，因此内部再次用 `use super::...` 引用模块内物品时要小心。

### 7. 更复杂的重复：匹配 key-value 对

宏不仅可以处理逗号分隔列表，还能处理 `key => value` 这类结构，映射到 HashMap 或自定义数据：

```rust
macro_rules! map {
    // 空 map
    () => {
        ::std::collections::HashMap::new()
    };
    // 形如 map! { "a" => 1, "b" => 2 }
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )*
            m
        }
    };
}

fn main() {
    let m = map! {
        "one" => 1,
        "two" => 2,
    };
    assert_eq!(m.get("one"), Some(&1));
}
```

这里用 `::std::collections::HashMap::new()` 而不是 `HashMap::new()`，是为了避免调用方没有 `use` HashMap 导致名字解析失败。在标准宏中这种**绝对路径**是常见做法。

### 8. 与过程宏（Procedural Macros）的区别

声明宏写起来快、无需额外 crate，但有两个主要限制：

1. **错误信息不友好**：匹配失败时只会告诉你没有匹配的规则，无法给出语义化错误。
2. **表达能力受限**：只能做语法层面匹配，无法访问类型信息或做复杂计算。

过程宏（自定义 derive、attribute-like、function-like）可以编译期解析 TokenStream，实现更复杂的代码生成，但通常需要单独的 proc-macro crate。

### 踩坑清单

1. **忘记尾随逗号**：`
($($x:expr),* $(,)?)` 的 `$(,)?` 虽然可选，但缺少它会让 `vec![1, 2, 3,]` 编译失败，与标准库行为不一致。

2. **重复数量不一致**：匹配侧用 `$(...),+`（至少一次），展开侧用 `$(...)*`（零次或多次）会报错。每个 metavariable 在展开侧必须保持与匹配侧相同的重复次数。

3. **把宏当函数用**：宏的参数先匹配语法片段，再展开代码。`my_vec![1 + 2, 3 * 4]` 没问题，但 `my_vec![return 1;]` 会作为表达式片段匹配，展开后可能产生意想不到的语句边界问题。

4. **忽略 hygiene 边界**：宏内部变量不会污染外部，但宏外部引用的类型/路径是按调用点解析的。跨 crate 宏内部引用自己的物品时务必用 `$crate`。

5. **宏规则顺序导致歧义**：`macro_rules!` 按规则从上到下匹配，第一条匹配成功的规则立即展开。把更宽泛的规则放在更具体的规则前面，会导致具体规则永远匹配不到。

6. **忘记 `#[macro_export]`**：宏默认私有于模块，即使所在模块 `pub` 也不会自动导出给 crate 外使用。

### 收获

- `macro_rules!` 是**模板 + 模式匹配**：输入的 Rust 语法片段被捕获成 metavariable，再按模板展开成普通代码。
- **重复 `$(...)* / + / ?`** 是声明宏的核心武器，让可变长参数和重复代码生成在编译期零开销完成。
- **卫生性**保证宏内部变量不会与调用者冲突，但路径解析仍需按 Rust 模块规则处理，跨 crate 宏用 `$crate` 绝对路径。
- **与过程宏不同**：声明宏适合简单、结构化的代码生成；需要类型级操作或自定义错误时转向过程宏。
- 调试宏的黄金法则是查看展开结果：`cargo expand`（需 `cargo install cargo-expand`）或 `rustc -Zunpretty=expanded`。

### 交叉链接

- → [所有权模型](../ownership-lifetimes/ownership.md) — 宏本身不做运行时检查，展开后的代码仍受所有权系统约束
- → [引用与借用](../ownership-lifetimes/reference-borrow.md) — 宏中 `&mut` 和 `&` 的展开仍遵循借用规则
- → [生命周期基础](../ownership-lifetimes/lifetime-basic.md) — 宏返回引用时同样要求生命周期标注
- → [生命周期进阶](../ownership-lifetimes/lifetime-advanced.md) — 宏与 HRTB、协变结合使用时的边界情况
- → [Trait 与泛型](../basic/trait-generic.md) — 宏与泛型互补：宏生成代码，泛型复用代码

---

## Ep 3: Lifetime Annotations

> **一句话**：生命周期标注不是"给引用续命"的咒语，而是**在类型系统里显式写出引用之间的存活关系**；当多个引用交织、型变规则与 HRTB 同时登场时，漏写一个 `'a` 就会让编译器拒绝一份本可安全的代码。

### 1. 为什么需要多个生命周期

初学者常犯的错误是：凡是引用都用同一个 `'a`。这在简单场景下能编译，但一旦涉及"输入两个不同来源的引用、返回其中一个"，单生命周期就会过度约束。

```rust
// 错误示范：两个输入和一个输出共享同一个 'a
fn bad_merge<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let local = String::from("short");
    let result;
    {
        let temp = String::from("loooooong");
        result = bad_merge(&local, &temp); // ❌ 编译失败
    }
    println!("{}", result);
}
```

编译器要求 `local` 和 `temp` 活得一样久，因为签名说它们共享 `'a`。实际上我们只要求**返回的那个引用**活得够久即可。

正确做法是为每个独立引用分配独立生命周期，再用约束表达它们的关系：

```rust
fn good_merge<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    // 返回的引用只与 x 绑定，y 的生命周期不受约束
    x
}

// 如果需要返回两者之一，则要求两个生命周期至少覆盖返回值
fn pick<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a, // 'b 比 'a 长（或相等），所以 &'b str 可以安全收窄为 &'a str
{
    if x.len() > y.len() { x } else { y }
}
```

**核心原则**：生命周期参数的数量 = 代码中**独立引用来源**的数量，不是语法上 `&` 出现的次数。

### 2. 子类型关系：`'long: 'short`

Rust 的子类型只发生在生命周期之间。`'a: 'b` 读作 "`'a` outlives `'b`"，即 `'a` 的存活范围包含 `'b`。

```rust
fn shrink<'a, 'b>(x: &'a str)
where
    'a: 'b,
{
    let y: &'b str = x; // ✅ 'a 比 'b 长，&'a str 可以当成 &'b str 用
    println!("{}", y);
}

fn main() {
    let s = String::from("hello");
    shrink(&s); // 这里 'a 和 'b 都被实例化为 s 的生命周期，满足约束
}
```

这与面向对象里的继承子类型完全不同：Rust 没有结构子类型，只有**生命周期子类型**。`'static` 是最"大"的生命周期，因此 `'static: 'a` 对任何 `'a` 都成立——这就是为什么 `&'static str` 可以传给任何接受 `&str` 的函数。

### 3. 型变（Variance）：子类型如何穿透类型构造器

型变回答：如果 `'a` 是 `'b` 的子类型，那么 `F<'a>` 与 `F<'b>` 是什么关系？

| 型变 | 含义 | 典型类型 |
|------|------|----------|
| **协变 (Covariant)** | 子类型关系同向传递 | `&'a T`, `Box<T>`, `Vec<T>`, `*const T` |
| **逆变 (Contravariant)** | 子类型关系反向传递 | `fn(T) -> U` 对参数 `T` |
| **不变 (Invariant)** | 子类型关系不传递 | `&'a mut T`, `Cell<T>`, `UnsafeCell<T>`；`fn(T) -> T` 综合参数与返回值后为不变 |

#### 3.1 协变：引用收窄是安全的

```rust
fn take_any<'a>(s: &'a str) {}

fn main() {
    let s: &'static str = "forever";
    take_any(s); // ✅ &'static str 协变为 &'_ str
}
```

`&'a T` 对 `'a` 是协变的：长生命周期引用可以自动收窄为短生命周期引用。这是安全的，因为只读引用不会通过它修改数据。

#### 3.2 逆变：函数参数位置反转

```rust
fn takes_static(s: &'static str) {}

fn caller<'a>(f: fn(&'a str), local: &'a str) {
    f(local);
}

fn main() {
    // 注意：fn(&'a str) 可以传给需要 fn(&'static str) 的位置
    // 因为函数参数是逆变的：接受更短引用的函数，也能接受更长的
    let f: fn(&'a str) = caller;
    // 但这里不能直接传入 takes_static，需要额外包装，因为逆变规则较隐晦
}
```

逆变在 Rust 日常代码中较少直接感知，但它解释了为什么**函数指针的参数位置**不能随意替换。

#### 3.3 不变：`&mut T` 为什么如此严格

```rust
fn overwrite<'a>(x: &'a mut String, y: &'a mut String) {
    std::mem::swap(x, y);
}

fn bad<'a>(x: &'a mut String) {
    let mut short = String::from("temp");
    let r: &'a mut String = &mut short; // ❌ 编译失败
    // 如果允许，r 变量在函数返回后被释放，x 仍可能指向已释放的 short
}
```

`&'a mut T` 对 `'a` 和 `T` 都是**不变的**。如果允许协变，我们就可以把一个 `&mut &'static str` 当成 `&mut &'a str` 使用，然后往里面写入一个短生命周期引用——这就是经典的**生命周期缩小导致的 use-after-free**。

### 4. 高阶 Trait Bound：`for<'a>`

有时我们需要表达"这个类型对**所有**生命周期都满足某约束"，而不是被某个具体生命周期参数绑定。这就是 HRTB（Higher-Ranked Trait Bounds）。

```rust
// 这个函数要求 F 必须能接受任意生命周期的 &str，并返回同生命周期的 &str
fn apply_to_all<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let local = String::from("hello");
    let result = f(&local);
    println!("{}", result); // result 的生命周期与 local 绑定
}

fn identity(s: &str) -> &str { s }

fn main() {
    apply_to_all(identity); // ✅ identity 对所有 'a 都成立
}
```

对比没有 HRTB 的版本：

```rust
// 错误示范：试图返回与输入生命周期无关的 &str，但签名未约束
fn broken<'a>(s: &'a str) -> &str {
    let local = String::from("hello");
    // ❌ 编译失败：返回值的生命周期被推断为 &'local，但签名说返回 &str
    // 编译器将其补全为 &'a str，而 &local 无法活到 'a
    &local
}
```

这个例子展示了**为什么 HRTB 是必要的**：当函数签名没有正确表达"返回值与输入引用同生命周期"时，试图返回局部引用会导致编译失败。HRTB 让 trait bound 能表达"对所有生命周期都成立"，而不是被某个具体生命周期绑定。

HRTB 的典型应用场景：
- **闭包 trait**：`Fn(&str) -> &str` 在复杂泛型上下文中经常需要显式写成 `for<'a> Fn(&'a str) -> &'a str`
- **Trait 对象**：`Box<dyn for<'a> Fn(&'a str) -> &'a str>`
- **序列化/反序列化框架**：如 `serde::Deserialize` 经常需要 `for<'de> Deserialize<'de>`

### 5. Trait 对象的生命周期

Trait 对象默认携带生命周期约束。写 `Box<dyn Trait>` 时，编译器会隐式补全为 `Box<dyn Trait + 'static>`——这常常成为意外编译错误的来源。

```rust
trait Parser {
    fn parse(&self, input: &str) -> bool;
}

struct SliceParser<'a> {
    pattern: &'a str,
}

impl<'a> Parser for SliceParser<'a> {
    fn parse(&self, input: &str) -> bool {
        input.contains(self.pattern)
    }
}

fn make_parser() -> Box<dyn Parser> {
    let local = String::from("needle");
    // ❌ 编译失败：Box<dyn Parser> 隐式要求 'static
    Box::new(SliceParser { pattern: &local })
}

// 正确做法：显式标注生命周期
fn make_parser_explicit<'a>(pattern: &'a str) -> Box<dyn Parser + 'a> {
    Box::new(SliceParser { pattern })
}
```

**规则**：
- `Box<dyn Trait>` = `Box<dyn Trait + 'static>`
- `&'a dyn Trait` = `&'a (dyn Trait + 'a)`
- 如果 trait 对象内部包含非 `'static` 引用，必须显式写出 `+ 'a`

### 6. `'static` 的常见误解

`'static` 不是"永远存在"的意思，而是"**不借用任何非 static 数据**"——它描述的是类型的内部结构，不是值在内存中的实际存活时间。

```rust
// 常见误解：认为 'static 意味着变量永不被释放
fn takes_static<T: 'static>(t: T) {}

fn main() {
    let s = String::from("I am not static");
    // ✅ String 是 'static：它拥有所有数据，内部不含非 'static 引用
    takes_static(s);

    // ✅ &'static str 是 'static，因为它指向编译期嵌入的字符串字面量
    takes_static("literal");

    // ✅ 拥有所有权的 i32 也是 'static（不包含任何引用）
    takes_static(42);

    // ⚠️ 常见误解：&String 不是 'static，因为它借用了可能很快被释放的 String
    // let r: &'static String = &String::from("temp"); // ❌ 编译失败
}
```

另一个陷阱是闭包与 `thread::spawn`：

```rust
fn spawn_with_data(data: &str) {
    // thread::spawn 要求 F: FnOnce() + Send + 'static
    // 如果闭包捕获了 &str，整个闭包就不是 'static
    // std::thread::spawn(move || println!("{}", data)); // ❌

    // 正确做法：把数据所有权移入闭包
    let owned = data.to_string();
    std::thread::spawn(move || println!("{}", owned)); // ✅
}
```

### 7. 省略规则（Elision）的边界

Rust 允许在函数签名中省略生命周期，但有三条严格规则，超出范围就必须手写：

1. **每个输入引用**获得独立生命周期参数
2. **单个输入引用**时，输出引用获得相同生命周期
3. **`&self` 或 `&mut self`** 时，输出引用获得 `self` 的生命周期

```rust
// 规则 2：单输入，可省略
fn first_word(s: &str) -> &str { ... }
// 等价于：fn first_word<'a>(s: &'a str) -> &'a str

// 多输入时不能省略——编译器不知道返回与哪个输入关联
fn longest(x: &str, y: &str) -> &str { ... } // ❌ 编译失败

// 必须显式标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

## 踩坑清单

1. **所有引用共享一个 `'a`**：当函数有多个独立来源的引用输入时，给每个来源独立的生命周期参数，只在需要时加 `where 'b: 'a` 约束。

2. **忽略 `&mut T` 的不变性**：试图把 `&mut &'long str` 转成 `&mut &'short str` 会编译失败——这是设计如此，不是编译器 bug。

3. **Trait 对象隐式 `'static`**：`Box<dyn Trait>` 默认要求 `'static`，如果内部包含短生命周期引用，必须写成 `Box<dyn Trait + 'a>`。

4. **把 `'static` 当成运行时期限**：`'static` 是类型属性，不是 GC 保证；局部变量可以通过 `Box::leak` 获得 `'static` 引用，但代价是永久泄漏内存。

5. **在返回引用时忘记输入生命周期**：`fn get() -> &str` 没有输入生命周期，编译器会推断为 `&'static str`；如果实际返回的是局部引用，编译会报错。

6. **HRTB 写错位置**：`for<'a>` 只能出现在 trait bound 前，不能用于类型定义或普通函数签名。

## 关键收获

- **生命周期是子类型关系**：`'a: 'b` 表示 `'a` 的范围包含 `'b`，长引用可以安全收窄为短引用。
- **型变决定子类型能否穿透**：`&T` 协变、`&mut T` 不变、函数参数逆变——理解它们才能解释编译器的拒绝理由。
- **HRTB 解耦生命周期与泛型参数**：`for<'a>` 让类型对所有生命周期成立，是闭包 trait 和 trait 对象高级用法的钥匙。
- **Trait 对象有隐藏生命周期**：`Box<dyn Trait>` 不是无生命周期的，它隐式携带 `'static`，忘记这一点会在泛型抽象时碰壁。
- **`'static` 描述的是类型结构**：只要类型内部不含非 static 引用，它就是 `'static`；这与值实际活多久无关。

## 交叉链接

- → [生命周期基础](../ownership-lifetimes/lifetime-basic.md) — 前置语法：省略规则、结构体生命周期标注
- → [生命周期进阶](../ownership-lifetimes/lifetime-advanced.md) — 深入型变推导、HRTB 形式化定义与 `Pin` 的关系
- → [所有权模型](../ownership-lifetimes/ownership.md) — 为什么 `&mut T` 必须是不变的：所有权与可变性共同决定型变规则
- → [智能指针](../ownership-lifetimes/smart-pointer.md) — `Box`, `Rc`, `Arc` 对生命周期的影响及协变行为
- → [自引用结构](../ownership-lifetimes/self-referential.md) — 当生命周期与 `Pin` 结合，解决自引用结构体的安全难题

---

> 视频列表：[Crust of Rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) by Jon Gjengset
