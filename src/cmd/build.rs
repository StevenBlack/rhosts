use crate::{types::Amalgam, Arguments};
use anyhow::Error;

// Build command implementation
// This is all very experimental right now.
pub async fn execute(args: Arguments) -> Result<(), Error> {
    if args.verbose {
        println!("Handled by 'build'.");
    }
    let amalgam = Amalgam::new(vec!(args.mainhosts)).await;



    if args.domains_sort {
        let sorteddomains = amalgam.sorteddomains();
        for domain in sorteddomains {
            if args.plain_output {
                println!("{}", domain);
            } else {
                println!("{} {}", args.iplocalhost, domain);
            }

        }
        return Ok(());

    }
    for domain in amalgam.domains {
        println!("{}", domain);
    }

    Ok(())
}
