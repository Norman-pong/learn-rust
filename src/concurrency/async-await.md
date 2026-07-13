# Async/Await

> **一句话定位**：Rust 的 `async`/`await` 是**协作式、零成本抽象的并发原语**——`async fn` 返回一个惰性的 `Future`，必须由 runtime（如 tokio）的 executor 轮询（poll）到完成；一个 OS 线程因此可以承载成千上万个 task，而无需为每个 task 分配线程栈。

## 与 JS/TS 的关键差异

| 维度 | Rust | TypeScript / JavaScript |
|------|------|------------------------|
| 调用即执行？ | 否。`async fn` 调用返回 `impl Future<Output = T>`，**不执行任何代码** | 是。`async function` 调用返回 `Promise`，函数体立即开始执行直到第一个 `await` |
| await 语法 | 后缀：`.await` | 前缀：`await expr`（或 `expr await` 在旧提案） |
| 事件循环 | 由外部 runtime 提供，executor 负责 `poll` | 引擎内置，自动调度 microtask / macrotask |
| 取消机制 | 丢弃 `Future` 即取消（协作式） | `AbortController` + `AbortSignal` |
| 错误通道 | 无内置 rejection，必须显式返回 `Result<T, E>` | `Promise` 可 reject，可用 `try/catch` |
| 多任务并发 | `tokio::join!` / `tokio::select!` | `Promise.all` / `Promise.race` / `Promise.allSettled` |

**核心差异**：Rust 的 `Future` 是**惰性构造的**状态机；JS 的 `Promise` 是**立即开始执行**的异步操作句柄。Rust 需要显式 `.await` 或交给 runtime 才会推进状态机。

## 代码对比

### 基础 async/await

```rust
// Rust：async fn 返回 Future，调用点不执行
async fn fetch_data() -> String {
    // tokio::time::sleep 是非阻塞的 async sleep
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    "data".to_string()
}

async fn process() {
    let data = fetch_data().await; // 真正开始执行并等待完成
    println!("got: {data}");
}

#[tokio::main]
async fn main() {
    process().await;
}
```

```typescript
// TypeScript：async function 调用后立即启动，直到第一个 await 才让出
async function fetchData(): Promise<string> {
    await new Promise(resolve => setTimeout(resolve, 100));
    return "data";
}

async function process(): Promise<void> {
    const data = await fetchData(); // Promise 已经在 resolve 路上了
    console.log(`got: ${data}`);
}

process();
```

### Future 模型

```rust
// Rust 标准库中的 Future trait（简化示意）
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// Pin<&mut Self>：防止 Future 在内存中被移动，因为 async 状态机可能自引用
// Context：提供 Waker，IO 就绪时唤醒该任务，让 executor 再次 poll
```

```typescript
// JS 没有显式的 Future/Promise 状态机暴露给开发者；
// 引擎内部维护 promise 状态（pending / fulfilled / rejected），
// 通过微任务队列自动执行 then/catch 回调。
```

### 并发等待多个任务

```rust
use tokio::time::{sleep, Duration};

async fn get_user() -> String { "user".into() }
async fn get_posts() -> String { "posts".into() }

async fn load_page() {
    // tokio::join! 类似 Promise.all：并发执行，等待全部完成
    let (user, posts) = tokio::join!(get_user(), get_posts());
    println!("{user}, {posts}");
}

async fn race() {
    // tokio::select! 类似 Promise.race：取最先完成的 Future
    tokio::select! {
        _ = sleep(Duration::from_secs(1)) => println!("1s passed"),
        _ = sleep(Duration::from_secs(2)) => println!("2s passed"),
    }
}
```

```typescript
async function getUser(): Promise<string> { return "user"; }
async function getPosts(): Promise<string> { return "posts"; }

async function loadPage(): Promise<void> {
    const [user, posts] = await Promise.all([getUser(), getPosts()]);
    console.log(`${user}, ${posts}`);
}

async function race(): Promise<void> {
    await Promise.race([
        new Promise(resolve => setTimeout(resolve, 1000)),
        new Promise(resolve => setTimeout(resolve, 2000)),
    ]);
    console.log("first finished");
}
```

### 与事件循环的对比

```rust
// Rust：没有全局事件循环；tokio 的 Runtime 管理多个线程与 task 队列
// 任务被 poll 到 await 点 -> 挂起 -> IO 完成 -> Waker 唤醒 -> 重新入队 poll
#[tokio::main]
async fn main() {
    tokio::spawn(async { /* 一个 task */ }); // 提交到 executor 的任务队列
    tokio::spawn(async { /* 另一个 task */ });
}
```

```typescript
// JS：每个执行上下文有唯一的事件循环
// 调用栈 -> 清空 -> 微任务队列（Promise.then） -> 宏任务队列（setTimeout等） -> 循环
async function main() {
    setTimeout(() => console.log("macrotask"), 0);
    Promise.resolve().then(() => console.log("microtask"));
    await Promise.resolve(); // 挂起当前 async 函数，让出主线程
    console.log(" resumed");
}
main();
// 输出顺序：microtask -> resumed -> macrotask
```

### Stream：异步迭代器

`Stream`（`tokio-stream` crate 或 `futures` crate）是 Future 的"多值版本"——类似异步的 `Iterator`，每次 `poll` 可能产出一个值或结束。在 tokio 1.x 中，stream 支持已拆分到独立的 `tokio-stream` crate。

```rust,ignore
use tokio_stream::StreamExt;

// 异步读取行流
async fn process_lines(mut lines: impl tokio_stream::Stream<Item = String>) {
    while let Some(line) = lines.next().await {
        println!("{line}");
    }
}
```

```typescript
// TypeScript 的 AsyncIterable 起同样作用
async function processLines(lines: AsyncIterable<string>) {
    for await (const line of lines) {
        console.log(line);
    }
}
```

> **Stream 不是 std 的正式部分**（截至 Rust 1.80 仍在 `std::stream` 实验特性中），实际使用通过 tokio 或 futures crate。
>
> **取消语义**：Rust 的 Future 是惰性的——丢弃（drop）一个未完成的 Future 就等于取消它，不需要 `AbortController`。但这是**协作式取消**：如果 Future 正在执行阻塞操作，drop 只能在下一个 `.await` 点生效。需要在 `select!` 中处理取消时的资源清理。

## 常见陷阱

1. **Future 是惰性的**——`let f = fetch_data();` 只是构造了一个状态机，不执行任何 IO；必须 `.await` 或 `spawn` 它。
2. **在 async 中做阻塞调用**——`std::thread::sleep` 会阻塞整个 OS 线程，导致该线程上的所有 task 一起卡住；应使用 `tokio::time::sleep`。
3. **忘记 `.await`**——`fetch_data();` 没有 `.await` 也不会报错，函数被构造后立即 drop，什么都不会发生。
4. **`.await` 持有锁**——`let guard = mutex.lock(); some_future.await;` 会让 guard 跨越 await 点，若 guard 不是 `Send` 或锁住的 Mutex 不是 async 锁，会编译失败或死锁。应使用 `tokio::sync::Mutex` 或把锁缩小到非 await 区域。
5. **自引用 Future 与 Pin**——由 `async fn` 生成的状态机经常 `!Unpin`，不能随意 `mem::swap` 或移出 `&mut` 后再取走；需要理解 `Pin` 语义才能安全组合 Future。
6. **运行时依赖**——纯 `std` 没有 executor，`async fn` 不能直接运行；必须引入 tokio / smol / async-std 等 runtime。

## 交叉链接

- → [线程](thread.md) — async task 是系统线程的协作式替代，理解何时用线程、何时用 async。
- → [Tokio 入门](tokio.md) — 最常用的异步 runtime，看 executor、spawn、channel 等具体用法。
- → [Send 与 Sync](send-sync.md) — 跨线程的 async task 对 `Send`/`'static` 有严格要求。
- → [生命周期主题](../compiler-pitfalls/lifetime-theme/index.md) — `async fn` 借用局部变量时容易出现 lifetime 问题。
