# code-readings · 源码阅读笔记

逐文件阅读 Rust 生态中高质量项目的源代码，学习生产级 Rust 的写法和架构模式。

## 阅读列表

| 项目 | 仓库 | 笔记状态 |
|------|------|---------|
| ripgrep | https://github.com/BurntSushi/ripgrep | 待开始 |
| tokio | https://github.com/tokio-rs/tokio | 待开始 |
| rust-analyzer | https://github.com/rust-lang/rust-analyzer | 待开始 |

## 阅读方法

1. 先看 `README` + 架构文档
2. 从 `main.rs` / `lib.rs` 入口开始
3. 挑一个你感兴趣的功能模块深读
4. 记录：使用了什么 crate、什么设计模式、哪里值得学习

> 笔记以文件级别原子保存，如 `code-readings/ripgrep-args.md`。
