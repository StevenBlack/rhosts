use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader}
};
use futures::Future;
use reqwest::{self, Response};

pub type Domain = String;
pub struct Host {
    ip: String,
    domain: Domain,
}

pub type Hosts = Vec<Host>;

#[derive(Debug, Default)]
pub struct Hostssource {
    pub location: String,
    pub raw_list: Vec<String>,
    pub list_header:  Vec<String>,
	pub domains: Vec<Domain>,
	pub tlds: HashMap<String, i32>,
	pub tldtallies: Vec<i32>,
	pub duplicates: Vec<Domain>
}

// impl HostsMethods for Hostssource {
impl Hostssource {
    #[tokio::main]
    pub async fn load(&mut self, src: &str) {
        self.location = src.to_string();
        let clean = &src.to_lowercase();
        let foo: Response;
        let bar: String;
        if &clean[..5] == "http" {
            let f1 = async {
                foo = reqwest::get(self.location).await?;
                Ok(())
            };

            let f2 = async {
                bar = foo.text().await?;
                Ok(())
            };

            self.raw_list = bar
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
    }

    fn normalize(&mut self) {
        self.trimlines();
        self.saveheader();
        self.removeblanklines();
        self.removecommentlines();
    }

    fn trimlines(&mut self) {
        let mut lines: Vec<String> = self.raw_list.clone();
        lines
        .iter_mut()
        .for_each(|line| *line = line
            .trim()
            .to_string()
        );
        self.domains = lines.clone();
    }

    fn removeblanklines(&mut self) {
        let mut lines: Vec<String> = self.domains.clone();
        lines.retain(|line | line.chars().count() > 0);
        self.domains = lines;
    }

    fn removecommentlines(&mut self) {
        let mut lines: Vec<String> = self.domains.clone();
        lines.retain(|line | !line.starts_with("#"));
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
}

#[test]
fn test_load() {
    // assert_eq!(2 + 2, 4);
    let mut s = Hostssource{
        ..Default::default()
    };
    s.load("/Users/Steve/Dropbox/dev/hosts/hosts");
    assert_eq!(s.location, "/Users/Steve/Dropbox/dev/hosts/hosts");
    assert!(s.list_header.len() > 0);
    assert!(s.raw_list.len() > 0);
    assert!(s.domains.len() > 0);
}
