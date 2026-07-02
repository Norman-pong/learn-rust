// solutions/move_semantics.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/move_semantics.rs`

// ============================================================
// Chapter 06: Move Semantics — 参考答案
// ============================================================

// Exercise move_semantics1
// take_vec 需要返回 Vec，才能让调用者继续用原变量
#[test]
fn move_semantics1() {
    fn take_vec(v: Vec<i32>) -> Vec<i32> {
        assert_eq!(v, vec![1, 2, 3]);
        v
    }

    let v = vec![1, 2, 3];
    let v = take_vec(v);
    assert_eq!(v.len(), 3);
}

// Exercise move_semantics2
// print_vec 接收不可变借用 &Vec<i32>，多次调用不会转移所有权
#[test]
fn move_semantics2() {
    fn print_vec(v: &Vec<i32>) {
        for i in v {
            println!("{}", i);
        }
    }

    let v = vec![1, 2, 3];
    print_vec(&v);
    print_vec(&v);
}

// Exercise move_semantics3
// push_one 使用 &mut Vec<i32> 可变借用，可修改且不转移所有权
#[test]
fn move_semantics3() {
    fn push_one(v: &mut Vec<i32>) {
        v.push(4);
    }

    let mut v = vec![1, 2, 3];
    push_one(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4]);
}

// Exercise move_semantics4
// 用 .clone() 显式复制 String，让 s1 和 s2 同时可用
#[test]
fn move_semantics4() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    assert_eq!(s1, "hello");
    assert_eq!(s2, "hello");
}

// Exercise move_semantics5
// i32 实现 Copy，按值复制即可，不需要额外处理
#[test]
fn move_semantics5() {
    let x = 5;
    let y = x;
    assert_eq!(x, 5);
    assert_eq!(y, 5);
}

// Exercise move_semantics6
// consume 接收 String 所有权后返回，返回的新 String 重新绑定给 s
#[test]
fn move_semantics6() {
    fn consume(s: String) -> String {
        s
    }

    let s = String::from("Rust");
    let s = consume(s);
    assert_eq!(s, "Rust");
}
