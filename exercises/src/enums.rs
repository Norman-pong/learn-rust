// exercises/src/enums.rs
// Chapter 08: 枚举 — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn enums1() {
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Write(String),
    }

    let msg = Message::Write(String::from("hello"));
    let text = match msg {
        Message::Quit => String::new(),
        Message::ChangeColor(_, _, _) => String::from("changed color"),
        Message::Write(s) => s,
    };

    assert_eq!(text, "hello");
}

// Exercise enums2
// 使用 if let 解构 Option，处理空值返回默认值。
#[test]
#[ignore]
    // I AM NOT DONE
fn enums2() {
    let maybe_value: Option<i32> = Some(42);
    let result = if let Some(v) = maybe_value {
        v * 2
    } else {
        0
    };

    assert_eq!(result, 84);
}

// Exercise enums3
// 为枚举实现方法，在 match 中根据变体执行不同计算。
#[test]
#[ignore]
    // I AM NOT DONE
fn enums3() {
    enum Operation {
        Add(i32, i32),
        Multiply(i32, i32),
    }

    impl Operation {
    // I AM NOT DONE
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
