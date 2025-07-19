//! This Rust module provides functionality for managing and processing hosts files
//! with various options for comparison, sorting, and filtering.
//!
//! We use `clap` for command-line argument parsing and `anyhow`
//! for error handling.
//!
//! # Modules
//!
//! - **cmd**: Handles specific commands and their implementations.
//! - **config**: Contains configuration management utilities and helpers.
//! - **types**: Defines data structures and types.
//! - **utils**: Includes utility functions for common tasks.
//!
//! # Main Structs and Enums
//!
//! ## `Arguments`
//!
//! The primary structure representing the parsed command-line arguments. This
//! structure includes options for specifying hosts files, output preferences, and
//! commands for execution.
//!
//! ### Fields
//!
//! - **mainhosts** (`String`): Specifies the main hosts file for comparison.
//!   Defaults to `"base"`.
//! - **comparehosts** (`Option<String>`): Specifies the hosts file to compare against the main hosts.
//! - **isolate** (`Option<String>`): Specifies the hosts list to isolate and compare to the main hosts.
//! - **iplocalhost** (`String`): Defines the IP address to use when listing hosts. Defaults to `"0.0.0.0"`.
//! - **adddefaults** (`bool`): Adds default hosts to the top of the host lists.
//! - **domains_sort** (`bool`): Enables domain sorting based on hierarchy.
//! - **output** (`Option<String>`): Specifies an output file; otherwise, stdout is used.
//! - **plain_output** (`bool`): Generates plain domain listings (domains only, without IP addresses).
//! - **quiet** (`bool`): Enables quiet mode, displaying only the number of domains.
//! - **stats** (`Option<bool>`): Displays statistics about the domains.
//! - **intersection_list** (`bool`): Outputs the intersection of `mainhosts` and `comparehosts`.
//! - **rootdomains** (`bool`): Outputs a count of root domains.
//! - **subdomains** (`bool`): Outputs a count of subdomains.
//! - **tld** (`bool`): Outputs a tally of top-level domains (TLDs).
//! - **limit** (`usize`): Limits the number of TLD/root domain listings. Defaults to 30; `0` for unlimited.
//! - **skipheaders** (`bool`): Omits file comment headers in the output.
//! - **showduplicates** (`bool`): Lists duplicate domains when reporting a hosts list.
//! - **showinvalids** (`bool`): Lists invalid domains when reporting a hosts list.
//! - **sysclipboard** (`bool`): Uses system clipboard contents as the compare hosts.
//! - **uniquelist** (`bool`): Outputs unique domain names.
//! - **verbose** (`bool`): Enables verbose output for development or debugging.
//! - **command** (`Option<Commands>`): Specifies a subcommand to execute.
//! - **skipcache** (`bool`): Prevents using cached data.
//!
//! ### Methods
//!
//! - **`Arguments::new`**: Generates a new `Arguments` instance with default or configured values.
//!   - **Returns**: `Arguments`
//!
//! ## `Commands`
//!
//! Enum representing subcommands for the application.
//!
//! ### Variants
//!
//! - **Build**: Builds hosts files with an optional formula.
//!   - **formula** (`Option<String>`): Specifies the formula to use.
//! - **Cache**: Manages application cache with subcommands for actions.
//!   - **cacheaction** (`Option<cmd::cache::CacheCommands>`): A subcommand for cache-specific actions.
//! - **Init**: Initializes cache and templates.
//! - **Info**: Displays additional application information.
//!
//! # Functions
//!
//! ## `show_info`
//!
//! Prints detailed application information such as version, description, author,
//! license, homepage, and repository, as well as specific information from various
//! application modules.
//!
//! ### Parameters
//!
//! - `args` (`Arguments`): The parsed command-line arguments.
//!
//! ### Returns
//!
//! - `Result<(), Error>`: Outputs `Ok(())` on success or an error if an issue occurs.
//!
//! ## `main`
//!
//! The entry point of the application that processes command-line inputs, initializes
//! configurations, and executes the specified command or default behavior.
//!
//! ### Asynchronous Execution
//!
//! - Leverages `async_std::main` for asynchronous execution of certain tasks.
//!
//! ### Returns
//!
//! - `Result<(), Error>`: Outputs `Ok(())` on success or an error if initialization or execution fails.
//!
//! ### Behavior Based on Commands
//!
//! - Executes the given `Commands` variant or defaults to core command execution if no subcommand is provided.
//!
//! ## `test_default_command_line_arguments`
//!
//! A unit test to validate the default behavior and configuration of the `Arguments` struct.
//!
//! ### Assertions
//!
//! - Verifies default parsing and initialization of critical fields in `Arguments`:
//!   - **`mainhosts`**: Fetched from the base key in shortcuts.
//!   - **`comparehosts`**, **`iplocalhost`**, **`tld`**, and **`stats`**.
//!
//! ### Test Outcome
//!
//! - Ensures expected defaults for arguments and initializes properly when no command-line arguments are provided.

extern crate clap;
use anyhow::Error;
use clap::{Parser, Subcommand};
use config::get_shortcuts;

mod cmd;
mod config;
mod types;
mod utils;

/// Command-line arguments structure.
///
/// This struct is designed to parse and represent the input arguments.
///
/// # Fields
///
/// - `mainhosts`:
///     The main hosts file acting as the basis for comparison. Defaults to "base".
///     - Short Flag: `-m`
///     - Long Flag: `--main`
///
/// - `comparehosts`:
///     The hosts file to compare against the `mainhosts`.
///     - Short Flag: `-c`
///     - Long Flag: `--compare`
///
/// - `isolate`:
///     A hosts file to isolate and compare against the `mainhosts`. This builds a temporary
///     adjusted `mainhosts` without the isolated file, then compares the temporary adjusted
///     `mainhosts` with the isolated file.
///     - Long Flag: `--isolate`
///
/// - `iplocalhost`:
///     The IP address to associate with listed hosts. Defaults to `0.0.0.0`.
///     - Long Flag: `--ip`
///
/// - `adddefaults`:
///     Flag to include default hosts at the top of the hosts lists, if enabled.
///     - Short Flag: `-d`
///     - Long Flag: `--default_hosts`
///
/// - `domains_sort`:
///     Flag to sort domains by order: domain, TLD, subdomains.
///     - Short Flag: `-s`
///     - Long Flag: `--sort`
///
/// - `output`:
///     Specifies an output file. By default, output is sent to `stdout`.
///     - Short Flag: `-o`
///     - Long Flag: `--output`
///
/// - `plain_output`:
///     Enables plain listing mode where only domains (without IP addresses) are listed.
///     - Short Flag: `-p`
///     - Long Flag: `--plain`
///
/// - `quiet`:
///     Enables quiet output mode, only showing the count of domains.
///     - Short Flag: `-q`
///     - Long Flag: `--quiet`
///
/// - `stats`:
///     Optional flag to display statistics about the domains.
///     - Long Flag: `--stats`
///
/// - `intersection_list`:
///     Prints the intersection of `mainhosts` and `comparehosts`.
///     - Short Flag: `-i`
///     - Long Flag: `--intersection`
///
/// - `rootdomains`:
///     Lists root domains and their respective counts.
///     - Short Flag: `-r`
///     - Long Flag: `--rootdomains`
///
/// - `subdomains`:
///     Lists subdomains and their respective counts.
///     - Short Flag: `-s`
///     - Long Flag: `--subdomains`
///
/// - `tld`:
///     Displays a tally of top-level domains (TLDs) in the list.
///     - Short Flag: `-t`
///     - Long Flag: `--tld`
///
/// - `limit`:
///     Sets a limit for listing TLDs and root domains, where `0` indicates no limit. Defaults to `30`.
///     - Long Flag: `--limit`
///
/// - `skipheaders`:
///     Omits file comment headers in the output.
///     - Long Flag: `--skipheaders`
///
/// - `showduplicates`:
///     Lists duplicate domains found in the hosts list.
///     - Long Flag: `--showduplicates`
///
/// - `showinvalids`:
///     Lists invalid domains detected in the hosts list.
///     - Long Flag: `--invalid`
///
/// - `sysclipboard`:
///     Uses the contents of the system clipboard as the `comparehosts` input.
///     - Long Flag: `--clip`
///
/// - `uniquelist`:
///     Lists unique domain names in the hosts list.
///     - Short Flag: `-u`
///     - Long Flag: `--unique`
///
/// - `verbose`:
///     Enables verbose output, useful for debugging or detailed inspection.
///     - Short Flag: `-v`
///     - Long Flag: `--verbose`
///
/// - `command`:
///     Specifies an optional subcommand to execute. Refer to the `Commands` enum for available
///     subcommand options.
///
/// - `skipcache`:
///     Disables caching to ensure fresh processing.
///     - Long Flag: `--skipcache`
#[derive(Debug, Default, Parser)]
#[clap(author, version, about, long_about = None)]
#[deny(missing_docs)]

#[derive(Clone)]
pub struct Arguments {
    #[clap(
        short,
        long = "main",
        default_value = "base",
        help = r#"The main hosts file, the basis for comparison.

A shortcut code, full URL, or a path to a local file.
Use the -c option to specify a comparison list.
Use the -clip option to use what is on the system clipboard

SHORTCUT CODES
==============
The following shortcut codes can be used to select among preset lists.

Amalgamated list shortcuts:
  -m b or -m base // use Steven Black's base amalgamated list.
  -m f    // use alternates/fakenews/hosts
  -m fg   // use alternates/fakenews-gambling/hosts
  -m fgp  // use alternates/fakenews-gambling-porn/hosts
  -m fgps // use alternates/fakenews-gambling-porn-social/hosts
  -m fgs  // use alternates/fakenews-gambling-social/hosts
  -m fp   // use alternates/fakenews-porn/hosts
  -m fps  // use alternates/fakenews-porn-social/hosts
  -m fs   // use alternates/fakenews-social/hosts
  -m g    // use alternates/gambling/hosts
  -m gp   // use alternates/gambling-porn/hosts
  -m gps  // use alternates/gambling-porn-social/hosts
  -m gs   // use alternates/gambling-social/hosts
  -m p    // use alternates/porn/hosts
  -m ps   // use alternates/porn-social/hosts
  -m s    // use alternates/social/hosts

Source list shortcuts:
  -m adaway           // adaway.github.io
  -m add2o7net        // FadeMind add.2o7Net hosts
  -m adddead          // FadeMind add.Dead hosts
  -m addrisk          // FadeMind add.Risk hosts
  -m addspam          // FadeMind add.Spam hosts
  -m adguard          // AdguardTeam cname-trackers
  -m baddboyz         // mitchellkrogza Badd-Boyz-Hosts
  -m clefspear        // Clefspeare13 pornhosts
  -m digitalside      // davidonzo Threat-Intel
  -m fakenews         // marktron/fakenews
  -m hostsvn          // bigdargon hostsVN
  -m kadhosts         // PolishFiltersTeam
  -m metamask         // MetaMask eth-phishing hosts
  -m mvps             // winhelp2002.mvps.or
  -m orca             // orca.pet notonmyshift hosts
  -m shady            // hreyasminocha shady hosts
  -m sinfonietta-gambling
  -m sinfonietta-porn
  -m sinfonietta-snuff
  -m sinfonietta-social
  -m someonewhocares  // Sam Pollock someonewhocares.org
  -m stevenblack      // Steven Black ad-hoc list
  -m tiuxo-porn
  -m tiuxo-social
  -m tiuxo            // tiuxo list.
  -m uncheckyads      // FadeMind UncheckyAds
  -m urlhaus          // urlhaus.abuse.ch
  -m yoyo             // Peter Lowe yoyo.org

"#
    )]
    mainhosts: String,


    #[clap(
        short,
        long = "compare",
        help = r#"The hosts file to compare to the main hosts file
A shortcut code, full URL, or a path to a local file.
Use the -m option for the main comparison list.
Use the -clip option to use what is on the system clipboard.

See the documentation for the -m flag for a list of shortcut codes
        "#
    )]
    comparehosts: Option<String>,

    #[clap(
        long = "isolate",
        help = r#"The hosts list to isolate and compare to mainhosts
A shortcut code, full URL, or a path to a local file.
See the documentation for the -m flag for a list of shortcut codes
        "#

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
        long,
        help = "List of subdomains (3+ characters) and their tally"
    )]
    subdomains: bool,

    #[clap(
        long,
        help = "Character chunking size for tallying within subdomains"
    )]
    chunking: Option<usize>,

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
        cacheaction: Option<cmd::cache::CacheCommands>,
    },
    /// Initialize cache and templates
    Init,
    /// Display additional information about the application
    Info,
}

/**
 * Displays detailed information about the current application, configuration, and related commands.
 *
 * This function provides an organized "info dump" that includes metadata about the application (such as version,
 * description, author, license, homepage, and repository), as well as additional information from various components.
 *
 * # Arguments
 *
 * * `args` - A structure of type `Arguments` that holds any necessary command-line arguments or context.
 *
 * # Returns
 *
 * * `Ok(())` if the operation completes successfully.
 * * `Err(Error)` if there is an issue during execution.
 *
 * # Metadata Displayed
 *
 * * Application version (from `CARGO_PKG_VERSION`).
 * * Description of the application (from `CARGO_PKG_DESCRIPTION`).
 * * Author(s) (from `CARGO_PKG_AUTHORS`).
 * * License information (from `CARGO_PKG_LICENSE`).
 * * Homepage URL (from `CARGO_PKG_HOMEPAGE`).
 * * Repository URL (from `CARGO_PKG_REPOSITORY`).
 *
 * # Side Effects
 *
 * * Outputs formatted information to the standard output (`stdout`).
 * * Delegates additional info retrieval and display to the following modules:
 *   - `config` (via `config::info`).
 *   - `cmd::cache` (via `cmd::cache::info`).
 *   - `cmd::core` (via `cmd::core::info`).
 *
 * # Formatting
 *
 * The output includes a centered title ("info dump") bounded by dashes, followed by sections of metadata
 * and component-specific information, all separated by blank lines for better readability.
 *
 * # Example
 *
 * ```
 * use your_crate::Arguments;
 *
 * let args = Arguments::new(); // Example initialization of `Arguments`.
 * if let Err(e) = show_info(args) {
 *     eprintln!("Error displaying information: {}", e);
 * }
 * ```
 */
fn show_info(args:Arguments) -> Result<(), Error> {
    println!();
    println!("{}",format!("{:-^1$}", " info dump ", 40));
    println!("rh version: {}", env!("CARGO_PKG_VERSION"));
    println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    println!();
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!();
    _ = config::info(args.clone());
    println!();
    _ = cmd::cache::info(args.clone());
    println!();
    _ = cmd::core::info(args.clone());
    println!();
    println!("{}",format!("{:-^1$}", "", 40));
    println!();

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let args = Arguments::parse();
    config::init(args.clone())?;
    cmd::cache::init(args.clone()).await?;

    // Check which subcomamnd the user specified, if any...
    let res = match &args.command {
        None => cmd::core::execute(args),
        Some(Commands::Init) => cmd::init::execute(args).await,
        Some(Commands::Build { formula: _ }) => cmd::build::execute(args).await,
        Some(Commands::Cache { cacheaction: _ }) => cmd::cache::execute(args).await,
        Some(Commands::Info) => {show_info(args)},
    };

    if let Err(e) = res {
        println!("Error {:?}", e);
        std::process::exit(101);
    }
    Ok(())
}


#[test]
fn test_default_command_line_arguments() {
    let arguments = Arguments::new();
    assert_eq!(
        arguments.mainhosts,
        get_shortcuts()
            .get("base")
            .expect("The base key does not exist")
            .to_owned(),
        "Expected mainhosts to be fetched by the base key"
    );
    assert_eq!(
        arguments.comparehosts
        , None,
        "Expected the comparehosts argument to be None"
    );
    assert_eq!(
        arguments.iplocalhost
        , "0.0.0.0".to_string(),
        "Expected the iplocalhost argument to be 0.0.0.0"
    );
    assert_eq!(
        arguments.tld
        , false,
        "Expected the tld argument to be false"
    );
    assert_eq!(
        arguments.stats, Some(true),
        "Expected the stats argument to be Some(true)"
    );
}
