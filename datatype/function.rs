fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x); // The value of x is: 6
    let y = plus_str();
    println!("The value of y is: {}", y); // The value of x is: 6
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_str() -> i32 {
    return 12 // The value of y is: 12
}