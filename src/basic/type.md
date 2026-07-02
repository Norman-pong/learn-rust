# 类型系统

> **一句话**：Rust 是强静态类型语言，类型推断贯穿全程；`&str` 与 `String` 区分借用的字符串与拥有的字符串，数字类型必须显式选择，自定义类型通过 `struct`/`enum` 和 `derive` 宏扩展能力。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 类型检查 | 编译期强制，无运行时类型擦除 | 编译期检查，运行时是裸 JavaScript |
| 字符串 | `&str`（借用切片） vs `String`（拥有堆字符串） | 统一 `string` |
| 数字类型 | `i32`, `u64`, `f64`, `usize` 等必须显式选择 | `number`，无大小/符号区分 |
| 集合 | `Vec<T>`, `HashMap<K, V>`, `HashSet<T>` | 数组、Map、Set |
| 泛型 | `Vec<T>` 单态化，零运行时开销 | `Array<T>` 类型擦除 |
| 类型推导 | 强但有限，复杂场景需要显式标注 | 宽松的鸭子类型推断 |

**核心差异**：TypeScript 的类型系统服务于编辑器体验，运行时被擦除；Rust 的类型是机器码的一部分，每个值在编译期都有确定大小与布局。`&str`/`String` 的区分就是典型例子：Rust 在类型层面区分"借来的切片"和"拥有的堆内存"，而 TypeScript 只有引用语义。

## 代码对比表

### 字符串类型：`&str` vs `String`

```rust
fn main() {
    let s1: &str = "hello";           // 字符串字面量，静态分配，借用
    let mut s2 = String::from("hello"); // 堆上拥有的字符串

    s2.push_str(", world");              // String 可修改

    println!("{s1}, {s2}");

    // 类型转换
    let owned: String = s1.to_string();   // &str → String
    let borrowed: &str = &s2;             // String → &str（自动解引用）
    println!("{owned}, {borrowed}");
}
```

```typescript
function main() {
    const s1 = "hello";        // 字符串字面量
    let s2 = "hello";          // 同样是 string

    s2 += ", world";           // 重新赋值，原字符串不可变

    console.log(`${s1}, ${s2}`);

    // 无 &str/String 区分，都是 string
    const borrowed: string = s2;
    console.log(borrowed);
}
```

### 数字类型

```rust
fn main() {
    let a: i32 = -42;      // 32 位有符号整数
    let b: u64 = 42;       // 64 位无符号整数
    let c: f64 = 3.14;     // 64 位浮点
    let d: usize = 0;      // 指针大小整数，用于索引

    let sum = a + 10;      // 类型推断保持 i32
    let idx = d + 1usize;  // 必须同类型才能运算

    println!("{a}, {b}, {c}, {d}, {sum}, {idx}");
}
```

```typescript
function main() {
    const a = -42;          // number
    const b = 42;           // number
    const c = 3.14;         // number
    const d = 0;          // number

    const sum = a + 10;     // 任意 number 运算
    const idx = d + 1;      // 无符号/符号区分

    console.log(a, b, c, d, sum, idx);
}
```

### 结构体与枚举 + derive

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug, PartialEq)]
enum Status {
    Active,
    Inactive,
    Banned(String),
}

fn main() {
    let u = User { id: 1, name: String::from("Alice") };
    let s = Status::Banned("spam".to_string());

    println!("{:?}, {:?}", u, s);

    let u2 = u.clone(); // derive(Clone) 提供
    assert_eq!(u, u2);  // derive(PartialEq) 提供
}
```

```typescript
interface User {
    id: number;
    name: string;
}

type Status =
    | { kind: "active" }
    | { kind: "inactive" }
    | { kind: "banned"; reason: string };

function main() {
    const u: User = { id: 1, name: "Alice" };
    const s: Status = { kind: "banned", reason: "spam" };

    console.log(u, s);

    // 深拷贝需要手动实现或借助库
    const u2: User = { ...u };
    console.log(u === u2 ? "same" : "shallow copy");
}
```

### HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Alice".to_string(), 100);
    scores.insert("Bob".to_string(), 85);

    match scores.get("Alice") {
        Some(score) => println!("Alice: {score}"),
        None => println!("not found"),
    }

    for (name, score) in &scores {
        println!("{name}: {score}");
    }
}
```

```typescript
function main() {
    const scores = new Map<string, number>();
    scores.set("Alice", 100);
    scores.set("Bob", 85);

    const score = scores.get("Alice");
    if (score !== undefined) {
        console.log(`Alice: ${score}`);
    }

    for (const [name, score] of scores) {
        console.log(`${name}: ${score}`);
    }
}
```

## 容易踩的坑

1. **`String` 和 `&str` 混用**——函数参数应尽量用 `&str`，可以接受 `String` 和字符串字面量；但返回时需要根据所有权决定。
2. **数字类型不匹配**——`let x: i32 = 1; x + 1u64` 会编译失败，必须显式转换 `1u64 as i32`。
3. **`derive` 不是所有 trait 都能自动**——`Display`、`From`、`Drop` 等需要手动实现。
4. **`HashMap` 的 key 需要 `Hash + Eq`**——自定义类型作为 key 必须 derive 或实现 `Hash` 和 `Eq`。
5. **`Vec` 索引越界会 panic**——`v[10]` 在越界时 panic，应优先用 `v.get(10)` 返回 `Option`。

## 交叉链接

- → [变量与绑定](variable.md) — 类型标注与 `let` 的关系
- → [结构体与枚举](struct-enum.md) — 自定义类型的更多细节
- → [Trait 与泛型](trait-generic.md) — `derive` 背后的 trait 系统
- → [错误处理](error.md) — `Option<T>` 替代 null/undefined
