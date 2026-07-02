// solutions/clippy.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/clippy.rs`

// ============================================================
// Chapter 22: Clippy — 参考答案
// ============================================================

// Exercise clippy1
// 使用 Option::is_some 替代 match 的布尔模式。
#[test]
fn clippy1() {
    fn is_some(x: Option<i32>) -> bool {
        x.is_some()
    }

    assert!(is_some(Some(42)));
    assert!(!is_some(None));
}

// Exercise clippy2
// 避免对字符串字面量使用 to_string()，改为 String::from。
#[test]
fn clippy2() {
    let s = String::from("hello");

    assert_eq!(s, "hello");
}
