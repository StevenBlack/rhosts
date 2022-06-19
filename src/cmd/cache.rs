use crate::{Action, Arguments};
use anyhow::{Error};
use clap::{arg, Arg, ArgMatches, Command};
use std::fs;

// Cache command implementation
pub fn initcache() {
    println!("Initializing cache.");
}

pub fn execute(args: Arguments) -> Result<(), Error> {
    println!("You selected 'cache'.");
    println!("{:?}", args);

    match &args.action {
        Some(Action::Cache { prime: _, clear: true }) => {
            clearcache();
        },

        Some(Action::Cache { prime: true, clear: _ }) => {
            primecache();
        },
        _ => {
            reportcache();
        }
    };

    Ok(())
}

fn clearcache() {
    println!("Clearing cache.");
}

fn primecache() {
    println!("Priming cache.");
}

fn reportcache() {
    println!("Reporting cache.");
}

