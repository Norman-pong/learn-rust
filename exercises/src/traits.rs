// exercises/src/traits.rs
// Chapter 15: Traits — rustlings fork
// 深做章节

// Trait 是 Rust 定义共享行为的接口。本章覆盖 trait 实现、derive、
// 默认方法、泛型 trait bound、Deref 与 Drop。

#[test]
#[ignore]
fn traits1() {
    // I AM NOT DONE
    // 为类型 u32 实现 Greet trait。
    trait Greet {
        fn greet(&self) -> String;
    }

    impl Greet for u32 {
        fn greet(&self) -> String {
            todo!("返回格式为 'I am number {self}' 的字符串")
        }
    }

    assert_eq!(42.greet(), "I am number 42".to_string());
}

#[test]
#[ignore]
fn traits2() {
    // I AM NOT DONE
    // 使用 #[derive] 让 Point 支持 Debug、PartialEq 和 Clone。
    #[derive(Debug, PartialEq, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    // I AM NOT DONE: 原代码 p2 = p1.clone() 因未实现 Clone 失败；此处用 todo!() 占位
    let p2 = todo!("创建 p1 的克隆");
    assert_eq!(p1, p2);
    assert_eq!(format!("{:?}", p1), "Point { x: 1, y: 2 }");
}

#[test]
#[ignore]
fn traits3() {
    // I AM NOT DONE
    // 实现一个 Summary trait，并为 NewsArticle 和 Tweet 提供不同默认实现。
    trait Summary {
        fn summarize(&self) -> String {
            "(Read more...)".to_string()
        }
    }

    struct NewsArticle {
        headline: String,
    }

    struct Tweet {
        username: String,
        content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("NEWS: {}", self.headline)
        }
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            todo!("返回格式为 '@username: content' 的字符串")
        }
    }

    let tweet = Tweet {
        username: "rustacean".to_string(),
        content: "learning traits".to_string(),
    };
    assert_eq!(tweet.summarize(), "@rustacean: learning traits");

    let article = NewsArticle {
        headline: "Rust 1.85 released".to_string(),
    };
    assert_eq!(article.summarize(), "NEWS: Rust 1.85 released");
}

#[test]
#[ignore]
fn traits4() {
    // I AM NOT DONE
    // 使用泛型 trait bound 实现一个“能打招呼就打印”的函数。
    trait Greet {
        fn greet(&self) -> String;
    }

    impl Greet for String {
        fn greet(&self) -> String {
            format!("Hello, {self}!")
        }
    }

    fn say_hello<T>(thing: T) -> String
    where
        T: Greet,
    {
        todo!("调用 thing.greet() 并返回结果")
    }

    assert_eq!(say_hello(String::from("world")), "Hello, world!");
}

#[test]
#[ignore]
fn traits5() {
    // I AM NOT DONE
    // 实现 Drop trait，在变量离开作用域时记录被释放的次数。
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DROPPED_COUNT: AtomicUsize = AtomicUsize::new(0);

    struct DroppingCounter {
        _id: usize,
    }

    impl Drop for DroppingCounter {
        fn drop(&mut self) {
            DROPPED_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    {
        let _a = DroppingCounter { _id: 1 };
        let _b = DroppingCounter { _id: 2 };
        let _c = DroppingCounter { _id: 3 };
    }

    let dropped: usize = todo!("有多少个值被 drop？");
    assert_eq!(DROPPED_COUNT.load(Ordering::SeqCst), dropped);
}
