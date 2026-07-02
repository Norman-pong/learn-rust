// solutions/traits.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/traits.rs`

// ============================================================
// Chapter 15: Traits — 参考答案
// ============================================================

// Exercise traits1
// 为 u32 实现 Greet trait。
#[test]
fn traits1() {
    trait Greet {
        fn greet(&self) -> String;
    }

    impl Greet for u32 {
        fn greet(&self) -> String {
            format!("I am number {self}")
        }
    }

    assert_eq!(42.greet(), "I am number 42".to_string());
}

// Exercise traits2
// 使用 #[derive] 让 Point 支持 Debug、PartialEq 和 Clone。
#[test]
fn traits2() {
    #[derive(Debug, PartialEq, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    assert_eq!(p1, p2);
    assert_eq!(format!("{:?}", p1), "Point { x: 1, y: 2 }");
}

// Exercise traits3
// 实现 Summary trait，并为 Tweet 提供 @username: content 格式。
#[test]
fn traits3() {
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
            format!("@{}: {}", self.username, self.content)
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

// Exercise traits4
// 使用泛型 trait bound 调用 greet。
#[test]
fn traits4() {
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
        thing.greet()
    }

    assert_eq!(say_hello(String::from("world")), "Hello, world!");
}

// Exercise traits5
// 实现 Drop trait，变量离开作用域时增加 DROPPED_COUNT。
#[test]
fn traits5() {
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

    assert_eq!(DROPPED_COUNT.load(Ordering::SeqCst), 3);
}
