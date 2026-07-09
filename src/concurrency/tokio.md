# Tokio 入门

> **一句话**：Tokio 是 Rust 事实标准的异步 runtime——它把 JS/TS 里“内置在引擎里的事件循环”变成了显式引入、可配置、多线程的 async 操作系统。

## 与 JS/TS 的关键差异

JS/TS 的异步基于单线程事件循环（libuv），`Promise`/`async`/`await` 由运行时自动调度。Rust 的 `async`/`await` 本身没有 runtime：必须安装一个外部 crate 才能执行 `Future`。Tokio 就是这个“外部事件循环”，但它把 worker threads、任务队列、I/O driver、定时器、signal handler 都暴露成可配置组件，而不是像 JS 那样全部内建在虚拟机里。

| Tokio | JS/TS |
|-------|-------|
| `#[tokio::main]` | Node.js 启动时隐式创建的事件循环 |
| `tokio::spawn` | 把回调/Promise 交给事件循环 |
| `tokio::time::sleep` | `setTimeout` / `setInterval` |
| `tokio::net::TcpListener` | `net.createServer()` |
| `tokio::sync::mpsc` | `EventEmitter`、自定义 channel 或 `BroadcastChannel` |
| `tokio::select!` | `Promise.race()` |
| `tokio::task::JoinHandle` | `Promise`（**不可**取消，只能等待 settle） |
| Runtime feature flags | 不需要 feature flags，API 全部内置 |

## 依赖声明

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```json
// package.json — JS/TS 不需要安装 runtime，但用 Node 的 net 模块
{
  "dependencies": {}
}
```

## 启动 runtime

```rust
#[tokio::main]      // 等价于：创建 runtime → block_on(async { ... })
async fn main() {
    println!("Hello from tokio!");
}
```

```typescript
// TypeScript 没有显式 runtime 启动
async function main() {
    console.log("Hello from Node.js event loop!");
}
main();
```

## 启动异步任务

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let handle: task::JoinHandle<i32> = task::spawn(async {
        // 在 tokio 的线程池上执行
        42
    });

    let result = handle.await.unwrap();
    assert_eq!(result, 42);
}
```

```typescript
// TypeScript：Promise 本身不提供“取消”语义
async function main(): Promise<void> {
    const task = (async () => 42)();
    const result = await task;
    console.assert(result === 42);
}
main();
```

## 定时器

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(1)).await;
    println!("1 second passed");
}
```

```typescript
import { setTimeout } from "node:timers/promises";

async function main(): Promise<void> {
    await setTimeout(1000);
    console.log("1 second passed");
}
main();
```

## 同时等待多个操作

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_secs(1)) => println!("1s won"),
        _ = sleep(Duration::from_secs(2)) => println!("2s won"),
    }
}
```

```typescript
async function main(): Promise<void> {
    const t1 = setTimeout(1000).then(() => "1s won");
    const t2 = setTimeout(2000).then(() => "2s won");

    const winner = await Promise.race([t1, t2]);
    console.log(winner);
}
main();
```

## 共享可变状态

```rust
use std::sync::{Arc, Mutex};
use tokio::task;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(task::spawn(async move {
            let mut n = c.lock().unwrap();
            *n += 1;
        }));
    }

    for h in handles { h.await.unwrap(); }
    assert_eq!(*counter.lock().unwrap(), 10);
}
```

```typescript
// TypeScript：单线程事件循环，无需锁，但 CPU 阻塞会饿死事件循环
async function main(): Promise<void> {
    let counter = 0;
    const tasks: Promise<void>[] = [];

    for (let i = 0; i < 10; i++) {
        tasks.push((async () => { counter += 1; })());
    }

    await Promise.all(tasks);
    console.assert(counter === 10);
}
main();
```

## TCP Echo Server

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
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return,          // 客户端断开
                    Ok(n) => n,
                    Err(e) => { eprintln!("read error: {e}"); return; }
                };

                if let Err(e) = socket.write_all(&buf[..n]).await {
                    eprintln!("write error: {e}"); return;
                }
            }
        });
    }
}
```

```typescript
import * as net from "node:net";

const server = net.createServer((socket) => {
    socket.on("data", (data) => {
        socket.write(data);
    });

    socket.on("end", () => {
        // 客户端断开
    });
});

server.listen(8080, "127.0.0.1", () => {
    console.log("listening on 127.0.0.1:8080");
});
```

## 常见坑

1. **忘记 `#[tokio::main]`**——没有 runtime 的 `async fn main` 不会自动执行。
2. **在 async 中使用 `std::thread::sleep`**——会阻塞整个 tokio worker 线程，应使用 `tokio::time::sleep`。
3. **`std::sync::Mutex` 在 async 中**——`.lock()` 是阻塞调用，长时间持有会阻塞 worker；跨 await 持锁时建议用 `tokio::sync::Mutex`。
4. **`tokio::spawn` 的生命周期**——spawn 的 Future 必须是 `'static`，不能借用局部变量。
5. **feature flags 缺失**——默认 tokio 只包含少量 feature；需要 `rt-multi-thread`、`net`、`time` 等时必须在 `Cargo.toml` 中开启。
6. **在同步代码里调用 `block_on`**——阻塞当前线程直到 Future 完成；不要在 tokio runtime 内部再 `block_on` 同 runtime 的 Future。

## 交叉链接

- → [Async/Await](async-await.md) — Future 与 async 语法基础
- → [线程](thread.md) — 何时用系统线程，何时用 tokio task
- → [Send 与 Sync](send-sync.md) — `tokio::spawn` 的 `Send` 与 `'static` 约束
