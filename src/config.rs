use anyhow::anyhow;
use std::{collections::BTreeMap, fmt, fs, path::PathBuf};

// See https://crates.io/crates/directories
extern crate directories;
// use directories::{BaseDirs, ProjectDirs, UserDirs};
use directories::ProjectDirs;

pub fn get_config_file() -> anyhow::Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "rhosts") {
        let config_dir = proj_dirs.config_dir();
        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        return Ok(config_dir.join("rhosts.toml"));
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
            Ok(file) => serde_json::from_str(&file).expect("Invalid JSON configuration."),
            Err(_) => "File read error".to_string(),
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipies {
    recipies: Vec<Recipe>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Recipe {
    name: String,
    alias: String,
    destination: String,
    components: Vec<String>,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "name: {}, destination: {}, tags: {:?}",
            self.name, self.destination, self.components
        )
    }
}

#[test]
fn test_get_recipe_json() {
    let json = get_recipe_json();
    let config: Vec<Recipe> = serde_json::from_str(json.as_str()).expect("Invalid JSON in recipe.");
    println!("{:?}", config);
    assert!(config.len() > 5);
}

#[test]
fn test_grouping_recipe_json() {
    // this test just lists all the products a group belongs to.
    let json = get_recipe_json();
    let config: Vec<Recipe> =
        serde_json::from_str(json.as_str()).expect("Invalid JSON recepe group specification.");

    let groups = vec!["general", "fakenews", "gambling", "porn", "social"];
    for group in groups {
        println!("\n# {}", &group);
        let mut c = config.clone();
        c.retain(|x| x.components.contains(&group.to_string()));
        for x in c {
            println!("{x}");
        }
    }
    assert_eq!(Some(2), Some(1 + 1));
}

#[allow(dead_code)]
pub fn get_recipe_json() -> String {
    let raw_config = r#"[
        {
            "name": "base",
            "alias": "base",
            "destination": "./",
            "components": ["general"]
        },
        {
            "name": "b",
            "alias": "b",
            "destination": "./",
            "components": ["general"]
        },
        {
            "name": "f",
            "alias": "f",
            "destination": "./alternates/fakenews",
            "components": ["general", "fakenews"]
        },
        {
            "name": "fg",
            "alias": "fg",
            "destination": "./alternates/fakenews-gamnbling",
            "components": ["general", "fakenews", "gambling"]
        },
        {
            "name": "fgp",
            "alias": "fgp",
            "destination": "./alternates/fakenews-gambling-porn",
            "components": ["general", "fakenews", "gambling", "porn"]
        },
        {
            "name": "fgps",
            "alias": "fgps",
            "destination": "./alternates/fakenews-gambling-porn-social",
            "components": ["general", "fakenews", "gambling", "porn", "social"]
        },
        {
            "name": "fgs",
            "alias": "fgs",
            "destination": "./alternates/fakenews-gambling-social",
            "components": ["general", "fakenews", "gambling", "social"]
        },
        {
            "name": "fp",
            "alias": "fp",
            "destination": "./alternates/fakenews-porn",
            "components": ["general", "fakenews", "porn"]
        },
        {
            "name": "fps",
            "alias": "fps",
            "destination": "./alternates/fakenews-porn-social",
            "components": ["general", "fakenews", "porn", "social"]
        },
        {
            "name": "fs",
            "alias": "fs",
            "destination": "./alternates/fakenews-social",
            "components": ["general", "fakenews", "social"]
        },
        {
            "name": "g",
            "alias": "g",
            "destination": "./alternates/gambling",
            "components": ["general", "gambling"]
        },
        {
            "name": "gp",
            "alias": "gp",
            "destination": "./alternates/gambling-porn",
            "components": ["general", "gambling", "porn"]
        },
        {
            "name": "gps",
            "alias": "gps",
            "destination": "./alternates/gambling-porn-social",
            "components": ["general", "gambling", "porn", "social"]
        },
        {
            "name": "gs",
            "alias": "gs",
            "destination": "./alternates/gambling-social",
            "components": ["general", "gambling", "social"]
        },
        {
            "name": "p",
            "alias": "p",
            "destination": "./alternates/porn",
            "components": ["general", "porn"]
        },
        {
            "name": "ps",
            "alias": "ps",
            "destination": "./alternates/porn-social",
            "components": ["general", "porn", "social"]
        },
        {
            "name": "s",
            "alias": "s",
            "destination": "./alternates/social",
            "components": ["general", "social"]
        }
    ]"#
    .trim()
    .to_string();
    raw_config
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    sources: Vec<Source>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Source {
    name: String,
    url: String,
    destination: String,
    tags: Vec<String>,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "name: {}, destination: {}, tags: {:?}",
            self.name, self.destination, self.tags
        )
    }
}

#[test]
fn test_get_config_json() {
    let json = get_config_json();
    let config: Vec<Source> =
        serde_json::from_str(json.as_str()).expect("Invalid JSON configuration.");
    assert!(config.len() > 5);
}

#[test]
fn test_grouping_config_json() {
    // this test lists all the sources of a group.
    let json = get_config_json();
    let config: Vec<Source> =
        serde_json::from_str(json.as_str()).expect("Invalid JSON for grouping.");

    let groups = vec!["general", "fakenews", "gambling", "porn", "social"];
    for group in groups {
        println!("\n# {}", &group);
        let mut c = config.clone();
        c.retain(|x| x.tags.contains(&group.to_string()));
        for x in c {
            println!("{x}");
        }
    }
    assert_eq!(Some(2), Some(1 + 1));
}

#[test]
fn test_grouping_config_json_data() {
    // this test tells us if data destination folders exist.
    use std::path::PathBuf;

    macro_rules! ternary {
        ($c:expr, $v:expr, $v1:expr) => {
            if $c {
                $v
            } else {
                $v1
            }
        };
    }

    let json = get_config_json();
    let config: Vec<Source> =
        serde_json::from_str(json.as_str()).expect("Invalid JSON for grouping.");
    for x in config {
        let path: PathBuf = ["/Users/Steve/Dropbox/dev/hosts", x.destination.as_str()]
            .iter()
            .collect();
        //  let b: bool = Path::new(x.destination.as_str()).is_dir();
        let b: bool = path.is_dir();
        // println!("{} - {}", x.destination, b);
        println!("{} {}", x.destination, ternary!(b, "✅", "❌"));
    }
    assert_eq!(Some(2), Some(1 + 1));
}

#[allow(dead_code)]
pub fn get_config_json() -> String {
    let raw_config = r#"[
        {
            "name": "adaway",
            "url": "https://raw.githubusercontent.com/AdAway/adaway.github.io/master/hosts.txt",
            "destination": "./data/adaway.org",
            "tags": ["general"]
        },
        {
            "name": "add2o7net",
            "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.2o7Net/hosts",
            "destination": "./data/add.2o7net",
            "tags": ["general"]
        },
        {
            "name": "adddead",
            "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Dead/hosts",
            "destination": "./data/add.dead",
            "tags": ["general"]
        },
        {
            "name": "addrisk",
            "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Risk/hosts",
            "destination": "./data/add.risk",
            "tags": ["general"]
        },
        {
            "name": "addspam",
            "url": "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Spam/hosts",
            "destination": "./data/add.spam",
            "tags": ["general"]
        },
        {
            "name": "adguard",
            "url": "https://raw.githubusercontent.com/AdguardTeam/cname-trackers/master/combined_disguised_trackers_justdomains.txt",
            "destination": "./data/Adguard-cname",
            "tags": ["general"]
        },
        {
            "name": "baddboyz",
            "url": "https://raw.githubusercontent.com/mitchellkrogza/Badd-Boyz-Hosts/master/hosts",
            "destination": "./data/Badd-Boyz-Hosts",
            "tags": ["general"]
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
            "tags": ["general"]
        },
        {
            "name": "kadhosts",
            "url": "https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt",
            "destination": "./data/KADhosts",
            "tags": ["general"]
        },
        {
            "name": "metamask",
            "url": "https://raw.githubusercontent.com/MetaMask/eth-phishing-detect/master/src/hosts.txt",
            "destination": "./data/MetaMask",
            "tags": ["general"]
        },
        {
            "name": "mvps",
            "url": "https://winhelp2002.mvps.org/hosts.txt",
            "destination": "./data/mvps.org",
            "tags": ["general"]
        },
        {
            "name": "shady",
            "url": "https://raw.githubusercontent.com/shreyasminocha/shady-hosts/main/hosts",
            "destination": "./data/shady-hosts",
            "tags": ["general"]
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
            "tags": ["general"]
        },
        {
            "name": "stevenblack",
            "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/data/StevenBlack/hosts",
            "destination": "./data/StevenBlack",
            "tags": ["general"]
        },
        {
            "name": "tiuxo-porn",
            "url": "https://raw.githubusercontent.com/tiuxo/hosts/master/porn",
            "destination": "./extensions/porn/tiuxo",
            "tags": ["porn"]
        },
        {
            "name": "tiuxo-social",
            "url": "https://raw.githubusercontent.com/tiuxo/hosts/master/social",
            "destination": "./extensions/social/tiuxo",
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
            "destination": "./data/UncheckyAds",
            "tags": ["general"]
        },
        {
            "name": "urlhaus",
            "url": "https://urlhaus.abuse.ch/downloads/hostfile/",
            "destination": "./data/URLhaus",
            "tags": ["general"]
        },
        {
            "name": "yoyo",
            "url": "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext&useip=0.0.0.0",
            "destination": "./data/yoyo.org",
            "tags": ["general"]
        }
    ]"#.trim().to_string();
    raw_config
}

#[test]
fn test_config_name_collisions() {
    /// this test ensures we have no name collisions between sources and recipies.
    use std::collections::HashSet;

    let json = get_config_json();
    let config: Vec<Source> =
        serde_json::from_str(json.as_str()).expect("Invalid JSON for sources.");
    let json = get_recipe_json();
    let recipies: Vec<Recipe> =
        serde_json::from_str(json.as_str()).expect("Invalid JSON for recipies.");
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
