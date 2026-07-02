// solutions/functions.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/functions.rs`

// ============================================================
// Chapter 02: functions — 参考答案
// ============================================================

// Exercise functions1
// 调用 call_me() 让函数执行并打印。
fn call_me() {
    println!("Hello functions!");
}

#[test]
fn functions1() {
    call_me();
}

// Exercise functions2
// 补全参数类型与返回类型：i32 乘以 0.8 返回 f64。
fn sale_price(price: i32) -> f64 {
    price as f64 * 0.8
}

#[test]
fn functions2() {
    assert_eq!(sale_price(100), 80.0);
}

// Exercise functions3
// 调用时传入两个参数 7 和 6。
fn fizz(x: i32, y: i32) -> i32 {
    x * y
}

#[test]
fn functions3() {
    assert_eq!(fizz(7, 6), 42);
}

// Exercise functions4
// Rust 函数是表达式，返回最后一条表达式的值；去掉末尾分号。
fn square(num: i32) -> i32 {
    num * num
}

#[test]
fn functions4() {
    assert_eq!(square(4), 16);
}

// Exercise functions5
// return 需要显式给出返回值。
fn count_to_ten(n: i32) -> i32 {
    if n >= 10 {
        return 10;
    }
    n
}

#[test]
fn functions5() {
    assert_eq!(count_to_ten(10), 10);
    assert_eq!(count_to_ten(5), 5);
}
