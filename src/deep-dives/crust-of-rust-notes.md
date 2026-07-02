# Crust of Rust 笔记

> 对 Jon Gjengset 的 [Crust of Rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) 系列视频的学习笔记。

---

## Ep 1: Held by a Thread

**核心问题**：`std::thread::spawn` 如何把数据传进线程？`Send` trait 如何在编译期保证线程安全？

### 关键点

1. **`thread::spawn` 的签名**：
   ```rust
   pub fn spawn<F, T>(f: F) -> JoinHandle<T>
   where
       F: FnOnce() -> T + Send + 'static,
       T: Send + 'static,
   ```

2. **`Send`**：类型的所有权可以安全地转移到另一个线程
   - `i32`, `String`, `Vec<T: Send>`, `Arc<T: Send>` → `Send`
   - `Rc<T>`, `*const T`, `*mut T` → 不是 `Send`
   - `RefCell<T>`：`Send if T: Send`

3. **`'static`**：类型不包含非 `'static` 的引用
   - 闭包必须不借用任何局部变量（或借用的是 `'static` 数据）
   - 解决：用 `move` 将数据所有权移入闭包

4. **为什么 `Rc<T>` 不是 `Send`**：
   - `Rc` 的引用计数不是原子操作
   - 两个线程同时 clone/drop `Rc` 会产生数据竞争
   - 用 `Arc<T>`（Atomic Reference Count）代替

5. **`Sync`**：类型的引用可以安全地在多个线程间共享
   - `Mutex<T: Send>` → `Sync`
   - `RefCell<T>` → 不是 `Sync`（内部可变性不是线程安全的）

### 收获

- `Send`/`Sync` 是 Rust 线程安全的核心抽象——它们通过类型系统在编译期消除数据竞争
- `move` 闭包不是"拷贝数据"，而是转移所有权——编译器保证原始数据不再被使用
- `Arc` vs `Rc` 的选择不是性能问题，而是正确性问题（编译器强制执行）

---

## Ep 2: Declarative Macros

*(待补)：声明宏 `macro_rules!` 的工作原理。*

---

## Ep 3: Lifetime Annotations

*(待补)：生命周期标注的深入讲解——子类型、协变、HRTB。*

---

> 视频列表：[Crust of Rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) by Jon Gjengset
