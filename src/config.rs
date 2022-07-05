use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

// See https://crates.io/crates/directories
extern crate directories;
use directories::{BaseDirs, ProjectDirs, UserDirs};

pub fn get_config_file() -> String {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "rhosts") {
        let config_dir = proj_dirs.config_dir();

        let config_file = fs::read_to_string(config_dir.join("rhosts.toml"));
        // dbg!(config_file);

        let configdata = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => "".to_string(),
        };
        configdata
        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
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
        "adaway".to_string(),
        "https://raw.githubusercontent.com/AdAway/adaway.github.io/master/hosts.txt".to_string(),
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
        "adguard".to_string(),
        "https://raw.githubusercontent.com/AdguardTeam/cname-trackers/master/combined_disguised_trackers_justdomains.txt"
            .to_string(),
    );
    ret.insert(
        "baddboyz".to_string(),
        "https://raw.githubusercontent.com/mitchellkrogza/Badd-Boyz-Hosts/master/hosts".to_string(),
    );
    ret.insert(
        "clefspear".to_string(),
        "https://raw.githubusercontent.com/Clefspeare13/pornhosts/master/0.0.0.0/hosts".to_string(),
    );
    ret.insert(
        "digitalside".to_string(),
        "https://raw.githubusercontent.com/davidonzo/Threat-Intel/master/lists/latestdomains.piHole.txt"
            .to_string(),
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
        "https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt"
            .to_string(),
    );
    ret.insert(
        "metamask".to_string(),
        "https://raw.githubusercontent.com/MetaMask/eth-phishing-detect/master/src/hosts.txt"
            .to_string(),
    );
    ret.insert(
        "mvps".to_string(),
        "https://winhelp2002.mvps.org/hosts.txt".to_string(),
    );
    ret.insert(
        "orca".to_string(),
        "https://orca.pet/notonmyshift/hosts.txt".to_string(),
    );
    ret.insert(
        "shady".to_string(),
        "https://raw.githubusercontent.com/shreyasminocha/shady-hosts/main/hosts".to_string(),
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
        "tiuxo-social".to_string(),
        "https://raw.githubusercontent.com/tiuxo/hosts/master/social".to_string(),
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

#[test]
fn test_config_file() {
    let cf = get_config_file();
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


use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    sources: Vec<Source>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Source {
    name: String,
    url: String,
    destination: String,
    tags: Vec<String>,
}

#[test]
fn test_get_config_json() {
    let json = get_config_json();
    // println!("serde_json {:?}", json);
    // println!("serde_json {:?}", serde_json::from_str(json.as_str()));
    let config: Vec<Source> = serde_json::from_str(json.as_str()).unwrap();
    println!("{:?}", config);
    assert_eq!(Some(2), Some(1 + 1));
}


pub fn get_config_json() -> String {
    let raw_config = r#"[
    {
    "name": "b",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts",
    "destination": "./data/b",
    "tags": ["general"]
    },
    {
    "name": "base",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts",
    "destination": "./data/base",
    "tags": ["general"]
    },
    {
    "name": "f",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews/hosts",
    "destination": "./data/f",
    "tags": ["fakenews"]
    },
    {
    "name": "fg",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling/hosts",
    "destination": "./data/fg",
    "tags": ["amalgamation"]
    },
    {
    "name": "fgp",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn/hosts",
    "destination": "./data/fgp",
    "tags": ["amalgamation"]
    },
    {
    "name": "fgps",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn-social/hosts",
    "destination": "./data/fgps",
    "tags": ["amalgamation"]
    },
    {
    "name": "fgs",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-social/hosts",
    "destination": "./data/fgs",
    "tags": ["amalgamation"]
    },
    {
    "name": "fp",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-porn/hosts",
    "destination": "./data/fp",
    "tags": ["amalgamation"]
    },
    {
    "name": "fps",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-porn-social/hosts",
    "destination": "./data/fps",
    "tags": ["amalgamation"]
    },
    {
    "name": "fs",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-social/hosts",
    "destination": "./data/fs",
    "tags": ["amalgamation"]
    },
    {
    "name": "g",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling/hosts",
    "destination": "./data/g",
    "tags": ["amalgamation"]
    },
    {
    "name": "gp",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-porn/hosts",
    "destination": "./data/gp",
    "tags": ["amalgamation"]
    },
    {
    "name": "gps",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-porn-social/hosts",
    "destination": "./data/gps",
    "tags": ["amalgamation"]
    },
    {
    "name": "gs",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-social/hosts",
    "destination": "./data/gs",
    "tags": ["amalgamation"]
    },
    {
    "name": "p",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn/hosts",
    "destination": "./data/p",
    "tags": ["amalgamation"]
    },
    {
    "name": "ps",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn-social/hosts",
    "destination": "./data/ps",
    "tags": ["amalgamation"]
    },
    {
    "name": "s",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/social/hosts",
    "destination": "./data/s",
    "tags": ["amalgamation"]
    },
    {
    "name": "adaway",
    "url": "https://raw.githubusercontent.com/AdAway/adaway.github.io/master/hosts.txt",
    "destination": "./data/adaway",
    "tags": ["general"]
    },
    {
    "name": "add2o7net",
    "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.2o7Net/hosts",
    "destination": "./data/add2o7net",
    "tags": ["general"]
    },
    {
    "name": "adddead",
    "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Dead/hosts",
    "destination": "./data/adddead",
    "tags": ["general"]
    },
    {
    "name": "addrisk",
    "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Risk/hosts",
    "destination": "./data/addrisk",
    "tags": ["general"]
    },
    {
    "name": "addspam",
    "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Spam/hosts",
    "destination": "./data/addspam",
    "tags": ["general"]
    },
    {
    "name": "adguard",
    "url": "https://raw.githubusercontent.com/AdguardTeam/cname-trackers/master/combined_disguised_trackers_justdomains.txt",
    "destination": "./data/adguard",
    "tags": ["general"]
    },
    {
    "name": "baddboyz",
    "url": "https://raw.githubusercontent.com/mitchellkrogza/Badd-Boyz-Hosts/master/hosts",
    "destination": "./data/baddboyz",
    "tags": ["general"]
    },
    {
    "name": "clefspear",
    "url": "https://raw.githubusercontent.com/Clefspeare13/pornhosts/master/0.0.0.0/hosts",
    "destination": "./data/clefspear",
    "tags": ["porn"]
    },
    {
    "name": "digitalside",
    "url": "https://raw.githubusercontent.com/davidonzo/Threat-Intel/master/lists/latestdomains.piHole.txt",
    "destination": "./data/digitalside",
    "tags": ["general"]
    },
    {
    "name": "fakenews",
    "url": "https://raw.githubusercontent.com/marktron/fakenews/master/fakenews",
    "destination": "./data/fakenews",
    "tags": ["fakenews"]
    },
    {
    "name": "hostsvn",
    "url": "https://raw.githubusercontent.com/bigdargon/hostsVN/master/option/hosts-VN",
    "destination": "./data/hostsvn",
    "tags": ["general"]
    },
    {
    "name": "kadhosts",
    "url": "https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt",
    "destination": "./data/kadhosts",
    "tags": ["general"]
    },
    {
    "name": "metamask",
    "url": "https://raw.githubusercontent.com/MetaMask/eth-phishing-detect/master/src/hosts.txt",
    "destination": "./data/metamask",
    "tags": ["general"]
    },
    {
    "name": "mvps",
    "url": "https://winhelp2002.mvps.org/hosts.txt",
    "destination": "./data/mvps",
    "tags": ["general"]
    },
    {
    "name": "orca",
    "url": "https://orca.pet/notonmyshift/hosts.txt",
    "destination": "./data/orca",
    "tags": ["general"]
    },
    {
    "name": "shady",
    "url": "https://raw.githubusercontent.com/shreyasminocha/shady-hosts/main/hosts",
    "destination": "./data/shady",
    "tags": ["general"]
    },
    {
    "name": "sinfonietta-gambling",
    "url": "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/gambling-hosts",
    "destination": "./data/sinfonietta-gambling",
    "tags": ["gambling"]
    },
    {
    "name": "sinfonietta-porn",
    "url": "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/pornography-hosts",
    "destination": "./data/sinfonietta-porn",
    "tags": ["porn"]
    },
    {
    "name": "sinfonietta-snuff",
    "url": "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/snuff-hosts",
    "destination": "./data/sinfonietta-snuff",
    "tags": ["porn"]
    },
    {
    "name": "sinfonietta-social",
    "url": "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/social-hosts",
    "destination": "./data/sinfonietta-social",
    "tags": ["social"]
    },
    {
    "name": "someonewhocares",
    "url": "https://someonewhocares.org/hosts/zero/hosts",
    "destination": "./data/someonewhocares",
    "tags": ["general"]
    },
    {
    "name": "stevenblack",
    "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/data/StevenBlack/hosts",
    "destination": "./data/stevenblack",
    "tags": ["general"]
    },
    {
    "name": "tiuxo-porn",
    "url": "https://raw.githubusercontent.com/tiuxo/hosts/master/porn",
    "destination": "./data/tiuxo-porn",
    "tags": ["porn"]
    },
    {
    "name": "tiuxo-social",
    "url": "https://raw.githubusercontent.com/tiuxo/hosts/master/social",
    "destination": "./data/tiuxo-social",
    "tags": ["social"]
    },
    {
    "name": "tiuxo",
    "url": "https://raw.githubusercontent.com/tiuxo/hosts/master/ads",
    "destination": "./data/tiuxo",
    "tags": ["general"]
    },
    {
    "name": "uncheckyads",
    "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/UncheckyAds/hosts",
    "destination": "./data/uncheckyads",
    "tags": ["general"]
    },
    {
    "name": "urlhaus",
    "url": "https://urlhaus.abuse.ch/downloads/hostfile/",
    "destination": "./data/urlhaus",
    "tags": ["general"]
    },
    {
    "name": "yoyo",
    "url": "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext&useip=0.0.0.0",
    "destination": "./data/yoyo",
    "tags": ["general"]
    }
    ]"#.trim().to_string();
    raw_config
}
