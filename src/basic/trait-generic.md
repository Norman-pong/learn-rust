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

struct Article { title: String, content: String }

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}...", &self.title[..20.min(self.title.len())])
    }
}

// 泛型 + trait bound
fn notify<T: Summary>(item: &T) {
    println!("Breaking: {}", item.summarize());
}
```

```typescript
// TypeScript — 接口 + 泛型
interface Summary {
    summarize(): string;
}

class Article implements Summary {
    constructor(public title: string, public content: string) {}
    summarize(): string { return `${this.title.slice(0, 20)}...`; }
}

function notify<T extends Summary>(item: T): void {
    console.log(`Breaking: ${item.summarize()}`);
}
```

### impl Trait（RPIT）— 简化返回类型

```rust
// 旧写法：指定具体类型
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// RPITIT（Return Position impl Trait in Trait）
trait Factory {
    fn create(&self) -> impl std::fmt::Display;  // Rust 1.75+
}
```

### dyn Trait — 动态分发

```rust
// 静态分发：编译期为每个 T 生成单独代码
fn static_dispatch<T: Summary>(item: &T) { ... }

// 动态分发：运行时通过 vtable 调用
fn dynamic_dispatch(item: &dyn Summary) { ... }

let article = Article { ... };
static_dispatch(&article);   // 单态化，略快
dynamic_dispatch(&article);  // vtable，略慢但灵活
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

// 编译器为每个使用的具体类型生成一份代码：
let a = identity(42);     // → fn identity_i32(x: i32) -> i32
let b = identity("hi");   // → fn identity_str(x: &str) -> &str
// 零运行时开销，但会增加二进制大小
```

## RPITIT — async fn in trait

```rust
// Rust 1.75+ 支持
trait AsyncService {
    async fn fetch(&self) -> String;
}
// 等价于：
// trait AsyncService {
//     fn fetch(&self) -> impl Future<Output = String>;
// }
```

## 容易踩的坑

1. **Orphan Rule（孤儿规则）**——不能为外部类型实现外部 trait（`impl Display for Vec<T>` ❌）
2. **`impl Trait` 不透明**——调用者不知道具体类型，不能用于关联类型
3. **`dyn Trait` 有大小限制**——trait object 是 `!Sized`，需要 `&dyn` 或 `Box<dyn>`
4. **泛型膨胀**——`fn foo<T: Trait>` 为每种 T 生成代码，太多泛型参数导致二进制变大
5. **`Copy` 和 `Drop` 互斥**——不能同时 derive/impl

## 交叉链接

- → [结构体与枚举](struct-enum.md) — trait impl 的主体
- → [闭包](closure.md) — `Fn`/`FnMut`/`FnOnce` trait
- → [错误处理](error.md) — `thiserror` 用 derive 实现 `Error` trait
