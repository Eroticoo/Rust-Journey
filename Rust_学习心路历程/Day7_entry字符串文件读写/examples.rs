use std::collections::HashMap;
use std::fs;

fn main() {
    // entry().or_insert() 词频统计
    let fruits = vec!["苹果", "香蕉", "苹果", "橘子", "香蕉", "苹果"];
    let mut count: HashMap<&str, i32> = HashMap::new();

    for fruit in &fruits {
        let entry = count.entry(fruit).or_insert(0);
        *entry += 1;
    }

    println!("count: {:?}", count);

    // String 与 &str
    let mut owned = String::from("hello");
    owned.push_str(" world");
    let borrowed: &str = &owned;
    println!("owned: {}, borrowed: {}", owned, borrowed);

    // 文件写入
    let data = "姓名,年龄,城市\n张三,25,北京\n李四,30,上海";
    match fs::write("output.csv", data) {
        Ok(()) => println!("写入成功"),
        Err(e) => println!("写入失败：{}", e),
    }

    // 文件读取
    match fs::read_to_string("output.csv") {
        Ok(text) => println!("读取内容:\n{}", text),
        Err(e) => println!("读取失败：{}", e),
    }
}
