// exercises/src/options.rs
// Chapter 12: Option & Result — rustlings fork
// 深做章节

// Option 与 Result 是 Rust 表达“可能缺失”和“可能失败”的核心类型。
// 本章练习要求你熟练使用 map、unwrap_or、? 与 map_err。

#[test]
#[ignore]
fn options1() {
    // I AM NOT DONE
    // 将 Option<u32> 通过 map 转换为 Option<String>。
    let maybe_number: Option<u32> = Some(42);

    let maybe_string: Option<String> = todo!("把 u32 转换为字符串");

    assert_eq!(maybe_string, Some("42".to_string()));

    let nothing: Option<u32> = None;
    let nothing_string: Option<String> = nothing.map(|n| n.to_string());
    assert_eq!(nothing_string, None);
}

#[test]
#[ignore]
fn options2() {
    // I AM NOT DONE
    // 从字符串解析 u32，失败时返回默认值 0（不使用 unwrap）。
    fn parse_or_zero(s: &str) -> u32 {
        todo!("使用 unwrap_or 或 unwrap_or_default")
    }

    assert_eq!(parse_or_zero("123"), 123);
    assert_eq!(parse_or_zero("not a number"), 0);
}

#[test]
#[ignore]
fn options3() {
    // I AM NOT DONE
    // 使用 Option 的组合方法，仅保留能被 3 整除的数，并将其转换为描述字符串。
    let numbers: Vec<Option<u32>> =
        vec![Some(1), Some(3), Some(5), Some(9), Some(12), None];

    let descriptions: Vec<String> =
        todo!("filter_map + map 的组合链：仅保留能被 3 整除的数并格式化");

    assert_eq!(
        descriptions,
        vec![
            "divisible by 3: 3",
            "divisible by 3: 9",
            "divisible by 3: 12"
        ]
    );
}
