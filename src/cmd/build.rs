use anyhow::{Context, Error};
use clap::{arg, Arg, ArgMatches, Command};
use crate::Arguments;

// Build command implementation
pub fn execute(args: Arguments) -> Result<(), Error> {
    println!("You selected 'build'.");
    println!("{:?}", args);
    Ok(())
}
