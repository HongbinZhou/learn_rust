use std::io::{self, Write};

#[allow(dead_code)]
fn use_calc() {
    print!("Please input: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!{"Your input is: {}", input};
    calc(&input);
}

#[allow(dead_code)]
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

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Op(Operator),
    Num(u32),
    Space,
    LParen,
    RParen,
    Unknown,
}

#[derive(Debug, PartialEq)]
enum AST {
    Num(u32),
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Div(Box<AST>, Box<AST>),
}

struct Input {
    input: String,
}

impl Input {

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
                    let tok = self.token(&substr);
                    match tok {
                        Token::Space => {},
                        _ => toks.push(tok),
                    }
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

    fn syntax(&self, current_ast: Option<AST>, vt: &[Token]) -> Option<AST> {
        use Token::*;
        use Operator::*;
        if let Some((first, elements)) = vt.split_first() {
            match first {
                &Space => {
                    self.syntax(current_ast, elements)
                },
                &Unknown => None,
                &Num(a) => {
                    let node = AST::Num(a);
                    if elements.len() > 1 {
                        self.syntax(Some(node), elements)
                    }
                    else
                    {
                        Some(node)
                    }

                },
                &Op(Mul) => {
                    let left = current_ast.unwrap();
                    let right = self.syntax(None, elements).unwrap();
                    Some(AST::Mul(Box::new(left), Box::new(right)))
                },
                &Op(Div) => {
                    let left = current_ast.unwrap();
                    if let Some((divider, remains)) = elements.split_first() {
                        let mut tmp = Vec::new();
                        tmp.push(divider.clone());
                        let ast = AST::Div(Box::new(left), Box::new(self.syntax(None, &tmp).unwrap_or(AST::Num(100))));
                        self.syntax(Some(ast), remains)

                    }else{
                        None
                    }
                },
                &Op(Add) => {
                    let left = current_ast.unwrap();
                    let right = self.syntax(None, elements).unwrap();
                    Some(AST::Add(Box::new(left), Box::new(right)))
                },
                &Op(Sub) => {
                    let left = current_ast.unwrap();
                    let right = self.syntax(None, elements).unwrap();
                    Some(AST::Sub(Box::new(left), Box::new(right)))
                },
                &LParen | &RParen => {
                    self.syntax(None, elements)
                }
            }
        } else {
            None
        }
    }

}

struct Parser {
    toks: Vec<Token>,
    offset: usize,
}

impl Parser {
    fn new(toks: Vec<Token>) -> Parser {
        Parser {
            toks: toks,
            offset: 0
        }
    }
    fn current(&self) -> &Token {
        &self.toks[self.offset]
    }
    fn bump(&mut self) {
        if self.offset + 1 > self.toks.len() {
            panic!{"toks vec is empty!"};
        }
        self.offset += 1
    }
    fn is_empty(&self) -> bool {
        self.offset + 1 > self.toks.len()
    }

    fn factor(&mut self) -> AST {
        let tok = self.current().clone();
        match tok {
            Token::Num(a) => {
                self.bump();
                AST::Num(a)
            }
            Token::LParen => {
                self.bump();
                let ast = self.exp();
                self.expect(&Token::RParen);
                ast
            }
            _ => {panic!{"todo"}}
        }
    }

    fn exp(&mut self) -> AST {
        let item = self.term();
        self.handle_add_sub(item)

    }

    fn handle_div_mul(&mut self, t: AST) -> AST {
        let tok = self.current().clone();
        match tok {
            Token::Op(Operator::Mul) => {
                self.bump();
                let rc = AST::Mul(Box::new(t), Box::new(self.factor()));
                if self.is_empty() {
                    rc
                }else {
                    self.handle_div_mul(rc)
                }
            },
            Token::Op(Operator::Div) => {
                self.bump();
                let rc = AST::Div(Box::new(t), Box::new(self.factor()));
                if self.is_empty() {
                    rc
                }else {
                    self.handle_div_mul(rc)
                }
            },
            _ => {t}
        }
    }
    fn handle_add_sub(&mut self, t:AST) -> AST {
        let tok = self.current().clone();
        match tok {
            Token::Op(Operator::Add) => {
                self.bump();
                let rc = AST::Add(Box::new(t), Box::new(self.term()));
                if self.is_empty() {
                    rc
                }else {
                    self.handle_add_sub(rc)
                }
            },
            Token::Op(Operator::Sub) => {
                self.bump();
                let rc = AST::Sub(Box::new(t), Box::new(self.term()));
                if self.is_empty() {
                    rc
                }else {
                    self.handle_add_sub(rc)
                }
            },
            _ => {t}
        }
    }

    fn term(&mut self) -> AST {
        let a = self.factor();
        if self.is_empty(){
            a
        }
        else {
            self.handle_div_mul(a)
        }
    }

    fn expect(&mut self, tok: &Token) {
        if self.current() == tok {
            self.bump();
        }else{
            panic!{"expect {:?} but got {:?}", tok, self.current()};
        }
    }
}

fn eval(t: &AST) -> u32 {
    use AST::*;
    match t {
        &Num(a) => a,
        &Add(ref x, ref y) => eval(&x) + eval(&y),
        &Sub(ref x, ref y) => eval(&x) - eval(&y),
        &Div(ref x, ref y) => eval(&x) / eval(&y),
        &Mul(ref x, ref y) => eval(&x) * eval(&y),
    }
}

#[test]
fn test_eval() {
    use AST::*;
    let mut input = Input {input: "1+2-3".to_string()};
    let mut lex = input.lex();
    println!("lex: {:?}", &lex);
    let mut parser = Parser::new(lex);
    let mut ast = parser.exp();
    println!{"ast: {:?}", ast};
    let mut result = eval(&ast);
    assert_eq!(result, 0);

    println!{"--------------------------------------------------"};
    input = Input {input: "1+2+3*12/(3 - 2)".to_string()};
    lex = input.lex();
    println!("lex: {:?}", &lex);
    parser = Parser::new(lex);
    ast = parser.exp();
    println!{"ast: {:?}", ast};
    result = eval(&ast);
    assert_eq!(result, 39);

}
#[test]
fn test_parser() {
    use AST::*;
    let mut input = Input {input: "1+2-3".to_string()};
    let mut lex = input.lex();
    println!("lex: {:?}", &lex);
    let mut parser = Parser::new(lex);
    let mut ast = parser.exp();
    println!{"ast: {:?}", ast};

    assert_eq!(ast, Sub(Box::new(Add(Box::new(Num(1)),
                                     Box::new(Num(2)))),
                        Box::new(Num(3))));

    println!{"--------------------------------------------------"};
    input = Input {input: "1+2-3*12/(2 - 3)".to_string()};
    lex = input.lex();
    println!("lex: {:?}", &lex);
    parser = Parser::new(lex);
    ast = parser.exp();
    println!{"ast: {:?}", ast};


    assert_eq!(ast, Sub(Box::new(Add(Box::new(Num(1)), Box::new(Num(2)))),
                        Box::new(Div(Box::new(Mul(Box::new(Num(3)), Box::new(Num(12)))),
                                     Box::new(Sub(Box::new(Num(2)), Box::new(Num(3))))))));

}

#[test]
fn test_syntax() {
    // let input = Input {input: "12* (3-1)/2-1"};
    // let input = Input {input: "1 + 2"};
    let input = Input {input: "12/2 - 3".to_string()};
    let lex = input.lex();
    println!("lex: {:?}", &lex);
    let ast = input.syntax(None, &lex);
    println!("ast: {:?}", ast);
}

#[test]
fn test_lex() {
    use Token::*;
    use Operator::*;
    let input = Input {input: "12* (3-1)/2-1".to_string()};
    assert_eq!(Num(12), input.lex()[0]);
    assert_eq!(Op(Mul), input.lex()[1]);
    assert_eq!(Num(3), input.lex()[4]);
}

fn main() {
    println!("main");
    test_syntax();
}
