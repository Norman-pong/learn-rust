// exercises/src/if_.rs
// Chapter 03: if — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn if1() {
    // I AM NOT DONE

    // 让 if 条件返回 true，使 assert 通过
    let b = false;
    let result = if b { 1 } else { 0 };
    assert_eq!(result, 1);
}

#[test]
#[ignore]
fn if2() {
    // I AM NOT DONE

    // if 分支的类型必须一致，请统一两个分支的类型
    let _condition = true; // 原代码 let condition = true; 加 _ 前缀避免 unused var 警告
    let number: i32 = todo!("原代码 let number = if condition { 5 } else { \"five\" }; if/else 分支类型不一致导致编译错误");
    assert_eq!(number, 5);
}

#[test]
#[ignore]
fn if3() {
    // I AM NOT DONE

    // 使用 else if 补全条件，返回正确的字符串
    fn number_type(n: i32) -> &'static str {
        if n > 0 {
            "positive"
        } else {
            "negative"
        }
    }

    assert_eq!(number_type(0), "zero");
}
