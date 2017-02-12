use std::thread;


fn main() {
    let handle = thread::spawn( ||{
        println!("hello form thread!");
    });
    thread::sleep(std::time::Duration::new(2,0));
    println!("{:?}", handle.join().unwrap());
    // println!("out of thread!");


    use std::sync::{Arc, Mutex};
    let data = Arc::new(Mutex::new(vec![1u32, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }
}
