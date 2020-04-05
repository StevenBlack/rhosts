use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

mod utils;

trait ListMethods {
    fn trimlines(&mut self);
    fn removeblanklines(&mut self);
    fn removecommentlines(&mut self);
    fn saveheader(&mut self);
}

impl ListMethods for Hostslist {
    fn trimlines(&mut self) {
        let mut lines: Vec<String> = self.domains.clone();
        lines.iter_mut().for_each(|line| *line = line.trim().to_string());
        self.domains = lines.clone();
    }
    fn removeblanklines(&mut self) {
        let mut lines: Vec<String> = self.domains.clone();
        lines.retain(|line | line.chars().count() > 0);
        self.domains = lines.clone();
    }
    fn removecommentlines(&mut self) {
        let mut lines: Vec<String> = self.domains.clone();
        lines.retain(|line | !line.starts_with("#"));
        self.domains = lines.clone();
    }
    fn saveheader(&mut self) {
        let mut lines: Vec<String> = self.domains.clone();
        lines.retain(|line | !line.starts_with("#"));
        self.domains = lines.clone();
    }
}

struct Hostslist {
    location: String,
    raw_list: Vec<String>,
    list_header:  Vec<String>,
	domains: Vec<String>
	// TLDs       map[string]int
	// TLDtallies []TLDtally
	// Duplicates []string
}

fn main() {
    let list_source = "/Users/Steve/Dropbox/dev/hosts/hosts";
    let file = File::open(list_source).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // preserve the file header
    let mut list_header = Vec::new();
    for line in lines.iter() {
        if line.starts_with("#") {
            list_header.push(line.clone());
        } else {
            break;
        }
    }

    let mut hf1 = Hostslist{
        location: String::from(list_source),
        raw_list: lines.clone(), 
        list_header,
        domains: lines.clone()
    };

    // trim all lines
    hf1.trimlines();
    // remove blank lines
    hf1.removeblanklines();
    // remove comments
    hf1.removecommentlines();

    utils::sep(40);
    println!("Location: {:?}", hf1.location);

    utils::sep(40);
    println!("{:?}", "File header:");
    for line in hf1.list_header {
        println!("{:?}", line);
    }

    utils::sep(40);
    let mut last = 10;
    for line in hf1.raw_list {
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