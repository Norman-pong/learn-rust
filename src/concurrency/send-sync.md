# Send 与 Sync

> **一句话**：`Send` 和 `Sync` 是 Rust 并发安全的两个 auto trait——它们由编译器自动推导，决定类型能否在线程间传递所有权或被多线程共享引用。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript (Node.js) |
|------|------|---------------------|
| 线程安全检查 | 编译期，通过 `Send`/`Sync` 自动推导 | 运行期，由 JS 引擎保证（单线程 + 结构化克隆） |
| 所有权跨线程 | 类型必须实现 `Send` | 无所有权概念，数据复制/共享 |
| 引用跨线程 | 类型必须实现 `Sync` | 无共享引用，JS 对象不能跨线程直接引用 |
| 手动控制 | 可 `unsafe impl Send`/`Sync` 手动标记 | 无对应机制 |
| 错误发现时机 | 编译错误 | 运行时才可能暴露数据竞争 |

**核心差异**：TypeScript/JavaScript 没有编译期线程安全检查；它通过单线程事件循环和结构化克隆（或 `SharedArrayBuffer` 的显式 `Atomics`）避免数据竞争。Rust 则把线程安全写进类型系统，违反 `Send`/`Sync` 的代码在编译阶段就被拒绝。

## 代码对比表

### Send：所有权跨线程

```rust
use std::thread;

fn main() {
    let s = String::from("owned"); // String 是 Send

    let handle = thread::spawn(move || {
        // 所有权被 move 进新线程
        println!("{s}");
    });

    handle.join().unwrap();
}
```

```typescript
// TypeScript — 通过 Worker 发送数据副本
import { Worker } from 'node:worker_threads';

const data = 'owned';
const worker = new Worker('./worker.js');
worker.postMessage(structuredClone(data)); // 复制数据
worker.on('message', (result) => console.log(result));
```

### Sync：引用跨线程

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0)); // Arc<Mutex<i32>> 是 Send + Sync

    let mut handles = vec![];
    for _ in 0..10 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut n = data.lock().unwrap();
            *n += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", *data.lock().unwrap());
}
```

```typescript
// TypeScript — 用 SharedArrayBuffer + Atomics 实现共享状态
import { Worker } from 'node:worker_threads';

const buffer = new SharedArrayBuffer(4);
const counter = new Int32Array(buffer);

for (let i = 0; i < 10; i++) {
    const worker = new Worker('./worker.js');
    worker.postMessage(buffer); // 共享同一块内存
}

// worker.js 内部
// Atomics.add(counter, 0, 1);
```

### 手动实现 Send（unsafe）

```rust,ignore
use std::thread;

struct MyType {
    raw: *const u8, // 裸指针默认不是 Send
}

unsafe impl Send for MyType {}

fn main() {
    let t = MyType { raw: std::ptr::null() };

    thread::spawn(move || {
        // 现在可以跨线程 move
        let _ = t;
    }).join().unwrap();
}
```

```typescript
// TypeScript — 没有编译期线程安全 trait
// 任何数据都可以通过 postMessage 发送，安全由运行时/开发者保证
class MyType {
    raw: ArrayBuffer;
    constructor() {
        this.raw = new ArrayBuffer(8);
    }
}

const worker = new Worker('./worker.js');
worker.postMessage(new MyType(), [myType.raw]); // transfer
```

### 用 PhantomData 控制 auto trait 推导

```rust
use std::marker::PhantomData;
use std::rc::Rc;

struct NotSend<T> {
    value: T,
    _marker: PhantomData<Rc<()>>, // 让编译器认为该结构体包含了 Rc
}

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<NotSend<i32>>();
    // ❌ 编译错误：NotSend<i32> 因 PhantomData<Rc<()>> 而不是 Send
}
```

```typescript
// TypeScript — 没有类型系统标记控制运行时跨线程能力
// 无法通过类型声明阻止对象被 postMessage
```

## 容易踩的坑

1. **`Rc<T>` 不是 Send**——`Rc` 的引用计数是非原子的，跨线程 move 会导致数据竞争；多线程共享请用 `Arc<T>`。
2. **`RefCell<T>` 不是 Sync**——它在运行时做借用检查，多个线程同时持有引用会导致未定义行为；多线程共享可变状态用 `Mutex<T>` 或 `RwLock<T>`。
3. **裸指针默认不是 Send/Sync**——`Box`、`Vec` 等拥有型指针是 Send，但 `*const T` / `*mut T` 默认不是，需要手动 `unsafe impl` 并自担安全责任。
4. **自定义结构体按字段自动推导**——编译器独立判断：`Send` 只看所有字段是否都实现 `Send`，`Sync` 只看所有字段是否都实现 `Sync`；两个 trait 的推导互不影响。只要有一个字段不满足某个 trait，该 trait 就不会被自动实现。例如 `RefCell<i32>` 是 `Send` 但不是 `Sync`，包含它的结构体也是 `Send` 但不是 `Sync`。不要把它理解成“字段 trait 的交集”。
5. **误用 `unsafe impl Send/Sync`**——手动实现意味着"我比编译器更懂这个类型"；如果内部真的包含 `Rc` 或 `RefCell`，会在运行时产生不可预测的数据竞争。

## 交叉链接

- → [线程](thread.md) — `spawn` 闭包对 `Send` 的实际要求
- → [消息传递](message-passing.md) — 不需要 `Sync` 的并发通信方式
- → [同步原语](sync-primitives.md) — `Arc<Mutex<T>>` 如何同时满足 `Send` 与 `Sync`
