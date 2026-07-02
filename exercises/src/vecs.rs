// exercises/src/vecs.rs
// Chapter ??: vecs.rs — rustlings fork
// 章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn vecs1() {
    let v = vec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
}

// Exercise vecs2
// 用 for 循环遍历可变引用，把每个元素乘以 2
#[test]
#[ignore]
    // I AM NOT DONE
fn vecs2() {
    let mut v = vec![1, 2, 3];
    for i in v.iter_mut() {
        *i *= 2;
    }
    assert_eq!(v, vec![2, 4, 6]);
}
