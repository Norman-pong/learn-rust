# 生命周期基础

> **一句话**：生命周期（lifetime）是 Rust 编译器证明引用不会悬垂（dangling）的标注系统——它不是延长变量的存活时间，而是标注引用之间的存活关系。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 引用有效性 | 编译期通过生命周期证明 | 运行期由 GC 保证可达 |
| 标注语法 | `'a`, `'static`, `'b: 'a` | 无生命周期概念 |
| 函数返回引用 | 返回类型必须标注生命周期，或依赖省略规则 | 自由返回字符串/对象引用 |
| 结构体引用 | 结构体必须声明生命周期参数 | 类的字段引用任意对象 |
| 悬垂引用 | 编译错误 | 理论上不会出现，但弱引用、闭包可能意外捕获 |

**核心差异**：生命周期标注是给编译器的"借条"，说明"我借来的这个引用，不会比它指向的东西活得更久"。JS/TS 没有对应概念，因为 GC 会保证对象只要还有人引用就存活。Rust 在编译期做这件事，换来了零运行时 GC 开销和内存安全。

## 代码对比表

### 最简生命周期

```rust
// 返回的引用生命周期 = x 和 y 中较短的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("short");
    let s2 = String::from("longer");
    let result = longest(&s1, &s2);
    // result 的生命周期不超过 s1 和 s2 中较短者
    println!("{}", result);
    // 这之后 s1 和 s2 被 drop，result 也失效
}
```

```typescript
// TypeScript — 无生命周期概念，GC 自动管理
function longest(x: string, y: string): string {
    return x.length > y.length ? x : y;
}

function main() {
    const s1 = "short";
    const s2 = "longer";
    const result = longest(s1, s2);
    console.log(result);
}
```

### 结构体中的生命周期

```rust
struct Excerpt<'a> {
    part: &'a str, // Excerpt 不能比 part 指向的字符串活得更久
}

fn main() {
    let novel = String::from("Call me Ishmael...");
    let excerpt = Excerpt {
        part: &novel[..5],
    };
    println!("{}", excerpt.part);
    // excerpt 必须在 novel 之前 drop
}
```

```typescript
class Excerpt {
    constructor(public part: string) {}
}

function main() {
    const novel = "Call me Ishmael...";
    const excerpt = new Excerpt(novel.slice(0, 5));
    console.log(excerpt.part);
    // novel 和 excerpt 的生存期由 GC 自动管理
}
```

### 生命周期省略规则（3 条）

```rust
// 规则 1：每个引用参数都有各自的生命周期
fn foo(x: &str, y: &str)            // 等价于
// fn foo<'a, 'b>(x: &'a str, y: &'b str)

// 规则 2：如果只有一个输入生命周期，它被赋给所有输出生命周期
fn first_word(s: &str) -> &str      // 等价于
// fn first_word<'a>(s: &'a str) -> &'a str

// 规则 3：如果 &self 或 &mut self，self 的生命周期赋给所有输出
struct Foo;
impl Foo {
    fn bar(&self, x: &str) -> &str   // 等价于
    // fn bar<'a, 'b>(&'a self, x: &'b str) -> &'a str
    {
        x
    }
}

fn main() {}
```

```typescript
// TypeScript 没有生命周期省略规则，因为引用类型没有生命周期参数
function foo(x: string, y: string): void {}
function firstWord(s: string): string {
    return s.split(" ")[0] ?? "";
}
class Foo {
    bar(x: string): string { return x; }
}
```

### 生命周期约束

```rust
fn copy_if_longer<'a, 'b>(s: &'a str, buffer: &'b mut String) -> bool
where
    'a: 'b, // s 的生命周期至少和 buffer 一样长
{
    if s.len() > 10 {
        buffer.push_str(s);
        true
    } else {
        false
    }
}

fn main() {
    let s = String::from("hello world");
    let mut buffer = String::new();
    copy_if_longer(&s, &mut buffer);
    println!("{buffer}");
}
```

```typescript
function copyIfLonger(s: string, buffer: string): boolean {
    if (s.length > 10) {
        buffer += s; // 注意：字符串不可变，这里需要重新赋值
        return true;
    }
    return false;
}

function main() {
    const s = "hello world";
    let buffer = "";
    copyIfLonger(s, buffer); // 不会修改外部 buffer
    console.log(buffer); // 空
}
```

## 容易踩的坑

1. **生命周期标注是"描述"不是"修改"**——`'a` 不会改变数据存活时间，只是告知编译器引用的有效期约束。
2. **返回局部引用**——`fn foo() -> &str { let s = "hi"; &s }` ❌，`s` 在函数结束时 drop。
3. **不同生命周期的引用**——`longest(&s1, &s2)` 中 result 的生命周期 = s1 和 s2 的重叠部分，也就是较短者。
4. **`'static` 不是万能的**——`&'static str` 表示存活到程序结束，不能随便加；拥有类型 `String` 本身不是 `'static`。
5. **闭包捕获引用**——闭包捕获引用时也会涉及生命周期推导，返回闭包时要小心捕获引用的生命周期。

## 交叉链接

- → [所有权模型](ownership.md) — 生命周期是所有权系统的自然延伸
- → [引用与借用](reference-borrow.md) — `&T` 和 `&mut T` 的借用规则
- → [生命周期进阶](lifetime-advanced.md) — HRTB、子类型、协变
- → [智能指针](smart-pointer.md) — `Box` 和 `Rc` 如何影响生命周期
