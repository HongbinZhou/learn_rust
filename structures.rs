#[derive(Debug)]
struct Point (f32, f32);
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        p1: Point (x1, y1),
        p2: Point (x2, y2)
    } = *rect;
    (x1-x2) * (y1-y2)
}

fn square(p: Point, width: f32) -> Rectangle {
    let p1 = p;
    let Point (x1, y1) = p1;
    let p2 = Point (x1 + width, y1 + width);
    Rectangle {
        p1: Point (x1, y1), p2: p2
    }
}

fn main(){
    let p1 = Point(1.1, 2.2);
    let p2 = Point(3.3, 4.4);

    let rect = Rectangle{p1:p1, p2:p2};
    println!("rec: {:?}", rect);
    println!("rect_area: {}", rect_area(&rect));

    let p1 = Point(1.1, 2.2);
    println!("square: {:?}", square(p1, 2.0));
}
