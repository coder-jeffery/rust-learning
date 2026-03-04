

// 所有权是 Rust 最独特的特性，对语言的其他部分有着深远的影响。它使 Rust 能够在无需垃圾回收器的情况下保证内存安全，因此理解所有权的工作原理至关重要。在本章中，我们将讨论所有权以及几个相关特性：借用、切片以及 Rust 如何在内存中布局数据。

//Rust 中的每个值都有一个所有者。

//同一时间只能有一个所有者。 | 当所有者超出范围时，价值将会下降。

//类型String

fn main(){
    println!("所有权是 Rust 最独特的特性，对语言的其他部分有着深远的影响。它使 Rust 能够在无需垃圾回收器的情况下保证内存安全，因此理解所有权的工作原理至关重要。在本章中，我们将讨论所有权以及几个相关特性：借用、切片以及 Rust 如何在内存中布局数据。");

    // 类型String

    let mut s  =  String::from("hello");

    s.push_str(", world");

    println!("value is {s}");


    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 value {s2}");

    let s3 = s2.clone();
    println!("s3 value =====> {s3}");

    println!("s1 value =====> {s1}",  s1 = s2.clone());


    // 范围和任务
    let mut str  = String::from("hello");
    str = String::from("jeffey");
    // str = str.push_str(", ddd");
    println!("{str} , world");

    let testStr = str.clone();
    // testStr = testStr.push_str(", ddd");
    println!("{testStr} , world");



    let x11 = 5;
    let y11 = x11;
    println!("x11 = {x11}, y11 =  {y11} ");


    let x12 = "hello";
    let y12 = x12;
    println!("x12 = {x12}, y12 =  {y12} ");


    let x13 = String::from("testing data");
    let y13 = x13;
    // println!("x13 = {x13}, y13 =  {y13} ");
}