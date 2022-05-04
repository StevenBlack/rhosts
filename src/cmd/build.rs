use anyhow::{Context, Error};
use clap::{arg, Arg, ArgMatches, Command};
use crate::Args;

// Build command implementation
pub fn execute(args: Args) -> Result<(), Error> {
    println!("You selected 'build'.");
    println!("{:?}", args);
    Ok(())
}
