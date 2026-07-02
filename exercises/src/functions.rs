// exercises/src/functions.rs
// Chapter 02: 函数 — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn functions1() {
    // I AM NOT DONE
    fn call_me() {
        println!("Hello functions!");
    }
    call_me();
}

#[test]
#[ignore]
fn functions2() {
    // I AM NOT DONE
    fn sale_price(price: i32) -> f64 {
        price as f64 * 0.8
    }
    assert_eq!(sale_price(100), 80.0);
}

#[test]
#[ignore]
fn functions3() {
    // I AM NOT DONE
    fn fizz(x: i32, y: i32) -> i32 {
        x * y
    }
    assert_eq!(fizz(7, 6), 42);
}

#[test]
#[ignore]
fn functions4() {
    // I AM NOT DONE
    fn square(num: i32) -> i32 {
        num * num
    }
    assert_eq!(square(4), 16);
}

#[test]
#[ignore]
fn functions5() {
    // I AM NOT DONE
    fn count_to_ten(n: i32) -> i32 {
        if n >= 10 {
            return 10;
        }
        n
    }
    assert_eq!(count_to_ten(10), 10);
    assert_eq!(count_to_ten(5), 5);
}
