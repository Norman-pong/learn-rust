// exercises/src/error_handling.rs
// Chapter 13: 错误处理 — rustlings fork
// 深做章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#![allow(dead_code, unused_imports)]

use std::io::Write;

#[test]
#[ignore]
fn error_handling1() {
    // I AM NOT DONE
    todo!();
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
    todo!();
}

#[test]
#[ignore]
fn error_handling3() {










    // I AM NOT DONE
    todo!();
}

#[test]
#[ignore]
fn error_handling4() {







    // I AM NOT DONE
    todo!();
}

#[test]
#[ignore]
fn error_handling5() {







    // I AM NOT DONE
    todo!();
}

#[test]
#[ignore]
fn error_handling6() {











    // I AM NOT DONE
    todo!();
}
