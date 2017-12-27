extern crate libc;
use self::libc::{c_int};

#[link(name = "collatz", kind = "static")]
extern {
    fn collatz_c(n: c_int) -> c_int;
}

pub fn collatz(n: i32) -> i32 {
    unsafe {
        collatz_c(n)
    }
}
