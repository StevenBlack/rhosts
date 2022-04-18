use anyhow::{Error};
use clap::{arg, Arg, ArgMatches, Command};
use std::fs;

// Create clap subcommand arguments
pub fn make_subcommand<'help>() -> Command<'help> {
    Command::new("clean")
        .about("Deletes local cache")
        .arg(
            Arg::new("cache-dir")
                .short('d')
                .long("cache-dir")
                .value_name("cache-dir")
                .help(
                    "Directory for local copy of data{n}\
                    Relative paths are interpreted relative to the app's root directory.{n}\
                    If omitted, rhosts uses build.cache-dir from rhosts.toml or defaults to `./rhosts`.",
                ),
        )
        .arg(arg!([dir]
            "Root directory for the data{n}\
            (Defaults to the Current Directory when omitted)"
        ))
}

// Clean command implementation
pub fn execute(args: &ArgMatches) -> Result<(), Error> {
    println!("You selected 'clean'.");
    Ok(())
}
