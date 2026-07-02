// exercises/src/threads.rs
// Chapter 20: 线程 — rustlings fork
// 深做章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

use std::sync::{Arc, Mutex};
use std::thread;

#[test]
#[ignore]
    // I AM NOT DONE
fn threads1() {
    let handle = thread::spawn(|| 42);

    let result = handle.join().unwrap();

    assert_eq!(result, 42);
}

// Exercise threads2
// 使用 move 闭包把数据传入线程。
#[test]
#[ignore]
    // I AM NOT DONE
fn threads2() {
    let data = String::from("hello from thread");
    let handle = thread::spawn(move || format!("{}", data));

    assert_eq!(handle.join().unwrap(), "hello from thread");
}

// Exercise threads3
// 使用 Arc<Mutex<T>> 让多个线程安全地累加计数器。
#[test]
#[ignore]
    // I AM NOT DONE
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
