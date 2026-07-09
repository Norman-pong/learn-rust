# 生命周期进阶

> **一句话**：生命周期不只是函数签名里的 `'a`，而是 Rust 类型系统的子类型关系、型变规则与高阶 trait 约束共同构成的证明体系；掌握它，才能理解 `Pin`、异步运行时和复杂泛型 API 的设计。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 生命周期 | 编译期类型系统的一部分，决定引用能否合法使用 | 无对应概念，GC 保证对象存活 |
| 子类型 | `'a: 'b` 表示生命周期之间的包含关系 | 主要用在结构/接口兼容，无生命周期子类型 |
| 型变 | 泛型参数对子类型的传导方向（协变/逆变/不变） | 泛型默认协变，但无编译期引用安全诉求 |
| 高阶约束 | `for<'a>` 可量化所有生命周期 | 无对应机制 |
| 不可移动 | `Pin` 在类型层面保证值不会被移动 | 无等价物，闭包捕获自然沿用原地址 |

**核心差异**：TypeScript 的类型系统在运行前就被擦除，生命周期更不存在；Rust 的生命周期是真实参与类型检查和子类型推理的编译期信息，错误的生命周期标注会直接导致编译失败。

## 代码对比表

### 生命周期子类型：`'long: 'short`

`'a: 'b` 读作 "`'a` 比 `'b` 长（或相等）"，也就是 `'a` 范围内存活的引用可以安全地当成 `'b` 使用。这是 Rust 里引用收窄（shrink lifetime）的合法依据。

```rust
// 一个 &'static str 可以传给要求 &'a str 的参数
fn takes_short<'a>(s: &'a str) {
    println!("{}", s);
}

fn main() {
    let forever: &'static str = "I am static";
    let local = String::from("I am local");

    takes_short(forever);       // ✅ 'static 比任何 'a 都长
    takes_short(&local);        // ✅ 这里 'a 被实例化为 local 的生命周期
}
```

```typescript
// TypeScript 没有生命周期概念，任何字符串引用都可以随意传递
function takesShort(s: string): void {
    console.log(s);
}

const forever = "I am in module scope";
function main() {
    const local = "I am local";
    takesShort(forever);
    takesShort(local);
}
```

### 型变（Variance）

型变回答：如果 `T` 是 `U` 的子类型，那么 `F<T>` 与 `F<U>` 是什么关系？

```rust
// 定义类型：Animal 是 trait，Cat / Dog 是实现它的具体类型
trait Animal { fn speak(&self) -> &'static str; }
struct Cat;
struct Dog;
impl Animal for Cat { fn speak(&self) -> &'static str { "meow" } }
impl Animal for Dog { fn speak(&self) -> &'static str { "woof" } }

// &'a T 对 T 是协变的：&'a Cat 可以当成 &'a dyn Animal 使用
fn accept_animal<'a>(a: &'a dyn Animal) {
    println!("{}", a.speak());
}

// &'a mut T 对 T 是不变的：当 T 被实例化为 Cat 后，不能再把 Dog 写入 *a
fn maybe_replace_with_dog<T: Animal>(_a: &mut T) {
    // *a = Dog; // 类型错误：T 已固定为 Cat，不能替换为 Dog
}

fn main() {
    let cat = Cat;
    accept_animal(&cat); // ✅ &Cat -> &dyn Animal

    let mut cat = Cat;
    maybe_replace_with_dog(&mut cat); // T 被实例化为 Cat，不能换成 Dog
}
```

```typescript
// TypeScript 里泛型对象默认协变，但数组/对象的可变性不直接对应 Rust 的 &mut
interface Animal { name: string }
interface Cat extends Animal { meow: boolean }

function acceptAnimal(a: Animal) {
    console.log(a.name);
}

const cat: Cat = { name: "Mimi", meow: true };
acceptAnimal(cat); // ✅ TS 结构子类型允许向上转型
```

### 高阶 Trait Bound：`for<'a>`

当需要某个类型对所有生命周期都满足某约束时，使用 HRTB。典型场景：闭包或函数指针接受任意生命周期的引用。

```rust
// 这个函数要求 F 对所有 'a 都能把 &'a str 变成 &'a str
fn apply_to_borrowed<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let local = String::from("hello");
    let result = f(&local);
    println!("{}", result);
}

fn identity(s: &str) -> &str { s }

fn main() {
    apply_to_borrowed(identity);
}
```

```typescript
// TypeScript 没有生命周期量化，泛型参数一次实例化即可
function applyToBorrowed<F extends (s: string) => string>(f: F): void {
    const local = "hello";
    const result = f(local);
    console.log(result);
}

function identity(s: string): string { return s; }
applyToBorrowed(identity);
```

### `Pin<T>`：不可移动的保证

`Pin<P>` 本身不是智能指针，而是对指针 `P` 所指向内容的"承诺"：一旦被 `Pin` 住，该值内存地址不会再变（除非实现 `Unpin`）。自引用结构体和异步 `Future` 都依赖它。

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

struct SelfRef {
    data: String,
    // 自引用指针，指向 data 内部
    ptr: *const String,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfRef {
            data,
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });

        let ptr: *const String = &boxed.data;
        // SAFETY: 我们刚把值放到 Box 里，且不会再移动它
        unsafe {
            let mut_ref: Pin<&mut SelfRef> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).ptr = ptr;
        }

        boxed
    }
}

fn main() {
    let s = SelfRef::new(String::from("pinned"));
    // s 不能安全地移动，ptr 始终有效
}
```

```typescript
// TypeScript / JavaScript 中闭包自然捕获引用，GC 保证地址不变
function makeSelfRef(data: string) {
    return {
        get length() { return data.length; },
    };
}

const s = makeSelfRef("pinned");
// 对象可以被赋值、传参，但内部引用的 data 仍由闭包持有
```

## 容易踩的坑

1. **把生命周期当成运行时代码**：`'a` 只在编译期存在，不会生成任何指令，也不能做运行时判断。
2. **认为生命周期能延长变量存活**：生命周期只是约束，不能真的让局部变量活更久。
3. **在返回引用时忘记约束**：`fn get() -> &str` 没有输入生命周期，编译器会报错 `E0106`（missing lifetime specifier），要求显式标注；如果实际返回的是局部引用，即使标注为 `&'static str` 也无法通过借用检查。
4. **在 trait 对象里混用生命周期**：`Box<dyn Trait + 'a>` 与 `Box<dyn Trait + 'static>` 是完全不同的类型，需要特别注意。
5. **滥用 `Pin::get_unchecked_mut`**：只有在确认不会破坏自引用或异步状态机的内部假设时才能用 `unsafe`。

## 交叉链接

- → [生命周期基础](lifetime-basic.md) — 前置语法：省略规则、结构体生命周期
- → [自引用结构](self-referential.md) — `Pin` 与自引用结构体的实战
- → [智能指针](smart-pointer.md) — `Box`、`Rc` 对生命周期的影响
- → [源码阅读](../deep-dives/code-readings.md) — 建议阅读 tokio 中生命周期与 `Pin` 的使用
