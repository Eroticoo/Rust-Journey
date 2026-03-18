use std::collections::HashMap;

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

mod kitchen {
    pub fn cook() {
        println!("做饭");
        wash();
    }

    fn wash() {
        println!("洗碗");
    }
}

mod data {
    pub mod parser {
        pub fn parse_line(line: &str) -> Vec<&str> {
            line.split(',').collect()
        }
    }
}

use data::parser::parse_line;

fn main() {
    println!("1 + 2 = {}", math::add(1, 2));
    kitchen::cook();

    let cols = parse_line("张三,25,北京");
    println!("{:?}", cols);

    let mut city_population = HashMap::new();
    city_population.insert("北京", 2154);
    city_population.insert("上海", 2487);
    city_population.insert("深圳", 1756);

    match city_population.get("北京") {
        Some(pop) => println!("北京人口：{}万", pop),
        None => println!("没找到"),
    }

    for (city, pop) in &city_population {
        println!("{}: {}万人", city, pop);
    }
}
