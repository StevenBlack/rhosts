# rhosts
Host file tools written in rust.  While stuck at home in a pandemic.

**Take note!** this is preliminary, not fit for consumption.

![MissionVsVision](https://user-images.githubusercontent.com/80144/158078813-87141f60-a03f-4367-a8c1-3d8da68de45e.gif)

# Vision for this project

I see this becoming a swiss-knife for assessing and working with amalgamated
hosts files.

# Mission for development
Ultimately this will
1. replace the python-based [hosts](https://github.com/StevenBlack/hosts) build
tools
2. replace [ghosts](https://github.com/StevenBlack/ghosts), the set of ancillary
tools, written in Go, to assess various hosts lists,

# Goals of this project

Here is the list of tangible goals for the project.

* Extensible architecture so development can progress on many fronts.
* Collect and maintain historical statistics about amalgamated lists oriduced,
and of the component lists that make up the amalgamated hosts.
* Ability to asses the impact of each list in the composition of amalgamated hosts.
* Ability to asses the impact of proposed additions to the amalgamated hosts.

## Related repositories

* [StevenBlack/hosts](https://github.com/StevenBlack/hosts) is my amalgamated hosts file, with custom variants, from various curated sources.
* [StevenBlack/ghosts](https://github.com/StevenBlack/ghosts) is a cli tool written in Go.
