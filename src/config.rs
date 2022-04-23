use std::{
    collections::HashMap,
};

#[derive(Debug, Default)]
pub struct Defaults {
    mainhosts: String,
    comparehosts: Option<String>,
    iplocalhost: Option<String>,
    adddefaults: Option<bool>,
    alpha_sort: Option<bool>,
    output: Option<bool>,
    plain_output: Option<bool>,
    stats: Option<bool>,
    intersection_list: Option<bool>,
    tld: Option<bool>,
    noheader: Option<bool>,
    sysclipboard: Option<bool>,
    uniquelist: Option<bool>,
    version: Option<bool>,
    root: Option<bool>,
}

impl Defaults {
    pub fn new() -> Defaults {
        // Special code goes here ...
        let mut shortcuts = get_shortcuts();
        let mut d = Defaults {
          mainhosts: shortcuts.get("base").unwrap().to_owned(),
          iplocalhost: Some("0.0.0.0".to_string()),
          stats: Some(true),
          ..Default::default()
        };
        d
    }
}

#[test]
fn test_default() {
    let d = Defaults::new();
    assert_eq!(d.mainhosts, get_shortcuts().get("base").unwrap().to_owned());
    assert_eq!(d.comparehosts, None);
    assert_eq!(d.iplocalhost, Some("0.0.0.0".to_string()));
    assert_eq!(d.tld, None);
    assert_eq!(d.stats, Some(true));
}

pub fn get_shortcuts() -> HashMap<String, String> {
  let mut ret = HashMap::new();
  ret.insert("b".to_string(),                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".to_string());
  ret.insert("base".to_string(),                 "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".to_string());
  ret.insert("f".to_string(),                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews/hosts".to_string());
  ret.insert("fg".to_string(),                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling/hosts".to_string());
  ret.insert("fgp".to_string(),                  "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn/hosts".to_string());
  ret.insert("fgps".to_string(),                 "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn-social/hosts".to_string());
  ret.insert("fgs".to_string(),                  "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-social/hosts".to_string());
  ret.insert("fp".to_string(),                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-porn/hosts".to_string());
  ret.insert("fps".to_string(),                  "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-porn-social/hosts".to_string());
  ret.insert("fs".to_string(),                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-social/hosts".to_string());
  ret.insert("g".to_string(),                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling/hosts".to_string());
  ret.insert("gp".to_string(),                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-porn/hosts".to_string());
  ret.insert("gps".to_string(),                  "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-porn-social/hosts".to_string());
  ret.insert("gs".to_string(),                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-social/hosts".to_string());
  ret.insert("p".to_string(),                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn/hosts".to_string());
  ret.insert("ps".to_string(),                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn-social/hosts".to_string());
  ret.insert("s".to_string(),                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/social/hosts".to_string());
  ret.insert("adaway".to_string(),               "https://raw.githubusercontent.com/AdAway/adaway.github.io/master/hosts.txt".to_string());
  ret.insert("add2o7net".to_string(),            "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.2o7Net/hosts".to_string());
  ret.insert("adddead".to_string(),              "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Dead/hosts".to_string());
  ret.insert("addrisk".to_string(),              "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Risk/hosts".to_string());
  ret.insert("addspam".to_string(),              "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Spam/hosts".to_string());
  ret.insert("adguard".to_string(),              "https://raw.githubusercontent.com/AdguardTeam/cname-trackers/master/combined_disguised_trackers_justdomains.txt".to_string());
  ret.insert("baddboyz".to_string(),             "https://raw.githubusercontent.com/mitchellkrogza/Badd-Boyz-Hosts/master/hosts".to_string());
  ret.insert("clefspear".to_string(),            "https://raw.githubusercontent.com/Clefspeare13/pornhosts/master/0.0.0.0/hosts".to_string());
  ret.insert("digitalside".to_string(),          "https://raw.githubusercontent.com/davidonzo/Threat-Intel/master/lists/latestdomains.piHole.txt".to_string());
  ret.insert("fakenews".to_string(),             "https://raw.githubusercontent.com/marktron/fakenews/master/fakenews".to_string());
  ret.insert("hostsvn".to_string(),              "https://raw.githubusercontent.com/bigdargon/hostsVN/master/option/hosts-VN".to_string());
  ret.insert("kadhosts".to_string(),             "https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt".to_string());
  ret.insert("metamask".to_string(),             "https://raw.githubusercontent.com/MetaMask/eth-phishing-detect/master/src/hosts.txt".to_string());
  ret.insert("mvps".to_string(),                 "https://winhelp2002.mvps.org/hosts.txt".to_string());
  ret.insert("orca".to_string(),                 "https://orca.pet/notonmyshift/hosts.txt".to_string());
  ret.insert("shady".to_string(),                "https://raw.githubusercontent.com/shreyasminocha/shady-hosts/main/hosts".to_string());
  ret.insert("sinfonietta-gambling".to_string(), "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/gambling-hosts".to_string());
  ret.insert("sinfonietta-porn".to_string(),     "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/pornography-hosts".to_string());
  ret.insert("sinfonietta-snuff".to_string(),    "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/snuff-hosts".to_string());
  ret.insert("sinfonietta-social".to_string(),   "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/social-hosts".to_string());
  ret.insert("someonewhocares".to_string(),      "https://someonewhocares.org/hosts/zero/hosts".to_string());
  ret.insert("stevenblack".to_string(),          "https://raw.githubusercontent.com/StevenBlack/hosts/master/data/StevenBlack/hosts".to_string());
  ret.insert("tiuxo-porn".to_string(),           "https://raw.githubusercontent.com/tiuxo/hosts/master/porn".to_string());
  ret.insert("tiuxo-social".to_string(),         "https://raw.githubusercontent.com/tiuxo/hosts/master/social".to_string());
  ret.insert("tiuxo".to_string(),                "https://raw.githubusercontent.com/tiuxo/hosts/master/ads".to_string());
  ret.insert("uncheckyads".to_string(),          "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/UncheckyAds/hosts".to_string());
  ret.insert("urlhaus".to_string(),              "https://urlhaus.abuse.ch/downloads/hostfile/".to_string());
  ret.insert("yoyo".to_string(),                 "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext&useip=0.0.0.0".to_string());
  ret
}

#[test]
fn test_shortcuts() {
    let hm = get_shortcuts();
    assert_eq!(hm.get(&"yoyo".to_string()), Some(&"https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext&useip=0.0.0.0".to_string()));
    assert_eq!(hm.get(&"zzz".to_string()), None);
}
