use crate::{types::Amalgam, Arguments};
use anyhow::Error;

// Build command implementation
pub async fn execute(args: Arguments) -> Result<(), Error> {
    if args.verbose {
        println!("Handled by 'build'.");
    }

    // for now, let's just build the base list
    // buildproduct("base".to_string()).await;
    // buildproduct("s-only".to_string()).await;
    // buildproduct("p".to_string()).await;
    buildproduct("p-only".to_string()).await;
    // buildproduct("xyz".to_string()).await;
    // buildproduct("fgps".to_string()).await;
    Ok(())
}

pub async fn buildproduct(name: String)  {
    let amalgam = Amalgam::new(vec!(name)).await;
    println!("{:?}", amalgam.domains.len());
}
