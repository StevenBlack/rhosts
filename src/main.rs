//! rhosts (rh) is a CLI for messing with hosts files

extern crate clap;
use anyhow::Error;
use clap::{Parser, Subcommand};
use config::get_shortcuts;

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
    #[clap(
        short,
        long = "main",
        default_value = "base",
        help = "The main hosts file, the basis for comparison"
    )]
    mainhosts: String,


    #[clap(
        short,
        long = "compare",
        help = "The hosts file to compare to mainhosts"
    )]
    comparehosts: Option<String>,

    #[clap(
        long = "isolate",
        help = "The hosts list to isolate and compare to mainhosts"
    )]
    isolate: Option<String>,

    #[clap(
        long = "ip",
        default_value = "0.0.0.0",
        help = "The ip address to use when listing hosts"
    )]
    iplocalhost: String,

    #[clap(
        short = 'd',
        long = "default_hosts",
        help = "Add default hosts for when listing hosts. The default hosts will be placed at the top of hosts lists"

    )]
    adddefaults: bool,

    #[clap(
        short = 's',
        long = "sort",
        help = "Sort the domains. The sort order is domain, tdl, subdomain1, subdomain2, etc"
    )]
    domains_sort: bool,

    #[clap(
        short,
        long,
        help = "The output file. By default, output is to std out"
    )]
    output: Option<String>,

    #[clap(
        short = 'p',
        long = "plain",
        help = "Plain listing - domains only, without addresses, when listing domains"
    )]
    plain_output: bool,

    ///
    #[clap(
        short,
        long,
        help = "Quiet, terse output mode. Outputs the number of domains only"
    )]
    quiet: bool,

    #[clap(long, help = "Print statistics about the domains")]
    stats: Option<bool>,

    #[clap(
        short,
        long = "intersection",
        help = "Print the intersection of mainhosts and comparehosts"
    )]
    intersection_list: bool,

    #[clap(
        short,
        long,
        help = "List of root domains and their tally"
    )]
    rootdomains: bool,

    #[clap(
        short,
        long,
        help = "Print a tally of top level domains found in the list"
    )]
    tld: bool,

    #[clap(
        short,
        long,
        default_value = "30",
        help = "Limit for listing TLD and root domains, 0 = unlimited"
    )]
    limit: usize,

    #[clap(long, help = "Omit the file comment headers in output")]
    skipheaders: bool,

    #[clap(long, help = "List duplicates when reporting on a hosts list")]
    showduplicates: bool,

    #[clap(long = "invalid", help = "List invalid domains when reporting on a hosts list")]
    showinvalids: bool,

    #[clap(long = "clip", help = "Use the contents of the system clipboard as compare hosts")]
    sysclipboard: bool,

    #[clap(short, long = "unique", help = "List the unique domain names")]
    uniquelist: bool,

    #[clap(short, long = "verbose", help = "Verbose output, useful for development")]
    verbose: bool,

    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(long = "skipcache", help = "Do not use cache")]
    skipcache: bool,
}

impl Arguments {
    pub fn new() -> Arguments {
        // Special code goes here ...
        let shortcuts = get_shortcuts();
        let d = Arguments {
            mainhosts: shortcuts
                .get("base")
                .expect("The base key is not defined.")
                .to_owned(),
            iplocalhost: "0.0.0.0".to_string(),
            stats: Some(true),
            skipcache: false,
            ..Default::default()
        };
        d
    }
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Build hosts files
    Build {
        #[clap(short, long)]
        /// The formula to build
        formula: Option<String>,
    },
    /// Application cache initialize, prime, clear, or report.
    Cache {
        /// Cache subcommand
        #[clap(subcommand)]
        cacheaction: Option<cmd::cache::CacheAction>,
    },
    /// Initialize cache and templates
    Init,
    /// Display additional information about the application
    Info,
}

#[test]
fn test_args() {
    let d = Arguments::new();
    assert_eq!(
        d.mainhosts,
        get_shortcuts()
            .get("base")
            .expect("The base key does not exist")
            .to_owned()
    );
    assert_eq!(d.comparehosts, None);
    assert_eq!(d.iplocalhost, "0.0.0.0".to_string());
    assert_eq!(d.tld, false);
    assert_eq!(d.stats, Some(true));
}

fn show_info(args:Arguments) -> Result<(), Error> {
    println!("");
    println!("{}",format!("{:-^1$}", " info dump ", 40));
    println!("rh version: {}", env!("CARGO_PKG_VERSION"));
    println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    println!("");
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!("");
    _ = config::info(args.clone());
    println!("");
    _ = cmd::cache::info(args.clone());
    println!("");
    _ = cmd::core::info(args.clone());
    println!("");
    println!("{}",format!("{:-^1$}", "", 40));
    println!("");

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let args = Arguments::parse();
    config::init(args.clone())?;
    cmd::cache::init(args.clone())?;

    // Check which subcomamnd the user specified, if any...
    let res = match &args.command {
        None => cmd::core::execute(args),
        Some(Commands::Init) => cmd::init::execute(args),
        Some(Commands::Build { formula: _ }) => cmd::build::execute(args).await,
        Some(Commands::Cache { cacheaction: _ }) => cmd::cache::execute(args),
        Some(Commands::Info) => {show_info(args)},
    };

    if let Err(e) = res {
        println!("Error {:?}", e);
        std::process::exit(101);
    }
    Ok(())
}
