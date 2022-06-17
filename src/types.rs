use std::{
    collections::BTreeSet,
    collections::HashMap,
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
};
// See also [Rust: Domain Name Validation](https://bas-man.dev/post/rust/domain-name-validation/)

use crate::config::get_shortcuts;
use crate::utils::{is_domain, norm_string, trim_inline_comments};
use num_format::{Locale, ToFormattedString};

pub type Domain = String;
pub type Domains = BTreeSet<Domain>;
pub type IPaddress = String;

#[derive(Debug, Default)]
pub struct Host {
    ip_address: IPaddress,
    domain: Domain,
}
pub type Hosts = Vec<Host>;

#[derive(Debug, Default)]
pub struct Hostssource {
    pub name: String,
    pub location: String,
    pub raw_list: Vec<String>,
    pub front_matter: Vec<String>,
    pub domains: Domains,
    pub hosts: Hosts,
    pub tlds: HashMap<String, i32>,
    pub tld_tallies: Vec<i32>,
    pub duplicates: Domains,
}

impl fmt::Display for Hostssource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "source: {}\nunique domains: {}\nduplicate domains: {}",
            self.location,
            self.domains.len().to_formatted_string(&Locale::en),
            self.duplicates.len().to_formatted_string(&Locale::en)
        )
    }
}

// impl HostsMethods for Hostssource {
impl Hostssource {
    pub fn new(location: String, name: String) -> Hostssource {
        // Special code goes here ...
        let mut hs = Hostssource {
            name,
            ..Default::default()
        };
        hs.load(&location);
        hs
    }

    pub async fn load(&mut self, src: &str) {
        let mut actualsrc = src;
        // check if src is a shortcut
        let shortcuts = get_shortcuts();
        let sc = shortcuts.get(src);
        if sc.is_some() {
            self.location = sc.unwrap().to_string();
            actualsrc = self.location.as_str();
        } else {
            self.location = actualsrc.to_string();
        }

        let clean = actualsrc.to_lowercase();

        if actualsrc.contains('\n') {
            self.raw_list = actualsrc
                .trim()
                .split('\n')
                .map(|l| l.trim().to_string())
                .collect::<Vec<String>>();
            self.location = "text input".to_string();
        } else if clean.starts_with("http") {
            let resp = reqwest::blocking::get(actualsrc).expect("request failed");
            let body = resp.text().expect("body invalid");

            self.raw_list = body.lines().map(|l| l.to_string()).collect();
        } else {
            let file = File::open(actualsrc).expect(&format!("File does not exist: {}", actualsrc));
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

        lines.iter_mut().for_each(|line| {
            *line = norm_string(line.as_str());
            *line = trim_inline_comments(line.to_owned());
            if line.chars().count() > 0 && !self.domains.insert(line.to_owned()) {
                self.duplicates.insert(line.to_owned());
            };
        });
        // self.domains = lines.clone();
    }

    fn extract_domains(&mut self) {
        let mut domains_result: Domains = BTreeSet::new();
        for line in &self.domains {
            for element in line.split_whitespace() {
                if element != "0.0.0.0" && element != "127.0.0.1" {
                    if is_domain(element) {
                        let unique = domains_result.insert(element.to_string());
                        if !unique {
                            self.duplicates.insert(element.to_string());
                        }
                    }
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
                self.front_matter.push(line.to_string());
            } else {
                break;
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
        assert!(s.front_matter.len() > 0);
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
        assert!(s.front_matter.len() > 4);
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
        assert!(s.front_matter.len() > 4);
        assert!(s.raw_list.len() > 50_000);
        assert!(s.domains.len() > 50_000);
    }

    #[test]
    fn test_load_from_shortcut() {
        let mut s = Hostssource {
            ..Default::default()
        };
        block_on(s.load("base"));
        assert_eq!(
            s.location,
            "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts"
        );
        assert!(s.front_matter.len() > 0);
        assert!(s.raw_list.len() > 50_000);
        assert!(s.domains.len() > 50_000);
    }

    #[test]
    fn test_load_multiline_1() {
        let mut s = Hostssource {
            ..Default::default()
        };
        block_on(s.load(
            r##"
            # test
            # test 2
            0.0.0.0 example.com
            # this is a comment
            0.0.0.0 www.example.com
            "##,
        ));
        assert!(s.front_matter.len() == 2);
        assert!(s.raw_list.len() == 5);
        assert!(s.domains.len() == 2);
    }

    #[test]
    fn test_duplicate_domains() {
        let mut s = Hostssource {
            ..Default::default()
        };
        block_on(s.load(
            r##"
            # test
            # test 2
            0.0.0.0 example.com
            0.0.0.0 www.example.com
            0.0.0.0 example.com
            "##,
        ));
        assert!(s.front_matter.len() == 2);
        assert!(s.raw_list.len() == 5);
        assert!(s.domains.len() == 2);
        assert!(s.duplicates.len() == 1);
    }

    #[test]
    fn test_normalize_line() {
        let mut s = Hostssource {
            ..Default::default()
        };
        block_on(s.load(
            r##"
            # test
            # test 2
            0.0.0.0 example.com
            0.0.0.0 www.example.com
            127.0.0.1 example.org www.example.org
            127.0.0.1 something.org
            # some comment
            127.0.0.1 something.else.org
            "##,
        ));
        assert!(s.domains.len() == 6);

        let expected_domains: BTreeSet<String> = BTreeSet::from([
            "example.com".to_string(),
            "www.example.com".to_string(),
            "example.org".to_string(),
            "www.example.org".to_string(),
            "something.org".to_string(),
            "something.else.org".to_string(),
        ]);
        assert!(s.domains == expected_domains);
    }

    #[test]
    fn test_multi_domain_line() {
        let mut s = Hostssource {
            ..Default::default()
        };
        block_on(s.load(
            r##"
            # test
            # test 2
            0.0.0.0 example.com www.example.com example.org
            # a comment foobar.com
            "##,
        ));
        assert!(s.domains.len() == 3);

        let expected_domains: BTreeSet<String> = BTreeSet::from([
            "example.com".to_string(),
            "www.example.com".to_string(),
            "example.org".to_string(),
        ]);
        assert!(s.domains == expected_domains);
    }
}
