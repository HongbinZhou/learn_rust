use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![String::from("one"),
                        String::from("two"),
                        String::from("three"),
                        String::from("four")];
        for val in vals {
            let _ = tx.send(val);
        }
    });

    // use rx as an iterator instead of calling rx.recv()
    // rx will automatically wait tx
    for received in rx {
        println!("rx.recv: {:?}", received);
    }
}
