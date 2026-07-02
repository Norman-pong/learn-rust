// solutions/conversions.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/conversions.rs`

// ============================================================
// Chapter 23: 类型转换 — 参考答案
// ============================================================

// Exercise conversions1
// 使用 AsRef<str> 让函数同时接受 &String 和 &str。
#[test]
fn conversions1() {
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
fn conversions2() {
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
fn conversions3() {
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
