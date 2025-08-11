

fn largest_u32(a: u32, b:u32) -> u32 {
    if a > b {
        a
    }else {
        b
    }
}


fn main(){
    println!("{} ********", largest_u32(1,2))

}