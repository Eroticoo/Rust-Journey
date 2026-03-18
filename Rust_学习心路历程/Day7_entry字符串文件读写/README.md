# Day 7 - entry().or_insert()、String vs &str、文件读写

## 今天学了什么
- `entry().or_insert()` 进行智能插入与计数
- `String` 与 `&str` 的所有权和可变性区别
- `fs::read_to_string` / `fs::write` 的文件 IO

## 心路历程
今天最关键的是把“引用”从语法记忆变成了操作直觉。
`or_insert` 返回的是引用，必须解引用后修改，这一点最开始绕，
但一旦和第一天的借用/解引用串起来就通了。

`String` 与 `&str` 的区分也更稳了：
拥有 vs 借用、可改 vs 只读、何时转换，这些边界变得清晰。

文件读写这部分因为前面已经建立了 `Result` 思维，吸收很顺，
Rust 的报错路径在我眼里不再是“麻烦”，而是“预期路径”。

## 学习代码
见本目录 `examples.rs`。
