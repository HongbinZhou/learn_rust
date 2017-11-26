#![feature(attr_literals)]

extern crate structopt;

#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;


#[derive(StructOpt)]
#[structopt(name = "make-cookie")]
struct MakeCookie {
    #[structopt(name = "supervisor", default_value = "Puck", required = false, long = "supervisor")]
    supervising_faerie: String,
    #[structopt(name = "tree")]
    /// The faerie tree this cookie is being made in.
    tree: Option<String>,
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    cmd: Command
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "pound")]
    /// Pound acorns into flour for cookie dough.
    Pound {
        acorns: u32
    },
    #[structopt(name = "sparkle")]
    /// Add magical sparkles -- the secret ingredient!
    Sparkle {
        #[structopt(short = "m")]
        magicality: u64,
        #[structopt(short = "c")]
        color: String
    },
    #[structopt(name = "finish")]
    Finish {
        #[structopt(short = "t")]
        time: u32,
        #[structopt(subcommand)]  // Note that we mark a field as a subcommand
        finishtype: FinishType
    }
}

#[derive(StructOpt)]
enum FinishType {
    #[structopt(name = "glaze")]
    Glaze {
        applications: u32
    },
    #[structopt(name = "powder")]
    Powder {
        flavor: String,
        dips: u32
    }
}

fn main() {
    let opt = MakeCookie::from_args();
    //println!("{:?}", opt);
}