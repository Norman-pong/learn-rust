// exercises/src/strings.rs
// Chapter 09: strings — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn strings1() {
    // I AM NOT DONE

    // String 和 &str 类型不同，原返回类型不匹配
    fn current() -> String {
        // 原错误: mismatched types: expected String, found &str
        "Rust".to_string()
    }

    assert_eq!(current(), "Rust");
}

#[test]
#[ignore]
fn strings2() {
    // I AM NOT DONE

    // 字符串追加需要 String 类型，原绑定为 &str
    let mut s = String::from("hello");
    // 原错误: no method named push_str found for reference &str in the current scope
    s.push_str(", world");
    assert_eq!(s, "hello, world");
}

#[test]
#[ignore]
fn strings3() {
    // I AM NOT DONE

    // 使用 format! 宏拼接字符串，原用 + 直接拼接 &str 与 i32
    let name = "Rust";
    let version = 2024;
    // 原错误: cannot add &str to &str / cannot add i32 to &str
    // 原错误: E0277 `todo!()` 返回 `!`，无法与 `&str` 比较，需要显式类型注解让编译通过
    let message = format!("Hello, {name} {version}");
    assert_eq!(message, "Hello, Rust 2024");
}

#[test]
#[ignore]
fn strings4() {
    // I AM NOT DONE

    // 字符串切片不能跨越字符边界，请修正范围
    let s = "Rust语言";
    let slice = &s[0..4];
    assert_eq!(slice, "Rust");
}
