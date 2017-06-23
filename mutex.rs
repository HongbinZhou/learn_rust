// example from book: Mastering Rust

// mutex-arc.rs
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
use std::time;
const THREADS: u64 = 1_00_00_00;
const START_NUMBER: u64 = 1;
fn main() {
    let one_millisecond = time::Duration::from_millis(1);
    let one_second = time::Duration::from_millis(1000);
    let mutexed_number = Arc::new(Mutex::new(START_NUMBER));

    // Invoking clone on Arc produces a new pointer to the same value in the
    // heap, just increasing the reference count.
    // https://doc.rust-lang.org/std/sync/struct.Arc.html
    let mutexed_number_2 = mutexed_number.clone();

    // use the firs thread as a container to open up other many many threads,
    // otherwise, the last checking thread will be blocked until all threads
    // spawned.
    thread::spawn(move || for _ in 1..THREADS {
                      let mutexed_number_clone = mutexed_number.clone();
                      thread::spawn(move || {
                                        thread::sleep(one_millisecond);
                                        let mut number = mutexed_number_clone.lock().unwrap();
                                        *number += 1;
                                    });
                  });

    loop {
        thread::sleep(one_second);

        // as mutexed_number has been moved, we have to use the cloned one.
        let number = mutexed_number_2.lock().unwrap();

        if *number != START_NUMBER + THREADS - 1 {
            println!("Not there yet, number is {}", *number);
        } else {
            println!("Got there! Number is {}", *number);
            break;
        }
    }
}
