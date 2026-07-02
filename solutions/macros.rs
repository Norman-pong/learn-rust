// solutions/macros.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/macros.rs`

// ============================================================
// Chapter 21: 宏 — 参考答案
// ============================================================

// Exercise macros1
// 使用 vec! 宏创建包含 1, 2, 3 的向量。
#[test]
fn macros1() {
    let v = vec![1, 2, 3];

    assert_eq!(v, vec![1, 2, 3]);
}

// Exercise macros2
// 使用 format! 宏构造一个字符串。
#[test]
fn macros2() {
    let name = "Rust";
    let age = 18usize;

    let info = format!("{name} is {age} years old");

    assert_eq!(info, "Rust is 18 years old");
}

// Exercise macros3
// 定义并使用一个简单的 macro_rules! 宏，计算参数个数。
#[test]
fn macros3() {
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

// Exercise macros4
// 定义一个 macro_rules! 宏，实现类似 vec![x; n] 的重复模式。
#[test]
fn macros4() {
    macro_rules! repeat {
        ($value:expr; $count:expr) => {
            {
                let mut v = Vec::new();
                for _ in 0..$count {
                    v.push($value);
                }
                v
            }
        };
    }

    let v = repeat!(7; 5);
    assert_eq!(v, vec![7, 7, 7, 7, 7]);
}
