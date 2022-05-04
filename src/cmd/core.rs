use anyhow::{Error};
use clap::{arg, Arg, ArgMatches, Command};
use std::fs;
use crate::Args;


// Clean command implementation
pub fn execute(args: Args) -> Result<(), Error> {
    println!("You fell through to 'core'.");
    println!("{:?}", args);
    Ok(())
}
