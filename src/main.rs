use clap::Parser;
use futures::executor::block_on;
use std::path::PathBuf;
mod types;
mod utils;

use crate::types::Hostssource;
use utils::sep;

/// Messing with hosts files
#[derive(Debug, Parser)]
#[clap(name = "rhosts")]
struct Args {
    /// The main hosts file
    #[clap(short, long, default_value = "base")]
    main_hosts: String,

    /// The comparison hosts file
    #[clap(short, long, default_value = "")]
    compare_hosts: String,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut hf1 = Hostssource {
        ..Default::default()
    };

    // block_on(hf1.load("/Users/Steve/Dropbox/dev/hosts/hosts"));
    block_on(
        hf1.load(
            "# Header line\n0.0.0.0 example.com\n0.0.0.0 www.example.com\n0.0.0.0 example.com",
        ),
    );
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
        last -= 1;
        if last == 0 {
            break;
        }
    }
    sep(40);
    println!("{:?}", "Domains:");
    last = 100;
    for line in hf1.domains {
        println!("{:?}", line);
        last -= 1;
        if last == 0 {
            break;
        }
    }
    sep(40);
}
