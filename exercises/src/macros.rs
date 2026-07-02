// exercises/src/macros.rs
// Chapter 21: Macros — rustlings fork
// 深做章节

// 宏分为声明宏 macro_rules! 和过程宏。本章仅涉及声明宏，要求你实现
// 简单的 vec!、format! 风格宏。

#[test]
#[ignore]
fn macros1() {
    // I AM NOT DONE
    // 调用 vec! 宏创建包含 1, 2, 3 的向量。
    let v: Vec<i32> = todo!("使用 vec! 宏");
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
#[ignore]
fn macros2() {
    // I AM NOT DONE
    // 使用 format! 宏构造一个字符串。
    let name = "Rust";
    let age = 18usize;

    let info: String = todo!("使用 format!");
    assert_eq!(info, "Rust is 18 years old");
}

#[test]
#[ignore]
fn macros3() {
    // I AM NOT DONE
    // 定义并使用一个简单的 macro_rules! 宏，计算参数个数。
    todo!("确认 count_args! 宏能正确计算参数个数");

    macro_rules! count_args {
        () => { 0 };
        ($($x:expr),+ $(,)?) => { {
            let mut count = 0;
            $(
                let _ = $x;
                count += 1;
            )+
            count
        } };
    }

    assert_eq!(count_args!(), 0);
    assert_eq!(count_args!(1, 2, 3), 3);
    assert_eq!(count_args!("a", "b", "c", "d"), 4);
}

#[test]
#[ignore]
fn macros4() {
    // I AM NOT DONE
    // 定义一个 macro_rules! 宏，实现类似 vec![x; n] 的重复模式。
    macro_rules! repeat {
        ($value:expr; $count:expr) => {
            todo!("生成 vec![$value; $count]")
        };
    }

    let v: Vec<i32> = repeat!(7; 5);
    assert_eq!(v, vec![7, 7, 7, 7, 7]);
}
