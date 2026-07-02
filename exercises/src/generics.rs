// exercises/src/generics.rs
// Chapter 14: Generics — rustlings fork
// 深做章节

// 泛型让你编写一份适用于多种类型的代码。本章要求你为结构体、函数添加
// 泛型参数，并理解 trait bound 对泛型函数的限制。

#[test]
#[ignore]
fn generics1() {
    // I AM NOT DONE
    // 补全 impl 块：为 Wrapper<T> 添加 value() 方法返回 &self.value
    struct Wrapper<T> {
        value: T,
    }

    impl<T> Wrapper<T> {
        fn new(value: T) -> Self {
            Wrapper { value }
        }

        fn value(&self) -> &T {
            todo!("返回 &self.value")
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
    // 为 larger 函数添加 trait bound，让它能比较并返回两个值中较大的一个。
    fn larger<T>(a: T, b: T) -> T {
        todo!("添加 trait bound 并返回较大的值")
    }

    assert_eq!(larger(3, 5), 5);
    assert_eq!(larger(3.14_f64, 2.71_f64), 3.14_f64);
    assert_eq!(larger('a', 'z'), 'z');
}
