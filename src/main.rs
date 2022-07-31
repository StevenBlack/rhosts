/// Messing with hosts files
extern crate clap;

use anyhow::{Error};
use clap::{Parser, Subcommand};
use config::get_shortcuts;

use crate::cmd::cache::initcache;

mod cmd;
mod config;
mod types;
mod utils;

#[derive(Debug, Default, Parser)]
#[clap(author, version, about, long_about = None)]
#[deny(missing_docs)]

/// Tools to mess with hosts file
#[derive(Clone)]
pub struct Arguments {
    /// The main hosts file, the basis for comparison
    #[clap(short, long = "main", default_value = "base")]
    mainhosts: String,

    /// The hosts file to compare to mainhosts
    #[clap(short, long = "compare")]
    comparehosts: Option<String>,

    /// The ip address to use when listing hosts
    #[clap(long = "ip", default_value = "0.0.0.0")]
    iplocalhost: String,

    /// Add default hosts to when listing hosts
    /// The default hosts will be placed at the top of hosts lists
    #[clap(short = 'd', long = "default_hosts")]
    adddefaults: Option<bool>,

    /// Sort the domains
    /// The sort order is domain, tdl, subdomain1, subdomain2, etc
    #[clap(short = 's', long = "sort")]
    alpha_sort: Option<bool>,

    /// The output file
    /// Otherwise, by default, output is to std out
    #[clap(short, long)]
    output: Option<bool>,

    /// Domains only, without addresses, when listing domains
    #[clap(short = 'p', long = "plain")]
    plain_output: Option<bool>,

    /// Quiet, terse output mode
    /// Outputs the number of domains only
    #[clap(short, long)]
    quiet: bool,

    /// Print statistics about the domaons
    #[clap(long)]
    stats: Option<bool>,

    /// Print the intersection of mainhosts and comparehosts
    #[clap(long)]
    intersection_list: Option<bool>,

    /// Print a tally of top level domains found in the list
    #[clap(long)]
    tld: Option<bool>,

    /// Omit the file comment headers in output
    #[clap(long)]
    noheader: Option<bool>,

    /// Use the contents of the system clipboard as compare hosts
    #[clap(long = "clip")]
    sysclipboard: bool,

    /// List the unique domain names
    #[clap(short, long = "unique")]
    uniquelist: Option<bool>,

     /// Verbose output, useful for development
    #[clap(short, long = "verbose")]
    verbose: bool,

    root: Option<bool>,

    #[clap(subcommand)]
    action: Option<Action>,

    /// Do not use cache
    #[clap(long = "nocache")]
    nocache: bool,

    #[clap(long)]
    dump: bool,
}

impl Arguments {
    pub fn new() -> Arguments {
        // Special code goes here ...
        let shortcuts = get_shortcuts();
        let d = Arguments {
            mainhosts: shortcuts.get("base").expect("The base key is not defined.").to_owned(),
            iplocalhost: "0.0.0.0".to_string(),
            stats: Some(true),
            nocache: false,
            ..Default::default()
        };
        d
    }
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    /// Build hosts files
    Build {
        #[clap(short, long)]
        /// The formula to build
        formula: Option<String>,
    },
    /// Application cache initialize, prime, clear, or report.
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
    assert_eq!(d.mainhosts, get_shortcuts().get("base").expect("The base key does not exist").to_owned());
    assert_eq!(d.comparehosts, None);
    assert_eq!(d.iplocalhost, "0.0.0.0".to_string());
    assert_eq!(d.tld, None);
    assert_eq!(d.stats, Some(true));
}

#[async_std::main]
async fn main()  -> Result<(), Error> {
    let args = Arguments::parse();
    initcache(args.clone())?;

    if args.dump {
        cmd::core::dump(args.clone());
    }

    // Check which subcomamnd the user specified, if any...
    let res = match &args.action {
        None => cmd::core::execute(args.clone()),
        Some(Action::Init) => cmd::init::execute(args.clone()),
        Some(Action::Build { formula: _ }) => cmd::build::execute(args.clone()),
        Some(Action::Cache { prime: _, clear: _ }) => cmd::cache::execute(args.clone()),
    };

    if let Err(e) = res {
        println!("Error {:?}", e);
        std::process::exit(101);
    }
    Ok(())
}
