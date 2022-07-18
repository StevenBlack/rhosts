/// Core behavior for the application
///
use anyhow::{anyhow, Error};
use clap::{arg, Arg, ArgMatches, Command};
use std::fs;
use crate::Arguments;
use crate::cmd::cache::get_cache_dir;
use crate::config::{read_config_file, get_config_file};
use crate::types::{Hostssource};
use futures::executor::block_on;
use arboard::Clipboard;

pub fn execute(args: Arguments) -> Result<(), Error> {
    // If we're here, no subcommand was specified
    if args.verbose {
        println!("You fell through to 'core'.");
        // println!("{:?}", args);
    }

    // step 1: load the mainhosts
    let mut mainhosts = Hostssource {
        ..Default::default()
    };
    block_on(mainhosts.load(&args.mainhosts));
    println!("{}", mainhosts);

    if args.sysclipboard {
        let mut clipboard = Clipboard::new().unwrap();
        let mut comparehosts = Hostssource {
            ..Default::default()
        };
        block_on(comparehosts.load(&clipboard.get_text().unwrap()));
        println!("{}", comparehosts);
    } else if args.comparehosts.is_some() {
        let mut comparehosts = Hostssource {
            ..Default::default()
        };
        block_on(comparehosts.load(&args.comparehosts.unwrap()));
        println!("{}", comparehosts);
    }

    //  return Err(anyhow!("Some error"));

    // Err(anyhow!("Some error"))
    Ok(())
}

/// Dump relavent config information.
pub fn dump(args: Arguments) {
    println!("");
    // println!("===================");
    // println!("Configuration dump");
    println!("{}",format!("{:-^1$}", " configuration dump ", 40));
    println!("{:?}", args);
    println!("");
    if let Ok(f) = get_config_file() {
        println!("Configuration file: {:?}", f);
    } else {
        println!("Config file problem.");
    }
    println!("");
    println!("Cache folder: {:?}", get_cache_dir());
    println!("{}",format!("{:-^1$}", "", 40));
    println!("");
}
