use anyhow;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    fs::File,
    io::{prelude::*, BufReader}, path::Path,
};
// See also [Rust: Domain Name Validation](https://bas-man.dev/post/rust/domain-name-validation/)
use crate::{
    cmd::cache, config::get_shortcuts
};
use crate::utils::{is_domain, norm_string, trim_inline_comments};
use crate::Arguments;
use futures::executor::block_on;
use num_format::{Locale, ToFormattedString};
use std::cmp::Ordering;

pub type Domain = String;
pub type Domains = HashSet<Domain>;
pub type Tag = String;
pub type Tags = Vec<Tag>;
// pub type IPaddress = String;


#[derive(Debug, Default, Clone)]
pub struct Host {
    // ip_address: IPaddress,
    // domain: Domain,
}

// Source: https://users.rust-lang.org/t/structs-with-similar-fields/99065/4
// Source: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e39ad82c6bfa82742428a10ee629c631
macro_rules! with_hosts_collection_shared_fields_and_impl {
    ($(#[$attr:meta])* struct $name:ident { $($field_name:ident: $field_type:tt,)*} ) => {
        $(#[$attr])*
        pub struct $name {
            pub name: String,
            pub location: String,
            pub raw_list: Vec<Domain>,
            pub front_matter: Vec<String>,
            pub domains: Domains,
            pub duplicates: Domains,
            pub invalids: Domains,
            pub args: Arguments,
            $(pub $field_name: $field_type,)*
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.args.quiet {
                    writeln!(f, "{}", self.domains.len())
                } else {
                    writeln!(
                        f,
                        "Name: {}\nLocation: {}\nDomains: {}\nDuplicate domains: {}\nInvalid domains: {}",
                        self.name,
                        self.location,
                        self.domains.len().to_formatted_string(&Locale::en),
                        self.duplicates.len().to_formatted_string(&Locale::en),
                        self.invalids.len().to_formatted_string(&Locale::en)
                    )?;
                    if self.args.showduplicates && self.duplicates.len() > 0 {
                        writeln!(f, "Duplicates list:")?;
                        for dup in &self.duplicates {
                            writeln!(f, "{}", dup)?;
                        }
                    }
                    if self.args.showinvalids && self.invalids.len() > 0 {
                        writeln!(f, "Invalids list:")?;
                        for invalid in &self.invalids {
                            writeln!(f, "{}", invalid)?;
                        }
                    }
                    if self.args.tld {
                        writeln!(f, "TLD:")?;
                        let tlds = self.tld();
                        for tld in tlds {
                            writeln!(f, "{:>15}: {:>8}", tld.0, tld.1.to_formatted_string(&Locale::en))?;
                        }
                    }
                    if self.args.rootdomains {
                        writeln!(f, "Root domains:")?;
                        let rootdomains = self.rootdomains();
                        for rd in rootdomains {
                            writeln!(f, "  {}: {}", rd.0, rd.1.to_formatted_string(&Locale::en))?;
                        }
                    }
                    Ok(())
                }
            }
        }

        impl $name {
            pub fn tld(&self)  -> Vec<(Domain, u32)> {
                // Step 1: Extract TLDs and count occurrences
                let mut count: HashMap<Domain, u32> = HashMap::new();
                for domain in &self.domains {
                    // Split the domain by '.' and get the last part
                    if let Some(tld) = domain.rsplit('.').next() {
                        *count.entry(tld.to_lowercase()).or_insert(0) += 1;
                    }
                }

                // Step 2: Sort the counts in descending order
                let mut count_vec: Vec<_> = count.into_iter().collect();
                count_vec.sort_by(|a, b| if a.1 == b.1 {
                    a.0.cmp(&b.0)
                } else {
                    b.1.cmp(&a.1)
                });
                if self.args.limit > 0 {
                    if count_vec.len() > self.args.limit {
                        count_vec.truncate(self.args.limit)
                    }
                }
                count_vec
            }

            pub fn rootdomains(&self)  -> Vec<(Domain, u32)> {
                // Step 1: Extract TLDs and count occurrences
                let mut count: HashMap<Domain, u32> = HashMap::new();
                for domain in &self.domains {
                    // Split the domain by '.' and get the last two parts
                    let parts: Vec<&str> = domain.split('.').collect();
                    if parts.len() >= 2 {
                        // Join the last two segments to form the root domain
                        let rootdomain = format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]);
                        *count.entry(rootdomain.to_lowercase()).or_insert(0) += 1;
                    }
                }

                // Step 2: Sort the counts in descending order
                let mut count_vec: Vec<_> = count.into_iter().collect();
                count_vec.sort_by(|a, b| if a.1 == b.1 {
                    a.0.cmp(&b.0)
                } else {
                    b.1.cmp(&a.1)
                });
                if self.args.limit > 0 {
                    if count_vec.len() > self.args.limit {
                        count_vec.truncate(self.args.limit)
                    }
                }
                count_vec
            }

            pub fn sorteddomains(&self)  -> Vec<(Domain)> {
                // Function to parse a domain into components: (subdomain, root domain, TLD)
                fn parse_domain(domain: &str) -> Vec<String> {
                    let parts: Vec<&str> = domain.split('.').collect();
                    let tld = parts.last().unwrap().to_string(); // Get TLD
                    let root = parts.get(parts.len() - 2).unwrap_or(&"").to_string(); // Get root domain
                    let subdomain = parts[..parts.len() - 2].join("."); // Join remaining parts as subdomain

                    let mut result = vec![subdomain, root, tld];
                    // If there are no subdomains, push an empty string
                    if result[0].is_empty() {
                        result[0] = "".to_string();
                    }
                    result
                }

                let v = self.domains.clone().into_iter().collect();
                // THIS IS ONLY PARTIALLY IMPLEMENTED
                v
                // ;
                // v.sort_by(|a: &Domain, b: &Domain| {
                //     let a_parts = parse_domain(a);
                //     let b_parts = parse_domain(b);

                //     // Compare by root domain and TLD first
                //     match a_parts[1].cmp(&b_parts[1]) {
                //         Ordering::Equal => {
                //             // Then compare by first-level subdomain
                //             match a_parts[0].cmp(&b_parts[0]) {
                //                 Ordering::Equal => {
                //                     // Finally, compare remaining subdomains
                //                     a_parts[2..].cmp(&b_parts[2..])
                //                 }
                //                 other => other,
                //             }
                //         }
                //         other => other,
                //     }
                // })
            }
        }
    }
}

with_hosts_collection_shared_fields_and_impl!(
    #[derive(Debug, Default, Clone)]
    struct Hostssource {}
);

pub type Hostssources = Vec<Hostssource>;

impl Hostssource {
    pub async fn new(location: impl Into<String>, name: impl Into<String>) -> Hostssource {
        let mut hs = Hostssource {
            name: name.into(),
            ..Default::default()
        };
        // Ignore the result for now.
        _ = hs.load(&location.into()).await;
        hs
    }

    pub async fn load(&mut self, src: &str) -> anyhow::Result<()> {
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
            // if it's a list of domains
            self.raw_list = actualsrc
                .trim()
                .split('\n')
                .map(|l| l.trim().to_string())
                .collect::<Vec<String>>();
            self.location = "text input".to_string();
        } else if clean.starts_with("http") {
            // if it's a URL
            // check the cache
            let cache_file = cache::get(clean.clone());
            if !self.args.skipcache && cache_file.is_some() {
                // read the cache
                if self.args.verbose {
                    println!("==> Loading from cache: {}", src);
                }
                let file = File::open(cache_file.unwrap()).expect(&format!("File does not exist: {}", actualsrc));
                let buf = BufReader::new(file);
                self.raw_list = buf
                    .lines()
                    .map(|l| l.expect("Could not parse line"))
                    .collect();
            } else {
                // if no cache
                if self.args.verbose {
                    println!("==> Loading over HTTP(S): {}", src);
                }
                let resp = reqwest::blocking::get(actualsrc).expect("request failed");
                let body = resp.text().expect("body invalid");
                self.raw_list = body.clone().lines().map(|l| l.to_string()).collect();
                // submit to cache
                _ = cache::set(clean.clone(), body);
            }
        } else if Path::new(actualsrc).exists(){
            // if it's a file
            let file = File::open(actualsrc).expect(&format!("Problem opening file: {}", actualsrc));
            let buf = BufReader::new(file);
            self.raw_list = buf
                .lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();
        } else {
            // To Do: bomb out more gracefully
            panic!("Shortcut, URL, or File \"{}\" does not exist.", actualsrc);
        }
        self.normalize();
        return Ok(());
    }

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
    }

    fn extract_domains(&mut self) {
        let mut domains_result: Domains = HashSet::new();
        // Domain aliases which are often found in hosts files which we do not want
        // to flag as formally invalid.
        let headertokens = vec![
            "::1",
            "broadcasthost",
            "fe80::1%lo0",
            "ff00::0",
            "ff02::1",
            "ff02::2",
            "ff02::3",
            "ip6-allhosts",
            "ip6-allnodes",
            "ip6-allrouters",
            "ip6-localhost",
            "ip6-localnet",
            "ip6-loopback",
            "ip6-mcastprefix",
            "local",
            "localhost"
        ];

        for line in &self.domains {
            for element in line.split_whitespace() {
                if element != "0.0.0.0" && element != "127.0.0.1" {
                    if is_domain(element) {
                        let unique = domains_result.insert(element.to_string());
                        if !unique {
                            self.duplicates.insert(element.to_string());
                        }
                    } else {
                        if !headertokens.contains(&element) {
                            self.invalids.insert(element.to_string());
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

with_hosts_collection_shared_fields_and_impl!(
    #[derive(Default)]
    struct Amalgam {
        sources: Hostssources,
    }
);

impl Amalgam {
    #[allow(dead_code)]
    pub async fn new(locations: Vec<impl Into<String> + Clone>) -> Amalgam {
        let mut amalgam: Amalgam = Amalgam {
            sources: Hostssources::new(),
            front_matter: vec![],
            domains: Domains::new(),
            ..Default::default()
        };
        for l in locations {
            let mut s = block_on(
                Hostssource::new(
                    l.clone().into(),
                    l.into(),
                )
            );
            amalgam.front_matter.append(&mut s.front_matter);
            for domain in s.domains.clone() {
                amalgam.domains.insert(domain);
            }
            amalgam.raw_list.append(&mut s.raw_list.clone());
            amalgam.sources.push(s);
        }
        amalgam
    }
}

#[async_std::test]
async fn test_amalgam() {
    use thousands::Separable;
    let a =
        Amalgam::new(
            vec![
                "stevenblack",
                "mvps",
                "yoyo",
                "someonewhocares",
            ]
        ).await
    ;
    let mut tally: usize = 0;
    for s in a.sources {
        tally += s.domains.len();
        println!("Source {}: {} domains", s.name, s.domains.len().separate_with_commas());
    }
    println!("Total: {} domains in all, {} domains net", tally.separate_with_commas(), a.domains.len().separate_with_commas());
    assert!(tally >= a.domains.len());
}

#[async_std::test]
async fn test_amalgam2() {
    let a =
        Amalgam::new(
            vec![
                "stevenblack",
            ]).await;
        let b =
        Amalgam::new(
            vec![
                "stevenblack",
                "stevenblack",
            ]).await;
    assert!(a.domains.len() == b.domains.len());
}

#[async_std::test]
async fn test_amalgam_shortcuts() {
    use thousands::Separable;
    let a =
        Amalgam::new(
            vec![
                "base",
                "p",
                "p-only",
                "g",
                "g-only",
                "fgps",
            ]
        ).await
    ;
    let mut tally: usize = 0;
    for s in a.sources {
        tally += s.domains.len();
        println!("Source {}: {} domains", s.name, s.domains.len().separate_with_commas());
    }
    println!("Total: {} domains in all, {} domains net", tally.separate_with_commas(), a.domains.len().separate_with_commas());
    assert!(tally >= a.domains.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_task_group::group;
    use futures::executor::block_on;

    #[async_std::test]
    async fn no_task() {
        let handle = group(|group| async move { Ok::<_, ()>(group) });
        assert!(handle.await.is_ok());
    }

    #[async_std::test]
    async fn one_empty_task() {
        let handle = group(|group| async move {
            group.spawn(async move { Ok(()) });
            Ok::<_, ()>(group)
        });
        assert!(handle.await.is_ok());
    }

    // ToDo: skip this test if the folder and file do not exist
    #[test]
    fn test_load_from_file() {
        let mut s = Hostssource {
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(s.load("/Users/Steve/Dropbox/dev/hosts/hosts"));
        assert_eq!(s.location, "/Users/Steve/Dropbox/dev/hosts/hosts");
        assert!(s.front_matter.len() > 0);
        assert!(s.raw_list.len() > 50_000);
        assert!(s.domains.len() > 50_000);
    }

    // ToDo: skip this test if the folder and file do not exist
    #[test]
    fn test_load_from_file_using_new() {
        let s =  block_on(
            Hostssource::new(
               "/Users/Steve/Dropbox/dev/hosts/hosts",
                "arbitrary name",
            )
        );
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
        // ignore the result of this load for now
        _ = block_on(s.load(&url));
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
        // ignore the result of this load for now
        _ = block_on(s.load(&url));
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
        // ignore the result of this load for now
        _ = block_on(s.load("base"));
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
        // ignore the result of this load for now
        _ = block_on(s.load(
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
        // ignore the result of this load for now
        _ = block_on(s.load(
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
        // ignore the result of this load for now
        _ = block_on(s.load(
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

        let expected_domains: HashSet<String> = HashSet::from([
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
        // ignore the result of this load for now
        _ = block_on(s.load(
            r##"
            # test
            # test 2
            0.0.0.0 example.com www.example.com example.org
            # a comment foobar.com
            "##,
        ));
        assert!(s.domains.len() == 3);

        let expected_domains: HashSet<String> = HashSet::from([
            "example.com".to_string(),
            "www.example.com".to_string(),
            "example.org".to_string(),
        ]);
        assert!(s.domains == expected_domains);
    }

    #[test]
    fn test_domains_type() {
        let mut d = Domains::new();
        d.insert("foo.com".to_string());
        d.insert("foo.com".to_string());
        d.insert("bar.com".to_string());
        assert!(d.len() == 2);
        let mut d2 = Domains::new();
        d2.insert("foo.com".to_string());
        d2.insert("foo.com".to_string());
        d2.insert("bar.com".to_string());
        for domain in d2 {
            d.insert(domain);
        }
        assert!(d.len() == 2);
    }
}
