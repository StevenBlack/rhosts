use crate::cmd::cache::get_cache_dir;
use crate::config::get_config_file;
use crate::types::Hostssource;
use crate::Arguments;
/// Core behavior for the application
///
use anyhow::Error;
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
    block_on(mainhosts.load(&args.mainhosts));
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
        block_on(comparehosts.load(&clipboard_text));
        println!("{}", comparehosts);
        intersection(mainhosts, comparehosts)?;
    } else if args.comparehosts.is_some() {
        let mut comparehosts = Hostssource {
            args: args.clone(),
            ..Default::default()
        };
        block_on(comparehosts.load(&args.comparehosts.unwrap()));
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
pub fn dump(args: Arguments) {
    println!("");
    // println!("===================");
    // println!("Configuration dump");
    println!("{}", format!("{:-^1$}", " configuration dump ", 40));
    println!("{:?}", args);
    println!("");
    if let Ok(f) = get_config_file() {
        println!("Configuration file: {:?}", f);
    } else {
        println!("Config file problem.");
    }
    println!("");
    println!("Cache folder: {:?}", get_cache_dir());
    println!("{}", format!("{:-^1$}", "", 40));
    println!("");
}
