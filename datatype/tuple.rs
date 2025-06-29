fn main() {
    // 自动推导类型
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y); // The value of y is: 6.4

    // 指定类型
    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of first is: {}", tup2.0) // The value of first is: 500

}
