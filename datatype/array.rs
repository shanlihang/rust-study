fn main(){
    // 指定类型声明
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 默认值
    let _b = [3; 5]; // b = [3, 3, 3, 3, 3]

    // 索引访问
    println!("The value of a[0] is: {}", a[0]); // The value of a[0] is: 1
}