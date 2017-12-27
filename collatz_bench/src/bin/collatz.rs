use std::env;

extern crate collatz_bench;

use collatz_bench::collatz;

fn usage() {
    println!("usage:
collatz <n>
    Calc the collatz steps for n  ");

}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            if let Ok(x) = args[1].parse() {
                let y = collatz::collatz(x);
                println!("collatz({})={}", x, y);
            }
        },
        _ => usage(),
    }
}

