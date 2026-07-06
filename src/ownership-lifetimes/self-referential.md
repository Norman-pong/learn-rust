# 自引用结构

> **一句话**：Rust 里"结构体内部一个字段引用另一个字段"之所以难，是因为值会被移动（move），而移动会改变被引用字段的地址；`Pin` 提供了一种在类型层面禁止移动的方案。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 字段间引用 | 生命周期 + 自引用结构需要特殊处理（Pin） | 对象字段可通过闭包/引用自然互相引用 |
| 地址稳定性 | 值移动后地址可能改变，导致自引用悬垂 | GC 保证对象地址不变，引用始终有效 |
| 不可移动 | 通过 `Pin` + `PhantomPinned` 显式声明 | 无对应概念，对象可任意赋值/传参 |
| 安全性 | 自引用通常需要 `unsafe` 或使用第三方 crate | 语言运行时自动处理 |

**核心差异**：JavaScript 的对象是 GC 托管的堆对象，闭包捕获引用后地址不会变；Rust 的 `String`、`Vec` 等堆分配类型在赋值或返回时会移动所有权，旧地址失效，所以字段内保存的指针会立刻变成悬垂指针。

## 代码对比表

###  naive 自引用：为什么编译不过？

```rust,ignore
// 这段代码不会通过编译，仅用于展示问题
struct Foo {
    data: String,
    ref_to_data: &str,  // 需要生命周期参数，但结构体无法描述"指向自己字段"
}

fn main() {
    let mut foo = Foo {
        data: String::from("hello"),
        ref_to_data: "",
    };
    foo.ref_to_data = &foo.data;

    let moved = foo;  // foo 被 move，data 地址改变
    // moved.ref_to_data 现在指向旧地址 → 悬垂
}
```

```typescript
// TypeScript 中对象内部引用彼此非常自然
const foo = {
    data: "hello",
    get refToData() { return this.data; },
};

const moved = foo;  // 引用复制，data 地址不变
console.log(moved.refToData); // ✅
```

### 方案演变：raw pointer → ouroboros → Pin

```rust
// 阶段 1：原始指针，但完全靠 unsafe 保证地址不变
struct RawSelfRef {
    data: String,
    ptr: *const u8,
}

impl RawSelfRef {
    fn new(s: String) -> Self {
        let mut me = RawSelfRef { data: s, ptr: std::ptr::null() };
        me.ptr = me.data.as_ptr(); // 警告：me 之后可能被 move
        me
    }
}

fn main() {
    let r = RawSelfRef::new(String::from("unsafe"));
    // 如果 r 被 move，ptr 就会失效，这里只是碰巧没有触发
}
```

```rust
// 阶段 2：使用 ouroboros crate 自动派生自引用结构体
use ouroboros::self_referencing;

#[self_referencing]
struct OuroborosSelfRef {
    data: String,
    #[borrows(data)]
    #[covariant]
    ref_to_data: &'this str,
}

fn main() {
    let s = OuroborosSelfRef::new(String::from("safe"), |data| data.as_str());
    s.with_ref_to_data(|r| println!("{}", r));
}
```

```rust
// 阶段 3：标准库 Pin 方案
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::ptr;

struct PinnedSelfRef {
    data: String,
    ptr: *const String,
    _pin: PhantomPinned,
}

impl PinnedSelfRef {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut pinned = Box::pin(PinnedSelfRef {
            data,
            ptr: ptr::null(),
            _pin: PhantomPinned,
        });

        let ptr = &pinned.as_ref().get_ref().data as *const String;
        unsafe {
            Pin::get_unchecked_mut(pinned.as_mut()).ptr = ptr;
        }
        pinned
    }

    fn data_ptr(&self) -> *const String {
        self.ptr
    }
}

fn main() {
    let s = PinnedSelfRef::new(String::from("pinned safely"));
    assert!(!s.data_ptr().is_null());
}
```

### 真实场景：async 块生成的自引用 Future

```rust
// async fn 编译后会生成一个自引用状态机
async fn read_file(path: &str) -> String {
    tokio::fs::read_to_string(path).await.unwrap()
}

fn main() {
    let fut = read_file("Cargo.toml");
    // fut 内部可能引用自己的缓冲区，因此 tokio::spawn 要求 Future 是 Unpin
    // Pin<Box<dyn Future>> 可以把 !Unpin 的 Future 变成可 spawn
    tokio::spawn(async move {
        let contents = fut.await;
        println!("{}", contents);
    });
}
```

```typescript
// TypeScript 异步函数本质也是状态机，但由运行时保证闭包引用有效
async function readFile(path: string): Promise<string> {
    const res = await fetch(path);
    return res.text();
}

async function main() {
    const contents = await readFile("package.json");
    console.log(contents);
}
```

## 容易踩的坑

1. **在栈上构造自引用结构**：即使使用了 `PhantomPinned`，栈上的值仍可能被 mem::swap 或 move 破坏，必须放在 `Pin<Box<T>>` 或 `Pin<&mut T>` 里。
2. **手动实现 `Unpin`**：如果给自引用结构实现 `Unpin`，`Pin` 的保护就失效了；通常需要 `impl !Unpin for MyStruct`（ nightly ）或显式 `PhantomPinned`。
3. **通过 `Pin::get_unchecked_mut` 修改被引用的字段**：这会破坏自引用指针，需要 `Pin::get_unchecked_mut` 时只修改不破坏地址不变性的字段。
4. **认为 `Pin<T>` 让 `T` 不能被 drop**：`Pin` 只阻止移动，不阻止析构；`drop` 仍会被调用。
5. **在 async 函数返回裸 Future 时忽略 `Unpin` 要求**：`tokio::spawn` 和 `.await` 点需要 `Future + Send + 'static`，自引用 Future 记得用 `Box::pin`。

## 交叉链接

- → [生命周期进阶](lifetime-advanced.md) — `Pin` 与 HRTB 的进阶细节
- → [所有权模型](ownership.md) — move 语义是自引用问题的根因
- → [智能指针](smart-pointer.md) — `Box`、`Rc` 与自引用结构体的配合
- → [源码阅读](../deep-dives/code-readings.md) — 阅读 tokio 中 Pin 与 Future 的实现
