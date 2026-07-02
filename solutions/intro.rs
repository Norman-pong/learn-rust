// solutions/intro.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/intro.rs`

// ============================================================
// Chapter 00: 介绍 — 参考答案
// ============================================================

// Exercise intro1
// 让 println! 输出 "Hello Rust!"
#[test]
fn intro1() {
    println!("Hello Rust!");
}

// Exercise intro2
// 使用正确的格式说明符打印 i32 值
#[test]
fn intro2() {
    let answer: i32 = 42;
    println!("The answer is {}", answer);
}

// Exercise intro3
// 修复 assert 使其通过
#[test]
fn intro3() {
    assert_eq!(1 + 1, 2);
}

// Exercise intro4
// 返回正确的类型：把字符串字面量改成 String
#[test]
fn intro4() {
    fn message() -> String {
        String::from("Hello")
    }

    assert_eq!(message(), "Hello");
}

// Exercise intro5
// Rust 的单元类型 () 表示没有返回值；请让 x 为 () 类型
#[test]
fn intro5() {
    let x: () = ();
    let _ = x;
}
