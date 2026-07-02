// exercises/src/clippy.rs
// Chapter ??: clippy.rs — rustlings fork
// 章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn clippy1() {
    // I AM NOT DONE
    fn is_some(x: Option<i32>) -> bool {
        x.is_some()
    }

    assert!(is_some(Some(42)));
    assert!(!is_some(None));
}

// Exercise clippy2
// 避免对字符串字面量使用 to_string()，改为 String::from。
#[test]
#[ignore]
    // I AM NOT DONE
fn clippy2() {
    let s = String::from("hello");

    assert_eq!(s, "hello");
}
