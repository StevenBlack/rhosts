use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

struct Hosts {
	raw:  Vec<String>,
    location: String,
	domains: Vec<String>
	// TLDs       map[string]int
	// TLDtallies []TLDtally
	// Duplicates []string
}

// fn print_type_of<T>(_: &T) {
    // println!("===> {}", std::any::type_name::<T>())
// }

// fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
//     let file = File::open(filename).expect("no such file");
//     let buf = BufReader::new(file);
//     buf.lines()
//         .map(|l| l.expect("Could not parse line"))
//         .collect()
// }

fn sep(n: usize) {
    println!("{}", "-".repeat(n));
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

    // let mut lines = lines_from_file(hostsfile);
    // print_type_of(&lines);

    // trim all lines
    lines.iter_mut().for_each(|line| *line = line.trim().to_string());
    // remove blank lines
    lines.retain(|line | line.chars().count() > 0);
    // remove comments
    lines.retain(|line | !line.starts_with("#"));
    hf1 = Hosts{ domains: lines, ..hf1};


    sep(40);
    println!("Location: {:?}", hf1.location);
    sep(40);


    let mut last = 10;
    for line in hf1.raw {
        last = last -1;
        if last == 0 {
            break;
        }
        println!("{:?}", line);
    }
    sep(40);
    last = 10;
    for line in hf1.domains {
        last = last -1;
        if last == 0 {
            break;
        }
        println!("{:?}", line);
    }
    sep(40);
}