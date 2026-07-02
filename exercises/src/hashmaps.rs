// exercises/src/hashmaps.rs
// Chapter 11: HashMap — rustlings fork
// 深做章节

// HashMap 是 Rust 的标准库关联容器。本章练习覆盖 entry、or_insert、
// 计数统计与累加更新。你需要让每一组 assert 通过。

#[test]
#[ignore]
fn hashmaps1() {
    // I AM NOT DONE
    // 创建一个 HashMap 并插入至少三种水果的热量。
    let mut fruit_calories: std::collections::HashMap<String, u32> =
        std::collections::HashMap::new();

    todo!("使用 insert 添加 apple、banana、grape 的热量");

    assert_eq!(fruit_calories["apple"], 52);
    assert_eq!(fruit_calories["banana"], 89);
    assert_eq!(fruit_calories["grape"], 69);
}

#[test]
#[ignore]
fn hashmaps2() {
    // I AM NOT DONE
    // 统计字符串中每个单词出现的次数，忽略大小写。
    fn count_words(text: &str) -> std::collections::HashMap<String, u32> {
        todo!("使用 entry 与 or_insert 更新计数")
    }

    let text = "Rust is fast and Rust is safe and Rust is fun";
    let counts = count_words(text);

    assert_eq!(counts["rust"], 3);
    assert_eq!(counts["is"], 3);
    assert_eq!(counts["and"], 2);
    assert_eq!(counts["fast"], 1);
}

#[test]
#[ignore]
fn hashmaps3() {
    // I AM NOT DONE
    // 给定一个已有 score 的游戏队伍，使用 entry API 只在 key 不存在时插入默认值。
    let mut scores = std::collections::HashMap::from([
        ("blue".to_string(), 10),
        ("yellow".to_string(), 50),
    ]);

    let blue_score: &mut u32 = todo!("当 blue 已存在时，or_insert 的返回值是什么？");
    assert_eq!(*blue_score, 10);

    let red_score: &mut u32 = todo!("为 red 插入默认 0 并返回可变引用");
    *red_score += 20;
    assert_eq!(scores["red"], 20);

    assert_eq!(scores["blue"], 10);
    assert_eq!(scores.len(), 3);
}
