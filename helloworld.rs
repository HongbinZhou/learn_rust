use std::fmt;

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl fmt::Display for Circle {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}, radius:{}", self.x, self.y, self.radius)
    }
}

impl Circle {
    fn reference(&self) {
        println!("taking self by reference!");
    }

    fn mutable_reference(&mut self) {
        println!("taking self by mutable reference!");
    }

    fn takes_ownership(self) {
        println!("taking ownership of self!");
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}

struct List(Vec<Circle>);
impl fmt::Display for List {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        let List(ref vec) = *self;
        for (idx, value) in vec.iter().enumerate() {
            if (idx != 0) {write!(f, ",");};
            try!(write!(f, "{}:Circle({}, {}, {})", idx, value.x, value.y, value.radius));
        }
        write!(f, "]")
    }
}

fn main(){
    println!["hello, world!"];
    let circle = Circle{x: 1.0, y: 2.0, radius: 3.0};
    println!("circle init: {}", circle);
    println!("circle init debug: {:?}", circle);
    println!("circle x is {}", circle.grow(2.0).area());

    let v = vec![1, 2, 3, 4, 5];
    println!("The v is {:?}", v);
    println!("The third element of v is {}", v[1]);

    let mylist = List(vec![Circle{x: 1.0, y: 2.0, radius: 3.0}, Circle{x: 1.0, y: 2.0, radius: 3.0}]);
    println!("mylist is好的 {}", mylist);

    let s = "Hello".to_string();
    takes_slice(&s);

    #[allow(dead_code)]
    let x: Option<i32> = Some(5_0_0);
    let y: Option<f64> = Some(5.0f64);
    println!("x is {:?}", x);

    let pi = 3.1415;
    println!("pi is {:?}", pi);

    let color = Color{red:128 ,green:1,blue:222};
    println!("color: {}", color);
}


#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "r:{0:}, g:{1:}, b:{2:}, 0x{0:02x}{1:02x}{2:02x}", self.red, self.green, self.blue)
    }
}

impl Drop for Circle {
    fn drop(&mut self) {
        println!("BOOM times {}!!!", self.area());
    }
}

#[test]
#[should_panic]
fn test(){
}
fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}

