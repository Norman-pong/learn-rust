# 生命周期编译错误案例

> 本章收集 Rust 编译器报出的生命周期相关错误——每个案例遵循"错码 → 报错 → 解释 → 修复"四段格式。
> 案例均来自实际编码中的真实错误，逐渐积累。

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

```
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

```
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

结构体持有引用时，必须标注生命周期——告诉编译器"这个结构体不能比它引用的数据活得更久"。

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

```
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
    | | { count += 1; count }  // error: closure may outlive `count`
}
```

### 报错

```
error[E0373]: closure may outlive the current function, but it borrows `count`
 --> src/main.rs:3:5
  |
3 |     | | { count += 1; count }
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

```
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
// 如果确定 result 只需和 s1 一样长，改调用方式：
fn main() {
    let s1 = String::from("always here");
    let result = longer(&s1, &s1);  // 两个引用生命周期相同即可
    println!("{result}");
}
```
