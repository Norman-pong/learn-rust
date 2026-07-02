// exercises/src/variables.rs
// Chapter 01: variables — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn variables1() {
    // I AM NOT DONE
    // x 默认不可变，不能重新赋值。修复：在 let 后加 mut
    let _x = 5; // 原代码 let x = 5; 加 _ 前缀避免 unused var 警告
    // assert_eq!(x, 7);
}

#[test]
#[ignore]
fn variables2() {
    // I AM NOT DONE
    // 字符串不能直接加数字，需要用 parse 转换
    let _x = "42"; // 原代码 let x = "42"; 加 _ 前缀避免 unused var 警告
    // assert_eq!(x + 1, 43);
}

#[test]
#[ignore]
fn variables3() {
    // I AM NOT DONE
    // const 必须显式标注类型，且 const THREE: i32 = todo!() 会因 todo!() 默认返回 () 导致类型不匹配
    let three: i32 = todo!("赋值为 3");
    assert_eq!(three, 3);
}

#[test]
#[ignore]
fn variables4() {
    // I AM NOT DONE
    // 遮蔽（shadowing）导致 x 变成了字符串，但 assert 期望 i32
    let _x = 10; // 原代码 let x = 10; 加 _ 前缀避免 unused var 警告
    // 提示：修改变量名，不要遮蔽
    // assert_eq!(x, 10);
}

#[test]
#[ignore]
fn variables5() {
    // I AM NOT DONE
    // number 不可变，不能用 +=。修复：加 mut
    let _number = 1; // 原代码 let number = 1; 加 _ 前缀避免 unused var 警告
    // assert_eq!(number, 2);
}

#[test]
#[ignore]
fn variables6() {
    // I AM NOT DONE
    // 整数除法得到 1（截断），不是 1.5。修复：用浮点类型
    let _a = 3; // 原代码 let a = 3; 加 _ 前缀避免 unused var 警告
    let _b = 2; // 原代码 let b = 2; 加 _ 前缀避免 unused var 警告
    // assert_eq!(result, 1.5);
}

#[test]
#[ignore]
fn variables7() {
    // I AM NOT DONE
    // &str 没有 push_str 方法，需要用 String
    let _s = "hello"; // 原代码 let s = "hello"; 加 _ 前缀避免 unused var 警告
    // assert_eq!(s, "hello world");
}

#[test]
#[ignore]
fn variables8() {
    // I AM NOT DONE
    // 不可变变量不能重新赋值。修复：使用遮蔽（let name = "Bob"）
    let _name = "Alice"; // 原代码 let name = "Alice"; 加 _ 前缀避免 unused var 警告
    // assert_eq!(name, "Bob");
}
