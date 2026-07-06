# 同步原语

> **一句话**：Rust 标准库 `std::sync` 提供了一组线程级同步工具：`Mutex`/`RwLock` 保护共享状态，`Arc` 提供原子引用计数，`Barrier`/`Condvar` 协调多线程执行节奏。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript (Node.js) |
|------|------|---------------------|
| 互斥锁 | `Mutex<T>`，编译期要求 `T: Send` | 无语言级互斥锁，靠 `Atomics` + 锁原语实现 |
| 读写锁 | `RwLock<T>`，多读单写 | 无直接等价物 |
| 引用计数 | `Arc<T>`（原子）/ `Rc<T>`（非原子） | 自动 GC，无显式引用计数 |
| 屏障 | `Barrier` | 无直接等价物，需手动 Promise/Barrier 实现 |
| 条件变量 | `Condvar` 的 `wait`/`notify` | `Atomics.wait`/`Atomics.notify`（仅 SharedArrayBuffer） |
| 锁管理 | `MutexGuard` 作用域结束时自动释放 | 必须手动释放锁或设计释放逻辑 |

**核心差异**：Rust 的同步原语与类型系统紧密结合，例如 `Mutex<T>` 本身持有数据，锁的守卫 `MutexGuard<T>` 在离开作用域时自动释放，避免忘记解锁。JavaScript 运行在单线程事件循环上，传统代码不需要锁；只有在 `Worker` + `SharedArrayBuffer` 场景下才使用 `Atomics` 做低级同步。

## 代码对比表

### Mutex + Arc：多线程共享可变数据

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // MutexGuard 在此作用域结束自动释放
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap()); // 10
}
```

```typescript
// TypeScript — SharedArrayBuffer + Atomics
import { Worker } from 'node:worker_threads';

const buffer = new SharedArrayBuffer(4);
const counter = new Int32Array(buffer);
const workers = [];

for (let i = 0; i < 10; i++) {
    const worker = new Worker('./worker.js');
    worker.postMessage(buffer);
    workers.push(worker);
}

// worker.js
// Atomics.add(counter, 0, 1);
```

### RwLock：多读单写

```rust
use std::sync::RwLock;

fn main() {
    let data = RwLock::new(vec![1, 2, 3]);

    {
        let read = data.read().unwrap();
        println!("read: {:?}", *read); // 多个 reader 可同时持有
    }

    {
        let mut write = data.write().unwrap();
        write.push(4); // writer 独占
    }

    println!("{:?}", *data.read().unwrap());
}
```

```typescript
// TypeScript — 无语言级读写锁
// 在 Worker 之间共享只读数据通常直接传递对象
// 写操作需要自行设计消息协议或串行队列
```

### Barrier：多线程同步点

```rust
use std::sync::Barrier;
use std::thread;

fn main() {
    let n = 3;
    let barrier = Barrier::new(n);
    let mut handles = vec![];

    for i in 0..n {
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("thread {i} before barrier");
            barrier.wait();
            println!("thread {i} after barrier");
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
```

```typescript
// TypeScript — 需要自行实现 Barrier
import { Worker } from 'node:worker_threads';

function createBarrier(n: number, onDone: () => void) {
    let count = 0;
    return () => {
        count++;
        if (count === n) onDone();
    };
}

// 每个 worker 完成阶段任务后回调 barrier
```

### Condvar：条件变量

```rust
use std::sync::{Condvar, Mutex};
use std::thread;

fn main() {
    let pair = (Mutex::new(false), Condvar::new());
    let pair = std::sync::Arc::new(pair);

    let pair2 = std::sync::Arc::clone(&pair);
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started = true");
}
```

```typescript
// TypeScript — Atomics.wait/notify 在 SharedArrayBuffer 上模拟条件变量
// worker.js
// Atomics.wait(int32, 0, 0); // 等待直到索引 0 的值不再是 0

// 主线程
// Atomics.store(int32, 0, 1);
// Atomics.notify(int32, 0, 1);
```

### Arc：原子引用计数

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            println!("{:?}", data);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
```

```typescript
// TypeScript — 无显式引用计数，对象共享由 GC 管理
// 多个 Worker 之间共享引用通过 postMessage 的 transfer list
const buffer = new SharedArrayBuffer(12);
worker.postMessage(buffer, [buffer]);
// 原上下文不再持有该 buffer
```

## 容易踩的坑

1. **`Mutex` 中毒**——持有锁的线程 panic 后，`Mutex` 进入 poisoned 状态，后续 `lock()` 返回 `PoisonError`；可用 `into_inner()` 或 `lock().unwrap_or_else(|e| e.into_inner())` 恢复。
2. **`RwLock` 写者饥饿**——标准库 `RwLock` 不保证读者/写者公平，大量读者可能饿死写者；需要公平策略时考虑第三方 crate。
3. **锁的粒度太粗**——把整个结构体放进一个 `Mutex` 会降低并发度；尽量只保护真正需要互斥的字段。
4. **`MutexGuard` 跨越 await 点**——`MutexGuard` 不是 `Send`（某些平台）或不应在异步中久持，否则会在 async 函数中引发编译错误；异步场景请使用 `tokio::sync::Mutex`。
5. **`Condvar` 忘记用 while 循环**——唤醒可能是虚假唤醒（spurious wakeup），必须用 `while !condition { guard = cvar.wait(guard).unwrap(); }` 而不是 `if`。

## 交叉链接

- → [线程](thread.md) — 同步原语与 `std::thread::spawn` 的配合
- → [消息传递](message-passing.md) — 不需要共享状态的替代并发方案
- → [Send 与 Sync](send-sync.md) — 理解 `Arc<Mutex<T>>` 为何是 `Send + Sync`
