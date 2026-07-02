// solutions/vecs.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/vecs.rs`

// ============================================================
// Chapter 05: Vec — 参考答案
// ============================================================

// Exercise vecs1
// 使用 vec! 宏创建包含 1, 2, 3 的 Vec<i32>
#[test]
fn vecs1() {
    let v = vec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
}

// Exercise vecs2
// 用 for 循环遍历可变引用，把每个元素乘以 2
#[test]
fn vecs2() {
    let mut v = vec![1, 2, 3];
    for i in v.iter_mut() {
        *i *= 2;
    }
    assert_eq!(v, vec![2, 4, 6]);
}
