// exercises/src/vecs.rs
// Chapter 05: vecs — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn vecs1() {
    // I AM NOT DONE

    // 使用 vec! 宏创建包含 1, 2, 3 的 Vec<i32>
    let v = vec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
#[ignore]
fn vecs2() {
    // I AM NOT DONE

    // 用 for 循环把每个元素乘以 2，注意 mut
    let mut v = vec![1, 2, 3];
    for i in v.iter_mut() {
        *i *= 2;
    }
    assert_eq!(v, vec![2, 4, 6]);
}
