fn frob<'a, 'abcd>(s: &'a str, t: &'abcd str) -> &'abcd str {
    println!("a:{}, b:{}", s, t);
    t
}
fn main() {
    let a = "aaa";
    let b = "bbb";
    frob(&a, &b);
}
