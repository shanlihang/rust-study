fn main() {
    /* 不改变类型 */
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x 是：{}", x); // x 是：12

    /* 改变类型 */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces 是：{}", spaces); // spaces 是：3
}