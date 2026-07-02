// exercises/src/threads.rs
// Chapter 20: Threads — rustlings fork
// 深做章节

// 线程允许并发执行。本章覆盖 thread::spawn、JoinHandle、以及
// Arc<Mutex<T>> 在线程间共享可变状态。

#[test]
#[ignore]
fn threads1() {
    // I AM NOT DONE
    // 使用 thread::spawn 创建一个新线程，并等待它返回一个值。
    use std::thread;

    let handle = thread::spawn(|| 42);

    let result: i32 = todo!("join handle 并获取结果");
    assert_eq!(result, 42);
}

#[test]
#[ignore]
fn threads2() {
    // I AM NOT DONE
    // 使用 move 闭包把数据传入线程。
    let data = String::from("hello from thread");
    let handle = std::thread::spawn(move || format!("{}", data));

    let expected: String = todo!("期望的字符串");
    assert_eq!(handle.join().unwrap(), expected);
}

#[test]
#[ignore]
fn threads3() {
    // I AM NOT DONE
    // 使用 Arc<Mutex<T>> 让多个线程安全地累加计数器。
    use std::sync::{Arc, Mutex};

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        todo!("等待所有线程结束");
    }

    let final_count: i32 = todo!("最终计数器值");
    assert_eq!(*counter.lock().unwrap(), final_count);
}
