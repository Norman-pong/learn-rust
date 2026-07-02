// exercises/src/intro.rs
// Chapter 00: intro — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn intro1() {
    // I AM NOT DONE

    // 让 println! 输出 "Hello Rust!"
    println!("");
}

#[test]
#[ignore]
fn intro2() {
    // I AM NOT DONE

    // 使用正确的格式说明符打印 i32 值
    let answer: i32 = 42;
    println!("The answer is {}", answer);
}

#[test]
#[ignore]
fn intro3() {
    // I AM NOT DONE

    // 修复 assert 使其通过
    assert_eq!(1 + 1, 3);
}

#[test]
#[ignore]
fn intro4() {
    // I AM NOT DONE

    // 返回正确的类型：把字符串字面量改成 String
    fn message() -> String {
        todo!("原代码返回 \"Hello\"，类型为 &str，与返回类型 String 不匹配")
    }

    assert_eq!(message(), "Hello");
}

#[test]
#[ignore]
fn intro5() {
    // I AM NOT DONE

    // Rust 的单元类型 () 表示没有返回值；请让 x 为 () 类型
    let _x: () = todo!("原代码 let x: () = 42; 类型不匹配：() 不能由整数初始化");
}
