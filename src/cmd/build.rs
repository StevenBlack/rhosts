use anyhow::{Context, Error};
use clap::{arg, Arg, ArgMatches, Command};
use crate::Arguments;

// Build command implementation
pub fn execute(args: Arguments) -> Result<(), Error> {
    if args.verbose {
        println!("Handled by 'build'.");
    }
    println!("Build is not implemented.");
    Ok(())
}
