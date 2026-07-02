// exercises/src/smart_pointers.rs
// Chapter 19: Smart Pointers — rustlings fork
// 深做章节

// 智能指针在堆上管理数据并提供额外语义。本章覆盖 Box、Rc 和 RefCell。

#[test]
#[ignore]
fn smart_pointers1() {
    // I AM NOT DONE
    // 使用 Box 在堆上分配一个递归类型：链表节点。
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

    let expected: i32 = todo!("计算链表元素之和");
    assert_eq!(sum, expected);
}

#[test]
#[ignore]
fn smart_pointers2() {
    // I AM NOT DONE
    // 使用 Rc 让多个所有者共享同一份数据。
    use std::rc::Rc;

    let data = Rc::new(String::from("shared"));
    let _a = Rc::clone(&data);
    let _b = Rc::clone(&data);
    let _c = Rc::clone(&data);

    let expected: usize = todo!("引用计数应该是多少？");
    assert_eq!(Rc::strong_count(&data), expected);

    drop(_a);
    assert_eq!(Rc::strong_count(&data), 3);
}

#[test]
#[ignore]
fn smart_pointers3() {
    // I AM NOT DONE
    // 使用 RefCell 在不可变引用内部修改数据。
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

    let actual: i32 = todo!("读取 RefCell 内部的值");
    assert_eq!(actual, 10);
}
