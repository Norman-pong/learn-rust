# 闭包

> **一句话**：Rust 闭包是匿名的函数值，语法紧凑 `|x| x + 1`，编译器会根据它如何使用捕获变量自动选择 `Fn`/`FnMut`/`FnOnce` 三种 trait 之一；需要把环境所有权移进闭包时用 `move` 关键字。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| 语法 | 管道参数 `\|x\| x + 1` | 箭头函数 `(x) => x + 1` |
| 捕获方式 | 按需要自动借用或移动变量，可由 `move` 强制取得所有权 | 捕获变量绑定（binding），闭包始终能看到变量的最新值；不存在所有权语义，所有引用可自由逃逸 |
| 可调用 trait | 三套 `Fn` / `FnMut` / `FnOnce` trait 区分调用次数与可变性 | 统一 `Function` 类型，没有“调用会消耗函数”的概念 |
| 类型推导 | 参数与返回类型通常可推断，但闭包类型是不透明的匿名类型 | 箭头函数类型可被显式写出 `(x: number) => number` |
| 环境所有权 | `move` 闭包把外部变量所有权移入；普通闭包可能仅借用 | 不存在所有权语义，闭包持有引用或值取决于运行时 |
| 函数指针 | `fn(i32) -> i32` 只指裸函数，不能捕获环境 | `() =>` 函数与箭头函数没有区分 |

**核心差异**：TypeScript 的函数是值，闭包捕获的是变量绑定；Rust 的闭包是一个不透明的匿名结构体，它把捕获的变量当作字段，并自动实现 `Fn`/`FnMut`/`FnOnce` 之一。调用者能否多次调用、能否修改捕获变量，都写在 trait bound 里，编译器据此检查。

## 代码对比表

### 基础闭包语法

```rust
fn main() {
    let add_one = |x: i32| x + 1;
    let add_two = |x| { x + 2 }; // 参数类型可推断

    println!("{}", add_one(5)); // 6
    println!("{}", add_two(5)); // 7
}
```

```typescript
const addOne = (x: number) => x + 1;
const addTwo = (x: number) => x + 2;

console.log(addOne(5)); // 6
console.log(addTwo(5)); // 7
```

### 捕获环境变量

```rust
fn make_greeter(name: String) -> impl Fn() -> String {
    move || format!("Hello, {name}!")
}

fn main() {
    let greeter = make_greeter(String::from("Rust"));
    println!("{}", greeter()); // Hello, Rust!
    println!("{}", greeter()); // 仍可再次调用
}
```

```typescript
function makeGreeter(name: string) {
    return () => `Hello, ${name}!`;
}

const greeter = makeGreeter("TypeScript");
console.log(greeter()); // Hello, TypeScript!
```

### Fn / FnMut / FnOnce 的 trait bound

```rust
fn call_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn call_mut_twice(f: &mut impl FnMut() -> i32) -> i32 {
    f() + f()
}

fn consume_once(f: impl FnOnce() -> String) {
    println!("{}", f());
}

fn main() {
    let n = 1;
    let readonly = |x| x + n;
    println!("{}", call_twice(readonly, 5)); // 7

    let mut acc = 0;
    let mut increment = || {
        acc += 1;
        acc
    };
    println!("{}", call_mut_twice(&mut increment)); // 3

    let msg = String::from("moved");
    let once = move || msg;
    consume_once(once);
}
```

```typescript
function callTwice(f: (x: number) => number, x: number): number {
    return f(f(x));
}

function callTwiceMut(f: () => number): number {
    return f() + f();
}

function consumeOnce(f: () => string): void {
    console.log(f());
}

function main() {
    const n = 1;
    const readonly = (x: number) => x + n;
    console.log(callTwice(readonly, 5)); // 7

    let acc = 0;
    const increment = () => {
        acc += 1;
        return acc;
    };
    console.log(callTwiceMut(increment)); // 3

    const msg = "moved";
    const once = () => msg;
    consumeOnce(once);
}
```

### move 闭包与所有权

```rust
fn spawn_worker(name: String) {
    let handle = std::thread::spawn(move || {
        println!("worker {name} running");
    });
    handle.join().unwrap(); // 等待线程完成，避免主线程提前退出
}

fn main() {
    let name = String::from("alpha");
    spawn_worker(name);
    // println!("{name}"); // 错误：name 已被 move 进闭包
}
```

```typescript
function spawnWorker(name: string) {
    setTimeout(() => {
        console.log(`worker ${name} running`);
    }, 0);
}

function main() {
    const name = "alpha";
    spawnWorker(name);
    console.log(name); // 合法，字符串被复制
}
```

### 返回闭包

闭包类型是不透明的匿名类型（编译器生成的结构体），无法直接写出类型签名。返回闭包有两种方式：

```rust
// 方式一：impl Trait（静态分发，零开销，最常用）
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n  // move 把 n 的所有权移入闭包
}

// 方式二：Box<dyn Fn>（动态分发，有堆分配开销，用于需要在集合中存放异构闭包）
fn make_adder_boxed(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}

fn main() {
    let add5 = make_adder(5);
    println!("{}", add5(3)); // 8

    let add10 = make_adder_boxed(10);
    println!("{}", add10(3)); // 13
}
```

```typescript
// TypeScript 闭包类型可显式写出，返回闭包没有特殊语法
function makeAdder(n: number): (x: number) => number {
    return (x) => x + n;
}

const add5 = makeAdder(5);
console.log(add5(3)); // 8
```

> **为什么需要 `move`？** 不加 `move` 时闭包默认借用 `n`，函数返回时 `n` 所在栈帧被回收，闭包持有的引用变成悬垂引用。`move` 把 `n`（`i32` 是 `Copy`，实际复制）的所有权移入闭包，闭包自洽。
>
> **`impl Fn` vs `Box<dyn Fn>` 选择**：单一闭包类型用 `impl Fn`（零开销）；需要在 `Vec` 或 trait object 中存多种不同闭包时用 `Box<dyn Fn>`。

### 闭包 vs 函数指针

```rust
fn bare_double(x: i32) -> i32 {
    x * 2
}

fn call_with_fn_pointer(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    let result = call_with_fn_pointer(bare_double, 7);
    println!("{}", result); // 14

    // 函数指针不能捕获环境
    // let factor = 2;
    // call_with_fn_pointer(|x| x * factor, 7); // 错误：闭包类型不能强转成 fn
}
```

```typescript
function bareDouble(x: number): number {
    return x * 2;
}

function callWithFunctionPointer(f: (x: number) => number, x: number): number {
    return f(x);
}

function main() {
    const result = callWithFunctionPointer(bareDouble, 7);
    console.log(result); // 14

    const factor = 2;
    callWithFunctionPointer((x) => x * factor, 7); // 合法
}
```

## 容易踩的坑

1. **闭包类型不能显式写出**——`let f: ??? = |x| x + 1;` 没有具体类型名，只能用 `impl Fn` 或 trait object 接收；需要手写签名时传给函数参数。
2. **选错 trait bound 会编译失败**——需要修改捕获变量时传 `Fn` 会报错 `cannot borrow data mutably`；只能调用一次的闭包传 `Fn` 会报 `expected a closure that implements FnMut` 或 `FnOnce`。
3. **move 闭包会转移外部变量所有权**——`move ||` 把变量所有权拿进闭包，之后外部再使用会报 `value used here after move`。
4. **闭包不能强转成函数指针 `fn`**——`fn(i32) -> i32` 只代表不捕获环境的裸函数，带捕获的闭包给它会报类型不匹配。
5. **返回闭包通常需要 `move` / `impl Trait`**——`fn make() -> impl Fn(i32) -> i32 { |x| x + n }` 如果不写 `move`，闭包默认借用 `n`，函数返回时 `n` 所在栈帧被回收，闭包持有的引用指向已释放内存，导致悬垂引用。

## 交叉链接

- → [函数](function.md) — 高阶函数与 `impl Fn` 的 trait bound 写法
- → [Trait 与泛型](trait-generic.md) — `impl Trait` 与 trait object 的区别
- → [所有权与借用](../ownership-lifetimes/ownership.md) — `move` 与所有权转移的完整规则
- → [线程与并发](../concurrency/thread.md) — `std::thread::spawn` 为何要求 `move` 闭包
