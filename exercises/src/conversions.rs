// exercises/src/conversions.rs
// Chapter 23: Type Conversions — rustlings fork
// 深做章节

// 类型转换包括 AsRef、From、Into、TryFrom。本章要求你实现这些 trait
// 以完成自定义类型之间的安全转换。

#[test]
#[ignore]
fn conversions1() {
    // I AM NOT DONE
    // 使用 AsRef<str> 让函数接受 &String 和 &str。
    fn print_length<S: AsRef<str>>(s: S) {
        println!("{}", s.as_ref().len());
    }

    let owned = String::from("Rust");
    let borrowed: &str = "Rust";

    print_length(&owned);
    print_length(borrowed);

    // I AM NOT DONE: 原代码 owned.as_ref() 因 AsRef 目标类型不明确而编译失败
    assert_eq!(AsRef::<str>::as_ref(&owned), "Rust");
}

#[test]
#[ignore]
fn conversions2() {
    // I AM NOT DONE
    // 为自定义类型实现 From，支持 From<&str> 和 From<String>。
    struct Person {
        name: String,
    }

    impl From<&str> for Person {
        fn from(name: &str) -> Self {
            Person {
                name: name.to_string(),
            }
        }
    }

    impl From<String> for Person {
        fn from(name: String) -> Self {
            todo!("实现 String -> Person")
        }
    }

    let p1: Person = "Alice".into();
    let p2: Person = String::from("Bob").into();

    assert_eq!(p1.name, "Alice");
    assert_eq!(p2.name, "Bob");
}

#[test]
#[ignore]
fn conversions3() {
    // I AM NOT DONE
    // 实现 TryFrom<i64>，拒绝负数并返回一个自定义错误。
    #[derive(Debug, PartialEq)]
    struct PositiveU32(u32);

    #[derive(Debug, PartialEq)]
    enum PositiveError {
        Negative,
        TooLarge,
    }

    impl std::fmt::Display for PositiveError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PositiveError::Negative => write!(f, "value is negative"),
                PositiveError::TooLarge => write!(f, "value does not fit in u32"),
            }
        }
    }

    impl std::error::Error for PositiveError {}

    impl std::convert::TryFrom<i64> for PositiveU32 {
        type Error = PositiveError;

        fn try_from(value: i64) -> Result<Self, Self::Error> {
            todo!("实现转换：负数返回 Negative，超过 u32 范围返回 TooLarge")
        }
    }

    assert_eq!(PositiveU32::try_from(42), Ok(PositiveU32(42)));
    assert_eq!(PositiveU32::try_from(-1), Err(PositiveError::Negative));
    assert_eq!(PositiveU32::try_from(5_000_000_000_i64), Err(PositiveError::TooLarge));
}
