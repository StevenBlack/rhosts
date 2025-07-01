# rhosts (rh)

Host file tools written in [Rust](https://www.rust-lang.org/) conceived while
stuck at home during a pandemic.

> [!NOTE]  
> This is all very preliminary. This is not presently fit for general consumption.

> [!NOTE]  
> **External non-rust dependency**: `openssl-dev`

## Calling `rh`

This is the output from `$ rh -h`.

```rust
$ rh -h

Tools to mess with hosts files.

Usage: rh [OPTIONS] [COMMAND]

Commands:
  build  Build hosts files
  cache  Application cache initialize, prime, clear, or report
  init   Initialize cache and templates
  info   Display additional information about the application
  help   Print this message or the help of the given subcommand(s)

Options:
  -m, --main <MAINHOSTS>        The main hosts file, the basis for comparison.

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

                                 [default: base]
  -c, --compare <COMPAREHOSTS>  The hosts file to compare to the main hosts file
                                A shortcut code, full URL, or a path to a local file.
                                Use the -m option for the main comparison list.
                                Use the -clip option to use what is on the system clipboard.

                                See the documentation for the -m flag for a list of shortcut codes

      --isolate <ISOLATE>       The hosts list to isolate and compare to mainhosts
                                A shortcut code, full URL, or a path to a local file.
                                See the documentation for the -m flag for a list of shortcut codes

      --ip <IPLOCALHOST>        The ip address to use when listing hosts [default: 0.0.0.0]
  -d, --default_hosts           Add default hosts for when listing hosts. The default hosts will be placed at the top of hosts lists
  -s, --sort                    Sort the domains. The sort order is domain, tdl, subdomain1, subdomain2, etc
  -o, --output <OUTPUT>         The output file. By default, output is to std out
  -p, --plain                   Plain listing - domains only, without addresses, when listing domains
  -q, --quiet                   Quiet, terse output mode. Outputs the number of domains only
      --stats <STATS>           Print statistics about the domains [possible values: true, false]
  -i, --intersection            Print the intersection of mainhosts and comparehosts
  -r, --rootdomains             List of root domains and their tally
  -t, --tld                     Print a tally of top level domains found in the list
  -l, --limit <LIMIT>           Limit for listing TLD and root domains, 0 = unlimited [default: 30]
      --skipheaders             Omit the file comment headers in output
      --showduplicates          List duplicates when reporting on a hosts list
      --invalid                 List invalid domains when reporting on a hosts list
      --clip                    Use the contents of the system clipboard as compare hosts
  -u, --unique                  List the unique domain names
  -v, --verbose                 Verbose output, useful for development
      --skipcache               Do not use cache
  -h, --help                    Print help information
  -V, --version                 Print version information

  ```

## Vision for this project

This is to be a **full-featured swiss-knife** for assessing and working with
amalgamated hosts files.

![MissionVsVision](https://user-images.githubusercontent.com/80144/158078813-87141f60-a03f-4367-a8c1-3d8da68de45e.gif)

## Mission for development

Ultimately this will

1. replace the python-based [hosts](https://github.com/StevenBlack/hosts) build tools
2. replace [ghosts](https://github.com/StevenBlack/ghosts), the set of ancillary
tools, written in Go, to assess various hosts lists,

## Goals of this project

Here is the list of tangible goals for the project.

* [Extensible architecture](https://github.com/StevenBlack/rhosts/wiki/Extensible-Architecture-Discussion) so development can progress cleanly on many fronts.
* Collect and maintain historical statistics about amalgamated lists oriduced,
and of the component lists that make up the amalgamated hosts.
* Ability to asses the impact of each list in the composition of amalgamated hosts.
* Ability to asses the impact of proposed additions to the amalgamated hosts.

## Related repositories

* [StevenBlack/hosts](https://github.com/StevenBlack/hosts) is my amalgamated hosts file, with custom variants, from various curated sources.
* [StevenBlack/ghosts](https://github.com/StevenBlack/ghosts) is a cli tool written in Go.
