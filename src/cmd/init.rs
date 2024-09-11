use crate::Arguments;
use anyhow::Error;

// Init command implementation
pub fn execute(args: Arguments) -> Result<(), Error> {
    if args.verbose {
        println!("Handled by 'init'.");
    }
    // for now, prime the cache
    crate::cmd::cache::prime(args.clone())?;
    Ok(())
}
