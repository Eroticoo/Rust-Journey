use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Student {
    name: String,
    score: u32,
    city: String,
}

fn read_file_safe(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

fn main() {
    // struct -> JSON
    let person = Student {
        name: "张三".to_string(),
        score: 88,
        city: "北京".to_string(),
    };

    let json_str = serde_json::to_string(&person).expect("序列化失败");
    println!("json: {}", json_str);

    // JSON -> struct
    let raw = r#"{"name":"李四","score":95,"city":"上海"}"#;
    let parsed: Student = serde_json::from_str(raw).unwrap();
    println!("parsed: {:?}", parsed);

    // Vec<struct> + 迭代器
    let students = vec![
        Student { name: "张三".to_string(), score: 88, city: "北京".to_string() },
        Student { name: "李四".to_string(), score: 95, city: "上海".to_string() },
        Student { name: "王五".to_string(), score: 72, city: "北京".to_string() },
        Student { name: "赵六".to_string(), score: 60, city: "深圳".to_string() },
    ];

    let good: Vec<&Student> = students.iter().filter(|s| s.score >= 80).collect();
    println!("good: {:#?}", good);

    let total: u32 = students.iter().map(|s| s.score).fold(0, |acc, x| acc + x);
    let avg = total / students.len() as u32;
    println!("avg: {}", avg);

    // 错误处理进阶
    match read_file_safe("data.txt") {
        Ok(text) => println!("file: {}", text),
        Err(e) => println!("file error: {}", e),
    }
}
