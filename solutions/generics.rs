// solutions/generics.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/generics.rs`

// ============================================================
// Chapter 14: Generics — 参考答案
// ============================================================

fn generics1_solution() {
    struct Wrapper<T> {
        value: T,
    }

    impl<T> Wrapper<T> {
        fn new(value: T) -> Self {
            Wrapper { value }
        }

        fn value(&self) -> &T {
            &self.value
        }
    }

    let w = Wrapper::new("hello");
    assert_eq!(w.value(), &"hello");

    let w = Wrapper::new(42);
    assert_eq!(w.value(), &42);
}

fn generics2_solution() {
    use std::cmp::PartialOrd;

    fn larger<T: PartialOrd>(a: T, b: T) -> T {
        if a > b { a } else { b }
    }

    assert_eq!(larger(3, 5), 5);
    assert_eq!(larger(3.14_f64, 2.71_f64), 3.14_f64);
    assert_eq!(larger('a', 'z'), 'z');
}
