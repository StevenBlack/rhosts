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
    let lines = lines_from_file(hostsfile);
                           
    for line in lines {
        println!("{:?}", line);
    }
}