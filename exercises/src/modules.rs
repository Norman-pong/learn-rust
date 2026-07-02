// exercises/src/modules.rs
// Chapter 10: modules — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

mod my_module {
    // 默认私有，需要让外部可以访问
    pub fn private_function() -> &'static str {
        "private"
    }
}

#[test]
#[ignore]
fn modules1() {
    // I AM NOT DONE

    // 请使 my_module::private_function 在模块外可见
    // 原错误: private_function is private
    assert_eq!(my_module::private_function(), "private");
}

mod parent_module {
    pub mod child {
        pub fn answer() -> i32 {
            42
        }
    }
}

#[test]
#[ignore]
fn modules2() {
    // I AM NOT DONE

    // 使用 use 语句简化路径
    use parent_module::child::answer;
    assert_eq!(answer(), 42);
}

// 同一个模块里，子项可以直接使用兄弟项，不需要前缀；修复调用路径
mod sibling_a {
    pub fn greet() -> &'static str {
        "hello"
    }
}

mod sibling_b {
    pub fn call() -> &'static str {
        // 原错误: cannot find crate::modules::sibling_a; sibling_a 是模块内的局部模块
        super::sibling_a::greet()
    }
}

#[test]
#[ignore]
fn modules3() {
    // I AM NOT DONE

    assert_eq!(sibling_b::call(), "hello");
}
