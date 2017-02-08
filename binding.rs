#![allow(dead_code)]
enum Person {
    Tom,
    Jack,
}
impl Person {
    fn a () {
        
    }
}
fn main() {
    let _a = 10;
    let _b = 20;
    let c = 1234523.0_f32;
    println!("a={}", _a);
    println!("c={:x}", c as u32);
    println!("c as int={}", c as u32 as u8);
    println!("1234523 % 256={}", c as u32 % 256);

    let d = 12;
    match d {
        n @ 1 ... 13 => println!("n={}", n),
        _ => println!("default"),
    }
}
