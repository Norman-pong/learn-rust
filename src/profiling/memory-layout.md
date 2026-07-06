# 内存布局

> **一句话**：Rust 的类型在编译期就有确定的大小和对齐——`size_of` / `align_of` 能精确告诉你一个类型占多少字节、从什么地址开始。struct 字段可能被编译器重排以节省空间，enum 通过"niche 优化"让 `Option<&T>` 和 `&T` 一样大。理解内存布局，才能理解 Rust 为什么比你预期的省内存——或者不省。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript (V8) |
|------|------|-----------------|
| 类型大小 | `std::mem::size_of::<T>()` 编译期常量 | 无——所有对象都是堆分配，由 V8 的 hidden class 决定 |
| 对齐 | `std::mem::align_of::<T>()` ，未对齐访问是 UB | JS 引擎内部处理，对开发者透明 |
| struct 布局 | 编译器可自由重排字段顺序 | 对象属性由插入顺序决定 hidden class 转换 |
| enum 优化 | niche 优化：`Option<&T>` 零开销 | `T \| null` 是独立类型，无内存优化 |
| 字节级控制 | `#[repr(C)]` / `#[repr(packed)]` / `transmute` | 无——JS 不暴露内存表示 |

**核心差异**：JS 把内存布局完全隐藏在引擎后面——你不需要也不能知道 `{ x: 1, y: 2 }` 在内存中的字节排布。Rust 则把布局暴露为可观测、可控的编译期属性：你想省一个字节？你可以。你想确保与 C ABI 兼容？`#[repr(C)]`。

## 代码对比表

### 基础：`size_of` / `align_of` 实战

```rust
use std::mem;

fn main() {
    println!("bool:       {} bytes, align {}", mem::size_of::<bool>(),       mem::align_of::<bool>());
    println!("i32:        {} bytes, align {}", mem::size_of::<i32>(),        mem::align_of::<i32>());
    println!("&i32:       {} bytes, align {}", mem::size_of::<&i32>(),       mem::align_of::<&i32>());
    println!("String:     {} bytes, align {}", mem::size_of::<String>(),     mem::align_of::<String>());
    println!("Vec<i32>:   {} bytes, align {}", mem::size_of::<Vec<i32>>(),   mem::align_of::<Vec<i32>>());
    println!("Box<i32>:   {} bytes, align {}", mem::size_of::<Box<i32>>(),   mem::align_of::<Box<i32>>());
    println!("\"hello\":    {} bytes (str len)", "hello".len());
}
```

典型输出（64 位平台）：

```text
bool:       1 bytes, align 1    ← 1 字节但只用了 1 bit
i32:        4 bytes, align 4
&i32:       8 bytes, align 8    ← 引用 = 指针 = 8 字节
String:     24 bytes, align 8   ← (ptr, len, cap) 三元组
Vec<i32>:   24 bytes, align 8   ← 同上，泛型不改变结构大小
Box<i32>:   8 bytes, align 8    ← Box = 指针，堆上的 i32 是另外的 4 字节
```

> `String` 占用 24 字节在**栈上**，实际字符串数据在**堆上**。`size_of::<String>()` 告诉你栈上的句柄大小，不是堆数据。这对理解"为什么传 `&str` 比 `String` 省"很重要。

TypeScript 无法做这类测量——V8 的对象布局随 hidden class 变化，且开发者没有 API 访问：

```typescript
// TypeScript — 无法获取内存信息
const s = "hello";
// s 在 V8 里是同字符串（interned string），大小取决于引擎实现
// 开发者完全不可观测
```

### struct 字段重排

Rust 默认会**重排 struct 字段**以减少内存浪费。同一个 struct 的默认布局和 `#[repr(C)]` 大小可能不同：

```rust
use std::mem;

struct DefaultLayout {
    a: u8,    // 1 byte
    b: u32,   // 4 bytes
    c: u16,   // 2 bytes
}
// 编译器可能重排为: u32 → u16 → u8 或 u32 → (u16, u8) 等

#[repr(C)]
struct CLayout {
    a: u8,    // 1 byte  + 3 bytes padding
    b: u32,   // 4 bytes
    c: u16,   // 2 bytes  + 2 bytes padding (对齐到 4)
}

fn main() {
    println!("DefaultLayout: {} bytes", mem::size_of::<DefaultLayout>());
    println!("CLayout:       {} bytes", mem::size_of::<CLayout>());
}
```

典型输出（64 位）：

```text
DefaultLayout: 8 bytes    ← 编译器重排后紧凑
CLayout:       12 bytes   ← C ABI 顺序 + padding = 浪费 4 字节
```

> 不要顺手加 `#[repr(C)]` 除非你需要 FFI 兼容。Rust 的默认布局通常更优——编译器有重排的自由度。

在 TypeScript 中，"字段顺序"影响的是 V8 hidden class 的转换路径，而不是内存字节数——两个引擎概念完全不同：

```typescript
// V8 的 hidden class 由属性**添加顺序**决定:
const a = { x: 1 };  a.y = 2;  // hidden class 路径 1
const b = { y: 2 };  b.x = 1;  // hidden class 路径 2（不同！）
// 但这是引擎优化细节，开发者不应依赖
```

### Enum niche 优化

Rust 的 enum 利用**非法位模式**（niche）做零开销包装。最常见的例子——`Option<&T>` 和 `&T` 一样大：

```rust
use std::mem;

fn main() {
    let r: &i32 = &42;
    let opt: Option<&i32> = Some(&42);
    let none: Option<&i32> = None;

    println!("&i32:           {} bytes", mem::size_of::<&i32>());
    println!("Option<&i32>:   {} bytes", mem::size_of::<Option<&i32>>());
    println!("Option<Box<i32>>: {} bytes", mem::size_of::<Option<Box<i32>>>());
    println!("Option<bool>:   {} bytes ({} for bool)", mem::size_of::<Option<bool>>(), mem::size_of::<bool>());
    println!("Option<i32>:    {} bytes ({} for i32)", mem::size_of::<Option<i32>>(), mem::size_of::<i32>());
}
```

典型输出（64 位）：

```text
&i32:           8 bytes
Option<&i32>:   8 bytes    ← 零开销！None 用全零指针表示
Option<Box<i32>>: 8 bytes  ← 同上，Box 内部也是指针
Option<bool>:   1 byte     ← 2 bits（true/false/None）塞进 1 byte
Option<i32>:    8 bytes    ← i32 的 4 字节 + 4 字节 discriminant（无合法 niche）
```

**规则**：Rust 能利用的 niche 包括——空指针（`&T`/`Box<T>`/`NonNull<T>` 不可能为 0）、`bool` 只占 1 bit 的值域。`i32` 这种满值域的类型没有 niche，`Option<i32>` 必须额外分配 discriminant。

JS/TS 中，`T | null` 或 `T | undefined` 是联合类型，运行时要么是 `T` 要么是 `null`，没有内存优化概念：

```typescript
type Opt<T> = T | null;
const x: Opt<number> = 42;
const y: Opt<number> = null;
// V8 内部: 两种分支对应不同的 hidden class，不是内存紧凑化的概念
```

## 容易踩的坑

1. `size_of` 是**栈上**大小——`String` 的 24 字节不包含堆数据，`Vec<T>` 同理
2. `#[repr(C)]` 固定字段顺序但引入 padding，不是"更优"的布局——只在 FFI / `transmute` 时用
3. `Option<&T>` 零开销的前提是 T 不含 niche——`Option<&NonZeroU32>` 和 `&NonZeroU32` 都 8 字节
4. enum 的 discriminant（标签）大小是编译器自动计算的——≤255 变体 = 1 字节，≥256 = 变长
5. 不要在安全代码里依赖 struct 字段顺序做 `transmute`——默认布局可能被重排；用 `#[repr(C)]` 保证顺序

## 交叉链接

- → [Miri 入门](miri.md) — Miri 能检测对齐违规、无效 enum 值等 UB
- → [性能调优](perf-tuning.md) — 内存布局理解是 cache 优化的基础
- 概念层：智能指针（`Box` / `Rc` / `Arc`）的栈上大小都只是指针
- 外部参考：[The Rust Reference: Type Layout](https://doc.rust-lang.org/reference/type-layout.html) · [Rustonomicon: Data Layout](https://doc.rust-lang.org/nomicon/data.html)
