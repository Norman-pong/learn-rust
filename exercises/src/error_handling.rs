// exercises/src/error_handling.rs
// Chapter 13: 错误处理 — rustlings fork
// 深做章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

use std::io::Write;

#[test]
#[ignore]
fn error_handling1() {
    // I AM NOT DONE
    fn safe_parse(s: &str) -> u32 {
        s.parse().unwrap_or(0)
    }
    assert_eq!(safe_parse("99"), 99);
    assert_eq!(safe_parse("abc"), 0);
}

#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidNumber,
    TooBig,
    Missing,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidNumber => write!(f, "invalid number"),
            ParseError::TooBig => write!(f, "number too big"),
            ParseError::Missing => write!(f, "missing value"),
        }
    }
}

impl std::error::Error for ParseError {}

#[test]
#[ignore]
fn error_handling2() {
    // I AM NOT DONE
    fn parse_small_number(s: &str) -> Result<u32, ParseError> {
        let n: u32 = s.parse().map_err(|_| ParseError::InvalidNumber)?;
        if n > 100 {
            return Err(ParseError::TooBig);
        }
        Ok(n)
    }

    assert_eq!(parse_small_number("99"), Ok(99));
    assert_eq!(parse_small_number("abc"), Err(ParseError::InvalidNumber));
    assert_eq!(parse_small_number("200"), Err(ParseError::TooBig));
}

#[test]
#[ignore]
fn error_handling3() {
    // I AM NOT DONE
    fn read_config(values: &[&str]) -> Result<(u32, u32), ParseError> {
        let width = values[0].parse().map_err(|_| ParseError::InvalidNumber)?;
        let height = values[1].parse().map_err(|_| ParseError::InvalidNumber)?;
        Ok((width, height))
    }

    assert_eq!(read_config(&["10", "20"]), Ok((10, 20)));
    assert_eq!(read_config(&["10", "abc"]), Err(ParseError::InvalidNumber));
}

#[test]
#[ignore]
fn error_handling4() {
    // I AM NOT DONE
    fn parse_optional(s: Option<&str>) -> Result<u32, ParseError> {
        let s = s.ok_or(ParseError::Missing)?;
        s.parse().map_err(|_| ParseError::InvalidNumber)
    }

    assert_eq!(parse_optional(Some("42")), Ok(42));
    assert_eq!(parse_optional(None), Err(ParseError::Missing));
    assert_eq!(parse_optional(Some("abc")), Err(ParseError::InvalidNumber));
}

#[test]
#[ignore]
fn error_handling5() {
    // I AM NOT DONE
    type MyResult<T> = Result<T, ParseError>;

    fn double_if_small(s: &str) -> MyResult<u32> {
        let n: u32 = s.parse().map_err(|_| ParseError::InvalidNumber)?;
        if n > 50 {
            return Err(ParseError::TooBig);
        }
        Ok(n * 2)
    }

    assert_eq!(double_if_small("20"), Ok(40));
    assert_eq!(double_if_small("60"), Err(ParseError::TooBig));
}

#[test]
#[ignore]
fn error_handling6() {
    // I AM NOT DONE
    fn read_and_sum(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let mut sum = 0;
        for line in content.lines() {
            let value: u32 = line.trim().parse()?;
            sum += value;
        }
        Ok(sum)
    }

    use std::io::Write;
    let mut file = std::fs::File::create("/tmp/test_sum.txt").unwrap();
    writeln!(file, "10").unwrap();
    writeln!(file, "20").unwrap();
    writeln!(file, "30").unwrap();
    drop(file);

    assert_eq!(read_and_sum("/tmp/test_sum.txt").unwrap(), 60);
    std::fs::remove_file("/tmp/test_sum.txt").ok();
}
