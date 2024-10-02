use crate::types::{Comparable, Hostssource};
use crate::Arguments;
/// Core behavior for the application
///
use anyhow::Error;
use futures::executor::block_on;
use arboard::Clipboard;
use num_format::{Locale, ToFormattedString};

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

    if args.sysclipboard {
        let mut clipboard = Clipboard::new().unwrap();
        let clipboard_text = clipboard.get_text().unwrap();
        if args.verbose {
            println!("Clipboard contents:\n{}", clipboard_text);
        }
        let mut comparisonhosts = Hostssource {
            args: args.clone(),
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(comparisonhosts.load(&clipboard_text));

        // now, compare the two
        mainhosts.compare(Box::new(comparisonhosts));

    } else if args.comparehosts.is_some() {
        let mut comparisonhosts = Hostssource {
            args: args.clone(),
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(comparisonhosts.load(&args.comparehosts.unwrap()));

        // now, compare the two
        mainhosts.compare(Box::new(comparisonhosts));
    } else {
        println!("{}", mainhosts);
    }

    //  return Err(anyhow!("Some error"));

    // Err(anyhow!("Some error"))
    Ok(())
}

/// Dump relavent config information
pub fn info(args: Arguments) {
    println!("Core information:");
    println!("Arguments received: {:?}", args);
}
