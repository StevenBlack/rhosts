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
rhosts 0.0.2
Steven Black <hosts@sbc.io>
Tools to mess with hosts files.

USAGE:
    rh [OPTIONS] [ROOT] [SUBCOMMAND]

ARGS:
    <ROOT>

OPTIONS:
    -c, --compare <COMPAREHOSTS>
            The hosts file to compare to mainhosts

        --clip
            Use the contents of the system clipboard as compare hosts

    -d, --default_hosts <ADDDEFAULTS>
            Add default hosts assigments

        --dump


    -h, --help
            Print help information

        --intersection-list <INTERSECTION_LIST>
            Print the intersection of lists

        --ip <IPLOCALHOST>
            The ip address to use for hosts [default: 0.0.0.0]

    -m, --main <MAINHOSTS>
            The main hosts file, the basis for comparison [default: base]

        --nocache
            Skip any cache

        --noheader <NOHEADER>
            Omit the file comment headers in output

    -o, --output <OUTPUT>
            Print the domains to std out

    -p, --plain <PLAIN_OUTPUT>
            Domains with no IP addresses

    -q, --quiet
            Quiet, single tally, terse output mode

    -s, --sort <ALPHA_SORT>
            Sort the domains

        --stats <STATS>
            Print the domains to std out

        --tld <TLD>
            Print top level domain tallies

    -u, --unique <UNIQUELIST>
            List the unique domain names

    -v, --verbose
            List the unique domain names

    -V, --version
            Print version information

SUBCOMMANDS:
    build    Build hosts files
    cache    Cache hosts files
    help     Print this message or the help of the given subcommand(s)
    init     Initialize cache and templates
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
