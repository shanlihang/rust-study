fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // 所有权转移，s1 无效

    let s3 = &s2; // 所有权借用

    // println!("{}", s1); // ❌ 报错：s1 已被移动
    println!("{}", s2); // hello
    println!("{}", s3); // hello
}