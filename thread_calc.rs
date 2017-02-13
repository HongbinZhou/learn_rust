use std::thread;
use std::sync::mpsc;

fn main() {
    // calc: 1*2 + 3*4 + 5*6 = ?

    let (tx, rx) = mpsc::channel();
    let (tx_str, rx_str) = mpsc::channel::<String>();

    {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send((1*2))
        });
    }

    {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(3*4)
        });
    }

    {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(5*6)
        });
    }

    thread::spawn(move || {
        let x = rx.recv().unwrap();
        let y = rx.recv().unwrap();
        let z = rx.recv().unwrap();

        println!("{:?} + {:?} + {:?} = {:?}", x, y, z, x+y+z);
        let result = format!("{:?} + {:?} + {:?} = {:?}", x, y, z, x+y+z);
        tx_str.send(result)
    });

    println!("{:?}", rx_str.recv());
}
