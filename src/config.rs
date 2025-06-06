#![allow(dead_code)]
use std::{
    collections::BTreeMap,
    path::PathBuf,
    fs, fmt,
};
use anyhow::anyhow;

use crate::{Arguments, types::Tags, utils::{Combinations, flatten}};
// use crate::alloc::{Allocator, Global};
extern crate directories;
// use directories::{BaseDirs, ProjectDirs, UserDirs};
use directories::ProjectDirs;

/// print configuration information
pub fn info(_args:Arguments) -> anyhow::Result<()> {
    println!("Configuration:");
    println!("Local config file: {}", get_config_file()?.to_string_lossy());
    Ok(())
}

pub fn init(_args:Arguments) -> anyhow::Result<()> {
    Ok(())
}

pub fn get_config_file() -> anyhow::Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "rh") {
        let config_dir = proj_dirs.config_dir();
        // Lin: /home/alice/.config/rh/rh.json
        // Win: C:\Users\Alice\AppData\rh\rh.json
        // Mac: /Users/Alice/Library/Application Support/rh/rh.json
        if !config_dir.exists() {
            // create the folder if it does not exists
            fs::create_dir_all(config_dir)?;
        }
        let config_file =  config_dir.join("rh.json");
        if !config_file.exists() {
            // create the file if it does not exist
            fs::File::create(&config_file)?;
        }
        return Ok(config_file);
    }
    return Err(anyhow!("Error reckoning config file."));
}


#[allow(dead_code)]
pub fn read_config_file() -> String {
    let config_file = get_config_file();
    if config_file.is_ok() {
        let config_file_contents_result =
            fs::read_to_string(config_file.expect("Problem with config file."));
        let configdata = match config_file_contents_result {
            Ok(file) => {
                let j = serde_json::from_str(&file);
                if j.is_ok() {
                    j.unwrap()
                } else {
                    "{}".to_string()
                }
            },
            Err(_) => "File read error".to_string(),
        };
        configdata
    } else {
        "".to_string()
    }
}

pub fn get_shortcuts() -> BTreeMap<String, String> {
    let mut ret = BTreeMap::new();
    ret.insert(
        "b".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".to_string(),
    );
    ret.insert(
        "base".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".to_string(),
    );
    ret.insert(
        "f".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews/hosts"
            .to_string(),
    );
    ret.insert(
        "f-only".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-only/hosts"
            .to_string(),
    );
    ret.insert(
        "fg".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling/hosts"
            .to_string(),
    );
    ret.insert(
        "fgp".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn/hosts"
            .to_string(),
    );
    ret.insert(
        "fgps".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn-social/hosts"
            .to_string(),
    );
    ret.insert(
        "fgs".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-social/hosts"
            .to_string(),
    );
    ret.insert(
        "fp".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-porn/hosts"
            .to_string(),
    );
    ret.insert(
        "fps".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-porn-social/hosts"
            .to_string(),
    );
    ret.insert(
        "fs".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-social/hosts"
            .to_string(),
    );
    ret.insert(
        "g".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling/hosts"
            .to_string(),
    );
    ret.insert(
        "g-only".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-only/hosts"
            .to_string(),
    );
    ret.insert(
        "gp".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-porn/hosts"
            .to_string(),
    );
    ret.insert(
        "gps".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-porn-social/hosts"
            .to_string(),
    );
    ret.insert(
        "gs".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-social/hosts"
            .to_string(),
    );
    ret.insert(
        "p".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn/hosts"
            .to_string(),
    );
    ret.insert(
        "p-only".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn-only/hosts"
            .to_string(),
    );
    ret.insert(
        "ps".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn-social/hosts"
            .to_string(),
    );
    ret.insert(
        "s".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/social/hosts"
            .to_string(),
    );
    ret.insert(
        "s-only".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/social-only/hosts"
            .to_string(),
    );
    ret.insert(
        "adaway".to_string(),
        // adaway is paused
        // "https://raw.githubusercontent.com/AdAway/adaway.github.io/master/hosts.txt".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/refs/heads/master/data/adaway.org/hosts".to_string(),
    );
    ret.insert(
        "add2o7net".to_string(),
        "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.2o7Net/hosts"
            .to_string(),
    );
    ret.insert(
        "adddead".to_string(),
        "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Dead/hosts".to_string(),
    );
    ret.insert(
        "addrisk".to_string(),
        "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Risk/hosts".to_string(),
    );
    ret.insert(
        "addspam".to_string(),
        "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Spam/hosts".to_string(),
    );
    ret.insert(
        "baddboyz".to_string(),
        "https://raw.githubusercontent.com/mitchellkrogza/Badd-Boyz-Hosts/master/hosts".to_string(),
    );
    ret.insert(
        "clefspear".to_string(),
        // clefspear is paused
        // "https://raw.githubusercontent.com/Clefspeare13/pornhosts/master/0.0.0.0/hosts".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/extensions/porn/clefspeare13/hosts".to_string(),
    );
    ret.insert(
        "fakenews".to_string(),
        "https://raw.githubusercontent.com/marktron/fakenews/master/fakenews".to_string(),
    );
    ret.insert(
        "hostsvn".to_string(),
        "https://raw.githubusercontent.com/bigdargon/hostsVN/master/option/hosts-VN".to_string(),
    );
    ret.insert(
        "kadhosts".to_string(),
        "https://raw.githubusercontent.com/FiltersHeroes/KADhosts/master/KADhosts.txt"
            .to_string(),
    );
    ret.insert(
        "mvps".to_string(),
        // mvps is paused
        // "https://winhelp2002.mvps.org/hosts.txt".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/refs/heads/master/data/mvps.org/hosts".to_string(),
    );
    ret.insert(
        "sinfonietta-gambling".to_string(),
        "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/gambling-hosts".to_string(),
    );
    ret.insert(
        "sinfonietta-porn".to_string(),
        "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/pornography-hosts"
            .to_string(),
    );
    ret.insert(
        "sinfonietta-snuff".to_string(),
        "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/snuff-hosts".to_string(),
    );
    ret.insert(
        "sinfonietta-social".to_string(),
        "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/social-hosts".to_string(),
    );
    ret.insert(
        "someonewhocares".to_string(),
        "https://someonewhocares.org/hosts/zero/hosts".to_string(),
    );
    ret.insert(
        "stevenblack".to_string(),
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/data/StevenBlack/hosts"
            .to_string(),
    );
    ret.insert(
        "tiuxo-porn".to_string(),
        "https://raw.githubusercontent.com/tiuxo/hosts/master/porn".to_string(),
    );
    ret.insert(
        "tiuxo".to_string(),
        "https://raw.githubusercontent.com/tiuxo/hosts/master/ads".to_string(),
    );
    ret.insert(
        "uncheckyads".to_string(),
        "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/UncheckyAds/hosts"
            .to_string(),
    );
    ret.insert(
        "urlhaus".to_string(),
        "https://urlhaus.abuse.ch/downloads/hostfile/".to_string(),
    );
    ret.insert(
        "yoyo".to_string(),
        "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext&useip=0.0.0.0"
            .to_string(),
    );
    ret
}

use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Component {
    pub name: String,
    pub destination: String,
    pub tags: Tags,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"name\": {}, \"destination\": {}, \"tags\": {:?}", self.name, self.destination, self.tags)
    }
}

pub type Components = Vec<Component>;

// HERE PLAYING WITH AN ALTERNATE WAY OF DEFINING Components

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Komponents(Vec<Component>);

// impl fmt::Display for Komponents {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Values:\n")?;
//         for v in &self.0 {
//             write!(f, "\t{}", v)?;
//         }
//         Ok(())
//     }
// }

// impl<'a, Komponent, A: Allocator> IntoIterator for &'a Vec<Component, A> {
//     type Item = &'a Component;
//     type IntoIter = slice::Iter<'a, Component>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter()
//     }
// }

// impl<'a, Komponent, A: Allocator> IntoIterator for &'a mut Vec<Component, A> {
//     type Item = &'a mut Component;
//     type IntoIter = slice::IterMut<'a, Component>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter_mut()
//     }
// }

// impl Komponents {
//     #[inline]
//     pub fn len(&self) -> usize {
//         self.0.len()
//     }

//     pub fn retain<F>(&mut self, mut f: F)
//     where
//         F: FnMut(&Component) -> bool,
//     {
//         self.0.retain_mut(|elem| f(elem));
//     }
// }



#[allow(dead_code)]
pub fn get_products_json() -> String {
    let products = r#"[
        {
            "name": "base",
            "destination": "./",
            "tags": ["base"]
        },
        {
            "name": "f-only",
            "destination": "./alternates/fakenews/only",
            "tags": ["fakenews"]
        },
        {
            "name": "f",
            "destination": "./alternates/fakenews",
            "tags": ["base", "fakenews"]
        },
        {
            "name": "fg",
            "destination": "./alternates/fakenews-gamnbling",
            "tags": ["base", "fakenews", "gambling"]
        },
        {
            "name": "fgp",
            "destination": "./alternates/fakenews-gambling-porn",
            "tags": ["base", "fakenews", "gambling", "porn"]
        },
        {
            "name": "fgps",
            "destination": "./alternates/fakenews-gambling-porn-social",
            "tags": ["base", "fakenews", "gambling", "porn", "social"]
        },
        {
            "name": "fgs",
            "destination": "./alternates/fakenews-gambling-social",
            "tags": ["base", "fakenews", "gambling", "social"]
        },
        {
            "name": "fp",
            "destination": "./alternates/fakenews-porn",
            "tags": ["base", "fakenews", "porn"]
        },
        {
            "name": "fps",
            "destination": "./alternates/fakenews-porn-social",
            "tags": ["base", "fakenews", "porn", "social"]
        },
        {
            "name": "fs",
            "destination": "./alternates/fakenews-social",
            "tags": ["base", "fakenews", "social"]
        },
        {
            "name": "g-only",
            "destination": "./alternates/gambling/only",
            "tags": ["gambling"]
        },
        {
            "name": "g",
            "destination": "./alternates/gambling",
            "tags": ["base", "gambling"]
        },
        {
            "name": "gp",
            "destination": "./alternates/gambling-porn",
            "tags": ["base", "gambling", "porn"]
        },
        {
            "name": "gps",
            "destination": "./alternates/gambling-porn-social",
            "tags": ["base", "gambling", "porn", "social"]
        },
        {
            "name": "gs",
            "destination": "./alternates/gambling-social",
            "tags": ["base", "gambling", "social"]
        },
        {
            "name": "p-only",
            "destination": "./alternates/porn/only",
            "tags": ["porn"]
        },
        {
            "name": "p",
            "destination": "./alternates/porn",
            "tags": ["base", "porn"]
        },
        {
            "name": "ps",
            "destination": "./alternates/porn-social",
            "tags": ["base", "porn", "social"]
        },
        {
            "name": "s-only",
            "destination": "./alternates/social/only",
            "tags": ["social"]
        },
        {
            "name": "s",
            "destination": "./alternates/social",
            "tags": ["base", "social"]
        }
    ]"#.trim().to_string();
    products
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    sources: SourcesSpecs,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceSpec {
    pub name: String,
    pub url: String,
    pub destination: String,
    pub tags: Tags,
}

type SourcesSpecs = Vec<SourceSpec>;

impl fmt::Display for SourceSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "name: {}, destination: {}, tags: {:?}",
            self.name, self.destination, self.tags
        )
    }
}

pub fn gettaggroups() -> Vec<Vec<String>> {
    let tags = get_unique_tags();
    let mut taggroups = vec!();
    for n in 1..tags.len() +1 {
        let groupsvec: Vec<_> = Combinations::new(tags.clone(), n).collect();
        taggroups.push(groupsvec);
        // println!("{:?}", groupsvec);
    }
    flatten(taggroups)
}

#[allow(dead_code)]
pub fn get_unique_tags() -> Tags {
    // yields all the unique tags we have
    use array_tool::vec::Uniq;
    let json = get_sources_json();
    let config: SourcesSpecs = serde_json::from_str(json.as_str()).expect("Invalid JSON for getting tags.");
    let mut tags: Tags= vec!();
    for x in config {
        for t in x.tags {
            tags.push(t);
        }
    }
    let mut uniquetags = tags.unique();
    uniquetags.sort();
    uniquetags
}

#[allow(dead_code)]
pub fn get_sources_by_tag(tag: String) -> Vec<SourceSpec> {
    let json = get_sources_json();
    let config: SourcesSpecs = serde_json::from_str(json.as_str()).expect("Invalid JSON for getting tags.");
    let mut sources = vec!();
    for x in config {
        if x.tags.contains(&tag) {
            sources.push(x);
        }
    }
    sources
}

#[allow(dead_code)]
pub fn get_source_names_by_tag(tag: String) -> Vec<String> {
    let json = get_sources_json();
    let config: SourcesSpecs = serde_json::from_str(json.as_str()).expect("Invalid JSON for getting tags.");
    let mut sources = vec!();
    for x in config {
        if x.tags.contains(&tag) {
            sources.push(x.name);
        }
    }
    sources
}


#[allow(dead_code)]
pub fn get_sources_json() -> String {
    let sources = r#"[
        {
            "name": "adaway",
            "url": "https://raw.githubusercontent.com/AdAway/adaway.github.io/master/hosts.txt",
            "destination": "./data/adaway.org",
            "tags": ["base"]
        },
        {
            "name": "add2o7net",
            "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.2o7Net/hosts",
            "destination": "./data/add.2o7net",
            "tags": ["base"]
        },
        {
            "name": "adddead",
            "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Dead/hosts",
            "destination": "./data/add.dead",
            "tags": ["base"]
        },
        {
            "name": "addrisk",
            "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Risk/hosts",
            "destination": "./data/add.risk",
            "tags": ["base"]
        },
        {
            "name": "addspam",
            "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Spam/hosts",
            "destination": "./data/add.spam",
            "tags": ["base"]
        },
        {
            "name": "baddboyz",
            "url": "https://raw.githubusercontent.com/mitchellkrogza/Badd-Boyz-Hosts/master/hosts",
            "destination": "./data/Badd-Boyz-Hosts",
            "tags": ["base"]
        },
        {
            "name": "bigdargon-gambling",
            "url": "https://raw.githubusercontent.com/bigdargon/hostsVN/master/extensions/gambling/hosts",
            "destination": "./extensions/gambling/bigdargon/",
            "tags": ["gambling"]
        },
        {
            "name": "bigdargon-porn",
            "url": "https://raw.githubusercontent.com/bigdargon/hostsVN/master/extensions/adult/hosts",
            "destination": "./extensions/porn/bigdargon/",
            "tags": ["porn"]
        },
        {
            "name": "clefspear",
            "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/extensions/porn/clefspeare13/hosts",
            "destination": "./extensions/porn/clefspeare13/",
            "tags": ["porn"]
        },
        {
            "name": "marktron-fakenews",
            "url": "https://raw.githubusercontent.com/marktron/fakenews/master/fakenews",
            "destination": "./extensions/fakenews",
            "tags": ["fakenews"]
        },
        {
            "name": "hostsvn",
            "url": "https://raw.githubusercontent.com/bigdargon/hostsVN/master/option/hosts-VN",
            "destination": "./data/hostsVN",
            "tags": ["base"]
        },
        {
            "name": "kadhosts",
            "url": "https://raw.githubusercontent.com/FiltersHeroes/KADhosts/master/KADhosts.txt",
            "destination": "./data/KADhosts",
            "tags": ["base"]
        },
        {
            "name": "mvps",
            "url": "https://winhelp2002.mvps.org/hosts.txt",
            "destination": "./data/mvps.org",
            "tags": ["base"]
        },
        {
            "name": "sinfonietta-gambling",
            "url": "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/gambling-hosts",
            "destination": "./extensions/gambling",
            "tags": ["gambling"]
        },
        {
            "name": "sinfonietta-porn",
            "url": "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/pornography-hosts",
            "destination": "./extensions/porn/sinfonietta",
            "tags": ["porn"]
        },
        {
            "name": "sinfonietta-snuff",
            "url": "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/snuff-hosts",
            "destination": "./extensions/porn/sinfonietta-snuff",
            "tags": ["porn"]
        },
        {
            "name": "sinfonietta-social",
            "url": "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/social-hosts",
            "destination": "./extensions/social/sinfonietta",
            "tags": ["social"]
        },
        {
            "name": "someonewhocares",
            "url": "https://someonewhocares.org/hosts/zero/hosts",
            "destination": "./data/someonewhocares.org",
            "tags": ["base"]
        },
        {
            "name": "stevenblack",
            "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/data/StevenBlack/hosts",
            "destination": "./data/StevenBlack",
            "tags": ["base"]
        },
        {
            "name": "tiuxo-porn",
            "url": "https://raw.githubusercontent.com/tiuxo/hosts/master/porn",
            "destination": "./extensions/porn/tiuxo",
            "tags": ["porn"]
        },
        {
            "name": "tiuxo",
            "url": "https://raw.githubusercontent.com/tiuxo/hosts/master/ads",
            "destination": "./data/tiuxo",
            "tags": ["base"]
        },
        {
            "name": "uncheckyads",
            "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/UncheckyAds/hosts",
            "destination": "./data/UncheckyAds",
            "tags": ["base"]
        },
        {
            "name": "urlhaus",
            "url": "https://urlhaus.abuse.ch/downloads/hostfile/",
            "destination": "./data/URLhaus",
            "tags": ["base"]
        },
        {
            "name": "yoyo",
            "url": "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext&useip=0.0.0.0",
            "destination": "./data/yoyo.org",
            "tags": ["base"]
        }
    ]"#.trim().to_string();
    sources
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_get_config_file_returns_an_actionable_file_path() {
        let cf = get_config_file();
        assert!(cf.is_ok_and(|fp| fp.is_file() && fp.exists()));
    }

    #[test]
    fn test_read_config_file() {
        let cf = read_config_file();
        dbg!(cf);
    }

    #[test]
    fn test_shortcuts() {
        let hm = get_shortcuts();
        assert_eq!(hm.get(&"yoyo".to_string()), Some(&"https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext&useip=0.0.0.0".to_string()));
        assert_eq!(hm.get(&"zzz".to_string()), None);
    }

    #[test]
    fn test_mut_shortcuts() {
        let mut hm = get_shortcuts();
        hm.insert("yoyo".to_string(), "foo.bar".to_string());
        assert_eq!(hm.get(&"yoyo".to_string()), Some(&"foo.bar".to_string()));
    }

    #[test]
    fn test_get_products_json() {
        let json = get_products_json();
        let products: Components = serde_json::from_str(json.as_str()).expect("Invalid JSON in recipe.");
        println!("{:?}", products);
        assert!(products.len() > 5);
    }

    #[test]
    fn test_taging_products_json() {
        // this test just lists all the products a tag belongs to.
        let json = get_products_json();
        let config: Components = serde_json::from_str(json.as_str()).expect("Invalid JSON recepe tag specification.");

        let tags = get_unique_tags();
        for tag in tags {
            println!("\n# {}", &tag);
            let mut c = config.clone();
            c.retain(|x| x.tags.contains(&tag.to_string()));
            for x in c {
                println!("{x}");
            }
        }
        assert_eq!(Some(2), Some(1 + 1));
    }

    #[test]
    fn test_get_sources_by_tag() {
        let tests = ["base", "fakenews", "gambling", "porn", "social"];
        for test in tests {
            println!();
            println!("== {} ==", test.to_string());
            let sources = get_sources_by_tag(test.to_string());
            for s in sources.clone() {
                println!("{:?}", s.name);
            }
            assert!(sources.len() > 0);
        }
    }

    #[test]
    fn test_get_sources_by_tag_fakenews() {
        let sources = get_sources_by_tag("fakenews".to_string());
        for s in sources.clone() {
            println!("{:?}", s.name);
        }
        assert!(sources.len() == 1);
    }

    #[test]
    fn test_get_config_json() {
        let json = get_sources_json();
        let config: SourcesSpecs = serde_json::from_str(json.as_str()).expect("Invalid JSON configuration.");
        for o in config.clone() {
            println!("{:?} ⬅️ {:?}", o.tags, o.url);
        }
        assert!(config.len() > 5);
    }

    #[test]
    fn test_taging_config_json() {
        // this test lists all the sources of a tag.
        let json = get_sources_json();
        let config: SourcesSpecs = serde_json::from_str(json.as_str()).expect("Invalid JSON for taging.");

        let tags = get_unique_tags();
        for tag in tags {
            println!("\n# {}", &tag);
            let mut c = config.clone();
            c.retain(|x| x.tags.contains(&tag.to_string()));
            for x in c {
                println!("{x}");
            }
        }
        assert_eq!(Some(2), Some(1 + 1));
    }

    #[test]
    fn test_gettaggroups() {
        println!("{:?}", gettaggroups());
        assert!(1 == 1)
    }

    #[test]
    fn test_grouping_config_json_data() {
        // this test tells us if data destination folders exist.
        use std::path::PathBuf;

        macro_rules! ternary {
            ($c:expr, $v:expr, $v1:expr) => {
                if $c {$v} else {$v1}
            };
        }

        let json = get_sources_json();
        let config: SourcesSpecs = serde_json::from_str(json.as_str()).expect("Invalid JSON for grouping.");
            for x in config {
                let path: PathBuf = ["/Users/Steve/Dropbox/dev/hosts", x.destination.as_str()].iter().collect();
                //  let b: bool = Path::new(x.destination.as_str()).is_dir();
                let b: bool = path.is_dir();
                // println!("{} - {}", x.destination, b);
                println!("{} {}", x.destination, ternary!(b,"✅", "❌"));
            }
        assert_eq!(Some(2), Some(1 + 1));
    }

    #[test]
    fn test_get_unique_tags() {
        // this test ensures we get a vec of unique tags.
        let tags = get_unique_tags();
        assert!(tags.contains(&"base".to_string()));
        assert!(tags.contains(&"porn".to_string()));
        println!("{:?}", tags);
    }

    #[test]
    fn test_config_name_collisions() {
        // this test ensures we have no name collisions between sources and recipies.
        use std::collections::HashSet;

        let json = get_sources_json();
        let config: SourcesSpecs = serde_json::from_str(json.as_str()).expect("Invalid JSON for sources.");
        let json = get_products_json();
        let recipies: Components = serde_json::from_str(json.as_str()).expect("Invalid JSON for recipies.");
        let mut check = HashSet::new();

        for source in config {
            if !check.insert(source.name.clone()) {
                println!("{} ❌ is a duplicate source", source.name);
            }
        }
        for recipe in recipies {
            if !check.insert(recipe.name.clone()) {
                println!("{} ❌ is a duplicate recipe", recipe.name);
            }
        }
        assert_eq!(Some(2), Some(1 + 1));
    }
}
