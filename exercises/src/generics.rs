// exercises/src/generics.rs
// Chapter 14: Generics — rustlings fork
// 深做章节

#[test]
#[ignore]
fn generics1() {
    // I AM NOT DONE
    // 实现一个泛型 Wrapper<T>，可以包装任意类型并解包
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

#[test]
#[ignore]
fn generics2() {
    // I AM NOT DONE
    // 实现一个泛型函数，要求 T 实现 PartialOrd，返回两个值中较大的一个
    fn larger<T: PartialOrd>(a: T, b: T) -> T {
        if a > b { a } else { b }
    }

    assert_eq!(larger(3, 5), 5);
    assert_eq!(larger(3.14_f64, 2.71_f64), 3.14_f64);
    assert_eq!(larger('a', 'z'), 'z');
}
