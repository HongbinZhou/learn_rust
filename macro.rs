macro_rules! create_function {
    ($func:ident) => {
        fn $func () {
            println!("create function {}", stringify!($func));
        }
    }
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("input is: {}", stringify!($expression));
    }
}
macro_rules! say_hello {
    () => {
        println!("hihi")
    }
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}", stringify!($left), stringify!($right), $left && $right);
    };
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}", stringify!($left), stringify!($right), $left || $right);
    };
}
create_function!(foo);

macro_rules! find_min {
    ($x:expr) => {$x};
    ($x:expr, $($y:expr),+) => {std::cmp::min($x,find_min!($($y),+))};
}
fn main() {
    foo();
    say_hello!();
    print_result!({
        foo();
        let x = 3;
        x * x
    });
    test!(1==1; and 2==2);
    test!(1==1; or 3==2);
    println!("{}", find_min!(1));
    println!("{}", find_min!(4,5,6));
}

