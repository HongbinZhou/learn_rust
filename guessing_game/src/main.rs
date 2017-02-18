use std::io;
use std::cmp::Ordering;

fn main() {

    println!("Game start!");
    let magic_number = 40;

    let mut input_raw = String::new();
    io::stdin().read_line(&mut input_raw).expect("Failed to read input");
    let input = input_raw.trim();
    match input.cmp(&magic_number.to_string()) {
        Ordering::Less => println!("{} is too small!", input),
        Ordering::Equal    => println!("{} is too small!", input),
        Ordering::Greater => println!("{} is good!", input),
    }
    println!("Game over!");

}
