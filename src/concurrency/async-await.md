# Async/Await

> **一句话**：Rust 的 `async`/`await` 是协作式并发——一个线程可以运行成千上万个异步任务，比系统线程轻量得多。

## 与 JS/TS 的关键差异

| 概念 | Rust | TypeScript |
|------|------|------------|
| async 函数 | 返回 `impl Future<Output = T>`（惰性） | 返回 `Promise<T>`（立即执行） |
| await | `.await` 后缀语法 | `await` 前缀关键字 |
| 执行模型 | 需要外部 runtime 驱动（tokio/smol） | 事件循环自动驱动 |
| 取消 | Drop Future（协作式） | AbortController |
| 错误处理 | Future 不内置错误通道（`Result` 包装） | Promise rejection |

**核心差异**：Rust 的 `async fn` 是惰性的——调用不执行任何代码，只是构造一个 `Future`。需要 `.await` 或交给 runtime 才会真正执行。这与 JS 的 Promise 立即执行完全不同。

## 代码对比表

### 基础 async/await

```rust
// 需要 tokio 或其他 runtime
async fn fetch_data() -> String {
    // 模拟 IO 操作
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    "data".to_string()
}

async fn process() {
    let data = fetch_data().await;  // 等待 Future 完成
    println!("got: {data}");
}

// 需要 runtime 来运行
// #[tokio::main]
// async fn main() {
//     process().await;
// }
```

```typescript
// TypeScript — Promise 立即执行
async function fetchData(): Promise<string> {
    await new Promise(r => setTimeout(r, 100));
    return "data";
}
// fetchData() 立即开始执行（返回 Promise）
```

### Future 模型

```rust
// Future trait（简化版）
// trait Future {
//     type Output;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// }

// Pin: 保证 Future 不会在内存中移动
// Context: 提供 waker 用于通知 runtime 可以再次 poll
```

### 并发执行多个 Future

```rust
// tokio::join! — 并发等待多个 Future
async fn get_user() -> String { "user".into() }
async fn get_posts() -> String { "posts".into() }

async fn load_page() {
    let (user, posts) = tokio::join!(get_user(), get_posts());
    println!("{user}, {posts}");
}

// tokio::select! — 竞速，取最先完成的
use tokio::time::{sleep, Duration};

async fn race() {
    tokio::select! {
        _ = sleep(Duration::from_secs(1)) => println!("1s passed"),
        _ = sleep(Duration::from_secs(2)) => println!("2s passed"),
    }
}
```

```typescript
// TypeScript — Promise.all 并发
const [user, posts] = await Promise.all([getUser(), getPosts()]);

// Promise.race — 竞速
await Promise.race([
    new Promise(r => setTimeout(r, 1000)),
    new Promise(r => setTimeout(r, 2000)),
]);
```

## 容易踩的坑

1. **Future 惰性**——`async fn` 不调用 `.await` 就不会执行任何代码
2. **忘记 `.await`**——编译器会 warn，但不会阻止 Future 被 drop
3. **阻塞操作**——`async fn` 中调用 `std::thread::sleep` 会阻塞整个 runtime
4. **Pin 的复杂性**——自引用类型的 async fn 产生 `!Unpin` Future，操作受限
5. **runtime 依赖**——没有 tokio/smol 就无法运行 async 代码

## 交叉链接

- → [线程](thread.md) — async 是系统线程的协作式替代
- → [Tokio 入门](tokio.md) — 最常用的异步 runtime
- → [Send 与 Sync](send-sync.md) — async task 的 Send 约束
