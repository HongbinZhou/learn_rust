
use std::thread;

fn main() {
    let child = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
        }
    });

    let _ = child.join();
    for i in 1..5 {
        println!("hi number {} from main thread!", i);
    }
}
