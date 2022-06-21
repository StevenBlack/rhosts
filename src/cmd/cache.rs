use crate::{Action, Arguments, config::get_shortcuts, types::Hostssource};
use anyhow::{Result, Error};
use directories::{ProjectDirs};
use clap::{arg, Arg, ArgMatches, Command};
use std::fs;
use std::path::PathBuf;

pub fn get_cache_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "rhosts").unwrap();
    proj_dirs.cache_dir().to_owned()
}

pub fn get_cache_key(s: String) -> String {
    s
    .replace("https", "")
    .replace("http", "")
    .replace(":", "")
    .replace("//", "")
    .replace("/", "_")
}

// Cache command implementation
pub fn initcache() -> anyhow::Result<()> {
    println!("Initializing cache.");
    fs::create_dir_all(get_cache_dir())?;
    Ok(())
}

pub fn deletecache() -> anyhow::Result<()> {
    println!("Deleting cache.");
    fs::remove_dir_all(get_cache_dir())?;
    Ok(())
}

pub fn execute(args: Arguments) -> anyhow::Result<()> {
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

async fn primecache() {
    println!("Priming cache.");
    clearcache();
    let mut shortcuts: Vec<String> = get_shortcuts().into_values().collect();
    shortcuts.dedup();
    for shortcut in shortcuts {
        let mut hs = Hostssource {
            name: shortcut.to_owned(),
            ..Default::default()
        };
        hs.load(&shortcut).await;
    }
}

fn reportcache() {
    println!("Reporting cache.");
}

