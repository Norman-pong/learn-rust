// exercises/src/clippy.rs
// Chapter 22: Clippy — rustlings fork
// 深做章节

// Clippy 是 Rust 的 lint 工具集合，用于捕获常见错误和风格问题。
// 本章通过手动修复代码，使其符合 clippy 建议。

#[test]
#[ignore]
fn clippy1() {
    // I AM NOT DONE
    // clippy 会提示：使用 Option::map_or 或 if let 代替 match 的布尔模式。
    todo!("手动修复以下代码以消除 clippy 警告");

    fn is_some(x: Option<i32>) -> bool {
        match x {
            Some(_) => true,
            None => false,
        }
    }

    assert!(is_some(Some(42)));
    assert!(!is_some(None));
}

#[test]
#[ignore]
fn clippy2() {
    // I AM NOT DONE
    // clippy 会提示：不要对常量字符串使用 .to_string()，应使用 .to_owned() 或 into()
    // 或者使用字面量。这里要求改为使用 String::from。
    todo!("手动修复以下代码以消除 clippy 警告");

    let s = "hello".to_string();

    assert_eq!(s, "hello");
}
