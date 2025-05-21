fn main() {
    // 解构元组
    let (x, y) = (1, 2);
    println!("x = {}, y = {}", x, y); // x = 1, y = 2

    // 解构结构体
    struct Point { x: i32, y: i32 }
    let p = Point { x: 10, y: 20 };
    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b); // a = 10, b = 20

    // 解构数组/切片 (使用_及..跳过元素)
    let arr = [1, 2, 3, 4, 5];
    let [first, _, third, ..] = arr;
    println!("first = {}, third = {}", first, third); // first = 1, third = 3

    // 解构枚举
    enum Color { Red, Green, Blue }
    let color = Color::Red;
    match color {
        Color::Red => println!("红色"),
        Color::Green => println!("绿色"),
        Color::Blue => println!("蓝色"),
    } // 红色

}