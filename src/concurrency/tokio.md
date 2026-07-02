# Tokio 入门

> **一句话**：Tokio 是 Rust 最流行的异步 runtime——提供 async I/O、多线程调度、定时器、信号处理等基础设施。

## 与 JS/TS 的关键差异

JS/TS 的异步由内置事件循环（libuv）驱动。Rust 没有内置 runtime——Tokio 就是你需要安装的"事件循环"。它像一个可配置的 async 操作系统。

## 入门

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]  // 启动 tokio runtime
async fn main() {
    println!("Hello from tokio!");
}
```

## 核心概念对比

| Tokio 概念 | 对应 JS/TS |
|-----------|-----------|
| `#[tokio::main]` | 隐式的 event loop 启动 |
| `tokio::spawn(task)` | `Promise` + 自动调度 |
| `tokio::time::sleep(dur)` | `setTimeout(fn, ms)` |
| `TcpListener::bind` | `net.createServer()` |
| `tokio::sync::mpsc` | EventEmitter / 自定义 channel |
| `tokio::select!` | `Promise.race()` |

## 代码对比表

### spawn — 启动异步任务

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let handle = task::spawn(async {
        // 异步任务，在 tokio 线程池上执行
        42
    });

    let result = handle.await.unwrap();  // 等待任务完成
    assert_eq!(result, 42);
}
```

```typescript
// TypeScript — 无显式 spawn，但类似 Promise
const result = await (async () => 42)();
```

### 共享状态 — Arc<Mutex<T>> 在 async 中

```rust
use std::sync::{Arc, Mutex};
use tokio::task;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(task::spawn(async move {
            let mut n = counter.lock().unwrap();
            *n += 1;
        }));
    }

    for h in handles { h.await.unwrap(); }
    assert_eq!(*counter.lock().unwrap(), 10);
}
```

### tokio::select! — 同时等待多个操作

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_secs(1)) => println!("1s"),
        _ = sleep(Duration::from_secs(2)) => println!("2s"),
        else => println!("all timed out"),  // 可选默认分支
    }
}
```

## 常见 Tokio 模式

### TCP Echo Server

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap_or(0);
                if n == 0 { return; }
                socket.write_all(&buf[..n]).await.unwrap();
            }
        });
    }
}
```

## 容易踩的坑

1. **忘记 `#[tokio::main]`**——async main 需要 runtime 属性宏
2. **在 async 中使用 `std::thread::sleep`**——会阻塞整个 tokio worker 线程，应用 `tokio::time::sleep`
3. **`std::sync::Mutex` 在 async 中**——`.lock()` 是阻塞调用，长时间持有锁会阻塞 worker
4. **`tokio::spawn` 的生命周期**——spawn 的 Future 必须是 `'static`（不能借用局部变量）
5. **feature flags**——默认 tokio 只包括少数 feature，需要显式开启如 `rt-multi-thread`

## 交叉链接

- → [Async/Await](async-await.md) — Future 模型基础
- → [线程](thread.md) — 何时用系统线程 vs tokio task
- → [Send 与 Sync](send-sync.md) — tokio::spawn 的 Send 约束
