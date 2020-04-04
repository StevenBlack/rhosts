use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

mod utils;

struct Hosts {
	raw:  Vec<String>,
    location: String,
	domains: Vec<String>
	// TLDs       map[string]int
	// TLDtallies []TLDtally
	// Duplicates []string
}

fn main() {
    let hostsfile = "/Users/Steve/Dropbox/dev/hosts/hosts";
    let file = File::open(hostsfile).expect("no such file");
    let buf = BufReader::new(file);
    let mut lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut hf1 = Hosts{
        raw: lines.clone(), 
        location: String::from(hostsfile),
        domains: lines.clone()
    };

    // trim all lines
    lines.iter_mut().for_each(|line| *line = line.trim().to_string());
    // remove blank lines
    lines.retain(|line | line.chars().count() > 0);
    // remove comments
    lines.retain(|line | !line.starts_with("#"));
    hf1 = Hosts{ domains: lines, ..hf1};


    utils::sep(40);
    println!("Location: {:?}", hf1.location);
    utils::sep(40);


    let mut last = 10;
    for line in hf1.raw {
        last = last -1;
        if last == 0 {
            break;
        }
        println!("{:?}", line);
    }
    utils::sep(40);
    last = 10;
    for line in hf1.domains {
        last = last -1;
        if last == 0 {
            break;
        }
        println!("{:?}", line);
    }
    utils::sep(40);
}