use std::collections::HashSet;

use crate::{config::{get_products_json, get_source_names_by_tag, get_sources_by_tag, Components}, types::Amalgam, Arguments};
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
    // buildproduct("p-only".to_string()).await;
    // buildproduct("xyz".to_string()).await;
    buildproduct("fgps".to_string()).await;
    Ok(())
}

pub async fn buildproduct(name: String)  {
    let json = get_products_json();
    let products: Components = serde_json::from_str(json.as_str()).expect("Invalid JSON in recipe.");

    let mut iter = products.iter();
    let product_spec = iter.find(|x| x.name == name);
    if product_spec.is_none() {
        return ();
    }
    let spec = product_spec.unwrap();
    let tags = spec.tags.clone();
    let mut hs: HashSet<String> = HashSet::new();
    for tag in tags {
        let sources = get_sources_by_tag(tag);
        for s in sources.clone() {
            hs.insert(s.url);
        }
    }
    let amalgam = Amalgam::new(Vec::from_iter(hs)).await;
    println!("{}", amalgam.domains.len());
    for s in amalgam.sources {
        println!("Source {}: {} domains", s.name, s.domains.len());
    }
}
