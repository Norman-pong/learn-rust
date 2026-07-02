// solutions/if_.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/if_.rs`

// ============================================================
// Chapter 03: if — 参考答案
// ============================================================

#[test]
fn if1() {
    // 让 if 条件返回 true，使 assert 通过
    let b = true;
    let result = if b { 1 } else { 0 };
    assert_eq!(result, 1);
}

#[test]
fn if2() {
    // if 分支的类型必须一致，请统一两个分支的类型
    let condition = true;
    let number = if condition { 5 } else { 5 };
    assert_eq!(number, 5);
}

#[test]
fn if3() {
    // 使用 else if 补全条件，返回正确的字符串
    fn number_type(n: i32) -> &'static str {
        if n > 0 {
            "positive"
        } else if n == 0 {
            "zero"
        } else {
            "negative"
        }
    }

    assert_eq!(number_type(0), "zero");
}
