use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};
// See also [Rust: Domain Name Validation](https://bas-man.dev/post/rust/domain-name-validation/)

use crate::utils::norm_string;

pub type Domain = String;
pub type Domains = Vec<Domain>;
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
    pub frontmatter: Vec<String>,
    pub domains: Domains,
    pub hosts: Hosts,
    pub tlds: HashMap<String, i32>,
    pub tldtallies: Vec<i32>,
    pub duplicates: Domains,
}

// impl HostsMethods for Hostssource {
impl Hostssource {
    pub async fn load(&mut self, src: &str) {
        self.location = src.to_string();
        let clean = src.to_lowercase();
        if src.contains('\n') {
            self.raw_list = src
                .split('\n')
                .map(|l| l.to_string())
                .collect::<Vec<String>>();

            self.location = "text input".to_string();
        } else if clean.starts_with("http") {
            let resp = reqwest::blocking::get(src).expect("request failed");
            let body = resp.text().expect("body invalid");

            self.raw_list = body.lines().map(|l| l.to_string()).collect();
        } else {
            let file = File::open(src).expect("no such file");
            let buf = BufReader::new(file);
            self.raw_list = buf
                .lines()
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
        self.frontmatter();
        self.removecommentlines();
        self.extract_domains();
    }

    fn trimlines(&mut self) {
        let mut lines: Vec<String> = self.raw_list.clone();

        lines
            .iter_mut()
            .for_each(|line| *line = norm_string(line.as_str()));

        self.domains = lines.clone();
    }

    fn extract_domains(&mut self) {
        let mut domains_result: Domains = Vec::new();

        for line in &self.domains {
            for element in line.split_whitespace() {
                if element != "0.0.0.0" && element != "127.0.0.1" {
                    domains_result.push(element.to_string());
                }
            }
        }

        self.domains = domains_result;
    }

    fn removeblanklines(&mut self) {
        self.domains.retain(|line| !line.is_empty());
    }

    fn frontmatter(&mut self) {
        for line in &self.raw_list {
            if line.starts_with('#') {
                self.frontmatter.push(line.to_string());
            }
        }
    }

    fn removecommentlines(&mut self) {
        self.domains.retain(|line| !line.starts_with('#'));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    #[test]
    fn test_load_from_file() {
        let mut s = Hostssource {
            ..Default::default()
        };
        block_on(s.load("/Users/Steve/Dropbox/dev/hosts/hosts"));
        assert_eq!(s.location, "/Users/Steve/Dropbox/dev/hosts/hosts");
        assert!(s.frontmatter.len() > 0);
        assert!(s.raw_list.len() > 50_000);
        assert!(s.domains.len() > 50_000);
    }

    #[test]
    fn test_load_from_github() {
        let mut s = Hostssource {
            ..Default::default()
        };
        let url = "https://raw.githubusercontent.com/StevenBlack/hosts/f5d5efab/data/URLHaus/hosts";
        block_on(s.load(&url));
        assert_eq!(s.location, url.to_string());
        assert!(s.frontmatter.len() > 4);
        assert!(s.raw_list.len() > 1000);
        assert!(s.domains.len() > 1000);
    }

    #[test]
    fn test_load_big_from_github() {
        let mut s = Hostssource {
            ..Default::default()
        };
        let url = "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts";
        block_on(s.load(&url));
        assert_eq!(s.location, url.to_string());
        assert!(s.frontmatter.len() > 4);
        assert!(s.raw_list.len() > 50_000);
        assert!(s.domains.len() > 50_000);
    }

    #[test]
    fn test_load_multiline() {
        let mut s = Hostssource {
            ..Default::default()
        };
        block_on(s.load("# test\n# test 2\n0.0.0.0 example.com\n0.0.0.0 www.example.com"));
        assert!(s.frontmatter.len() == 2);
        assert!(s.raw_list.len() == 4);
        assert!(s.domains.len() == 2);
    }

    #[test]
    fn test_normalize_line() {
        let mut s = Hostssource {
            ..Default::default()
        };
        block_on(s.load("# test\n# test 2\n0.0.0.0 example.com\n0.0.0.0 www.example.com\n127.0.0.1 example.org www.example.org"));
        assert!(s.domains.len() == 4);

        let expected_domains = vec![
            "example.com",
            "www.example.com",
            "example.org",
            "www.example.org",
        ];
        assert!(s.domains == expected_domains);
    }
}
