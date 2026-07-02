// exercises/src/hashmaps.rs
// Chapter 11: HashMap — rustlings fork
// 深做章节

// 每题含 `// I AM NOT DONE` 注释，运行前删除即可

use std::collections::HashMap;

#[test]
#[ignore]
    // I AM NOT DONE
fn hashmaps1() {
    let mut fruit_calories: HashMap<String, u32> = HashMap::new();
    fruit_calories.insert(String::from("apple"), 52);
    fruit_calories.insert(String::from("banana"), 89);
    fruit_calories.insert(String::from("grape"), 69);

    assert_eq!(fruit_calories["apple"], 52);
    assert_eq!(fruit_calories["banana"], 89);
    assert_eq!(fruit_calories["grape"], 69);
}

// Exercise hashmaps2
// 使用 entry 与 or_insert 统计词频，忽略大小写。
#[test]
#[ignore]
    // I AM NOT DONE
fn hashmaps2() {
    let text = "Rust is fast and Rust is safe and Rust is fun";
    let mut counts: HashMap<String, u32> = HashMap::new();

    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    assert_eq!(counts["rust"], 3);
    assert_eq!(counts["is"], 3);
    assert_eq!(counts["and"], 2);
    assert_eq!(counts["fast"], 1);
}

// Exercise hashmaps3
// 使用 entry API 在 key 不存在时插入默认值。
#[test]
#[ignore]
    // I AM NOT DONE
fn hashmaps3() {
    let mut scores = HashMap::from([
        ("blue".to_string(), 10),
        ("yellow".to_string(), 50),
    ]);

    let blue_score = scores.entry(String::from("blue")).or_insert(0);
    assert_eq!(*blue_score, 10);

    let red_score = scores.entry(String::from("red")).or_insert(0);
    *red_score += 20;
    assert_eq!(scores["red"], 20);

    assert_eq!(scores["blue"], 10);
    assert_eq!(scores.len(), 3);
}
