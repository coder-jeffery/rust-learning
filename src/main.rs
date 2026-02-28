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


    let mut c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    c =  'A';
    println!("{}", c);

}


fn testcase_function(){
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn testcase_bool(){


    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}