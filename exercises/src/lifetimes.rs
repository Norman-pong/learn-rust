// exercises/src/lifetimes.rs
// Chapter 16: Lifetimes — rustlings fork
// 深做章节

// 生命周期标注帮助编译器验证引用有效性。本章覆盖函数、结构体和
// impl 块上的生命周期标注。

#[test]
#[ignore]
fn lifetimes1() {
    // I AM NOT DONE
    // 标注函数签名，让返回的引用与输入字符串中活得较短的那个一样长。
    fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // I AM NOT DONE: 补全函数体，返回 x 和 y 中较短者
        if x.len() < y.len() { x } else { y }
    }

    let s1 = String::from("Rust");
    let s2 = String::from("is");
    let result = shortest(&s1, &s2);
    assert_eq!(result, "is");
}

#[test]
#[ignore]
fn lifetimes2() {
    // I AM NOT DONE
    // 为 Excerpt 结构体及其方法添加生命周期参数。
    struct Excerpt<'a> {
        part: &'a str,
    }

    impl<'a> Excerpt<'a> {
        fn level(&self) -> i32 {
            42
        }
    }

    let text = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt {
        part: &text[..],
    };

    assert_eq!(excerpt.level(), 42);
}

#[test]
#[ignore]
fn lifetimes3() {
    // I AM NOT DONE
    // 实现一个函数，返回从字符串中按第一个空格切分的前半部分。
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or(s)
    }

    let s = String::from("hello world");
    assert_eq!(first_word(&s), "hello");

    let s = "Rust";
    assert_eq!(first_word(s), "Rust");
}
