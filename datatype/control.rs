fn main() {
    let number = 6;

    // 单分支
    if number == 0 {
        println!("number is divisible by 0");
    }

    // 双分支
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 多分支
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
