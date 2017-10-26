use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        if let Ok(mut num) = m.lock() {
            *num = 6;
            println!("first lock!")
        }
        println!("m = {:?}", m);
        if let Ok(mut num) = m.lock() {
            *num = 7;
            println!("second lock!")
        }
        println!("m = {:?}", m);
    }
}
