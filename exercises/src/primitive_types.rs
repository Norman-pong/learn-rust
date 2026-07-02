// exercises/src/primitive_types.rs
// Chapter 04: primitive_types — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
fn primitive_types1() {
    // I AM NOT DONE

    // 元组可以包含不同类型的值，请补全类型
    let tuple: (i32, &str) = (500, "Rust");
    let (x, y) = tuple;
    assert_eq!(x, 500);
    assert_eq!(y, "Rust");
}

#[test]
#[ignore]
fn primitive_types2() {
    // I AM NOT DONE

    // 数组的长度是类型的一部分，请让长度匹配
    // 原错误：数组类型 [i32; 4] 与初始化长度 3 不匹配
    todo!();
}

#[test]
#[ignore]
fn primitive_types3() {
    // I AM NOT DONE

    // 切片允许你引用数组的一部分，请补全范围
    let arr = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[2..5];
    assert_eq!(slice, &[30, 40]);
}

#[test]
#[ignore]
fn primitive_types4() {
    // I AM NOT DONE

    // 字符类型使用单引号，请修复类型
    let c = "R";
    // 原错误：c 是 &str，不能与字符 'R' 比较
    todo!();
}

#[test]
#[ignore]
fn primitive_types5() {
    // I AM NOT DONE

    // 布尔值只能比较相等，不能用 + 运算；请修复
    let a = true;
    let b = false;
    // 原错误：布尔值不能相加 (a + b)
    todo!();
}

#[test]
#[ignore]
fn primitive_types6() {
    // I AM NOT DONE

    // 元组索引使用点号，请补全正确的索引
    let pair = ("Rust", 2024);
    // 原错误：pair.0 是 &str，不能与整数 2024 比较
    todo!();
}
