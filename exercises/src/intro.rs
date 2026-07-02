// exercises/src/intro.rs
// Chapter ??: intro.rs — rustlings fork
// 章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn intro1() {
    println!("Hello Rust!");
}

// Exercise intro2
// 使用正确的格式说明符打印 i32 值
#[test]
#[ignore]
    // I AM NOT DONE
fn intro2() {
    let answer: i32 = 42;
    println!("The answer is {}", answer);
}

// Exercise intro3
// 修复 assert 使其通过
#[test]
#[ignore]
    // I AM NOT DONE
fn intro3() {
    assert_eq!(1 + 1, 2);
}

// Exercise intro4
// 返回正确的类型：把字符串字面量改成 String
#[test]
#[ignore]
    // I AM NOT DONE
fn intro4() {
    // I AM NOT DONE
    fn message() -> String {
        String::from("Hello")
    }

    assert_eq!(message(), "Hello");
}

// Exercise intro5
// Rust 的单元类型 () 表示没有返回值；请让 x 为 () 类型
#[test]
#[ignore]
    // I AM NOT DONE
fn intro5() {
    let x: () = ();
    let _ = x;
}
