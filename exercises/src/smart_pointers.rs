// exercises/src/smart_pointers.rs
// Chapter 19: 智能指针 — rustlings fork
// 深做章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn smart_pointers1() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let mut sum = 0;
    let mut current = &list;
    while let List::Cons(value, next) = current {
        sum += value;
        current = next;
    }

    assert_eq!(sum, 6);
}

// Exercise smart_pointers2
// 使用 Rc 共享数据，并验证其强引用计数。
#[test]
#[ignore]
    // I AM NOT DONE
fn smart_pointers2() {
    use std::rc::Rc;

    let data = Rc::new(String::from("shared"));
    let a = Rc::clone(&data);
    let _b = Rc::clone(&data);
    let _c = Rc::clone(&data);

    // data + a + b + c 共 4 个强引用
    assert_eq!(Rc::strong_count(&data), 4);

    drop(a);
    assert_eq!(Rc::strong_count(&data), 3);
}

// Exercise smart_pointers3
// 使用 RefCell 在不可变引用内部修改数据。
#[test]
#[ignore]
    // I AM NOT DONE
fn smart_pointers3() {
    use std::cell::RefCell;

    let value = RefCell::new(0);

    {
        let mut borrow = value.borrow_mut();
        *borrow += 5;
    }

    {
        let mut borrow = value.borrow_mut();
        *borrow *= 2;
    }

    // 0 + 5 = 5; 5 * 2 = 10
    assert_eq!(*value.borrow(), 10);
}
