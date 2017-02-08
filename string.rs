fn main() {
    let setence = "my is 今天！";
    for i in setence.chars() {
        println!("{:?}", i);
    }
    for i in setence.as_bytes() {
        println!("{:?}", i);
    }

    let ref_setence = &setence[0..30];
    // println!("{:?}", ref_setence[0]);

}
