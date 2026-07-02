// solutions/threads.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/threads.rs`

// ============================================================
// Chapter 20: 线程 — 参考答案
// ============================================================

use std::sync::{Arc, Mutex};
use std::thread;

// Exercise threads1
// 创建线程并等待其返回结果。
#[test]
fn threads1() {
    let handle = thread::spawn(|| 42);

    let result = handle.join().unwrap();

    assert_eq!(result, 42);
}

// Exercise threads2
// 使用 move 闭包把数据传入线程。
#[test]
fn threads2() {
    let data = String::from("hello from thread");
    let handle = thread::spawn(move || format!("{}", data));

    assert_eq!(handle.join().unwrap(), "hello from thread");
}

// Exercise threads3
// 使用 Arc<Mutex<T>> 让多个线程安全地累加计数器。
#[test]
fn threads3() {
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
}
