// exercises/src/traits.rs
// Chapter 15: Traits — rustlings fork
// 深做章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn traits1() {
    trait Greet {
    // I AM NOT DONE
        fn greet(&self) -> String;
    }

    impl Greet for u32 {
    // I AM NOT DONE
        fn greet(&self) -> String {
            format!("I am number {self}")
        }
    }

    assert_eq!(42.greet(), "I am number 42".to_string());
}

// Exercise traits2
// 使用 #[derive] 让 Point 支持 Debug、PartialEq 和 Clone。
#[test]
#[ignore]
    // I AM NOT DONE
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
#[ignore]
    // I AM NOT DONE
fn traits3() {
    trait Summary {
    // I AM NOT DONE
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
    // I AM NOT DONE
        fn summarize(&self) -> String {
            format!("NEWS: {}", self.headline)
        }
    }

    impl Summary for Tweet {
    // I AM NOT DONE
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
#[ignore]
    // I AM NOT DONE
fn traits4() {
    trait Greet {
    // I AM NOT DONE
        fn greet(&self) -> String;
    }

    impl Greet for String {
    // I AM NOT DONE
        fn greet(&self) -> String {
            format!("Hello, {self}!")
        }
    }
    // I AM NOT DONE

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
#[ignore]
    // I AM NOT DONE
fn traits5() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DROPPED_COUNT: AtomicUsize = AtomicUsize::new(0);

    struct DroppingCounter {
        _id: usize,
    }

    impl Drop for DroppingCounter {
    // I AM NOT DONE
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
