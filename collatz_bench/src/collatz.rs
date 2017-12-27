
pub fn collatz(mut i: i32) -> i32 {
    let mut l = 1;
    while i != 1 {
        i = match i % 2 {
            0 => i / 2,
            _ => 3 * i + 1,
        };
        l += 1;
    }
    l
}
