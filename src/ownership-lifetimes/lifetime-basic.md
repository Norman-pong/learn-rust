# 生命周期基础

> **一句话**：生命周期（lifetime）是 Rust 编译器证明引用不会悬垂（dangling）的标注系统——它不是延长变量的存活时间，而是标注引用之间的存活关系。

## 与 JS/TS 的关键差异

JS/TS 由 GC 保证引用始终有效（只要对象可达）。Rust 没有 GC，编译器需要在编译期证明所有引用都不会比它们指向的数据活得更久。生命周期标注（`'a`）就是这个证明的一部分。

**核心直觉**：生命周期标注是给编译器的"借条"，说明"我借来的这个引用，不会比它指向的东西活得更久"。

## 代码对比表

### 最简生命周期

```rust
// 返回的引用生命周期 = x 和 y 中较短的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

let s1 = String::from("short");
let s2 = String::from("longer");
let result = longest(&s1, &s2);
// result 的生命周期不超过 s1 和 s2 中较短者
println!("{}", result);
// 这之后 s1 和 s2 被 drop，result 也失效
```

```typescript
// TypeScript — 无生命周期概念，GC 自动管理
function longest(x: string, y: string): string {
    return x.length > y.length ? x : y;
}
// 返回的引用随 GC 保证安全，无编译期约束
```

### 结构体中的生命周期

```rust
struct Excerpt<'a> {
    part: &'a str,  // Excerpt 不能比 part 指向的字符串活得更久
}

let novel = String::from("Call me Ishmael...");
let excerpt = Excerpt { part: &novel[..5] };
// excerpt 必须在 novel 之前 drop
```

### 生命周期省略规则（3 条）

```rust
// 规则 1：每个引用参数都有各自的生命周期
fn foo(x: &str, y: &str)           → fn foo<'a, 'b>(x: &'a str, y: &'b str)

// 规则 2：如果只有一个输入生命周期，它被赋给所有输出生命周期
fn foo(x: &str) -> &str             → fn foo<'a>(x: &'a str) -> &'a str

// 规则 3：如果 &self 或 &mut self，self 的生命周期赋给所有输出
impl Foo {
    fn bar(&self, x: &str) -> &str  → fn bar<'a, 'b>(&'a self, x: &'b str) -> &'a str
}
```

## 容易踩的坑

1. **生命周期标注是"描述"不是"修改"**——`'a` 不会改变数据存活时间，只是告知编译器引用的有效期约束
2. **返回局部引用**——`fn foo() -> &str { let s = "hi"; &s }` ❌ `s` 在函数结束时 drop
3. **不同生命周期的引用**——`longest(&s1, &s2)` 中 result 的生命周期 = s1 和 s2 的重叠部分
4. **'static 不是万能的**——`&'static str` 表示存活到程序结束，不能随便加
5. **闭包捕获引用**——闭包捕获引用时也会涉及生命周期推导

## 交叉链接

- → [所有权模型](ownership.md) — 生命周期是所有权系统的自然延伸
- → [引用与借用](reference-borrow.md) — `&T` 和 `&mut T` 的借用规则
- → [生命周期进阶](lifetime-advanced.md) — HRTB、子类型、协变
