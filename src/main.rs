
use rand::Rng;
use std::io;  //prelude 


fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("数字：{} ", secret_number);

    println!("猜测一个数字");

    // let mut foo = 1; // mut 可变变量
    // let  bar = foo;
    // foo =2;

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取");

    println!("你猜测的数字是：{} ", guess );
}
