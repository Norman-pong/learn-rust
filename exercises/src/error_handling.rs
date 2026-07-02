// exercises/src/error_handling.rs
// Chapter 13: Error Handling — rustlings fork
// 深做章节

// Rust 使用 Result 与 ? 运算符进行显式错误传播。
// 本章涵盖 map_err、?、自定义错误类型与 Result 链式处理。

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum ParseError {
    Empty,
    TooBig,
    NotNumber,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "input is empty"),
            ParseError::TooBig => write!(f, "number is too big"),
            ParseError::NotNumber => write!(f, "not a number"),
        }
    }
}

impl std::error::Error for ParseError {}

#[test]
#[ignore]
fn error_handling1() {
    // I AM NOT DONE
    // 安全地解析字符串，失败时返回 0。
    fn safe_parse(s: &str) -> u32 {
        todo!("使用 unwrap_or 或 unwrap_or_default")
    }

    assert_eq!(safe_parse("99"), 99);
    assert_eq!(safe_parse("abc"), 0);
}

#[test]
#[ignore]
fn error_handling2() {
    // I AM NOT DONE
    // 使用 map_err 把标准库的 ParseIntError 转换成自定义 ParseError。
    fn parse_small_number(s: &str) -> Result<u32, ParseError> {
        if s.is_empty() {
            return Err(ParseError::Empty);
        }
        let n: u32 = todo!("str::parse 并用 map_err 转换错误类型");
        if n > 100 {
            return Err(ParseError::TooBig);
        }
        Ok(n)
    }

    assert_eq!(parse_small_number("42"), Ok(42));
    assert_eq!(parse_small_number(""), Err(ParseError::Empty));
    assert_eq!(parse_small_number("999"), Err(ParseError::TooBig));
    assert_eq!(parse_small_number("abc"), Err(ParseError::NotNumber));
}

#[test]
#[ignore]
fn error_handling3() {
    // I AM NOT DONE
    // 使用 ? 运算符传播多个可能失败的步骤。
    fn read_config(values: &[&str]) -> Result<(u32, u32), ParseError> {
        let width = todo!("解析 values[0]");
        let height = todo!("解析 values[1]");
        Ok((width, height))
    }

    assert_eq!(read_config(&["10", "20"]), Ok((10, 20)));
    assert_eq!(read_config(&["10", "big"]), Err(ParseError::NotNumber));
}

#[test]
#[ignore]
fn error_handling4() {
    // I AM NOT DONE
    // 使用 ? 与 ok_or/map_err 组合，把 Option 缺失也转换为 ParseError。
    fn parse_optional(s: Option<&str>) -> Result<u32, ParseError> {
        todo!("使用 ? 与 ok_or/map_err 组合")
    }

    assert_eq!(parse_optional(Some("42")), Ok(42));
    assert_eq!(parse_optional(None), Err(ParseError::Empty));
    assert_eq!(parse_optional(Some("101")), Err(ParseError::TooBig));
}

#[test]
#[ignore]
fn error_handling5() {
    // I AM NOT DONE
    // 实现一个自定义 Result 别名，并通过 ? 在函数中使用它。
    type MyResult<T> = Result<T, ParseError>;

    fn double_if_small(s: &str) -> MyResult<u32> {
        let n: u32 = s.parse().map_err(|_| ParseError::NotNumber)?;
        if n > 50 {
            return Err(ParseError::TooBig);
        }
        todo!("返回 n * 2")
    }

    assert_eq!(double_if_small("10"), Ok(20));
    assert_eq!(double_if_small("60"), Err(ParseError::TooBig));
    assert_eq!(double_if_small("x"), Err(ParseError::NotNumber));
}

#[test]
#[ignore]
fn error_handling6() {
    // I AM NOT DONE
    // 使用 ? 运算符的自动转换功能：函数返回 Result<_, Box<dyn std::error::Error>>。
    fn read_and_sum(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let mut sum = 0u32;
        for line in contents.lines() {
            let value: u32 = line.parse()?;
            sum += value;
        }
        todo!("返回正确的 sum")
    }

    // 注意：这个测试需要一个临时文件，请在你的答案中通过 std::io::Write 创建。
    let tmp = std::env::temp_dir().join("learn_rust_error_handling6.txt");
    {
        let mut file = std::fs::File::create(&tmp).unwrap();
        use std::io::Write;
        writeln!(file, "10").unwrap();
        writeln!(file, "20").unwrap();
        writeln!(file, "30").unwrap();
    }

    assert_eq!(read_and_sum(tmp.to_str().unwrap()).unwrap(), 60);
}
