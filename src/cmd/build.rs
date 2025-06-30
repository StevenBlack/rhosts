use crate::{types::Amalgam, Arguments};
use anyhow::Error;
use std::fs::File;
use std::io::{self, Write};
use std::boxed::Box;

// Build command implementation
// This is all very experimental right now.
pub async fn execute(args: Arguments) -> Result<(), Error> {
    // Choose output: file or stdout
    let mut writer: Box<dyn Write> = if let Some(ref path) = args.output {
        Box::new(File::create(path)?)
    } else {
        Box::new(io::stdout())
    };

    if args.verbose {
        println!("Handled by 'build'.");
    }
    let amalgam = Amalgam::new(vec!(args.mainhosts)).await;

    if args.domains_sort {
        let sorteddomains = amalgam.sorteddomains();
        for domain in sorteddomains {
            if args.plain_output {
                writeln!(writer, "{}", domain)?;
            } else {
                writeln!(writer, "{} {}", args.iplocalhost, domain)?;
            }
        }
        return Ok(());
    }
    for domain in amalgam.domains {
        writeln!(writer, "{}", domain)?;
    }

    Ok(())
}
