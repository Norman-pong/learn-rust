// solutions/modules.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/modules.rs`

// ============================================================
// Chapter 10: 模块 — 参考答案
// ============================================================

// Exercise modules1
// 使用 pub 让模块内的函数对外可见
mod my_module {
    pub fn private_function() -> &'static str {
        "private"
    }
}

#[test]
fn modules1() {
    assert_eq!(my_module::private_function(), "private");
}

// Exercise modules2
// 用 use 语句将嵌套模块中的函数导入当前作用域
mod parent_module {
    pub mod child {
        pub fn answer() -> i32 {
            42
        }
    }
}

#[test]
fn modules2() {
    use parent_module::child::answer;
    assert_eq!(answer(), 42);
}

// Exercise modules3
// 同一模块下的兄弟子模块之间通过 super 引用，不需要 crate 前缀
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
fn modules3() {
    assert_eq!(sibling_b::call(), "hello");
}
