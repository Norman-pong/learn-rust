// exercises/src/modules.rs
// Chapter 10: 模块 — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn modules1() {
    // I AM NOT DONE
    mod my_module {
        pub fn private_function() -> &'static str {
            "private"
        }
    }
    assert_eq!(my_module::private_function(), "private");
}

#[test]
#[ignore]
fn modules2() {
    // I AM NOT DONE
    mod parent_module {
        pub mod child {
            pub fn answer() -> i32 {
                42
            }
        }
    }
    use parent_module::child::answer;
    assert_eq!(answer(), 42);
}

mod sibling_a {
    pub fn greet() -> &'static str {
        "hello"
    }
}

mod sibling_b {
    pub fn call() -> &'static str {
        super::sibling_a::greet()
    }
}

#[test]
#[ignore]
fn modules3() {
    // I AM NOT DONE
    assert_eq!(sibling_b::call(), "hello");
}
