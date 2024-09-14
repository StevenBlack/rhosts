# rhosts (rh)

Host file tools written in [Rust](https://www.rust-lang.org/) conceived while
stuck at home during a pandemic.

## Take note

This is all very preliminary. This is not presently fit for general consumption.

**External non-rust dependency**: `openssl-dev`

## Calling `rh`

This is the output from `$ rh -h`.

```rust
$ rh -h                                                                                                

Tools to mess with hosts files.

Usage: rh [OPTIONS] [ROOT] [COMMAND]

Commands:
  build  Build hosts files
  cache  Application cache initialize, prime, clear, or report
  init   Initialize cache and templates
  info   Display additional information about the application
  help   Print this message or the help of the given subcommand(s)

Arguments:
  [ROOT]  [possible values: true, false]

Options:
  -m, --main <MAINHOSTS>        The main hosts file, the basis for comparison [default: base]
  -c, --compare <COMPAREHOSTS>  The hosts file to compare to mainhosts
      --ip <IPLOCALHOST>        The ip address to use when listing hosts [default: 0.0.0.0]
  -d, --default_hosts           Add default hosts to when listing hosts The default hosts will be placed at the top of hosts lists
  -s, --sort                    Sort the domains The sort order is domain, tdl, subdomain1, subdomain2, etc
  -o, --output <OUTPUT>         The output file Otherwise, by default, output is to std out
  -p, --plain                   Plain listing - domains only, without addresses, when listing domains
  -q, --quiet                   Quiet, terse output mode Outputs the number of domains only
      --stats <STATS>           Print statistics about the domains [possible values: true, false]
  -i, --intersection            Print the intersection of mainhosts and comparehosts
  -t, --tld                     Print a tally of top level domains found in the list
      --noheader                Omit the file comment headers in output
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

I see this becoming a **full-featured swiss-knife** for assessing and working with amalgamated
hosts files.

![MissionVsVision](https://user-images.githubusercontent.com/80144/158078813-87141f60-a03f-4367-a8c1-3d8da68de45e.gif)

## Mission for development

Ultimately this will

1. replace the python-based [hosts](https://github.com/StevenBlack/hosts) build
tools
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
