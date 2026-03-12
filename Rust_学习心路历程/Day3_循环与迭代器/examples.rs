fn main() {
    // loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }

    // while
    let mut n = 1;
    while n < 5 {
        n += 1;
    }

    // for
    let fruits = vec!["苹果", "香蕉", "橘子"];
    for fruit in &fruits {
        println!("今天吃: {}", fruit);
    }

    // 迭代器
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    let passed: Vec<&i32> = numbers.iter().filter(|x| **x >= 3).collect();
    println!("passed: {:?}", passed);

    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("sum: {}", sum);

    let scores = vec![55, 82, 91, 67, 78, 95, 43, 88];
    let result = scores
        .iter()
        .filter(|x| **x >= 60)
        .map(|x| x + 10)
        .fold(0, |acc, x| acc + x);
    println!("及格学生补分后总分: {}", result);
}
