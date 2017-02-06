fn main() {
    let f = |&x| x*2;
    let a = vec![1,2,3];
    let a2:Vec<_> = a.iter().map(f).collect();
    println!{"a = {:?}", a};
    println!{"a2 = {:?}", a2};
    println!{"a = {:?}", a};
}
