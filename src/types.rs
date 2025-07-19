use anyhow;
use indexmap::IndexSet;
use std::{
    collections::HashMap,
    fmt,
    fmt::Display,
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
pub type Domains = IndexSet<Domain>;
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
            pub _name: String,
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
                        // "Domain report for: {}\n{}\nDomains: {}\nDuplicate domains: {}\nInvalid domains: {}",
                        "Domain report for: {}\n{}\nDomains: {}",
                        self.location,
                        format!("🔎: {}",self.location.replace("https://raw.githubusercontent.com/","")),
                        self.domains.len().to_formatted_string(&Locale::en),
                        // self.duplicates.len().to_formatted_string(&Locale::en),
                        // self.invalids.len().to_formatted_string(&Locale::en)
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

                    if self.args.tld && self.args.rootdomains {
                        // lay them up side by side
                        writeln!(f, "Top {} TLD and root domains:", self.args.limit)?;
                        let tld = self.tld();
                        let rootdomains = self.rootdomains();
                        let left_pad = tld.iter().map(|(tld, count)| format!("{:>10}: {:>7}", tld, count.to_formatted_string(&Locale::en)).len()).max().unwrap_or(0);
                        for ((tld, tld_count), (root, root_count)) in tld.iter().zip(rootdomains.iter()) {
                            writeln!(f, "{:width$}  {}: {}", format!("{:>10}: {:>7}   ", format!(".{}", tld), tld_count.to_formatted_string(&Locale::en)), root, root_count.to_formatted_string(&Locale::en), width = left_pad)?;
                        }
                    } else {
                        if self.args.tld {
                            writeln!(f, "TLD:")?;
                            let tlds = self.tld();
                            for tld in tlds {
                                writeln!(f, "  {:>10}: {:>7}", format!(".{}", tld.0), tld.1.to_formatted_string(&Locale::en))?;
                            }
                        }
                        if self.args.rootdomains {
                            writeln!(f, "Root domains:")?;
                            let rootdomains = self.rootdomains();
                            for rd in rootdomains {
                                writeln!(f, "  {}: {}", rd.0, rd.1.to_formatted_string(&Locale::en))?;
                            }
                        }
                    }
                    if self.args.subdomains {
                        writeln!(f, "Subdomains:")?;
                        let subdomains = self.subdomains();
                        for sd in subdomains {
                            writeln!(f, "  {}: {}", sd.0, sd.1.to_formatted_string(&Locale::en))?;
                        }
                        // chunking works within subdomains
                        if let Some(chunk_size) = self.args.chunking {
                            if let Some(chunked_subdomains) = self.chunked_subdomains() {
                                writeln!(f, "Chunked subdomains ({} characters):", chunk_size)?;
                                for cs in chunked_subdomains {
                                    writeln!(f, "  {}: {}", cs.0, cs.1.to_formatted_string(&Locale::en))?;
                                }
                            }
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

            pub fn subdomains(&self) -> Vec<(String, u32)> {
                let mut count: HashMap<String, u32> = HashMap::new();
                for domain in &self.domains {
                    let parts: Vec<&str> = domain.split('.').collect();
                    // Ignore TLD and root domain (last two parts)
                    if parts.len() > 2 {
                        for sub in &parts[..parts.len() - 2] {
                            if sub.len() > 3 {
                                *count.entry(sub.to_lowercase()).or_insert(0) += 1;
                            }
                        }
                    }
                }
                // Sort by count descending, then alphabetically
                let mut count_vec: Vec<_> = count.into_iter().collect();
                count_vec.sort_by(|a, b| if a.1 == b.1 {
                    a.0.cmp(&b.0)
                } else {
                    b.1.cmp(&a.1)
                });
                if self.args.limit > 0 && count_vec.len() > self.args.limit {
                    count_vec.truncate(self.args.limit);
                }
                count_vec
            }

            pub fn chunked_subdomains(&self) -> Option<Vec<(String, u32)>> {
                if self.args.chunking.is_none() {
                    return None; // If chunking is not specified, return None
                }
                let chunk_size = self.args.chunking.unwrap();
                let mut count: HashMap<String, u32> = HashMap::new();
                for domain in &self.domains {
                    let parts: Vec<&str> = domain.split('.').collect();
                    // Ignore TLD and root domain (last two parts)
                    if parts.len() > 2 {
                        for sub in &parts[..parts.len() - 2] {
                            // Only consider subdomains longer than 3 characters
                            if sub.len() > 3 && sub.len() > chunk_size {
                                // cycle through the chunks of the subdomain
                                for i in 0..=sub.len().saturating_sub(chunk_size) {
                                    let chunk_str = &sub[i..i + chunk_size];
                                    *count.entry(chunk_str.to_lowercase()).or_insert(0) += 1;
                                }
                                // for chunk in sub.chars().collect::<Vec<_>>().chunks(chunk_size) {
                                //     let chunk_str = chunk.iter().collect::<String>();
                                //     *count.entry(chunk_str.to_lowercase()).or_insert(0) += 1;
                                // }
                            }
                        }
                    }
                }
                // Sort by count descending, then alphabetically
                let mut count_vec: Vec<_> = count.into_iter().collect();
                count_vec.sort_by(|a, b| if a.1 == b.1 {
                    a.0.cmp(&b.0)
                } else {
                    b.1.cmp(&a.1)
                });
                if self.args.limit > 0 && count_vec.len() > self.args.limit {
                    count_vec.truncate(self.args.limit);
                }
                Some(count_vec)
            }

            pub fn sorteddomains(&self)  -> Vec<Domain> {
                // Function to parse a domain into components: (subdomain, root domain, TLD)
                fn parse_domain(domain: &str) -> Vec<String> {
                    let parts: Vec<&str> = domain.split('.').collect();
                    let tld = parts.last().unwrap().to_string(); // Get TLD
                    let root = parts.get(parts.len() - 2).unwrap_or(&"").to_string(); // Get root domain
                    let subdomain = parts[..parts.len() - 2].join("."); // Join remaining parts as subdomain

                    let mut r = vec![subdomain, root, tld];
                    // If there are no subdomains, push an empty string
                    if r[0].is_empty() {
                        r[0] = "".to_string();
                    }
                    r
                }

                let mut v: Vec<Domain> = self.domains.clone().into_iter().collect();
                v.sort_by(|a, b| {
                    let a_parts = parse_domain(a);
                    let b_parts = parse_domain(b);

                    // Compare by root domain and TLD first
                    match a_parts[1].cmp(&b_parts[1]) {
                        Ordering::Equal => {
                            // Then compare by first-level subdomain
                            match a_parts[0].cmp(&b_parts[0]) {
                                Ordering::Equal => {
                                    // Finally, compare remaining subdomains
                                    a_parts[2..].cmp(&b_parts[2..])
                                }
                                other => other,
                            }
                        }
                        other => other,
                    }
                });
                v
            }
        }

        impl Comparable for $name {
            fn get_domains(&self) -> &IndexSet<Domain> {
                &self.domains
            }
            fn get_args(&self) -> &Arguments {
                &self.args
            }
        }
    }
}

with_hosts_collection_shared_fields_and_impl!(
    #[derive(Debug, Default, Clone)]
    struct Hostssource {}
);


pub trait Comparable: Display + Send + Sync {
    fn get_domains(&self) -> &IndexSet<Domain>;
    fn get_args(&self) -> &Arguments;

    fn compare(&self, thing: Box<dyn Comparable + Send + Sync>) {
        println!("{}", self);
        println!("{}", thing);
        if self.get_args().intersection_list {
            _ = self.intersection(thing);
        }
    }

    /// Tally the intersection of two domain lists
    fn intersection(&self, comp: Box<dyn Comparable + Send + Sync>) -> () {
        let first = self.get_domains().len();
        let second = comp.get_domains().len();
        let mut combined = self.get_domains().clone();
        for domain in comp.get_domains().clone() {
            combined.insert(domain);
        }
        println!("Intersection: {} domains", (first + second - combined.len()).to_formatted_string(&Locale::en));
        ()
    }
}

pub type Hostssources = Vec<Hostssource>;

impl Hostssource {
    pub async fn new(location: impl Into<String>, name: impl Into<String>) -> Hostssource {
        let mut hs = Hostssource {
            _name: name.into(),
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
        let shortcut = shortcuts.get(src);
        if shortcut.is_some() {
            self.location = shortcut.unwrap().to_string();
            actualsrc = self.location.as_str();
        } else {
            self.location = actualsrc.to_string();
        }

        let normalizedsrc = actualsrc.to_lowercase();

        if actualsrc.contains('\n') {
            // if it's a list of domains
            self.raw_list = actualsrc
                .trim()
                .split('\n')
                .map(|l| l.trim().to_string())
                .collect::<Vec<String>>();
            self.location = "text input".to_string();
        } else if normalizedsrc.starts_with("http") {
            // if it's a URL
            // check the cache
            let cache_file = cache::get(normalizedsrc.clone()).await;
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
                _ = cache::set(normalizedsrc.clone(), body);
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
        if self.args.domains_sort {
            let sorted = self.sorteddomains();
            self.domains.drain(..);
            self.domains = sorted.into_iter().collect();
        }
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
        let mut domains_result: Domains = IndexSet::new();
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
            "localhost",
            "localhost.localdomain"
        ];

        for line in &self.domains {
            for element in line.split_whitespace() {
                if element != "0.0.0.0"
                && element != "127.0.0.1"
                && element != "255.255.255.255"
                && !headertokens.contains(&element) {
                    if is_domain(element) {
                        let unique = domains_result.insert(element.to_string());
                        if !unique {
                            self.duplicates.insert(element.to_string());
                        }
                    } else {
                        self.invalids.insert(element.to_string());
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
async fn test_create_amalgam_with_lists_has_domains() {
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
        println!("Source {}: {} domains", s._name, s.domains.len().separate_with_commas());
    }
    println!("Total: {} domains in all, {} domains net", tally.separate_with_commas(), a.domains.len().separate_with_commas());
    assert!(
        tally >= a.domains.len(),
        "Expected total domains to be greater than or equal to net domains"
    );
}

#[async_std::test]
async fn test_create_amalgam_with_duplicate_lists_does_not_double_count_domains() {
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
    assert!(
        a.domains.len() == b.domains.len(),
        "Expected resultant number of domainsto be equal"
    );
}

#[async_std::test]
async fn test_create_amalgam_with_shortcuts_has_domains() {
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
        println!("Source {}: {} domains", s._name, s.domains.len().separate_with_commas());
    }
    println!("Total: {} domains in all, {} domains net", tally.separate_with_commas(), a.domains.len().separate_with_commas());
    assert!(
        tally >= a.domains.len(),
        "Expected total domains to be greater or equal to net domains"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_task_group::group;
    use futures::executor::block_on;

    #[async_std::test]
    async fn no_task() {
        let handle = group(|group| async move { Ok::<_, ()>(group) });
        assert!(
            handle.await.is_ok(),
            "Expectred task group to be ok"
        );
    }

    #[async_std::test]
    async fn one_empty_task() {
        let handle = group(|group| async move {
            group.spawn(async move { Ok(()) });
            Ok::<_, ()>(group)
        });
        assert!(
            handle.await.is_ok(),
            "Expected task group to be ok"
        );
    }

    // ToDo: skip this test if the folder and file do not exist
    #[test]
    fn test_hostssource_load_from_file_has_domains() {
        let mut s = Hostssource {
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(s.load("/Users/Steve/Dropbox/dev/hosts/hosts"));
        assert_eq!(
            s.location, "/Users/Steve/Dropbox/dev/hosts/hosts",
            "Loading from local file, expected location to be /Users/Steve/Dropbox/dev/hosts/hosts"
        );
        assert!(
            s.front_matter.len() > 0,
            "Loading from local file, expected front matter length to be greater than 0"
        );
        assert!(
            s.raw_list.len() > 1_000,
            "Loading from local file, expected raw list length to be greater than 1,000"
        );
        assert!(
            s.domains.len() > 1_000,
            "Loading from local file, expected number of domains to be greater than 1,000"
        );
    }

    // ToDo: skip this test if the folder and file do not exist
    #[test]
    fn test_hostssource_new_from_file_has_domains() {
        let s =  block_on(
            Hostssource::new(
               "/Users/Steve/Dropbox/dev/hosts/hosts",
                "arbitrary name",
            )
        );
        assert_eq!(
            s.location,
            "/Users/Steve/Dropbox/dev/hosts/hosts",
            "Loading from local file, expected location to be /Users/Steve/Dropbox/dev/hosts/hosts");
        assert!(
            s.front_matter.len() > 0,
            "Loading from local file, expected front matter length to be greater than 0"
        );
        assert!(
            s.raw_list.len() > 1_000,
            "Loading from local file, expected raw list length to be greater than 1,000"
        );
        assert!(
            s.domains.len() > 1_000,
            "Loading from local file, expected number of domains to be greater than 1,000"
        );
        assert!(
            s.raw_list.len() > 1_000,
            "Loading from local file, expected raw list length to be greater than 1,000"
        );
        assert!(
            s.domains.len() > 1_000,
            "Loading from local file, expected number of domains to be greater than 1,000"
        );
    }

    #[test]
    fn test_hostssource_load_from_github_has_domains() {
        let mut s = Hostssource {
            ..Default::default()
        };
        let url = "https://raw.githubusercontent.com/StevenBlack/hosts/f5d5efab/data/URLHaus/hosts";
        // ignore the result of this load for now
        _ = block_on(s.load(&url));
        assert_eq!(
            s.location,
            url.to_string(),
            "Loading from GitHub, expected location to and URL to be the same"
        );
        assert!(
            s.front_matter.len() > 4,
            "Loading from GitHub, expected front matter length to be greater than 4"
        );
        assert!(
            s.raw_list.len() > 1000,
            "Loading from GitHub, expected raw list length to be greater than 1,000"
        );
        assert!(
            s.domains.len() > 1000,
            "Loading from GitHub, expected number of domains to be greater than 1,000"
        );
    }

    #[test]
    fn test_hostssource_load_big_file_from_github_has_domains() {
        let mut s = Hostssource {
            ..Default::default()
        };
        let url = "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts";
        // ignore the result of this load for now
        _ = block_on(s.load(&url));
        assert_eq!(
            s.location, url.to_string(),
            "Loading a big file from GitHub, expected location to and URL to be the same"
        );
        assert!(
            s.front_matter.len() > 4,
            "Loading a big file from GitHub, expected front matter length to be greater than 4"
        );
        assert!(
            s.raw_list.len() > 1_000,
            "Loading a big file from GitHub, expected raw list length to be greater than 1,000"
        );
        assert!(
            s.domains.len() > 1_000,
            "Loading a big file from GitHub, expected number of domains to be greater than 1,000"
        );
    }

    #[test]
    fn test_hostssource_load_from_shortcut_has_domains() {
        let mut s = Hostssource {
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(s.load("base"));
        assert_eq!(
            s.location,
            "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts",
            "When using a shortcut, expected location to and URL to be the same"
        );
        assert!(
            s.front_matter.len() > 0,
            "When using a shortcut, expected front matter length to be greater than 0"
        );
        assert!(
            s.raw_list.len() > 1_000,
            "When using a shortcut, expected raw list length to be greater than 1,000"
        );
        assert!(
            s.domains.len() > 1_000,
            "When using a shortcut, expected number of domains to be greater than 1,000"
        );
    }

    #[test]
    fn test_hostssource_load_from_multi_line_text_has_domains() {
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
        assert!(
            s.front_matter.len() == 2,
            "Expected front matter length to be 2, but got: {}",
            s.front_matter.len()
        );
        assert!(
            s.raw_list.len() == 5,
            "Expected raw list length to be 5, but got: {}",
            s.raw_list.len()
        );
        assert!(
            s.domains.len() == 2,
            "Expected the number of domains to be 2, but got: {}",
            s.domains.len()
        );
        assert!(
            s.duplicates.len() == 0,
            "Expected the number of duplicates to be 0, but got: {}",
            s.duplicates.len()
        );
        assert!(
            s.invalids.len() == 0,
            "Expected the number of invalids to be 0, but got: {}",
            s.invalids.len()
        );
    }

    #[test]
    fn test_hostssource_load_from_single_line_text_has_domains() {
        let mut s = Hostssource {
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(s.load(
            r##"
            0.0.0.0 example.com
            "##,
        ));
        assert!(
            s.front_matter.len() == 0
            , "Expected front matter to be 0, but got: {}",
            s.front_matter.len()
        );
        assert!(
            s.raw_list.len() == 1,
            "Expected raw list length to be 1, but got: {}",
            s.raw_list.len()
        );
        assert!(
            s.domains.len() == 1,
            "Expected the number of domains to be 1, but got: {}",
            s.domains.len()
        );
        assert!(
            s.duplicates.len() == 0,
            "Expected the number of duplicates to be 0, but got: {}",
            s.duplicates.len()
        );
        assert!(
            s.invalids.len() == 0,
            "Expected the number of invalids to be 0, but got: {}",
            s.invalids.len()
        );
    }

    #[test]
    fn test_hostssource_load_from_multi_line_text_with_duplicates_has_no_duplicate_domains() {
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
        assert!(
            s.front_matter.len() == 2,
            "Expected front matter to be 2, but got: {}",
            s.front_matter.len()
        );
        assert!(
            s.raw_list.len() == 5,
            "Expected raw list length to be 5, but got: {}",
            s.raw_list.len()
        );
        assert!(
            s.domains.len() == 2,
            "Expected the number of domains to be 2, but got: {}",
            s.domains.len()
        );
        assert!(
            s.duplicates.len() == 1,
            "Expected the number of duplicates to be 1, but got: {}",
            s.duplicates.len()
        );
    }

    #[test]
    fn test_hostssource_load_from_multi_line_text_with_multiple_domains_per_line_produces_normalized_list_of_domains() {
        let mut s = Hostssource {
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(s.load(
            r##"
            # comment line
            # comment line 2
            0.0.0.0 example.com
            0.0.0.0 www.example.com
            127.0.0.1 example.org www.example.org
            127.0.0.1 something.org
            # some other comment
            127.0.0.1 something.else.org
            "##,
        ));
        assert!(
            s.domains.len() == 6,
            "Expected domains to be 6, but got: {}",
            s.domains.len()
        );

        let expected_domains: IndexSet<String> = IndexSet::from([
            "example.com".to_string(),
            "www.example.com".to_string(),
            "example.org".to_string(),
            "www.example.org".to_string(),
            "something.org".to_string(),
            "something.else.org".to_string(),
        ]);
        assert!(
            s.domains == expected_domains,
            "Expected domains to be identical, but got: {:?} expected: {:?}",
            s.domains,
            expected_domains);
    }

    #[test]
    fn test_hostssource_load_from_multi_line_text_with_three_domains_per_line_produces_normalized_list_of_domains() {
        let mut s = Hostssource {
            ..Default::default()
        };
        // ignore the result of this load for now
        _ = block_on(s.load(
            r##"
            # comment line
            # comment line 2
            0.0.0.0 example.com www.example.com example.org
            # some other comment
            "##,
        ));
        assert!(s.domains.len() == 3);

        let expected_domains: IndexSet<String> = IndexSet::from([
            "example.com".to_string(),
            "www.example.com".to_string(),
            "example.org".to_string(),
        ]);
        assert!(
            s.domains == expected_domains,
            "Expected domains to be identical, but got: {:?} expected: {:?}",
            s.domains,
            expected_domains
        );
    }

    #[test]
    fn test_domains_type_inserting_duplicates_does_not_produce_duplicate_domains() {
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
        assert!(
            d.len() == 2,
            "Expected domain set to be 2, but got {}",
            d.len()
        );
    }
}
