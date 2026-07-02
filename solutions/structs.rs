// solutions/structs.rs — 本文件不会被 cargo 编译进 exercises
// 做题前先删掉 exercises/ 对应文件的 // I AM NOT DONE 标记再实现
// 需要查看时直接 `cat solutions/structs.rs`

// ============================================================
// Chapter 07: 结构体 — 参考答案
// ============================================================

// Exercise structs1
// 补全结构体字段并创建实例，age 应为 10。
#[test]
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
fn structs3() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle { width: 3, height: 4 };
    assert_eq!(rect.area(), 12);
}
