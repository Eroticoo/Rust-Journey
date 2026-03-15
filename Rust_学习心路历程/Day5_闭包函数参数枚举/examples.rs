// 普通函数
fn double(x: i32) -> i32 {
    x * 2
}

fn apply_discount(price: f64, discount: fn(f64) -> f64) -> f64 {
    discount(price)
}

fn ninety_percent(price: f64) -> f64 {
    price * 0.9
}

enum OrderStatus {
    Pending,
    Shipped(String),
    Cancelled(String),
}

impl OrderStatus {
    fn is_active(&self) -> bool {
        match self {
            OrderStatus::Cancelled(_) => false,
            _ => true,
        }
    }
}

fn main() {
    // 闭包
    let bonus = 10;
    let add_bonus = |score| score + bonus;
    println!("bonus score: {}", add_bonus(90));

    // 闭包多种写法
    let times2 = |x| x * 2;
    let add = |x, y| x + y;
    let complex = |x| {
        let result = x * 2;
        result + 1
    };
    println!("{} {} {}", times2(4), add(2, 3), complex(5));

    // 函数作为参数
    let p1 = apply_discount(200.0, ninety_percent);
    println!("price1: {}", p1);

    // 闭包作为参数（不捕获外部变量时可退化为 fn）
    let p2 = apply_discount(200.0, |p| p * 0.85);
    println!("price2: {}", p2);

    // 枚举与 match
    let order = OrderStatus::Shipped(String::from("SF1234567890"));
    match order {
        OrderStatus::Pending => println!("待付款"),
        OrderStatus::Shipped(number) => println!("快递单号：{}", number),
        OrderStatus::Cancelled(reason) => println!("取消原因：{}", reason),
    }

    let cancelled = OrderStatus::Cancelled(String::from("用户主动取消"));
    println!("is_active: {}", cancelled.is_active());

    // 复习：函数调用
    println!("double: {}", double(6));
}
