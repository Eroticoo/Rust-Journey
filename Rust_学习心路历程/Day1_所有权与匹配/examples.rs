fn print_length(s: &String) {
    println!("长度是: {}", s.len());
}

fn main() {
    // 所有权转移
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    // 借用
    let s = String::from("hello");
    print_length(&s);
    println!("{}", s);

    // 借用冲突（正确顺序）
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("{}", first);
    v.push(4);

    // match
    let number = 7;
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 => println!("质数"),
        13..=19 => println!("十几"),
        _ => println!("其他"),
    }

    // Option
    let fish: Option<&str> = Some("🐟");
    match fish {
        Some(f) => println!("今天吃: {}", f),
        None => println!("今天没得吃"),
    }
}
