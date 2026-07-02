// exercises/src/structs.rs
// Chapter 07: 结构体 — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn structs1() {
    struct Person {
        name: String,
        age: i32,
    }

    let person = Person {
        name: String::from("Rust"),
        age: 10,
    };

    assert_eq!(person.age, 10);
}

// Exercise structs2
// 使用结构体更新语法简化创建。
#[test]
#[ignore]
    // I AM NOT DONE
fn structs2() {
    struct RGB {
        r: u8,
        g: u8,
        b: u8,
    }

    let base = RGB { r: 0, g: 0, b: 0 };
    let red = RGB {
        r: 255,
        ..base
    };

    assert_eq!(red.r, 255);
    assert_eq!(red.g, 0);
    assert_eq!(red.b, 0);
}

// Exercise structs3
// 为结构体实现方法，area 应接收 &self 以避免转移所有权。
#[test]
#[ignore]
    // I AM NOT DONE
fn structs3() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
    // I AM NOT DONE
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle { width: 3, height: 4 };
    assert_eq!(rect.area(), 12);
}
