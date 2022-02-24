use futures::executor::block_on;
use std::path::PathBuf;
use structopt::StructOpt;
mod utils;
mod types;

use crate::types::{Hostssource};
use utils::{sep};

/// Messing with hosts files
#[derive(StructOpt, Debug)]
#[structopt(name = "rhosts")]
struct Opt {
    /// The main hosts file
    #[structopt(short, long, default_value = "base")]
    mainHosts: String,

    /// The comparison hosts file
    #[structopt(short, long, default_value = "")]
    compareHosts: String,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,
}


fn main() {

    let opt = Opt::from_args();

    let mut hf1 = Hostssource{
        ..Default::default()
    };

    // block_on(hf1.load("/Users/Steve/Dropbox/dev/hosts/hosts"));
    block_on(hf1.load("# Header line\n0.0.0.0 example.com\n0.0.0.0 www.example.com\n0.0.0.0 example.com"));
    sep(40);
    println!("Location: {:?}", hf1.location);

    sep(40);
    println!("{:?}", "File header:");
    for line in hf1.list_header {
        println!("{:?}", line);
    }

    sep(40);
    println!("{:?}", "Raw list:");
    let mut last = 50;
    for line in hf1.raw_list {
        println!("{:?}", line);
        last = last -1;
        if last == 0 {
            break;
        }
    }
    sep(40);
    println!("{:?}", "Domains:");
    last = 100;
    for line in hf1.domains {
        println!("{:?}", line);
        last = last -1;
        if last == 0 {
            break;
        }
    }
    sep(40);
}
