// exercises/src/variables.rs
// Chapter 01: 变量 — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn variables1() {
    let mut x = 5;
    x = 7;
    assert_eq!(x, 7);
}

// Exercise variables2
// 给变量标注 i32 类型，整数才能做加法。
#[test]
#[ignore]
    // I AM NOT DONE
fn variables2() {
    let x: i32 = 42;
    assert_eq!(x + 1, 43);
}

// Exercise variables3
// 常量必须显式标注类型。
#[test]
#[ignore]
    // I AM NOT DONE
fn variables3() {
    const THREE: i32 = 3;
    assert_eq!(THREE, 3);
}

// Exercise variables4
// 避免遮蔽带来的类型混淆，把字符串绑定换个名字。
#[test]
#[ignore]
    // I AM NOT DONE
fn variables4() {
    let x = 10;
    let _description = "ten";
    assert_eq!(x, 10);
}

// Exercise variables5
// 用 `mut` 声明可变变量，并通过 `+=` 自增。
#[test]
#[ignore]
    // I AM NOT DONE
fn variables5() {
    let mut number = 1;
    number += 1;
    assert_eq!(number, 2);
}

// Exercise variables6
// 把 a 改成浮点数，除法结果就是 1.5。
#[test]
#[ignore]
    // I AM NOT DONE
fn variables6() {
    let a = 3.0;
    let b = 2.0;
    let result = a / b;
    assert_eq!(result, 1.5);
}

// Exercise variables7
// 使用可变的 `String`，才能调用 `push_str` 追加内容。
#[test]
#[ignore]
    // I AM NOT DONE
fn variables7() {
    let mut s = String::from("hello");
    s.push_str(" world");
    assert_eq!(s, "hello world");
}

// Exercise variables8
// 利用遮蔽重新绑定同名变量。
#[test]
#[ignore]
    // I AM NOT DONE
fn variables8() {
    let name = "Alice";
    let name = "Bob";
    assert_eq!(name, "Bob");
}
