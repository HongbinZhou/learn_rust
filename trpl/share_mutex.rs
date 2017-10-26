use std::sync::Mutex;
use std::sync::Arc;
use std::thread;


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for i in 0..10 {
        let count = counter.clone();
        let handler = thread::spawn(move || {
            if let Ok(mut cnt) = count.lock() {
                *cnt += i;
            }; // note: the ending ; here can't be omitted! otherwise error: "dropped while still borrowed"
        });
        handlers.push(handler);
    }

    for handler in handlers {
        let _ = handler.join();
    }

    println!("counter is: {}", counter.lock().unwrap());
}
