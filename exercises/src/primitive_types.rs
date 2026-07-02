// exercises/src/primitive_types.rs
// Chapter 04: 基本类型 — rustlings fork
// 快进章节（预计 1h 内完成）

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

#[test]
#[ignore]
    // I AM NOT DONE
fn primitive_types1() {
    let tuple: (i32, &str) = (500, "Rust");
    let (x, y) = tuple;
    assert_eq!(x, 500);
    assert_eq!(y, "Rust");
}

// Exercise primitive_types2
// 数组长度是类型的一部分，应让声明的长度与初始化元素个数一致。
#[test]
#[ignore]
    // I AM NOT DONE
fn primitive_types2() {
    let nums: [i32; 3] = [1, 2, 3];
    assert_eq!(nums.len(), 3);
}

// Exercise primitive_types3
// 切片是数组的视图，范围 2..4 对应元素 [30, 40]。
#[test]
#[ignore]
    // I AM NOT DONE
fn primitive_types3() {
    let arr = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[2..4];
    assert_eq!(slice, &[30, 40]);
}

// Exercise primitive_types4
// 字符类型使用单引号，字符串字面量 "R" 应改为字符字面量 'R'。
#[test]
#[ignore]
    // I AM NOT DONE
fn primitive_types4() {
    let c = 'R';
    assert_eq!(c, 'R');
}

// Exercise primitive_types5
// 布尔值不支持 + 运算，使用 if 表达式将其转换为 i32 后求和。
#[test]
#[ignore]
    // I AM NOT DONE
fn primitive_types5() {
    let a = true;
    let b = false;
    let result = if a { 1 } else { 0 } + if b { 1 } else { 0 };
    assert_eq!(result, 1);
}

// Exercise primitive_types6
// 元组索引使用点号，pair.1 才是第二个元素 2024。
#[test]
#[ignore]
    // I AM NOT DONE
fn primitive_types6() {
    let pair = ("Rust", 2024);
    assert_eq!(pair.1, 2024);
}
