// exercises/src/variables.rs
// Chapter 01: variables — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn variables1() {
    // I AM NOT DONE
    // x 默认不可变，不能重新赋值。修复：在 let 后加 mut
    let mut x = 5;
    x = 7;
    assert_eq!(x, 7);
}

#[test]
#[ignore]
fn variables2() {
    // I AM NOT DONE
    // 字符串不能直接加数字，需要用 parse 转换
    let x: i32 = 42;
    assert_eq!(x + 1, 43);
}

#[test]
#[ignore]
fn variables3() {
    // I AM NOT DONE
    // const 必须显式标注类型，且 const THREE: i32 = todo!() 会因 todo!() 默认返回 () 导致类型不匹配
    const THREE: i32 = 3;
    assert_eq!(THREE, 3);
}

#[test]
#[ignore]
fn variables4() {
    // I AM NOT DONE
    // 遮蔽（shadowing）导致 x 变成了字符串，但 assert 期望 i32
    let x = 10;
    // 提示：修改变量名，不要遮蔽
    let _description = "ten";
    assert_eq!(x, 10);
}

#[test]
#[ignore]
fn variables5() {
    // I AM NOT DONE
    // number 不可变，不能用 +=。修复：加 mut
    let mut number = 1;
    number += 1;
    assert_eq!(number, 2);
}

#[test]
#[ignore]
fn variables6() {
    // I AM NOT DONE
    // 整数除法得到 1（截断），不是 1.5。修复：用浮点类型
    let a = 3.0;
    let b = 2.0;
    let result = a / b;
    assert_eq!(result, 1.5);
}

#[test]
#[ignore]
fn variables7() {
    // I AM NOT DONE
    // &str 没有 push_str 方法，需要用 String
    let mut s = String::from("hello");
    s.push_str(" world");
    assert_eq!(s, "hello world");
}

#[test]
#[ignore]
fn variables8() {
    // I AM NOT DONE
    // 不可变变量不能重新赋值。修复：使用遮蔽（let name = "Bob"）
    let name = "Alice";
    let name = "Bob";
    assert_eq!(name, "Bob");
}
