// exercises/src/tests.rs
// Chapter 17: Writing Tests — rustlings fork
// 深做章节

// 测试是保证代码正确性的重要手段。本章覆盖单元测试、#[should_panic]
// 与自定义错误信息。

#[test]
#[ignore]
fn tests1() {
    // I AM NOT DONE
    // 补全 assert 的期望值，使测试通过。
    let result: i32 = 2 + 2;
    let expected: i32 = todo!("填入 2 + 2 的结果");
    assert_eq!(result, expected);
}

#[test]
#[ignore]
fn tests2() {
    // I AM NOT DONE
    // 使用 assert! 检查字符串是否非空，并添加自定义错误信息。
    todo!("使用 assert! 检查 message 非空，并添加自定义错误信息");
    let message = "hello";
    assert!(message.is_empty(), "TODO: add a custom message");
}

#[test]
#[ignore]
#[should_panic(expected = "division by zero")]
fn tests3() {
    // I AM NOT DONE
    // 实现一个会 panic 的除法函数，并 panic 时包含指定信息。
    fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            todo!("panic with message 'division by zero'")
        }
        a / b
    }

    let _ = divide(4, 0);
}
