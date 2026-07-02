# Rust 运行时语义陷阱

> 与 TypeScript 不同，Rust 的“安全”更多指内存安全与并发安全；以下陷阱不会触发编译错误，却会让程序在运行时表现“意外”。

## 1. 迭代器是惰性求值的

### 陷阱描述

调用 `Iterator` 的适配器（如 `map`、`filter`）不会立刻执行，只有被消费时（如 `collect`、`for_each`）才真正开始工作。

### 代码示例

```rust
let v = vec![1, 2, 3];
let mut sum = 0;
v.iter().map(|x| sum += x); // 没有触发消费，这行不执行任何加法
assert_eq!(sum, 0);          // 结果仍是 0
```

### 为什么意外

有 C#/Python/JS 背景的人会习惯“链式调用即执行”；Rust 的迭代器更像 LINQ 的延迟查询或 Python 的生成器表达式，必须显式消费。

### 正确做法

```rust
let v = vec![1, 2, 3];
let sum: i32 = v.iter().sum(); // 显式消费
assert_eq!(sum, 6);
```

---

## 2. UTF-8 字节切片与字符索引混用

### 陷阱描述

Rust 字符串按 UTF-8 字节索引，而非“字符”索引；对多字节字符使用中文字符位置去 `slice` 会 panic。

### 代码示例

```rust
let s = "你好";
let _ = &s[0..1]; // panic：'你' 占 3 字节，[0..1] 截断在字符中间
```

### 为什么意外

TypeScript/JavaScript 字符串索引按 UTF-16 code unit；Python 2 按字节、Python 3 按 code point，都让人误以为“一个字符就是一个索引”。

### 正确做法

```rust
let s = "你好";
let ch = s.chars().nth(0).unwrap(); // 按 Unicode scalar value 取字符
let bytes = s.as_bytes();            // 需要字节时再按字节切片
```

---

## 3. `f32` / `f64` 默认不支持 `Eq`

### 陷阱描述

浮点数因为 NaN 的存在，不满足等价关系（自反性不成立），所以 Rust 没有给 `f32`/`f64` 实现 `Eq` trait，只能用于 `PartialEq`。

### 代码示例

```rust
#[derive(Eq, PartialEq)]
struct Point {
    x: f64, // 编译错误：f64 只实现 PartialEq，不满足 Eq 的全序/自反要求
    y: f64,
}
```

### 为什么意外

多数语言允许 `==` 比较浮点，并容忍 `NaN != NaN` 的特例；Rust 把这一特例提升到类型系统，强制区分“部分相等”与“完全相等”。

### 正确做法

```rust
#[derive(PartialEq)] // 只派生 PartialEq
struct Point {
    x: f64,
    y: f64,
}

// 若需要集合/哈希键，可转整数或包装类型
#[derive(Eq, PartialEq, Hash)]
struct IntegerPoint {
    x: i64,
    y: i64,
}
```

---

## 4. `Mutex` 被 panic 污染后会进入 poisoned 状态

### 陷阱描述

如果一个线程在持有 `Mutex` 时 panic，Rust 会标记该锁为“poisoned”；后续 `lock()` 不会 panic，但会返回 `Err(PoisonError)`。

### 代码示例

```rust
use std::sync::Mutex;
use std::thread;

let m = Mutex::new(0);
let _ = thread::spawn({
    let m = m.clone(); // 假设 Arc<Mutex<i32>>
    move || {
        let mut guard = m.lock().unwrap();
        *guard += 1;
        panic!("boom");
    }
}).join();

let guard = m.lock(); // 返回 Err(PoisonError)
```

### 为什么意外

Go/Java 的锁在持有者 panic 时不会主动“毒害”锁；Rust 默认把锁污染视为数据可能不一致的信号，必须显式处理。

### 正确做法

```rust
use std::sync::{Mutex, Arc};
use std::thread;

let m = Arc::new(Mutex::new(0));
let m2 = m.clone();
let handle = thread::spawn(move || {
    let mut guard = m2.lock().unwrap();
    *guard += 1;
    panic!("boom");
});
let _ = handle.join();

// 显式处理 poison：恢复锁或中止流程
let mut guard = m.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
*guard += 1;
```

---

## 5. 局部变量的 Drop 顺序与解构顺序相反

### 陷阱描述

同一作用域内多个局部变量按声明顺序初始化，但在作用域结束时按声明的**逆序**调用 `Drop`。

### 代码示例

```rust
struct Loud(String);
impl Drop for Loud {
    fn drop(&mut self) { println!("drop {}", self.0); }
}

{
    let a = Loud("a".into());
    let b = Loud("b".into());
} // 先打印 "drop b"，再打印 "drop a"
```

### 为什么意外

C++ 有类似的规则，但 JavaScript/Go/Python 这类带 GC 的语言通常不保证对象析构顺序；Rust 的确定性析构在写锁、文件句柄或临时文件时很容易被忽略。

### 正确做法

```rust
// 若顺序敏感，按需要的析构顺序反向声明
let lock_a = Mutex::new(());
let lock_b = Mutex::new(());
// 退出作用域时：先释放 b，再释放 a
```

---

## 6. `match` 绑定会移动值，除非使用 `ref` / `ref mut`

### 陷阱描述

在 `match` 中直接绑定变量会**按值移动**被匹配的值；如果之后还想使用原值，会触发所有权错误。

### 代码示例

```rust
let s = Some(String::from("hello"));
match s {
    Some(t) => println!("{}", t), // t 拿走了 String 的所有权
    None => {},
}
// s 已不可用
```

### 为什么意外

TypeScript 的 `switch`/`if` 只比较引用；Rust 的 `match` 是模式匹配，默认执行语义化绑定，很容易在“只是判断一下”时误转移所有权。

### 正确做法

```rust
let s = Some(String::from("hello"));
match s {
    Some(ref t) => println!("{}", t), // 只借用
    None => {},
}
println!("still: {:?}", s); // OK

// 或者使用 if let 只匹配不移动
if let Some(ref t) = s { println!("{}", t); }
```

---

## 7. `read_dir` / `lines` 返回的迭代器会持有底层资源

### 陷阱描述

某些 IO 迭代器（如 `std::fs::read_dir`、`BufReader::lines`）在迭代时一直持有文件句柄，中途退出迭代可能导致资源未释放或平台级错误。

### 代码示例

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

let f = File::open("log.txt")?;
let reader = BufReader::new(f);
let first = reader.lines().next(); // 迭代器仍持有 reader 和 File
// 在 Windows 等平台上，若此时想删除文件会被拒绝
```

### 为什么意外

TypeScript 的 `fs.createReadStream` 或 Python 的 `open(...)` 也持有资源，但垃圾回收/事件循环会延后释放；Rust 的确定性 Drop 让“迭代器即 RAII”这件事更显眼，也更容易被忽略。

### 正确做法

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

let first = {
    let f = File::open("log.txt")?;
    let reader = BufReader::new(f);
    reader.lines().next()
}; // reader 与 File 在此处统一释放，之后再操作文件路径更安全
```
