use anyhow::{Context, Error};
use clap::{arg, Arg, ArgMatches, Command};
use std::io;
use std::io::Write;
use crate::Args;

// Create clap subcommand arguments
pub fn make_subcommand<'help>() -> Command<'help> {
    Command::new("init")
        .about("Creates the boilerplate structure and files for a new amalgamated hosts file")
        // the {n} denotes a newline which will properly aligned in all help messages
        .arg(arg!([dir]
            "Directory to create the hosts in{n}\
            (Defaults to the Current Directory when omitted)"
        ))
        .arg(arg!(--force "Skips confirmation prompts"))
}

// Init command implementation
pub fn execute(args: Args) -> Result<(), Error> {
    println!("You selected 'init'.");
    println!("{:?}", args);
    Ok(())
}
