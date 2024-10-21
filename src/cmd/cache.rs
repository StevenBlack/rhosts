//! Cache related sommands and services
//!

// #![allow(dead_code)]
use anyhow::{bail, anyhow};
use crate::{Action, Arguments, config::get_shortcuts, types::Hostssource, utils::hash};
use clap::Subcommand;
use anyhow::Context;
use directories::ProjectDirs;
use futures::executor::block_on;
use std::{
    fs::{self, File},
    io::prelude::*,
    path::{Path,PathBuf}
};

#[derive(Hash)]
/// Enum containing the possible cacheable types
pub enum Cacheable {
    Vec(Vec<String>),
    String(String),
}

#[derive(Clone, Debug, Subcommand)]
/// Enum containing the possible actions for the `cache` subcommand.
pub enum CacheAction {
    /// clean the cache
    Clear,
    /// Prime the cache
    Prime,
    /// Report on the cache
    Report,
}

/// Display information about the application cache.
pub fn info(_args:Arguments) -> anyhow::Result<()> {
    let cache_dir = get_cache_dir();
    println!("Cache information:");
    println!("Local cache folder: {}", cache_dir.display());
    Ok(())
}

/// Initialize the application cache.
pub fn init(args:Arguments) -> anyhow::Result<()> {
    let cache_dir = get_cache_dir();
    if !Path::new(&cache_dir).is_dir() {
        if args.verbose {
            println!("Initializing empty cache.");
        }
        fs::create_dir_all(cache_dir)?;
    }
    Ok(())
}

/// Get cached item from the application cache.
pub fn get(s: String) -> Option<PathBuf> {
    let pb = get_cache_dir().join(get_cache_key(Cacheable::String(s)));
    if pb.is_file() {
        Some(pb)
    } else {
        None
    }
}

/// Set cached item in the application cache.
pub fn set(file: String, body: String) -> anyhow::Result<()> {
    let mut output = File::create(get_cache_dir().join(get_cache_key(Cacheable::String(file)))).expect("Unable to cache HTTP request result.");
    if write!(output, "{}", body).is_ok() {
        Ok(())
    } else {
        Err(anyhow!("Unable to cache HTTP request result."))
    }
}

/// Deletes all cache data.
pub fn delete(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Deleting cache.");
    }
    fs::remove_dir_all(get_cache_dir())?;
    Ok(())
}

/// Get the cache directory.
pub fn execute(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Handled by 'cache'.");
        _ = info(args.clone());
    }

    match &args.action {
        Some(Action::Cache { cacheaction: Some(CacheAction::Clear) }) => {
            clear(args.clone())?;
        },
        Some(Action::Cache { cacheaction: Some(CacheAction::Prime) }) => {
            prime(args.clone())?;
        },
        Some(Action::Cache { cacheaction: Some(CacheAction::Report) }) => {
            report(args.clone())?;
        },
        _ => {
            bail!("No such cache subcommand.");
        }
    };
    Ok(())
}

/// Delete and reinitialize cache
fn clear(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Clearing cache.");
    }
    delete(args.clone()).context(format!("unable to delete cache"))?;
    init(args.clone()).context(format!("Unable to initialize cache"))?;
    Ok(())
}

/// Prime all caches
pub(crate) fn prime(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Priming cache.");
    }
    clear(args.clone()).context(format!("unable to delete cache"))?;
    let mut shortcuts: Vec<String> = get_shortcuts().into_values().collect();
    shortcuts.dedup();
    for shortcut in shortcuts {
        if args.verbose {
            println!("Priming {}", shortcut.to_owned());
        }
        block_on(Hostssource::new(shortcut.to_owned(), shortcut.to_owned()));
    }
    Ok(())
}

/// Report information about the current state of cache
fn report(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Reporting cache.");
        println!("Arguments received: {:?}", args);
    }
    println!("Cache report is to be implemented.");
    Ok(())
}

/// Returns the cache folder following the user's OS conventions.
pub fn get_cache_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "rh").unwrap();
    proj_dirs.cache_dir().to_owned()
}

/// Returns the hashed cache key.
pub fn get_cache_key(s: Cacheable) -> String {
    match s {
        Cacheable::Vec(v) => {
            let mut mv = v.clone();
            mv.sort();
            hash(mv.join(""))
        }
        Cacheable::String(s) => hash(s),
    }
}
