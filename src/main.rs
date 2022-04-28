/// Messing with hosts files
#[macro_use]
extern crate clap;

use anyhow::anyhow;
use anyhow::{Context, Error};
use chrono::Local;
use clap_complete::Shell;
use clap::{AppSettings, Arg, ArgMatches, Command, Parser};
use std::env;
use std::io::Write;

use config::get_shortcuts;

mod config;
mod cmd;
mod types;
mod utils;

#[derive(Debug, Default, Parser)]
#[clap(author, version, about, long_about = None)]
#[deny(missing_docs)]

/// All the app settings
pub struct Args {
    /// The main hosts file, the basis for comparison.
    #[clap(short, long="main")]
    mainhosts: String,

    /// The hosts file to compare to mainhosts.
    #[clap(short, long="compare")]
    comparehosts: Option<String>,

    /// The ip address to use for hosts
    #[clap(long="ip", default_value="0.0.0.0")]
    iplocalhost: String,

    /// Add default hosts assigments
    #[clap(short='d', long="default_hosts")]
    adddefaults: Option<bool>,

    /// Sort the domains.
    #[clap(short='s', long="sort")]
    alpha_sort: Option<bool>,

    /// Print the domains to std out.
    #[clap(short, long)]
    output: Option<bool>,

    /// Domains with no IP addresses.
    #[clap(short='p', long="plain")]
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
    #[clap(long="clip")]
    sysclipboard: Option<bool>,

    /// List the unique domain names
    #[clap(short, long="unique")]
    uniquelist: Option<bool>,

    root: Option<bool>,
}

impl Args {
    pub fn new() -> Args {
        // Special code goes here ...
        let mut shortcuts = get_shortcuts();
        let mut d = Args {
          mainhosts: shortcuts.get("base").unwrap().to_owned(),
          iplocalhost: "0.0.0.0".to_string(),
          stats: Some(true),
          ..Default::default()
        };
        d
    }
}

#[test]
fn test_args() {
    let d = Args::new();
    assert_eq!(d.mainhosts, get_shortcuts().get("base").unwrap().to_owned());
    assert_eq!(d.comparehosts, None);
    assert_eq!(d.iplocalhost, "0.0.0.0".to_string());
    assert_eq!(d.tld, None);
    assert_eq!(d.stats, Some(true));
}

fn main() {

    // let args = Args::parse();

    let app = create_clap_app();

println!("{:?}", create_clap_app().get_matches());

    // Check which subcomamnd the user ran...
    let res = match app.get_matches().subcommand() {
        Some(("init", sub_matches)) => cmd::init::execute(sub_matches),
        Some(("build", sub_matches)) => cmd::build::execute(sub_matches),
        Some(("clean", sub_matches)) => cmd::clean::execute(sub_matches),
        None => cmd::core::execute(),
        _ => unreachable!(),
    };

    if let Err(e) = res {
        std::process::exit(101);
    }
    println!("we are here!");
}

/// Create a list of valid arguments and sub-commands
fn create_clap_app() -> Command<'static> {
    let app = Command::new(crate_name!())
        .about(crate_description!())
        .author("Steven Black <rhosts@sbc.io>")
        .author(crate_authors!())
        .version(crate_version!())
        // .setting(AppSettings::PropagateVersion)
        // .setting(AppSettings::ArgRequiredElseHelp)
        .after_help(
            "For more information about a specific command, try `rhosts <command> --help`\n\
             The source code for rhosts is available at: https://github.com/StevenBlack/rhosts",
        )
        .subcommand(cmd::init::make_subcommand())
        .subcommand(cmd::build::make_subcommand())
        .subcommand(cmd::clean::make_subcommand())
        .subcommand(cmd::core::make_subcommand());

    // Base (main) hosts file
    let main_option = Arg::new("main")
        .long("main") // allow --name
        .short('m')
        .takes_value(true)
        .help("the main hosts file")
        .required(false);

    // now add in the argument we want to parse
    let app = app.arg(main_option);


    // Hosts file to compare
    let compare_option = Arg::new("compare")
        .long("compare") // allow --name
        .short('c')
        .takes_value(true)
        .help("hosts file to compare to")
        .required(false);

    // now add in the argument we want to parse
    let app = app.arg(compare_option);

    app
}
