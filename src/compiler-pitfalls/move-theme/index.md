# Move 语义编译错误案例

> 一句话定位：这里是 Rust 编译器报出的 move 相关错误实战汇编，每个案例按“错码 → 报错 → 解释 → 修复”四段拆解，目标是从错误信息反推所有权规则，而不是背诵 `Copy` / `Clone` 的定义。

---

## 与 JS/TS 的关键差异

JS/TS 里没有编译期所有权检查。对象、数组、函数在传递时共享引用，运行时由垃圾回收器决定何时释放；浅拷贝和深拷贝是开发者自己操心的事。Rust 在编译阶段就要回答“这份数据属于谁、被谁消费、被谁释放”。当值的所有权被转移（move）后，原变量会变成“未初始化”状态，继续使用就会触发编译错误。

| 场景 | Rust 行为 | JS/TS 行为 |
|---|---|---|
| 把变量赋给另一个变量 | 如果未实现 `Copy`，原变量被 move | 共享引用，两边都能用 |
| 函数传参 | 值语义类型会转移所有权 | 对象/数组按引用传递 |
| 循环里重复消费同一值 | 第二次消费报错 use of moved value | 每次循环共享同一引用 |
| 闭包捕获外部变量 | 默认按引用捕获；move 闭包会转移所有权 | 闭包自然按引用捕获 |
| 条件分支里使用变量 | 不同分支可能分别 move，导致后续不可用 | 随便用，没有限制 |
| 函数返回值 | 返回值是新的所有者；调用点之后原变量不可用 | 原变量始终可用 |

---

## 容易踩的坑

1. 看到 `use of moved value`，不要第一反应 `clone()`，先检查是否该用引用、`&str` 替代 `String`，或者把变量重新声明在循环内部。
2. 闭包捕获 `String` 时，`move` 闭包会让外部变量失效；如果闭包只执行一次，可以考虑用 `let s = s;` 显式 shadow 再 move。
3. 条件分支里 `if ok { take(v) } else { keep(v) }` 这种写法在 Rust 里行不通，因为两个分支都会尝试获取 `v` 的所有权。
4. 函数返回拥有所有权的值不代表调用者“保留原变量”——调用者拿到的是新所有权，原变量已经被 move 进函数。
5. `Copy` 类型（如 `i32`、`bool`、固定大小数组）不会被 move，赋值和传参会复制；非 `Copy` 的堆分配类型（`String`、`Vec`、`HashMap`）才会 move。

---

## 交叉链接

- 概念层：[所有权基础](../../ownership-lifetimes/ownership.md) · [引用与借用](../../ownership-lifetimes/reference-borrow.md)
- 深入层：[Crust of Rust 笔记](../../deep-dives/crust-of-rust-notes.md)（Send / Sync / `'static` / move 闭包）
- 练习层：见 `exercises/src/move_semantics.rs`（rustlings move semantics 章节）

---

## Case 1: use of moved value（基础 move）

### 错码

```rust,ignore
fn main() {
    let s = String::from("hello");
    let t = s;          // 所有权 move 到 t
    println!("{s}");    // error: use of moved value
}
```

### 报错

```text
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:4:15
  |
2 |     let s = String::from("hello");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     let t = s;
  |             - value moved here
4 |     println!("{s}");
  |               ^ value borrowed here after move
```

### 解释

`String` 是堆分配类型，没有实现 `Copy`。`let t = s;` 把 `s` 拥有的堆内存转移给 `t`，`s` 变成未初始化。`println!("{s}")` 的展开形式会借用 `s`，而借用一个已 move 的变量是编译器拒绝的。

### 修复

```rust
fn main() {
    let s = String::from("hello");
    let t = s.clone();    // 显式深拷贝，s 仍拥有原数据
    println!("{s}");      // OK
    println!("{t}");      // OK
}
```

---

## Case 2: 循环中重复 move

### 错码

```rust,ignore
fn consume(s: String) {
    println!("{s}");
}

fn main() {
    let name = String::from("rust");
    for _ in 0..3 {
        consume(name);    // error: use of moved value
    }
}
```

### 报错

```text
error[E0382]: use of moved value: `name`
 --> src/main.rs:7:17
  |
5 |     let name = String::from("rust");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
6 |     for _ in 0..3 {
7 |         consume(name);
  |                 ^^^^ value moved into loop body, in subsequent iteration
  |                 |
  |                 value used here after move
```

### 解释

第一次调用 `consume(name)` 时，`name` 的所有权被转移到 `consume` 的参数并在函数返回时 drop。第二次循环迭代再次使用 `name` 时，它已经不存在了。循环体不像 `Copy` 类型那样会自动“重置”变量。

### 修复

```rust
fn consume(s: String) {
    println!("{s}");
}

fn main() {
    let name = String::from("rust");
    for _ in 0..3 {
        consume(name.clone());    // 每次迭代传入一个副本
    }
}
```

方案 2：如果 `consume` 不需要所有权，改为借用

```rust
fn print(s: &str) {
    println!("{s}");
}

fn main() {
    let name = String::from("rust");
    for _ in 0..3 {
        print(&name);               // 每次只借不可变引用
    }
}
```

---

## Case 3: 闭包捕获后 move

### 错码

```rust,ignore
fn main() {
    let s = String::from("data");

    let c = || {
        println!("{s}");    // 闭包捕获 s 的引用
    };

    c();
    println!("{s}");        // 这里仍然想用 s

    let moved = move || {
        println!("{s}");    // 把 s 的所有权 move 进闭包
    };
    moved();
    println!("{s}");        // error: use of moved value
}
```

### 报错

```text
error[E0382]: borrow of moved value: `s`
  --> src/main.rs:15:15
   |
4  |     let c = || {
   |             -- ...
12 |     let moved = move || {
   |                 ------- `s` moved into closure here
...
15 |     println!("{s}");
   |               ^ value borrowed here after move
```

### 解释

`move` 闭包会把它捕获的环境变量按值移入闭包体内。`s` 一旦进入 `moved` 闭包，外部作用域就不再拥有它。即使闭包已经执行完毕，`s` 也随着闭包的 drop 而释放，外部不能再访问。

### 修复

```rust
fn main() {
    let s = String::from("data");

    let c = || {
        println!("{s}");    // 非 move 闭包按引用捕获，s 仍可用
    };

    c();
    println!("{s}");        // OK

    // 如果闭包必须 move，且之后仍需要 s，先 clone
    let s_clone = s.clone();
    let moved = move || {
        println!("{s_clone}");
    };
    moved();
    println!("{s}");        // OK
}
```

---

## Case 4: 条件分支中的 move

### 错码

```rust,ignore
fn take(s: String) -> String { s }

fn main() {
    let v = String::from("value");
    let ok = true;

    if ok {
        take(v);            // 分支 A 尝试 move v
    } else {
        take(v);            // 分支 B 也尝试 move v → error
    }

    println!("{v}");        // 同样不可用
}
```

### 报错

```text
error[E0382]: use of moved value: `v`
 --> src/main.rs:8:14
  |
4 |     let v = String::from("value");
  |         - move occurs because `v` has type `String`, which does not implement the `Copy` trait
5 |     if ok {
6 |         take(v);
  |              - value moved here
7 |     } else {
8 |         take(v);
  |              ^ value used here after move
```

### 解释

Rust 的所有权分析不考虑运行时条件。编译器看到两个分支都试图取得 `v` 的所有权，于是报错。即使运行时只有一个分支会执行，编译器也要求在静态上保证 `v` 不会被重复 move。同理，`println!("{v}")` 在分支之后也会失败，因为 `v` 已经在某个分支里被 move 了。

### 修复

```rust
fn take(s: String) -> String { s }

fn main() {
    let v = String::from("value");
    let ok = true;

    // 方案 1：按需 clone
    if ok {
        take(v.clone());
    } else {
        take(v.clone());
    }
    println!("{v}");        // OK，因为 v 仍保留所有权

    // 方案 2：在 match / if 表达式中消费，并把结果绑定给新变量
    let result = if ok {
        take(v)             // 这里 move v
    } else {
        take(v)              // 仍然报错：不能两边都 move
    };

    // 正确写法：只有一个分支能消费 v，或者让 take 返回后继续使用
    let v = String::from("value");
    let result = if ok {
        take(v)
    } else {
        v                    // 另一个分支不消费，直接返回 v
    };
    println!("{result}");    // OK
}
```

---

## Case 5: 函数返回后 move

### 错码

```rust,ignore
fn build() -> String {
    String::from("built")
}

fn main() {
    let s = build();        // 所有权从函数返回值转移到 s
    println!("{s}");        // OK

    let t = s;              // 现在所有权 move 到 t
    println!("{s}");        // error: use of moved value
}
```

### 报错

```text
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:10:15
  |
7 |     let s = build();
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
8 |     println!("{s}");
  |               ----- ...
9 |     let t = s;
  |             - value moved here
10 |     println!("{s}");
   |               ^ value borrowed here after move
```

### 解释

`build()` 返回的 `String` 是一个临时值，被绑定到 `s` 后所有权由 `s` 持有。这看起来像是“新建了一份数据”，但所有权规则仍然适用：`let t = s;` 把 `s`  move 到 `t`，之后 `s` 不再可用。很多初学者误以为“函数返回的是新值，所以可以无限复制”，实际上返回的是新所有者，不是可复制的副本。

### 修复

```rust
fn build() -> String {
    String::from("built")
}

fn main() {
    let s = build();
    println!("{s}");        // 先用 s

    let t = s.clone();      // 需要保留 s 则显式 clone
    println!("{s}");        // OK
    println!("{t}");        // OK

    // 如果 t 是最终所有者，后面不要再碰 s
    let s = build();
    let t = s;                // t 拥有数据
    println!("{t}");          // OK
}
```

---

## 小结

| 错误模式 | 核心原因 | 常见修复 |
|---|---|---|
| use of moved value | 非 `Copy` 类型被赋值/传参/返回后原变量失效 | 用 `.clone()`、改用 `&str`/`&T` 借用、或重构作用域 |
| 循环中重复 move | 每次迭代都想消费同一所有权变量 | 循环内 clone，或把变量声明在循环里，或改为借用 |
| 闭包捕获后 move | `move` 闭包把外部变量所有权移入闭包 | 非 move 闭包按引用捕获；必要时先 clone 再 move |
| 条件分支中的 move | 两个分支都想取得所有权，或分支后继续使用 | 保证只有一个消费路径；分支表达式返回结果时避免两边都 move |
| 函数返回后 move | 返回值是新所有者，不是无限制副本 | 按需 clone，或在原变量 still alive 时用完再转移 |
