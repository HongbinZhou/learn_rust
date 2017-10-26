use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move|| {
        let _ = tx.send("send 0 using tx");
        thread::sleep(Duration::from_secs(1));
        let _ = tx.send("send 1 using tx");
    });

    thread::spawn(move|| {
        let _ = tx1.send("send 0 using tx1");
        let _ = tx1.send("send 1 using tx1");
    });

    for r in rx {
        println!("main: {:?}", r);
    }

}
