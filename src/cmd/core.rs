/// Core behavior for the application
///
use anyhow::{anyhow, Error};
use clap::{arg, Arg, ArgMatches, Command};
use std::fs;
use crate::Args;
use crate::types::{Hostssource};
use futures::executor::block_on;

// Clean command, no subcommand implementation
pub fn execute(args: Args) -> Result<(), Error> {
    println!("You fell through to 'core'.");
    println!("{:?}", args);

    // step 1: load the mainhosts
    let mut mainhosts = Hostssource {
        ..Default::default()
    };
    block_on(mainhosts.load(&args.mainhosts));
    println!("{}", mainhosts);
    //  return Err(anyhow!("Some error"));

    // Err(anyhow!("Some error"))
    Ok(())
}
