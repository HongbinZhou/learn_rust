use std::io::{self, Write};

fn use_calc() {
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

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug)]
enum Token {
    Op(Operator),
    Num(u32),
    Space,
    LParen,
    RParen,
    Unknown,
}

struct AST {
    node: Token,
    left: Box<AST>,
    right: Box<AST>
}

struct Input<'a> {
    input: &'a str,
}

impl<'a> Input<'a> {
    fn token(&self, input: &str) -> Token {
        use Token::*;
        let mut tok = Unknown;

        if input.len() >= 1 && input.chars().all(|x| x.is_digit(10)) {
            tok = Num(input.parse::<u32>().unwrap());
        } else if input.len() == 1 {
            match input.chars().nth(0).unwrap() {
                ' ' => tok = Space,
                '(' => tok = LParen,
                ')' => tok = RParen,
                '+' => tok = Op(Operator::Add),
                '-' => tok = Op(Operator::Sub),
                '*' => tok = Op(Operator::Mul),
                '/' => tok = Op(Operator::Div),
                char @ _ => {
                    panic!("unknown input char: '{}'!", char);
                },
            }
        }
        tok
    }

    // function: convert input string to vector of tokens
    fn lex(&self) -> Vec<Token>{
        let mut idx = 0;
        let mut toks = Vec::new();
        loop {
            match self.substr_at(idx) {
                Some(substr) => {
                    toks.push(self.token(&substr));
                    idx += substr.len();
                }
                None => {break;}
            }
        }
        toks
    }

    fn substr_at(&self, cur_pos: usize) -> Option<String> {
        let mut tmp_str = String::new();
        let mut head = self.input.chars().skip(cur_pos);
        match head.next() {
            Some(char) => {
                tmp_str.push(char);
                if char.is_digit(10) {
                    loop {
                        match head.next() {
                            Some(char) => {
                                if char.is_digit(10) {
                                    tmp_str.push(char);
                                }else{
                                    break;
                                }
                            }
                            None => {break;}
                        }
                    }
                }
            }
            None => {}
        }
        if tmp_str.is_empty() {
            None
        } else {
            Some(tmp_str)
        }
    }

}

fn main() {
    let input = Input {input: "12* (3-1)/2-1"};
    println!("{:?}", input.split());
}
