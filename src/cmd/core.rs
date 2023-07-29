use crate::cmd::cache::get_cache_dir;
use crate::config::get_config_file;
use crate::types::Hostssource;
use crate::Arguments;
/// Core behavior for the application
///
use anyhow::{Error};
use crate::Arguments;
use crate::types::{Hostssource};
use futures::executor::block_on;
use arboard::Clipboard;
use futures::executor::block_on;

pub fn execute(args: Arguments) -> Result<(), Error> {
    // If we're here, no subcommand was specified
    if args.verbose {
        println!("Handled by 'core'.");
    }

    // step 1: load the mainhosts
    let mut mainhosts = Hostssource {
        args: args.clone(),
        ..Default::default()
    };
    // ignore the result of this load for now
    _ = block_on(mainhosts.load(&args.mainhosts));
    println!("{}", mainhosts);

    if args.sysclipboard {
        let mut clipboard = Clipboard::new().unwrap();
        let clipboard_text = clipboard.get_text().unwrap();
        if args.verbose {
            println!("Clipboard contents:\n{}", clipboard_text);
        }
        let mut comparehosts = Hostssource {
            args: args.clone(),
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(comparehosts.load(&clipboard_text));
        println!("{}", comparehosts);
        intersection(mainhosts, comparehosts)?;
    } else if args.comparehosts.is_some() {
        let mut comparehosts = Hostssource {
            args: args.clone(),
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(comparehosts.load(&args.comparehosts.unwrap()));
        println!("{}", comparehosts);
        intersection(mainhosts, comparehosts)?;
    }

    //  return Err(anyhow!("Some error"));

    // Err(anyhow!("Some error"))
    Ok(())
}

/// Tally the intersection of two domain lists
pub fn intersection(main: Hostssource, comp: Hostssource) -> Result<(), Error> {
    let first = main.domains.len();
    let second = comp.domains.len();
    let mut combined = main.domains.clone();
    combined.append(&mut comp.domains.clone());
    println!("Intersection: {} domains", first + second - combined.len());

    Ok(())
}

/// Dump relavent config information
pub fn info(args: Arguments) {
    println!("Core information:");
    println!("Arguments received: {:?}", args);
}
