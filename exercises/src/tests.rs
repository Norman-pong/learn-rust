// exercises/src/tests.rs
// Chapter 17: 测试 — rustlings fork
// 深做章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn tests1() {
    // 补全 assert 的左值，使测试通过。
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
#[ignore]
    // I AM NOT DONE
fn tests2() {
    // 使用 assert! 检查字符串是否非空。
    let message = "hello";
    assert!(!message.is_empty());
}

#[test]
#[ignore]
#[should_panic(expected = "division by zero")]
    // I AM NOT DONE
fn tests3() {
    // 实现一个会 panic 的除法函数，并 panic 时包含指定信息。
    // I AM NOT DONE
    fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("division by zero");
        }
        a / b
    }

    let _ = divide(4, 0);
}
