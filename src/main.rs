use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn main() {
    let hostsfile = "/Users/Steve/Dropbox/dev/hosts/hosts";
    let mut lines = lines_from_file(hostsfile);

    // remove blank lines
    lines.retain(|line | line.chars().count() > 0);

    for line in lines {
        println!("{:?}", line);
    }
}