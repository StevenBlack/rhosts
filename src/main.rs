/// Messing with hosts files
#[macro_use]
extern crate clap;

use anyhow::anyhow;
use anyhow::{Context, Error};
use chrono::Local;
use clap::{AppSettings, Arg, ArgMatches, Command, Parser, Subcommand};
use clap_complete::Shell;
use config::get_shortcuts;
use std::env;
use std::io::Write;

mod cmd;
mod config;
mod types;
mod utils;

#[derive(Debug, Default, Parser)]
#[clap(author, version, about, long_about = None)]
#[deny(missing_docs)]

/// All the app settings
/// Very useful: https://github.com/clap-rs/clap/tree/master/examples/tutorial_derive
pub struct Arguments {
    /// The main hosts file, the basis for comparison.
    #[clap(short, long = "main", default_value = "base")]
    mainhosts: String,

    /// The hosts file to compare to mainhosts.
    #[clap(short, long = "compare")]
    comparehosts: Option<String>,

    /// The ip address to use for hosts
    #[clap(long = "ip", default_value = "0.0.0.0")]
    iplocalhost: String,

    /// Add default hosts assigments
    #[clap(short = 'd', long = "default_hosts")]
    adddefaults: Option<bool>,

    /// Sort the domains.
    #[clap(short = 's', long = "sort")]
    alpha_sort: Option<bool>,

    /// Print the domains to std out.
    #[clap(short, long)]
    output: Option<bool>,

    /// Domains with no IP addresses.
    #[clap(short = 'p', long = "plain")]
    plain_output: Option<bool>,

    /// Print the domains to std out.
    #[clap(long)]
    stats: Option<bool>,

    /// Print the intersection of lists.
    #[clap(long)]
    intersection_list: Option<bool>,

    /// Print top level domain tallies.
    #[clap(long)]
    tld: Option<bool>,

    /// Omit the file comment headers in output.
    #[clap(long)]
    noheader: Option<bool>,

    /// Use the contents of the system clipboard as compare hosts.
    #[clap(long = "clip")]
    sysclipboard: bool,

    /// List the unique domain names
    #[clap(short, long = "unique")]
    uniquelist: Option<bool>,

    root: Option<bool>,

    #[clap(subcommand)]
    action: Option<Action>,

    /// Skip any cache
    #[clap(long = "nocache")]
    nocache: bool,
}

impl Arguments {
    pub fn new() -> Arguments {
        // Special code goes here ...
        let mut shortcuts = get_shortcuts();
        let mut d = Arguments {
            mainhosts: shortcuts.get("base").unwrap().to_owned(),
            iplocalhost: "0.0.0.0".to_string(),
            stats: Some(true),
            nocache: false,
            ..Default::default()
        };
        d
    }
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Build hosts files
    Build {
        #[clap(short, long)]
        /// The formula to build
        formula: Option<String>,
    },
    /// Cache hosts files
    Cache {
        #[clap(short, long)]
        /// Prime or refresh the cache
        prime: bool,

        #[clap(short, long)]
        /// Clear the cache
        clear: bool,
    },
    /// Initialize cache and templates
    Init,
}

#[test]
fn test_args() {
    let d = Arguments::new();
    assert_eq!(d.mainhosts, get_shortcuts().get("base").unwrap().to_owned());
    assert_eq!(d.comparehosts, None);
    assert_eq!(d.iplocalhost, "0.0.0.0".to_string());
    assert_eq!(d.tld, None);
    assert_eq!(d.stats, Some(true));
}

fn main() {
    let args = Arguments::parse();

    // Check which subcomamnd the user specified, if any...
    let res = match &args.action {
        Some(Action::Init) => cmd::init::execute(args),
        Some(Action::Build { formula: _ }) => cmd::build::execute(args),
        Some(Action::Cache { prime: _, clear: _ }) => cmd::cache::execute(args),
        None => cmd::core::execute(args),
        _ => unreachable!(),
    };

    if let Err(e) = res {
        println!("Error {:?}", e);
        std::process::exit(101);
    }

    println!("The run is done.");
}
