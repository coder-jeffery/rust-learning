use std::io;


fn main(){
    let spaces = "   ";
    let spaces = spaces.len();

    println!("space value is {}", spaces);

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    testcase01();
    testcase02();
    testcase03();
    testcase04();
    testcase05_add_sub_mult_div_rem();

    testcase06();
    testcase07();

    testcase_arr();

    print_labeled_measurement(5, 'h');

    luck_number();

    testcase_if(4);

}

fn testcase01(){
    let  spaces = "   ";
    let spaces = spaces.len();
    println!("spaces value is {}", spaces)
}

fn testcase02(){
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn testcase03(){
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("testcase03 value is {}", guess);
}

fn testcase04(){
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("f value is {}", f);
}

fn testcase05_add_sub_mult_div_rem(){

    // addition
    let sum = 5 + 10;
    println!("sum value is {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference value is {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product value is {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("truncated value is {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("remainder value is {}", remainder);

}

fn testcase06(){
    // let a = [1, 2, 3, 4, 5];
    // println!("value is {}", a);
}

fn testcase07(){

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    println!("first value is {}", first);

    let second = a[3];
    println!("second value is {}", second);
}


fn testcase_arr(){

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn luck_number(){

    let lucky_number = 7; // I'm feeling lucky today
    println!("luck number value is {}", lucky_number);
}

fn testcase_if(value: u32){

    let mut number = value;

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