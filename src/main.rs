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

mod config;
mod cmd;
mod types;
mod utils;

fn main() {
    let app = create_clap_app();

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
        .subcommand(cmd::clean::make_subcommand());

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
