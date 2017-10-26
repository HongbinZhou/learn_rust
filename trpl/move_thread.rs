use std::thread;

fn main() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move ||{
        println!("what child see the vector is: {:?}", v);
    });
    // !! compile error if access v again !!
    // println!("v in main: {:?}", v);
    let _ = handle.join();
}
