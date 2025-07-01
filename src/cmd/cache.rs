//! Cache related sommands and services
//!

// #![allow(dead_code)]
use anyhow::{bail, anyhow};
use async_std::println;
use crate::{Commands, Arguments, config::get_shortcuts, types::Hostssource, utils::hash};
use clap::Subcommand;
use anyhow::Context;
use directories::ProjectDirs;
use futures::executor::block_on;
use std::{
    fs::{self, File},
    io::prelude::*,
    path::{Path,PathBuf}
};

#[allow(dead_code)]
#[derive(Hash)]
/// Enum containing the possible cacheable types
pub enum Cacheable {
    Vec(Vec<String>),
    String(String),
}

#[derive(Clone, Debug, Subcommand)]
/// Enum containing the possible actions for the `cache` subcommand.
pub enum CacheCommands {
    /// clean the cache
    Clear,
    /// Prime the cache.
    Prime,
    /// Report on the cache
    Report,
    /// Information about the cache
    Info,
}

/// Display information about the application cache.
pub async fn info(_args:Arguments) -> anyhow::Result<()> {
    let cache_dir = get_cache_dir().await;
    println!("Cache information:").await;
    println!("Local cache folder: {}", cache_dir.display()).await;
    Ok(())
}

/// Initialize the application cache.
pub async fn init(args:Arguments) -> anyhow::Result<()> {
    let cache_dir = get_cache_dir().await;
    if !Path::new(&cache_dir).is_dir() {
        if args.verbose {
            println!("Initializing empty cache.").await;
        }
        fs::create_dir_all(cache_dir)?;
    }
    Ok(())
}

/// Get cached item from the application cache.
pub async fn get(s: String) -> Option<PathBuf> {
    let pb = get_cache_dir().await.join(get_cache_key(Cacheable::String(s)));
    if pb.is_file() {
        Some(pb)
    } else {
        None
    }
}

/// Set cached item in the application cache.
pub async fn set(file: String, body: String) -> anyhow::Result<()> {
    let mut output = File::create(get_cache_dir().await.join(get_cache_key(Cacheable::String(file)))).expect("Unable to cache HTTP request result.");
    if write!(output, "{}", body).is_ok() {
        Ok(())
    } else {
        Err(anyhow!("Unable to cache HTTP request result."))
    }
}

/// Deletes all cache data.
pub async fn delete(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Deleting cache.").await;
    }
    fs::remove_dir_all(get_cache_dir().await)?;
    Ok(())
}

/// Get the cache directory.
pub async fn execute(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Handled by 'cache'.").await;
        _ = info(args.clone());
    }

    match &args.command {
        Some(Commands::Cache { cacheaction: Some(CacheCommands::Clear) }) => {
            clear(args.clone()).await?;
        },
        Some(Commands::Cache { cacheaction: Some(CacheCommands::Prime) }) => {
            prime(args.clone()).await?;
        },
        Some(Commands::Cache { cacheaction: Some(CacheCommands::Report) }) => {
            report(args.clone()).await?;
        },
        Some(Commands::Cache { cacheaction: Some(CacheCommands::Info) }) => {
            info(args.clone()).await?;
        },
        _ => {
            bail!("No such cache subcommand.");
        }
    };
    Ok(())
}

/// Delete and reinitialize cache
async fn clear(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Clearing cache.").await;
    }
    delete(args.clone()).await.context(format!("unable to delete cache"))?;
    init(args.clone()).await.context(format!("Unable to initialize cache"))?;
    Ok(())
}

/// Prime all caches
pub(crate) async fn prime(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Priming cache.").await;
    }
    clear(args.clone()).await.context(format!("unable to delete cache"))?;
    let mut shortcuts: Vec<String> = get_shortcuts().into_values().collect();
    shortcuts.dedup();
    for shortcut in shortcuts {
        if args.verbose {
            println!("Priming {}", shortcut.to_owned()).await;
        }
        block_on(Hostssource::new(shortcut.to_owned(), shortcut.to_owned()));
    }
    Ok(())
}

/// Report information about the current state of cache
async fn report(args: Arguments) -> anyhow::Result<()> {
    if args.verbose {
        println!("Reporting cache.").await;
        println!("Arguments received: {:?}", args).await;
    }
    println!("Cache report is to be implemented.").await;
    Ok(())
}

/// Returns the cache folder following the user's OS conventions.
pub async fn get_cache_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "rh").unwrap();
    let cache_dir = proj_dirs.cache_dir();
    if !cache_dir.exists() {
        // create the folder if it does not exists
        let create_dir_result:Result<(), std::io::Error> = fs::create_dir_all(cache_dir);
        if create_dir_result.is_err() {
            async_std::println!("Unable to create cache folder").await;
            panic!();
        }
    }

    // proj_dirs.cache_dir().to_owned()
    cache_dir.to_owned()
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
