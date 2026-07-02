// solutions/lifetimes.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/lifetimes.rs`

// ============================================================
// Chapter 16: Lifetimes — 参考答案
// ============================================================

fn lifetimes1_solution() {
    // 给 longer 函数添加生命周期标注
    fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let s1 = String::from("hello");
    let s2 = String::from("world!");
    assert_eq!(longer(&s1, &s2), "world!");
}

fn lifetimes2_solution() {
    // 结构体持有引用时需要生命周期标注
    struct Excerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: first_sentence };
    assert!(!excerpt.part.is_empty());
}

fn lifetimes3_solution() {
    // 多个生命周期参数：返回值引用最短的那个
    fn select<'a, 'b>(first: &'a str, second: &'b str, pick_first: bool) -> &'a str
    where
        'b: 'a,
    {
        if pick_first { first } else { second }
    }

    let s1 = String::from("first");
    let s2 = String::from("second");
    assert_eq!(select(&s1, &s2, true), "first");
}
