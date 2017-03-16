fn main() {
    let x = 5;
    let y = &x;
    println!("&x={:p}", &5);
    println!("&x={:?}", &5 as * const i32);
}
