# 消息传递

> **一句话**：Rust 的并发哲学是"通过消息传递共享内存，而不是通过共享内存通信"；`std::sync::mpsc` 提供的多生产者单消费者（MPSC）channel 是这一哲学的标准实现。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript (Node.js) |
|------|------|---------------------|
| 通信模型 | 基于 channel 的消息传递 | `Worker` 的 `postMessage` / `on('message')` |
| 多生产者 | `Sender<T>` 可复制，`tx.clone()` 创建多生产者 | Worker 本身是 1:1 的，需要手动分发 |
| 接收方式 | 阻塞 `recv()` 与非阻塞 `try_recv()` | 事件驱动，异步回调 |
| 类型安全 | 编译期类型检查，channel 类型在创建时确定 | 序列化后运行时检查，类型易漂移 |
| 数据拷贝 | 发送值 move 进 channel（按类型可能是复制或移动） | 结构化克隆（structured clone） |

**核心差异**：Rust 的 channel 在编译期就固定了传输类型，发送与接收端共享同一类型契约；TypeScript 的 `postMessage` 依赖运行时序列化，发送与接收之间没有编译器保证，类型错误只能在运行时发现。

## 代码对比表

### 基础 mpsc::channel

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        tx.send("hello from thread".to_string()).unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("{msg}");
}
```

```typescript
// TypeScript (Node.js Worker Threads)
import { Worker } from 'node:worker_threads';

const worker = new Worker('./worker.js');

worker.postMessage({ text: 'hello from worker' });

worker.on('message', (msg) => {
    console.log(msg.text);
});
```

### 多生产者单消费者

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let mut handles = vec![];

    for id in 0..5 {
        let tx = tx.clone(); // 每个线程一个 Sender 副本
        handles.push(thread::spawn(move || {
            tx.send(id * 10).unwrap();
        }));
    }

    drop(tx); // 关闭原始 sender，等所有 clone 都 drop 后 rx.recv 会返回 Err

    let mut sum = 0;
    while let Ok(value) = rx.recv() {
        sum += value;
    }

    println!("sum = {sum}");

    for h in handles {
        h.join().unwrap();
    }
}
```

```typescript
// TypeScript — 需要手动把任务分发给多个 worker
import { Worker } from 'node:worker_threads';

const workers = Array.from({ length: 5 }, () => new Worker('./worker.js'));
let completed = 0;
let sum = 0;

for (let i = 0; i < workers.length; i++) {
    workers[i].postMessage({ id: i });
    workers[i].on('message', (value) => {
        sum += value;
        completed++;
        if (completed === workers.length) {
            console.log(`sum = ${sum}`);
        }
    });
}
```

### 非阻塞接收

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    tx.send(42).unwrap();

    // 非阻塞尝试接收
    match rx.try_recv() {
        Ok(v) => println!("got {v}"),
        Err(std::sync::mpsc::TryRecvError::Empty) => println!("empty"),
        Err(std::sync::mpsc::TryRecvError::Disconnected) => println!("disconnected"),
    }

    // 此时 channel 已空，try_recv 返回 Empty
    assert!(rx.try_recv().is_err());
}
```

```typescript
// TypeScript — 没有内置 tryReceive，通常用事件循环
import { Worker } from 'node:worker_threads';

const worker = new Worker('./worker.js');
worker.postMessage('ping');

worker.on('message', (msg) => {
    console.log('got', msg);
});

// 没有“peek”或“try”语义，消息只能回调处理
```

## 容易踩的坑

1. **原始 `Sender` 未 `drop`**——主线程保留 `tx` 不关闭，`rx.recv()` 会永远阻塞，即使所有克隆线程都已退出。
2. **多生产者时忘记 `tx.clone()`**——`Sender` 是 Move 语义，直接 move 进一个线程后其他线程无法再发送。
3. **在循环外只 `clone()` 一次**——所有线程共享同一个 `Sender` 副本，导致串行发送，失去并行意义。
4. **用 `recv()` 做轮询**——需要非阻塞时应使用 `try_recv()` 或 `recv_timeout()`，否则阻塞主线程。
5. **channel 类型不统一**——`mpsc::channel::<T>()` 一旦创建，发送与接收类型即被锁定，混用类型会导致编译错误。

## 交叉链接

- → [线程](thread.md) — 与 `std::thread::spawn` 配合使用
- → [Send 与 Sync](send-sync.md) — 为什么 `T` 必须实现 `Send` 才能进 channel
- → [同步原语](sync-primitives.md) — 共享可变状态时的替代方案
