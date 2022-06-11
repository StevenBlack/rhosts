/// Core behavior for the application
///
use anyhow::{anyhow, Error};
use clap::{arg, Arg, ArgMatches, Command};
use std::fs;
use crate::Args;
use crate::types::{Hostssource};
use futures::executor::block_on;
use arboard::Clipboard;

pub fn execute(args: Args) -> Result<(), Error> {
    // If we're here, no subcommand was specified
    println!("You fell through to 'core'.");
    println!("{:?}", args);

    // step 1: load the mainhosts
    let mut mainhosts = Hostssource {
        ..Default::default()
    };
    block_on(mainhosts.load(&args.mainhosts));
    println!("{}", mainhosts);

    if args.sysclipboard {
        let mut clipboard = Clipboard::new().unwrap();
        let mut comparehosts = Hostssource {
            ..Default::default()
        };
        block_on(comparehosts.load(&clipboard.get_text().unwrap()));
        println!("{}", comparehosts);
    } else if args.comparehosts.is_some() {
        let mut comparehosts = Hostssource {
            ..Default::default()
        };
        block_on(comparehosts.load(&args.comparehosts.unwrap()));
        println!("{}", comparehosts);
    }

    //  return Err(anyhow!("Some error"));

    // Err(anyhow!("Some error"))
    Ok(())
}
