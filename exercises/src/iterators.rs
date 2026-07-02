// exercises/src/iterators.rs
// Chapter 18: 迭代器 — rustlings fork
// 深做章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn iterators1() {
    let squares: Vec<i32> = (1..=4).map(|x| x * x).collect();
    assert_eq!(squares, vec![1, 4, 9, 16]);
}

// Exercise iterators2
// 使用 filter 与 collect 保留 1 到 6 中的偶数。
#[test]
#[ignore]
    // I AM NOT DONE
fn iterators2() {
    let evens: Vec<i32> = (1..=6).filter(|x| *x % 2 == 0).collect();
    assert_eq!(evens, vec![2, 4, 6]);
}

// Exercise iterators3
// 使用 find 与 copied 找到第一个大于 10 的数。
#[test]
#[ignore]
    // I AM NOT DONE
fn iterators3() {
    let nums = vec![3, 7, 12, 9, 15];
    let first_big: Option<i32> = nums.iter().find(|&&x| x > 10).copied();
    assert_eq!(first_big, Some(12));
}

// Exercise iterators4
// 使用 fold 累计字符串长度。
#[test]
#[ignore]
    // I AM NOT DONE
fn iterators4() {
    let words = ["hello", " ", "world", "rust", "!!"];
    let total_len: usize = words.iter().fold(0, |acc, s| acc + s.len());
    assert_eq!(total_len, 17);
}

// Exercise iterators5
// 使用 collect 把 Vec<Result<i32, &str>> 中的 Ok 值收集成 Result<Vec<i32>, &str>。
#[test]
#[ignore]
    // I AM NOT DONE
fn iterators5() {
    let results: Vec<Result<i32, &str>> = vec![Ok(1), Ok(3), Ok(4)];
    let collected: Result<Vec<i32>, &str> = results.into_iter().collect();
    assert_eq!(collected, Ok(vec![1, 3, 4]));
}
