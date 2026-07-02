// exercises/src/iterators.rs
// Chapter 18: Iterators — rustlings fork
// 深做章节

// 迭代器是 Rust 中处理集合的强大抽象。本章覆盖 iterator、map、filter、
// collect、find 与 fold。

#[test]
#[ignore]
fn iterators1() {
    // I AM NOT DONE
    let numbers = vec![1, 2, 3, 4];
    let squares: Vec<i32> = todo!("使用 map 和 collect 计算平方");
    assert_eq!(squares, vec![1, 4, 9, 16]);
}

#[test]
#[ignore]
fn iterators2() {
    // I AM NOT DONE
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = todo!("使用 filter 和 collect 保留偶数");
    assert_eq!(evens, vec![2, 4, 6]);
}

#[test]
#[ignore]
fn iterators3() {
    // I AM NOT DONE
    let numbers = vec![4, 8, 12, 15, 16];
    let first_big: Option<i32> = todo!("使用 find 和 copied 找到第一个大于 10 的数");
    assert_eq!(first_big, Some(12));
}

#[test]
#[ignore]
fn iterators4() {
    // I AM NOT DONE
    let words = vec!["hello", "from", "iterator"];
    let total_len: usize = todo!("使用 fold 累计字符串长度");
    assert_eq!(total_len, 17);
}

#[test]
#[ignore]
fn iterators5() {
    // I AM NOT DONE
    // 使用 iterator 消费器把 Vec<Result<i32, &str>> 中的 Ok 值收集。
    let values = vec![Ok(1), Err("skip"), Ok(3), Ok(4)];
    let collected: Result<Vec<i32>, &str> = todo!("使用 collect 收集 Result 中的 Ok 值");
    assert_eq!(collected, Ok(vec![1, 3, 4]));
}
