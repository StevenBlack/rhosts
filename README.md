# rhosts

Host file tools written in [Rust](https://www.rust-lang.org/) conceived while
stuck at home during a pandemic.

## Take note!**

This is all very preliminary. This is not presently fit for general consumption.

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
