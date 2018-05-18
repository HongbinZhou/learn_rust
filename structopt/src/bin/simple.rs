#![feature(attr_literals)]

extern crate structopt;

#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "simple")]
struct Simple {
   i_want_a_u64: u64,
    #[structopt(short="v", long="verbose", parse(from_occurrences))]
   num_of_repetitions_of_this_parameter: u64,
}



fn main() {
    let opt = Simple::from_args();
    let max_u32 = 2_u64.pow(32);
    println!("2.pow(32) = {}", max_u32);
    println!("{:?}", opt);
}
