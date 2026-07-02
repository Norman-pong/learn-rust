// exercises/src/if_.rs
// Chapter ??: if_.rs — rustlings fork
// 章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn if1() {
    // 让 if 条件返回 true，使 assert 通过
    let b = true;
    let result = if b { 1 } else { 0 };
    assert_eq!(result, 1);
}

#[test]
#[ignore]
    // I AM NOT DONE
fn if2() {
    // if 分支的类型必须一致，请统一两个分支的类型
    let condition = true;
    let number = if condition { 5 } else { 5 };
    assert_eq!(number, 5);
}

#[test]
#[ignore]
    // I AM NOT DONE
fn if3() {
    // 使用 else if 补全条件，返回正确的字符串
    // I AM NOT DONE
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
