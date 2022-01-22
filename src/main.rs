use futures::executor::block_on;

mod utils;
mod types;

use crate::types::{Hostssource};
use utils::{sep};

fn main() {
    let mut hf1 = Hostssource{
        ..Default::default()
    };

    block_on(hf1.load("/Users/Steve/Dropbox/dev/hosts/hosts"));

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
