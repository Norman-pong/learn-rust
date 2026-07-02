// solutions/options.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/options.rs`

// ============================================================
// Chapter 12: Option & Result — 参考答案
// ============================================================

// Exercise options1
// 将 Option<u32> 通过 map 转换为 Option<String>。
#[test]
fn options1() {
    let maybe_number: Option<u32> = Some(42);

    let maybe_string: Option<String> = maybe_number.map(|n| n.to_string());

    assert_eq!(maybe_string, Some("42".to_string()));

    let nothing: Option<u32> = None;
    let nothing_string: Option<String> = nothing.map(|n| n.to_string());
    assert_eq!(nothing_string, None);
}

// Exercise options2
// 从字符串解析 u32，失败时返回默认值 0（不使用 unwrap）。
#[test]
fn options2() {
    let input = "123";
    let bad_input = "not a number";

    fn parse_or_zero(s: &str) -> u32 {
        s.parse::<u32>().unwrap_or_default()
    }

    assert_eq!(parse_or_zero(input), 123);
    assert_eq!(parse_or_zero(bad_input), 0);
}

// Exercise options3
// 使用 Option 的组合方法，仅保留能被 3 整除的数，并将其转换为描述字符串。
#[test]
fn options3() {
    let numbers: Vec<Option<u32>> = vec![Some(1), Some(3), Some(5), Some(9), Some(12), None];

    let descriptions: Vec<String> = numbers
        .iter()
        .filter_map(|opt| opt.filter(|n| n % 3 == 0))
        .map(|n| format!("divisible by 3: {}", n))
        .collect();

    assert_eq!(descriptions, vec!["divisible by 3: 3", "divisible by 3: 9", "divisible by 3: 12"]);
}
