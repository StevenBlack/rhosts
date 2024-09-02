use crate::{config::get_sources_by_tag, types::Amalgam, Arguments};
use anyhow::Error;

// Build command implementation
pub async fn execute(args: Arguments) -> Result<(), Error> {
    if args.verbose {
        println!("Handled by 'build'.");
    }

    // for now, let's just build the base list
    buildproduct("base".to_string()).await;
    Ok(())
}

pub async fn buildproduct(target: String)  {
    let sources = get_sources_by_tag(target);
    let mut myvec = vec!();
    for s in sources.clone() {
        myvec.push(s.name);
    }
    let amalgam = Amalgam::new(myvec).await;
    println!("{}", amalgam.domains.len());
}
