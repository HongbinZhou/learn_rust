fn main() {
    let setence = "my is 今天！";
    for i in setence.chars() {
        print!("{:?}-", i);
    }
    println!("");
    for i in setence.as_bytes() {
        print!("{:?}-", i);
    }

    let ref_setence = &setence[0..30];
    // println!("{:?}", ref_setence[0]);

}
