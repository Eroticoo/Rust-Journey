use std::fs;

struct Person {
    name: String,
    age: u32,
    height: u32,
}

impl Person {
    fn introduce(&self) {
        println!("你好，我叫{}，今年{}岁", self.name, self.age);
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn read_file() -> Result<String, std::io::Error> {
    let content = fs::read_to_string("hello.txt")?;
    Ok(content)
}

fn main() {
    let p = Person {
        name: String::from("小明"),
        age: 18,
        height: 175,
    };

    p.introduce();
    println!("身高：{}", p.height);
    println!("成年：{}", p.is_adult());

    match read_file() {
        Ok(content) => println!("文件内容：{}", content),
        Err(e) => println!("读取失败：{}", e),
    }
}
