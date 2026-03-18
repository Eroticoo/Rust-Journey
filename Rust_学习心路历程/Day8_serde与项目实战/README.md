# Day 8 - serde/JSON、derive、错误处理进阶、Vec<struct> + 项目实战

## 今天学了什么
- `serde` 与 JSON 的序列化/反序列化
- `#[derive(...)]` 自动实现 trait
- `unwrap` / `expect` / `match` / `?` 的错误处理策略
- `Vec<struct>` 配合迭代器进行筛选统计
- Duke 数据工程仓库 3 个子项目实战

## 心路历程
今天是“语法 + 项目”双线并进的一天。
语法侧，我把 `serde`、`derive`、错误处理和结构体数组连成了一条线；
项目侧，我在真实仓库里跑通了 `vector-fruit-salad`、`hashset-fruit`、`hashmap-count`。

最大的转折点是：
我开始把编译器错误当作反馈系统，而不是阻力。
遇到 `E0425` 后能快速定位作用域问题，也通过 `{:#?}` 学会了更高质量调试输出。

今天确认了学习策略：
**语法当天学，项目当天用，知识就会固化。**

## 学习代码
见本目录 `examples.rs`。
