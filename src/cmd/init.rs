use anyhow::{Context, Error};
use clap::{arg, Arg, ArgMatches, Command};
use std::io;
use std::io::Write;
use crate::Arguments;

// Init command implementation
pub fn execute(args: Arguments) -> Result<(), Error> {
    println!("You selected 'init'.");
    println!("{:?}", args);
    Ok(())
}
