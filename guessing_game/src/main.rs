extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Game start!");
    let magic_number = rand::thread_rng().gen_range(1, 101);

    loop {

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess : u32 = guess.trim().parse().expect("Please input an integer!");

        match guess.cmp(&magic_number) {
            Ordering::Less  => println!("{} is too small!", guess),
            Ordering::Equal => {
                println!("{} is good!", guess);
                break;
            },
            Ordering::Greater => println!("{} is too big!", guess),
        }
    }

    println!("Game over, The magic number is: {}", magic_number);

}
