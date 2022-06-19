use crate::Arguments;
use anyhow::{Error};
use clap::{arg, Arg, ArgMatches, Command};
use std::fs;

// Cache command implementation
pub fn execute(args: Arguments) -> Result<(), Error> {
    println!("You selected 'cache'.");
    println!("{:?}", args);
    Ok(())
}
