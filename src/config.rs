use std::{
    collections::BTreeMap,
    path::{PathBuf},
    fs, fmt,
};
use anyhow::{anyhow};

use crate::{Arguments, types::Ingredients, utils::{Combinations, flatten}};
extern crate directories;
// use directories::{BaseDirs, ProjectDirs, UserDirs};
use directories::{ProjectDirs};

pub fn info(_args:Arguments) -> anyhow::Result<()> {
    println!("Configuration:");
    println!("Local config file: {}", get_config_file()?.to_string_lossy());
    Ok(())
}

pub fn get_config_file() -> anyhow::Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "rhosts") {
        let config_dir = proj_dirs.config_dir();
        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        return Ok(config_dir.join("rhosts.json"));
    }
    return Err(anyhow!("Error reckoning config file."));
}

#[allow(dead_code)]
pub fn read_config_file() -> String {
    let config_file = get_config_file();
    if config_file.is_ok() {
        let config_file_contents_result = fs::read_to_string(config_file.expect("Problem with config file."));
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
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Recipe {
    name: String,
    alias: String,
    destination: String,
    ingredients: Ingredients,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "name: {}, destination: {}, tags: {:?}", self.name, self.destination, self.ingredients)
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
    let config: Vec<Recipe> = serde_json::from_str(json.as_str()).expect("Invalid JSON recepe group specification.");

    let groups = gettags();
    for group in groups {
        println!("\n# {}", &group);
        let mut c = config.clone();
        c.retain(|x| x.ingredients.contains(&group.to_string()));
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
            "ingredients": ["base"]
        },
        {
            "name": "b",
            "alias": "b",
            "destination": "./",
            "ingredients": ["base"]
        },
        {
            "name": "f",
            "alias": "f",
            "destination": "./alternates/fakenews",
            "ingredients": ["base", "fakenews"]
        },
        {
            "name": "fg",
            "alias": "fg",
            "destination": "./alternates/fakenews-gamnbling",
            "ingredients": ["base", "fakenews", "gambling"]
        },
        {
            "name": "fgp",
            "alias": "fgp",
            "destination": "./alternates/fakenews-gambling-porn",
            "ingredients": ["base", "fakenews", "gambling", "porn"]
        },
        {
            "name": "fgps",
            "alias": "fgps",
            "destination": "./alternates/fakenews-gambling-porn-social",
            "ingredients": ["base", "fakenews", "gambling", "porn", "social"]
        },
        {
            "name": "fgs",
            "alias": "fgs",
            "destination": "./alternates/fakenews-gambling-social",
            "ingredients": ["base", "fakenews", "gambling", "social"]
        },
        {
            "name": "fp",
            "alias": "fp",
            "destination": "./alternates/fakenews-porn",
            "ingredients": ["base", "fakenews", "porn"]
        },
        {
            "name": "fps",
            "alias": "fps",
            "destination": "./alternates/fakenews-porn-social",
            "ingredients": ["base", "fakenews", "porn", "social"]
        },
        {
            "name": "fs",
            "alias": "fs",
            "destination": "./alternates/fakenews-social",
            "ingredients": ["base", "fakenews", "social"]
        },
        {
            "name": "g",
            "alias": "g",
            "destination": "./alternates/gambling",
            "ingredients": ["base", "gambling"]
        },
        {
            "name": "gp",
            "alias": "gp",
            "destination": "./alternates/gambling-porn",
            "ingredients": ["base", "gambling", "porn"]
        },
        {
            "name": "gps",
            "alias": "gps",
            "destination": "./alternates/gambling-porn-social",
            "ingredients": ["base", "gambling", "porn", "social"]
        },
        {
            "name": "gs",
            "alias": "gs",
            "destination": "./alternates/gambling-social",
            "ingredients": ["base", "gambling", "social"]
        },
        {
            "name": "p",
            "alias": "p",
            "destination": "./alternates/porn",
            "ingredients": ["base", "porn"]
        },
        {
            "name": "ps",
            "alias": "ps",
            "destination": "./alternates/porn-social",
            "ingredients": ["base", "porn", "social"]
        },
        {
            "name": "s",
            "alias": "s",
            "destination": "./alternates/social",
            "ingredients": ["base", "social"]
        }
    ]"#.trim().to_string();
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
        write!(f, "name: {}, destination: {}, tags: {:?}", self.name, self.destination, self.tags)
    }
}

#[test]
fn test_get_config_json() {
    let json = get_config_json();
    let config: Vec<Source> = serde_json::from_str(json.as_str()).expect("Invalid JSON configuration.");
    for o in config.clone() {
        println!("{:?} ⬅️ {:?}", o.tags, o.url);
    }
    assert!(config.len() > 5);
}

#[test]
fn test_grouping_config_json() {
    // this test lists all the sources of a group.
    let json = get_config_json();
    let config: Vec<Source> = serde_json::from_str(json.as_str()).expect("Invalid JSON for grouping.");

    let groups = gettags();
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

pub fn gettaggroups() -> Vec<Vec<String>> {
    let tags = gettags();
    let mut taggroups = vec!();
    for n in 1..tags.len() +1 {
        let groupsvec: Vec<_> = Combinations::new(tags.clone(), n).collect();
        taggroups.push(groupsvec);
        // println!("{:?}", groupsvec);
    }
    flatten(taggroups)
}

#[test]
fn test_gettaggroups() {
    println!("{:?}", gettaggroups());
    assert!(1 == 1)
}

#[test]
fn test_grouping_config_json_data() {
    // this test tells us if data destination folders exist.
    use std::path::{PathBuf};

    macro_rules! ternary {
        ($c:expr, $v:expr, $v1:expr) => {
            if $c {$v} else {$v1}
        };
    }

    let json = get_config_json();
    let config: Vec<Source> = serde_json::from_str(json.as_str()).expect("Invalid JSON for grouping.");
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
fn test_gettags() {

    let tags = gettags();
    assert!(tags.contains(&"base".to_string()));
    assert!(tags.contains(&"porn".to_string()));
}

#[allow(dead_code)]
pub fn gettags() -> Vec<String> {
    // yields all the unique tags we have
    use array_tool::vec::Uniq;
    let json = get_config_json();
    let config: Vec<Source> = serde_json::from_str(json.as_str()).expect("Invalid JSON for getting tags.");
    let mut tags: Vec<String> = vec!();
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
pub fn get_config_json() -> String {
    let raw_config = r#"[
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
            "name": "adguard",
            "url": "https://raw.githubusercontent.com/AdguardTeam/cname-trackers/master/combined_disguised_trackers_justdomains.txt",
            "destination": "./data/Adguard-cname",
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
            "url": "https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt",
            "destination": "./data/KADhosts",
            "tags": ["base"]
        },
        {
            "name": "metamask",
            "url": "https://raw.githubusercontent.com/MetaMask/eth-phishing-detect/master/src/hosts.txt",
            "destination": "./data/MetaMask",
            "tags": ["base"]
        },
        {
            "name": "mvps",
            "url": "https://winhelp2002.mvps.org/hosts.txt",
            "destination": "./data/mvps.org",
            "tags": ["base"]
        },
        {
            "name": "shady",
            "url": "https://raw.githubusercontent.com/shreyasminocha/shady-hosts/main/hosts",
            "destination": "./data/shady-hosts",
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
            "name": "tiuxo-social",
            "url": "https://raw.githubusercontent.com/tiuxo/hosts/master/social",
            "destination": "./extensions/social/tiuxo",
            "tags": ["social"]
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
    raw_config
}

#[test]
fn test_config_name_collisions() {
    /// this test ensures we have no name collisions between sources and recipies.
    use std::collections::HashSet;

    let json = get_config_json();
    let config: Vec<Source> = serde_json::from_str(json.as_str()).expect("Invalid JSON for sources.");
    let json = get_recipe_json();
    let recipies: Vec<Recipe> = serde_json::from_str(json.as_str()).expect("Invalid JSON for recipies.");
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
