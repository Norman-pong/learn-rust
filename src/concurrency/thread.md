# 线程

> **一句话**：Rust 的 `std::thread` 提供 1:1 系统线程——配合所有权系统，编译器在编译期防止数据竞争。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript (Node.js) |
|------|------|---------------------|
| 线程模型 | 1:1 系统线程 (`std::thread::spawn`) | 单线程事件循环 + Worker Threads |
| 数据共享 | 所有权模型保证线程安全（编译期） | 结构化克隆或 SharedArrayBuffer |
| 同步 | Mutex/Arc/Channel | Atomics/MessageChannel |
| 返回值 | `JoinHandle&lt;T&gt;.join()` 返回 `Result&lt;T&gt;` |

**核心差异**：Rust 的类型系统在编译期检查线程安全——`Send` 和 `Sync` trait 决定类型能否在线程间传递。

## 代码对比表

### 基础 spawn + join

```rust
use std::thread;

let handle = thread::spawn(|| {
    // 在新线程中执行
    println!("Hello from thread!");
    42  // 返回值
});

// 主线程继续执行...
let result = handle.join().unwrap();  // 等待线程结束，获取返回值
assert_eq!(result, 42);
```

```typescript
// TypeScript (Node.js Worker Threads)
import { Worker } from 'node:worker_threads';
const worker = new Worker('./worker.js');
worker.on('message', (result) => console.log(result));
```

### move 闭包 — 转移所有权到线程

```rust
let data = vec![1, 2, 3];

let handle = thread::spawn(move || {
    // data 的所有权被 move 到新线程
    println!("{:?}", data);
});
// println!("{:?}", data);  // ❌ data 已被 move

handle.join().unwrap();
```

```typescript
// TypeScript — structuredClone 显式复制
const data = [1, 2, 3];
const worker = new Worker('./worker.js');
worker.postMessage(structuredClone(data));
console.log(data);  // ✅ 仍可用
```

### `Arc&lt;Mutex&lt;T&gt;&gt;` — 多线程共享可变数据

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

assert_eq!(*counter.lock().unwrap(), 10);
```

```typescript
// TypeScript — Atomics
const buffer = new SharedArrayBuffer(4);
const counter = new Int32Array(buffer);
// Atomics.add(counter, 0, 1);  // 原子操作
```

## Send 与 Sync trait（概述）

| Trait | 含义 | 示例 |
|-------|------|------|
| `Send` | 类型的所有权可以在线程间传递 | `i32`, `String`, `Arc<T>` 是 Send；`Rc<T>` 不是 |
| `Sync` | 类型的引用可以在线程间共享 | `Mutex<T>` 是 Sync；`RefCell<T>` 不是 |

> 详细参见 [Send 与 Sync](send-sync.md)

## 容易踩的坑

1. **闭包捕获了非 Send 类型**——`Rc<T>` 不能传进线程，用 `Arc<T>` 替代
2. **忘记 move**——线程闭包默认借用，需要 `move` 转移所有权
3. **Mutex 中毒**——持有锁的线程 panic 会导致 Mutex 中毒（poisoned）
4. **死锁**——多次 `lock()` 嵌套调用，尤其在同一个线程中
5. **`JoinHandle` 不 join**——线程变成 detached，资源可能泄漏

## 交叉链接

- → [Async/Await](async-await.md) — 协程式并发 vs 系统线程
- → [Tokio 入门](tokio.md) — 异步运行时替代手动线程管理
- → [Send 与 Sync](send-sync.md) — 线程安全 trait 详解
