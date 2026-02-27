fn main(){

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    let z = 5;
    let y = 10;

    println!("z = {z} and y + 2 = {}", y + 2);

    another_function();

    another_function2(5);

    print_labeled_measurement(8, 'h');

    print_number();

    bool_case01();
    
    con_case02();
}
fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn print_number(){
    // let x = (let y = 6);
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    // println!("this number value is {}", x)

    let x = five();
    println!("The value of x is: {x}");

    let xx = plus_one(5);

    println!("The value of xx is: {xx}");

    let lucky_number = 7; // I'm feeling lucky today
    println!("lucy number is {}", lucky_number);

    compare_number(6);
}


fn five() -> i32 {
    5
}

//调试程序
// rustc --explain E0308 


fn plus_one(xx: i32) -> i32 {
    xx + 1
}

fn compare_number(x: i32){

    let number  =  x;

    if number < 5{
        println!("this is true");
    }else{
        println!("this is false")
    }
}

fn bool_case01(){
    let number = 3;

    if number!= 0 {
        println!("number was three");
    }
}

fn con_case02(){
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}