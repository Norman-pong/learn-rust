// exercises/src/conversions.rs
// Chapter 23: 类型转换 — rustlings fork
// 深做章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn conversions1() {
    // I AM NOT DONE
    fn print_length<S: AsRef<str>>(s: S) {
        println!("{}", s.as_ref().len());
    }

    let owned = String::from("Rust");
    let borrowed: &str = "Rust";

    print_length(&owned);
    print_length(borrowed);

    // String 实现 AsRef<str>，as_ref() 返回 "Rust"。
    // 显式使用 UFCS 指定 AsRef<str>，避免多 impl 歧义。
    assert_eq!(<String as AsRef<str>>::as_ref(&owned), "Rust");
}

// Exercise conversions2
// 为自定义类型实现 From<&str> 和 From<String>。
#[test]
#[ignore]
    // I AM NOT DONE
fn conversions2() {
    struct Person {
        name: String,
    }

    impl From<&str> for Person {
    // I AM NOT DONE
        fn from(name: &str) -> Self {
            Person {
                name: name.to_string(),
            }
        }
    }

    impl From<String> for Person {
    // I AM NOT DONE
        fn from(name: String) -> Self {
            // String 直接作为 name，避免再次复制。
            Person { name }
        }
    }

    let p1: Person = "Alice".into();
    let p2: Person = String::from("Bob").into();

    assert_eq!(p1.name, "Alice");
    assert_eq!(p2.name, "Bob");
}

// Exercise conversions3
// 实现 TryFrom<i64>，拒绝负数并返回自定义错误。
#[test]
#[ignore]
    // I AM NOT DONE
fn conversions3() {
    #[derive(Debug, PartialEq)]
    struct PositiveU32(u32);

    #[derive(Debug, PartialEq)]
    enum PositiveError {
        Negative,
        TooLarge,
    }

    impl std::fmt::Display for PositiveError {
    // I AM NOT DONE
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
    // I AM NOT DONE

        fn try_from(value: i64) -> Result<Self, Self::Error> {
            if value < 0 {
                Err(PositiveError::Negative)
            } else if value > u32::MAX as i64 {
                Err(PositiveError::TooLarge)
            } else {
                Ok(PositiveU32(value as u32))
            }
        }
    }

    assert_eq!(PositiveU32::try_from(42), Ok(PositiveU32(42)));
    assert_eq!(PositiveU32::try_from(-1), Err(PositiveError::Negative));
    assert_eq!(PositiveU32::try_from(5_000_000_000_i64), Err(PositiveError::TooLarge));
}
