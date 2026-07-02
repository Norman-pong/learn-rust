// exercises/src/enums.rs
// Chapter 08: Enums — rustlings fork
// 快进章节

// 枚举是 Rust 中表达"多选一"数据的方式。本章练习枚举定义、模式匹配和 match。

#[test]
#[ignore]
fn enums1() {
    // I AM NOT DONE
    // 定义一个枚举 Message 包含三种变体：Quit、ChangeColor(i32, i32, i32)、Write(String)
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Write(String),
    }

    let msg = Message::Write(String::from("hello"));
    // TODO: 用 match 匹配 msg 并返回内容
    let text = match msg {
        Message::Quit => String::new(),
        Message::ChangeColor(_, _, _) => String::from("changed color"),
        Message::Write(s) => s,
    };

    assert_eq!(text, "hello");
}

#[test]
#[ignore]
fn enums2() {
    // I AM NOT DONE
    // 使用 Option 枚举表达可能为空的值
    let maybe_value: Option<i32> = Some(42);
    // TODO: 用 if let 解构 maybe_value
    let result = if let Some(v) = maybe_value {
        v * 2
    } else {
        0
    };

    assert_eq!(result, 84);
}

#[test]
#[ignore]
fn enums3() {
    // I AM NOT DONE
    // 为枚举实现方法
    enum Operation {
        Add(i32, i32),
        Multiply(i32, i32),
    }

    impl Operation {
        fn compute(&self) -> i32 {
            match self {
                Operation::Add(a, b) => a + b,
                Operation::Multiply(a, b) => a * b,
            }
        }
    }

    let op = Operation::Multiply(6, 7);
    assert_eq!(op.compute(), 42);
}
