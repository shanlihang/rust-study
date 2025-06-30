struct User {
    name:String,
    age:u32,
    active:bool,
}

fn main(){
    let user = User{
        name:String::from("tom"),
        age:1,
        active:true,
    };
    from key in user {
        println!("字段：{}",key);
    }

}