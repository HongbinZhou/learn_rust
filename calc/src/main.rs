use std::io::{self, Write};

fn main() {
    print!("Please input: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!{"Your input is: {}", input};
    calc(&input);
}

fn calc(input:&str) -> i32 {
    // let mut raw = String::from(input);
    for char in input.chars() {
        match char {
            ' ' => println!("blank"),
            '+' => println!("plus"),
            '0'...'9'   => println!("{}", char),
            _ => println!("unknown"),
        }

    }
    // println!{"Input str: {}", chars.as_str()};
    1
}

#[test]
fn test_calc() {
    let result = calc(" 1 + 2  ");
    assert_eq!(result, 3);
}

