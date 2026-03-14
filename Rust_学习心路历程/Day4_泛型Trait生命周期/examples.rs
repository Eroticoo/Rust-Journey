use std::fmt::Display;

// 泛型函数
fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 泛型结构体
struct MyBox<T> {
    content: T,
}

// trait 定义能力
trait Area {
    fn calculate(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Area for Circle {
    fn calculate(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Area for Square {
    fn calculate(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Area>(shape: T) {
    println!("面积是: {}", shape.calculate());
}

// 生命周期示例
fn important<'a>(notice1: &'a str, notice2: &'a str) -> &'a str {
    if notice1.len() > notice2.len() {
        notice1
    } else {
        notice2
    }
}

fn show<T: Display>(x: T) {
    println!("{}", x);
}

fn main() {
    println!("larger int: {}", larger(3, 7));
    println!("larger float: {}", larger(1.5, 0.8));

    let int_box = MyBox { content: 100 };
    let str_box = MyBox { content: String::from("苹果") };
    show(int_box.content);
    show(str_box.content);

    let c = Circle { radius: 5.0 };
    let s = Square { side: 4.0 };
    print_area(c);
    print_area(s);

    let a = "短句";
    let b = "这是更长的一句";
    println!("更重要的是: {}", important(a, b));
}
