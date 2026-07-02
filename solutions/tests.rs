// solutions/tests.rs
// 本文件由脚本生成，请勿手动修改。参见 README.md 与 LICENSE。
// Copyright 2026 本人及贡献者

// Chapter 17: Writing Tests — 参考答案

#[test]
fn tests1() {
    // 补全 assert 的左值，使测试通过。
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn tests2() {
    // 使用 assert! 检查字符串是否非空。
    let message = "hello";
    assert!(!message.is_empty());
}

#[test]
#[should_panic(expected = "division by zero")]
fn tests3() {
    // 实现一个会 panic 的除法函数，并 panic 时包含指定信息。
    fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("division by zero");
        }
        a / b
    }

    let _ = divide(4, 0);
}
