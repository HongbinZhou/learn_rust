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
    println!("\n---- {}", &input);
    let mut result: i32 = 0;
    let mut last_op: char = ' ';
    for char in input.chars() {
        match char {
            ' ' => {}
            '+' | '-' | '*' | '/' => {
                last_op = char;
            }
            '0'...'9' => {
                let num = char.to_digit(10).unwrap() as i32;
                match last_op {
                    '+' => result += num,
                    '-' => result -= num,
                    '*' => result *= num,
                    '/' => result /= num,
                    _   => result = num,
                }
                println!("current num: {}, last op: {}, result is:{}",
                         num, last_op, result);
            },
            _ => println!("unknown"),
        }

    }
    result
}

#[test]
fn test_calc() {
    assert_eq!(calc(" 1 + 2  "), 3);
    assert_eq!(calc(" 0 - 3  "), -3);
    assert_eq!(calc(" 2 / 2  "), 1);
    assert_eq!(calc(" 2 * 2  "), 4);
    assert_eq!(calc(" 2 * 3 * 4 /2 "), 12);
}

