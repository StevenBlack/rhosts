use crate::{Action, Arguments, config::get_shortcuts, types::Hostssource};
use anyhow::{Context};
use directories::{ProjectDirs};
use futures::executor::block_on;
use std::fs;
use std::path::PathBuf;

/// A function to return the cache folder following user OS conventions.
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

/// A function to create the application cache folder if it doesn't exist
pub fn initcache(args:Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Initializing cache.");
    }

    fs::create_dir_all(get_cache_dir())?;
    Ok(())
}

/// A function that deletes all cache data
pub fn deletecache(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Deleting cache.");
    }
    fs::remove_dir_all(get_cache_dir())?;
    Ok(())
}

pub fn execute(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Handled by 'cache'.");
    }

    match &args.action {
        Some(Action::Cache { prime: _, clear: true }) => {
            clearcache(args.clone())?;
        },
        Some(Action::Cache { prime: true, clear: _ }) => {
            primecache(args.clone())?;
        },
        _ => {
            reportcache(args.clone())?;
        }
    };
    Ok(())
}

/// A function to delete and reinitialize cache
fn clearcache(args: Arguments) -> anyhow::Result<()> {
    deletecache(args.clone()).context(format!("unable to delete cache"))?;
    initcache(args.clone()).context(format!("Unable to initialize cache"))?;
    Ok(())
}

/// A function to prime all caches
fn primecache(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Priming cache.");
    }
    clearcache(args.clone()).context(format!("unable to delete cache"))?;
    let mut shortcuts: Vec<String> = get_shortcuts().into_values().collect();
    shortcuts.dedup();
    for shortcut in shortcuts {
        block_on(Hostssource::new(shortcut.to_owned(), shortcut.to_owned()));
    }
    Ok(())
}

/// A function to report information about the current state of cache
fn reportcache(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Reporting cache.");
    }
    println!("Cache report is to be implemented.");
    Ok(())
}
