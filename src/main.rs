/// Messing with hosts files
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use anyhow::anyhow;
use anyhow::{Context, Error};
use chrono::Local;
use clap_complete::Shell;
use clap::{AppSettings, Arg, ArgMatches, Command, Parser};
use env_logger::Builder;
use log::LevelFilter;
use std::env;
use std::io::Write;
mod cmd;
mod types;
mod utils;
const VERSION: &str = concat!("v", crate_version!());

fn main() {
    init_logger();
    let app = create_clap_app();

    // Check which subcomamnd the user ran...
    let res = match app.get_matches().subcommand() {
        Some(("init", sub_matches)) => cmd::init::execute(sub_matches),
        Some(("build", sub_matches)) => cmd::build::execute(sub_matches),
        Some(("clean", sub_matches)) => cmd::clean::execute(sub_matches),
        _ => unreachable!(),
    };

    if let Err(e) = res {
        utils::log_backtrace(&e);

        std::process::exit(101);
    }


}

/// Create a list of valid arguments and sub-commands
fn create_clap_app() -> Command<'static> {
    let app = Command::new(crate_name!())
        .about(crate_description!())
        .author("Mathieu David <mathieudavid@mathieudavid.org>")
        .version(VERSION)
        .setting(AppSettings::PropagateVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .after_help(
            "For more information about a specific command, try `rhosts <command> --help`\n\
             The source code for rhosts is available at: https://github.com/StevenBlack/rhosts",
        )
        .subcommand(cmd::init::make_subcommand())
        .subcommand(cmd::build::make_subcommand())
        .subcommand(cmd::clean::make_subcommand())
;


    app
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
        // Filter extraneous html5ever not-implemented messages
        builder.filter(Some("html5ever"), LevelFilter::Error);
    }

    builder.init();
}