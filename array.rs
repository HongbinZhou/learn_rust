use std::mem;

fn main() {
    let arr = [1,2,3,4,5];
    println!("{:?}", arr);
    println!("{:?}",  arr.len());
    println!("size of val: {:?}",  mem::size_of_val(&arr));
    println!("0: {}", arr[0]);
    println!("5: {}", arr[5]);

}
