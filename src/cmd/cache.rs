use crate::{Action, Arguments, config::get_shortcuts};
use anyhow::{Result, Error};
use directories::{BaseDirs, ProjectDirs, UserDirs};
use clap::{arg, Arg, ArgMatches, Command};
use std::fs;

// Cache command implementation
pub fn initcache() -> Result<(), Error> {
    println!("Initializing cache.");
    let proj_dirs = ProjectDirs::from("", "", "rhosts").unwrap();
        let cache_dir = proj_dirs.cache_dir();
        fs::create_dir_all(cache_dir)?;
        Ok(())
}
pub fn deletecache() -> Result<(), Error> {
    println!("Deleting cache.");
    let proj_dirs = ProjectDirs::from("", "", "rhosts").unwrap();
        let cache_dir = proj_dirs.cache_dir();
        fs::remove_dir_all(cache_dir)?;
        Ok(())
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
    deletecache();
    initcache();
}

fn primecache() {
    println!("Priming cache.");
    clearcache();
    let mut shortcuts: Vec<String> = get_shortcuts().into_values().collect();
    shortcuts.dedup();
    println!("{} - {:?}", shortcuts.len(), shortcuts);
}

fn reportcache() {
    println!("Reporting cache.");
}

