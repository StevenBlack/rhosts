use futures::executor::block_on;

use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    net::{IpAddr},
};
// See also [Rust: Domain Name Validation](https://bas-man.dev/post/rust/domain-name-validation/)

use reqwest;
use crate::utils::{norm_string};

pub type Domain = String;
pub type IPaddress = String;

#[derive(Debug, Default)]
pub struct Host {
    ip_address: IPaddress,
    domain: Domain,
}

pub type Hosts = Vec<Host>;
#[derive(Debug, Default)]
pub struct Hostssource {
    pub location: String,
    pub raw_list: Vec<String>,
    pub list_header:  Vec<String>,
	pub domains: Vec<Domain>,
    pub hosts: Hosts,
	pub tlds: HashMap<String, i32>,
	pub tldtallies: Vec<i32>,
	pub duplicates: Vec<Domain>
}

// impl HostsMethods for Hostssource {
impl Hostssource {
    pub async fn load(&mut self, src: &str) {
        self.location = src.to_string();
        let clean = src.to_lowercase();
        if src.contains("\n") {
            self.raw_list = src
                .split("\n")
                .map(|l| l.to_string())
                .collect::<Vec<String>>();

            self.location = "text input".to_string();

        } else if clean.starts_with("http") {
            let resp = reqwest::blocking::get(src).expect("request failed");
            let body = resp.text().expect("body invalid");

            self.raw_list = body
                .lines()
                .map(|l| l.to_string())
                .collect();

        } else {
            let file = File::open(src).expect("no such file");
            let buf = BufReader::new(file);
            self.raw_list = buf.lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();
        }

        self.normalize();
        self.process();
    }

    fn process(&mut self) {}

    fn normalize(&mut self) {
        self.trimlines();
        self.removeblanklines();
        self.saveheader();
        self.removecommentlines();
    }

    fn trimlines(&mut self) {
        let mut lines: Vec<String> = self.raw_list.clone();

        lines
        .iter_mut()
        .for_each(|line|
            *line = norm_string(line.as_str()).to_string()
        );

        self.domains = lines.clone();
    }

    fn removeblanklines(&mut self) {
        let mut lines: Vec<String> = self.domains.clone();
        lines.retain(|line | line.chars().count() > 0);
        self.domains = lines;
    }

    fn saveheader(&mut self) {
        for x in 0..self.raw_list.len() {
            let line = self.raw_list[x].clone();
            if line.starts_with("#") {
              self.list_header.push(line);
            }
        }
    }

    fn removecommentlines(&mut self) {
        let mut lines: Vec<String> = self.domains.clone();
        lines.retain(|line | !line.starts_with("#"));
        self.domains = lines;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    #[test]
    fn test_load_from_file() {
        let mut s = Hostssource{
            ..Default::default()
        };
        block_on(s.load("/Users/Steve/Dropbox/dev/hosts/hosts"));
        assert_eq!(s.location, "/Users/Steve/Dropbox/dev/hosts/hosts");
        assert!(s.list_header.len() > 0);
        assert!(s.raw_list.len() > 0);
        assert!(s.domains.len() > 50_000);
    }

    #[test]
    fn test_load_from_github() {
        let mut s = Hostssource{
            ..Default::default()
        };
        let url = "https://raw.githubusercontent.com/StevenBlack/hosts/f5d5efab/data/URLHaus/hosts";
        block_on(s.load(&url));
        assert_eq!(s.location, url.to_string());
        assert!(s.list_header.len() > 4);
        assert!(s.raw_list.len() > 1000);
        assert!(s.domains.len() > 1000);
    }

    #[test]
    fn test_load_big_from_github() {
        let mut s = Hostssource{
            ..Default::default()
        };
        let url = "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts";
        block_on(s.load(&url));
        assert_eq!(s.location, url.to_string());
        assert!(s.list_header.len() > 4);
        assert!(s.raw_list.len() > 50_000);
        assert!(s.domains.len() > 50_000);
    }

    #[test]
    fn test_load_multiline() {
        let mut s = Hostssource{
            ..Default::default()
        };
        block_on(s.load("# test\n# test 2\n0.0.0.0 example.com\n0.0.0.0 www.example.com"));
        assert!(s.list_header.len() == 2);
        assert!(s.raw_list.len() == 4);
        assert!(s.domains.len() == 2);
    }

}
