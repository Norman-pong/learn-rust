// exercises/src/structs.rs
// Chapter 07: structs — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn structs1() {
    // I AM NOT DONE

    // 补全结构体字段并创建实例
    struct Person {
        name: String,
        age: i32,
    }

    let person = Person {
        // 原错误：name 是 &str，但字段类型要求 String
        name: String::from("Rust"),
        // 原错误：缺少 age 字段
        age: 10,
    };

    assert_eq!(person.age, 10);
}

#[test]
#[ignore]
fn structs2() {
    // I AM NOT DONE

    // 使用结构体更新语法简化创建
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

#[test]
#[ignore]
fn structs3() {
    // I AM NOT DONE

    // 为结构体实现方法，计算矩形面积
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
    // 原错误：rect.area() 消费 self，与调用语义/后续使用预期不匹配（此处 area 应借 self）
    assert_eq!(rect.area(), 12);
}
