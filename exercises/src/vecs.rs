// exercises/src/vecs.rs
// Chapter 05: vecs — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn vecs1() {
    // I AM NOT DONE

    // 使用 vec! 宏创建包含 1, 2, 3 的 Vec<i32>
    let v: Vec<i32> = Vec::new();
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
#[ignore]
fn vecs2() {
    // I AM NOT DONE

    // 用 for 循环把每个元素乘以 2，注意 mut
    let v = vec![1, 2, 3];
    // 原错误：for i in v 会移动 Vec，之后无法再使用 v
    //  learners should iterate by reference or index to mutate elements
    todo!();
}
