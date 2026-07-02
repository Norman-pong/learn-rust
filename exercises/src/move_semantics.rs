// exercises/src/move_semantics.rs
// Chapter 06: move_semantics — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn move_semantics1() {
    // I AM NOT DONE

    // Vec 在函数调用时会被移动，请让函数返回它以便继续使用
    fn take_vec(v: Vec<i32>) {
        assert_eq!(v, vec![1, 2, 3]);
    }

    let v = vec![1, 2, 3];
    take_vec(v);
    // 原错误：Vec 被 move 进 take_vec 后再次使用 v
    todo!();
}

#[test]
#[ignore]
fn move_semantics2() {
    // I AM NOT DONE

    // 使用借用代替移动，让函数签名兼容调用
    fn print_vec(v: Vec<i32>) {
        for i in v {
            println!("{}", i);
        }
    }
    let v = vec![1, 2, 3];
    print_vec(v);
    // 原错误：Vec 已被 move 进 print_vec，不能再调用 print_vec(v)
    todo!();
}

#[test]
#[ignore]
fn move_semantics3() {
    // I AM NOT DONE

    // 可变借用允许修改但不转移所有权，请修改签名
    fn push_one(v: Vec<i32>) {
        // 原错误：v 未声明为 mutable，无法 push
        todo!();
    }
    let mut v = vec![1, 2, 3];
    push_one(v);
    // 原错误：Vec 被 move 进 push_one，且函数内部无法可变借用
    todo!();
}

#[test]
#[ignore]
fn move_semantics4() {
    // I AM NOT DONE

    // 使用 .clone() 显式复制一个 String，使两个变量都可用
    let s1 = String::from("hello");
    let s2 = s1;
    // 原错误：s1 已 move 给 s2，下面使用 s1 会编译失败
    todo!();
}

#[test]
#[ignore]
fn move_semantics5() {
    // I AM NOT DONE

    // 实现 Copy 的 i32 默认按值复制，不需要移动；这里修复的是赋值后使用
    let x = 5;
    let y = x;
    assert_eq!(x, 5);
    assert_eq!(y, 5);
}

#[test]
#[ignore]
fn move_semantics6() {
    // I AM NOT DONE

    // 当你不打算再使用变量时，可以把它移动进函数
    fn consume(s: String) -> String {
        s
    }

    let s = String::from("Rust");
    let s = consume(s);
    assert_eq!(s, "Rust");
}
