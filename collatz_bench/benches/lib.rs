// #![feature(concat_idents)]

#![feature(test)]
extern crate test;

extern crate collatz_bench;

use collatz_bench::collatz;
use collatz_bench::collatz_c;

use test::Bencher;

// #[macro_use]
// extern crate lazy_static;
// use std::collections::HashMap;

// failed to write tests with macro...
// see concat_idents! issues: https://github.com/rust-lang/rust/issues/29599
//
// type CollatzFunc = fn(i32) -> i32;
// lazy_static! {
//     static ref FUNC_MAP: HashMap<&'static str, CollatzFunc> = {
//         let mut m: HashMap<&'static str, CollatzFunc> = HashMap::new();
//         m.insert("collatz_c", collatz_c::collatz);
//         m.insert("collatz_rs", collatz::collatz);
//         m
//     };
// }

// macro_rules! test_collatz_c {
//     ($func:ident, $n:expr) => {
//         #[bench]
//         fn $func_$n(b: &mut Bencher) {
//             b.iter(|| FUNC_MAP.get(stringify!($func)).unwrap()($n))
//         }
//     }
// }

// test_collatz_c!(collatz_c, 2223);
// test_collatz_c!(collatz_rs, 2223);

// test_collatz_c!(collatz_c, 10971);
// test_collatz_c!(collatz_rs, 10971);

// test_collatz_c!(collatz_c, 106239);
// test_collatz_c!(collatz_rs, 106239);


#[bench]
fn test_collatz_c_2223(b: &mut Bencher) {
    b.iter(|| collatz_c::collatz(2223));
}

#[bench]
fn test_collatz_rs_2223(b: &mut Bencher) {
    b.iter(|| collatz::collatz(2223));
}
#[bench]
fn test_collatz_c_10971(b: &mut Bencher) {
    b.iter(|| collatz_c::collatz(10971));
}

#[bench]
fn test_collatz_rs_10971(b: &mut Bencher) {
    b.iter(|| collatz::collatz(10971));
}
#[bench]
fn test_collatz_c_106239(b: &mut Bencher) {
    b.iter(|| collatz_c::collatz(106239));
}

#[bench]
fn test_collatz_rs_106239(b: &mut Bencher) {
    b.iter(|| collatz::collatz(106239));
}
