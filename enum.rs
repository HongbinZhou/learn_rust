#![allow(dead_code)]

use List::*;

enum Person {
    Tom,
    Jack,
    Height(u32),
    Weight(u32),
}

enum List {
    Cons(u32, Box<List>),
    Nil
}

impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => {
                1 + tail.len()
            },
            Nil => 0
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(num, ref tail) => {
                format!("{} {}", num, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}
fn main() {
    let person = Person::Jack;
    match person {
        Person::Tom => println!("tom"),
        _ => println!("default"),
    }

    // Create an empty linked list
    let mut list = List::new();

    // Append some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

}
