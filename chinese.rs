fn main() {
    let str = "今天";
    let bytes = str.as_bytes();
    println!("str: {}", str);
    println!("bytes: {:?}", bytes);
    bytes
        .iter()
        .enumerate()
        .for_each(|(i, x)| println!("bytes[{}]=0o{:o}", i, x))
}
