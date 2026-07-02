# 生命周期编译错误案例

> 一句话定位：这里是 Rust 编译器报出的生命周期相关错误实战汇编，每个案例按“错码 → 报错 → 解释 → 修复”四段拆解，目标是从错误信息反推借用规则，而不是背诵签名。

---

## 与 JS/TS 的关键差异

JS/TS 里没有“生命周期”这一编译期概念。对象引用在垃圾回收器（GC）管理下可以任意传递、返回、闭包捕获，只要运行时还有可达路径就不会悬垂。Rust 没有 GC，引用的有效性必须在编译阶段被证明：函数签名、结构体字段、闭包捕获都必须显式或隐式地声明“借用能活多久”。这带来的最大体感差异是：在 JS 里你可以返回一个局部数组的切片或让闭包引用局部变量；在 Rust 里，这些行为会被编译器在写代码时直接拒绝，而不是在运行时才偶尔出现 `undefined` 或内存异常。

| 场景 | Rust 行为 | JS/TS 行为 |
|---|---|---|
| 返回局部变量引用 | 编译错误（E0515 / E0597） | 运行时可能 GC 保留，也可能 `undefined` |
| 结构体存引用 | 必须写 `<'a>` 生命周期参数 | 对象属性可直接引用另一对象 |
| 同时读写同一数据 | 编译错误（E0502 / E0506） | 允许，可能产生时序 bug |
| 闭包捕获局部变量 | 必须 `move` 进闭包，否则报错 | 闭包自然捕获变量引用 |
| 函数返回两个输入中的较长引用 | 需要单独生命周期参数 `'b: 'a` | 无需考虑 |

---

## 容易踩的坑

1. 看到 `cannot return reference to temporary value`，第一反应不是改返回类型，而是检查“返回的引用到底是谁拥有的”。
2. 给结构体加生命周期时不要只加字段，忘记 `struct Foo<'a>` 头上的参数声明。
3. 可变借用与不可变借用冲突时，优先“缩小引用作用域”而不是到处加 `clone()`。
4. 闭包想返回时，局部变量要么 `move` 进去，要么放进 `Arc`/channel，不能直接借用。
5. 给函数参数统一标同一个 `'a` 不一定对；返回值的生命周期通常应该由“最短的那个输入”或单独参数决定。

---

## 交叉链接

- 概念层：[生命周期基础](../../ownership-lifetimes/lifetime-basic.md) · [引用与借用](../../ownership-lifetimes/reference-borrow.md)
- 深入层：[Crust of Rust 笔记](../../deep-dives/crust-of-rust-notes.md)（Send / Sync / `'static` / move 闭包）
- 练习层：见 `exercises/src/lifetimes.rs`（rustlings 第 14 章）

---

## Case 1: 返回临时引用的生命周期

### 错码

```rust
fn first_word(s: &str) -> &str {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[0]  // error: cannot return reference to temporary value
}
```

### 报错

```text
error[E0515]: cannot return reference to local variable `words`
 --> src/main.rs:3:5
  |
3 |     words[0]
  |     ^^^^^^^^ returns a reference to data owned by the current function
```

### 解释

`words` 在函数内创建，函数返回时被 drop。`words[0]` 是 `words` 中元素的引用，函数结束后变成悬垂指针。编译器拒绝。

### 修复

```rust
// 方案 1：返回 String（拥有所有权）
fn first_word(s: &str) -> String {
    s.split_whitespace().next().unwrap().to_string()
}

// 方案 2：返回原始 &str 的切片（借用传入参数）
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

---

## Case 2: 结构体生命周期缺失

### 错码

```rust
struct User {
    name: &str,  // error: missing lifetime specifier
    email: &str,
}
```

### 报错

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:11
  |
2 |     name: &str,
  |           ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 | struct User<'a> {
2 |     name: &'a str,
```

### 解释

结构体持有引用时，必须标注生命周期——告诉编译器“这个结构体不能比它引用的数据活得更久”。

### 修复

```rust
struct User<'a> {
    name: &'a str,
    email: &'a str,
}
```

---

## Case 3: 不同生命周期的借用冲突

### 错码

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let r = &v[0];       // 不可变借用
    v.push(4);           // error: cannot borrow `v` as mutable
    println!("{r}");
}
```

### 报错

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let r = &v[0];
  |              - immutable borrow occurs here
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("{r}");
  |               --- immutable borrow later used here
```

### 解释

Rust 的借用规则：不能同时有不可变借用 `r` 和可变借用 `v.push()`。NLL（Non-Lexical Lifetime）会追踪引用的实际使用范围，但 `println!("{r}")` 在 `v.push(4)` 之后，所以 `r` 的借用范围跨越了 `v.push(4)`。

### 修复

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let r = &v[0];
    println!("{r}");     // 先用完 r
    v.push(4);           // 然后可变借用
}
```

---

## Case 4: 闭包捕获引用的生命周期

### 错码

```rust
fn create_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    || { count += 1; count }  // error: closure may outlive `count`
}
```

### 报错

```text
error[E0373]: closure may outlive the current function, but it borrows `count`
 --> src/main.rs:3:5
  |
3 |     || { count += 1; count }
  |     ^^^      ----- `count` is borrowed here
  |     |
  |     may outlive borrowed value `count`
  |
note: `count` is declared here
 --> src/main.rs:2:9
  |
2 |     let mut count = 0;
```

### 解释

闭包捕获了局部变量 `count` 的引用。当函数返回闭包时，`count` 已经被 drop 了——闭包持有的引用变成悬垂。

### 修复

```rust
fn create_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || { count += 1; count }  // move 将 count 的所有权移入闭包
}
```

---

## Case 5: 生命周期标注过度约束

### 错码

```rust
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("hello");
    let result;
    {
        let s2 = String::from("world");
        result = longer(&s1, &s2);
    }  // s2 在这里 drop
    println!("{result}");  // error: `s2` does not live long enough
}
```

### 报错

```text
error[E0597]: `s2` does not live long enough
  --> src/main.rs:8:31
   |
7  |         let s2 = String::from("world");
   |             -- binding `s2` declared here
8  |         result = longer(&s1, &s2);
   |                               ^^ borrowed value does not live long enough
9  |     }
   |     - `s2` dropped here while still borrowed
10 |     println!("{result}");
   |              -------- borrow later used here
```

### 解释

`longer<'a>` 要求 `x` 和 `y` 有相同的生命周期 `'a`。`&s2` 的生命周期短于 `&s1`，但返回值的生命周期被约束为 `'a`（等于较短者），所以 `result` 在 `s2` drop 后不能用。

### 修复

```rust
// 方案 1：如果语义允许，让 result 留在 s2 的作用域内
fn main() {
    let s1 = String::from("hello");
    {
        let s2 = String::from("world");
        let result = longer(&s1, &s2);
        println!("{result}");  // result 只在这里使用
    }
}

// 方案 2：如果确实需要返回与较长输入同生命周期的引用，
// 使用两个生命周期参数，并声明 outlives 关系
fn longer_v2<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```