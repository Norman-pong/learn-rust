// exercises/src/functions.rs
// Chapter 02: functions — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn functions1() {
    // I AM NOT DONE

    // 调用下面定义的函数，让它打印 "Hello functions!"
    fn call_me() {
        println!("Hello functions!");
    }

    // 在这里调用
    call_me();
}

#[test]
#[ignore]
fn functions2() {
    // I AM NOT DONE
    // 修复函数签名：price 缺少类型标注，返回值也需标注
    // 原始代码: fn sale_price(price) { price * 0.8 }
    fn sale_price(price: i32) -> f64 {
        price as f64 * 0.8
    }

    assert_eq!(sale_price(100), 80.0);
}

#[test]
#[ignore]
fn functions3() {
    // I AM NOT DONE
    // fizz 需要 2 个参数但调用时只给了 1 个
    fn fizz(x: i32, y: i32) -> i32 {
        x * y
    }

    assert_eq!(fizz(7, 6), 42);
}

#[test]
#[ignore]
fn functions4() {
    // I AM NOT DONE
    // square 最后一行有分号，导致返回 () 而不是 i32
    // 修复：去掉最后一行末尾的分号
    fn square(num: i32) -> i32 {
        num * num
    }

    assert_eq!(square(4), 16);
}

#[test]
#[ignore]
fn functions5() {
    // I AM NOT DONE
    // return 语句缺少返回值
    fn count_to_ten(n: i32) -> i32 {
        if n >= 10 {
            return 10;
        }
        n
    }

    assert_eq!(count_to_ten(10), 10);
    assert_eq!(count_to_ten(5), 5);
}
