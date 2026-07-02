// solutions/error_handling.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/error_handling.rs`

// ============================================================
// Chapter 13: 错误处理 — 参考答案
// ============================================================

use std::fs::File;
use std::io::Write;

// Exercise error_handling1
// 使用 unwrap_or 在解析失败时返回默认值 0，避免 panic。
fn safe_parse(s: &str) -> u32 {
    s.parse::<u32>().unwrap_or(0)
}

#[test]
fn error_handling1() {
    assert_eq!(safe_parse("99"), 99);
    assert_eq!(safe_parse("abc"), 0);
}

// Exercise error_handling2
// 使用 map_err 把标准库的 ParseIntError 转换成自定义 ParseError。
#[derive(Debug, PartialEq)]
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

fn parse_small_number(s: &str) -> Result<u32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    let n: u32 = s.parse().map_err(|_| ParseError::NotNumber)?;
    if n > 100 {
        return Err(ParseError::TooBig);
    }
    Ok(n)
}

#[test]
fn error_handling2() {
    assert_eq!(parse_small_number("42"), Ok(42));
    assert_eq!(parse_small_number(""), Err(ParseError::Empty));
    assert_eq!(parse_small_number("999"), Err(ParseError::TooBig));
    assert_eq!(parse_small_number("abc"), Err(ParseError::NotNumber));
}

// Exercise error_handling3
// 使用 ? 运算符传播多个可能失败的步骤。
fn read_config(values: &[&str]) -> Result<(u32, u32), ParseError> {
    let width: u32 = values[0].parse().map_err(|_| ParseError::NotNumber)?;
    let height: u32 = values[1].parse().map_err(|_| ParseError::NotNumber)?;
    Ok((width, height))
}

#[test]
fn error_handling3() {
    assert_eq!(read_config(&["10", "20"]), Ok((10, 20)));
    assert_eq!(read_config(&["10", "big"]), Err(ParseError::NotNumber));
}

// Exercise error_handling4
// 使用 ? 与 map_err 组合，把 Option 缺失也转换为 ParseError。
fn parse_optional(s: Option<&str>) -> Result<u32, ParseError> {
    let text = s.ok_or(ParseError::Empty)?;
    let n: u32 = text.parse().map_err(|_| ParseError::NotNumber)?;
    if n > 100 {
        Err(ParseError::TooBig)
    } else {
        Ok(n)
    }
}

#[test]
fn error_handling4() {
    assert_eq!(parse_optional(Some("42")), Ok(42));
    assert_eq!(parse_optional(None), Err(ParseError::Empty));
    assert_eq!(parse_optional(Some("101")), Err(ParseError::TooBig));
}

// Exercise error_handling5
// 实现一个自定义 Result 别名，并通过 ? 在函数中使用它。
type MyResult<T> = Result<T, ParseError>;

fn double_if_small(s: &str) -> MyResult<u32> {
    let n: u32 = s.parse().map_err(|_| ParseError::NotNumber)?;
    if n > 50 {
        return Err(ParseError::TooBig);
    }
    Ok(n * 2)
}

#[test]
fn error_handling5() {
    assert_eq!(double_if_small("10"), Ok(20));
    assert_eq!(double_if_small("60"), Err(ParseError::TooBig));
    assert_eq!(double_if_small("x"), Err(ParseError::NotNumber));
}

// Exercise error_handling6
// 使用 ? 运算符的自动转换功能：函数返回 Result<_, Box<dyn std::error::Error>>。
fn read_and_sum(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let mut sum = 0u32;
    for line in contents.lines() {
        let value: u32 = line.parse()?;
        sum += value;
    }
    Ok(sum)
}

#[test]
fn error_handling6() {
    // 创建一个临时文件供 read_and_sum 读取。
    let tmp = std::env::temp_dir().join("learn_rust_error_handling6.txt");
    {
        let mut file = File::create(&tmp).unwrap();
        writeln!(file, "10").unwrap();
        writeln!(file, "20").unwrap();
        writeln!(file, "30").unwrap();
    }

    assert_eq!(read_and_sum(tmp.to_str().unwrap()).unwrap(), 60);
}
