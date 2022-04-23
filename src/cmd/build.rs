use anyhow::{Context, Error};
use clap::{arg, Arg, ArgMatches, Command};

// Create clap subcommand arguments
pub fn make_subcommand<'help>() -> Command<'help> {
    Command::new("build")
        .about("Builds hosts file from sources")
        .arg(
            Arg::new("cache-dir")
                .short('d')
                .long("cache-dir")
                .value_name("cache-dir")
                .help(
                    "Directory for local copy of data{n}\
                    Relative paths are interpreted relative to the app's's root directory.{n}\
                    If omitted, rhosts uses build.cache-dir from rhosts.toml or defaults to `./rhosts`.",
                ),
        )
        .arg(arg!([dir]
            "Root directory for the hosts{n}\
            (Defaults to the Current Directory when omitted)"
        ))
        .arg(arg!(-o --open "Opens the amalgamated hosts file."))
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<(), Error> {
    println!("You selected 'build'.");
    println!("{:?}", args);

    Ok(())
}

/*
$  cargo run -- build -d mydir

ArgMatches {
    valid_args: [cache-dir, dir, open, help, version],
    valid_subcommands: [],
    disable_asserts: false,
    args: {
        cache-dir: MatchedArg {
            occurs: 1,
            ty: Some(CommandLine),
            indices: [2],
            vals: [["mydir"]],
            ignore_case: false,
            invalid_utf8_allowed: Some(false)
        }
    },
    subcommand: None
}

*/
