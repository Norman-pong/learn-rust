// exercises/src/strings.rs
// Chapter 09: 字符串 — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn strings1() {
    // String 和 &str 类型不同，请修复返回类型
    // I AM NOT DONE
    fn current() -> String {
        "Rust".to_string()
    }

    assert_eq!(current(), "Rust");
}

// Exercise strings2
// 字符串追加需要可变 String，用 String::from 把字面量转换。
#[test]
#[ignore]
    // I AM NOT DONE
fn strings2() {
    // 字符串追加需要 String 类型，请把字面量转换为 String
    let mut s = String::from("hello");
    s.push_str(", world");
    assert_eq!(s, "hello, world");
}

// Exercise strings3
// format! 宏可以混合不同类型的变量，无需手动转字符串。
#[test]
#[ignore]
    // I AM NOT DONE
fn strings3() {
    // 使用 format! 宏拼接字符串
    let name = "Rust";
    let version = 2024;
    let message = format!("Hello, {name} {version}");
    assert_eq!(message, "Hello, Rust 2024");
}

// Exercise strings4
// 字符串切片按字节索引，"Rust" 占 4 字节，所以 0..4 正好。
#[test]
#[ignore]
    // I AM NOT DONE
fn strings4() {
    // 字符串切片不能跨越字符边界，请修正范围
    let s = "Rust语言";
    let slice = &s[0..4];
    assert_eq!(slice, "Rust");
}
