fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    testcase_function();


    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces value is {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess value is {}", guess);


    // let mut c = 'x';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    // c =  'A';
    // println!("{}", c);


    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    print_labeled_measurement(5, 'h');


    let x1 =  { let y1 = 6;  y1 - 0 };
    println!("x1 = {x1}"); // 10f2c

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");

    testcase_control(3);
    testcase_if(3);
    testcase_mult_if(10);
    fn_testcase_condition(false);


    testcase_loop();

    testcase_loop_counting_up();

    testcase_while();

    testcase_for();

    testcase_for2();

}

// hello, world

fn testcase_function(){
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// sudo xcodebuild -license

fn testcase_control(value: i32)  {

    let number = value; // 

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn testcase_if(value: i32){
    let number = value;

    if number != 0{
        println!("number was something other than zero");
    }
}

fn testcase_mult_if(value: i32){
    if value % 4 == 0 {
        println!("number is divisible by 4");
    } else if value % 3 == 0 {
        println!("number is divisible by 3");
    } else if value % 2 == 0 {
        println!("value is divisible by 2");
    } else {
        println!("value is not divisible by 4, 3, or 2");
    }
}


fn fn_testcase_condition(value: bool){

    let condition = value;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn testcase_loop(){

    let mut count  = 0;

    // help: if this is intentional, prefix it with an underscore: `_result
    let  _result  = loop {
        count += 1;
        
        println!("test case print");
        if count == 3 {
            break;
        }
    };
    println!("test case print");
   
}

//基本数据类型
//条件语句
//分支结构
//

fn testcase_loop_counting_up(){

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


fn testcase_while(){

    let mut number  =4;

    while number !=0 {
        println!("{number}!");
        number -= 1;
    }

    println!("lift off !");
}

fn testcase_for(){
    let a  =  [10,20,30,40,50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn testcase_for2(){

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("lift off !");
}