use std::collections::HashMap;



//! Subcommand modules for the `rhosts` binary.
pub mod build;
pub mod clean;
pub mod init;

pub fn getShortcuts() -> HashMap<String, String> {
  let mut ret = HashMap::new();
  ret.insert("b",                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts");
  ret.insert("base",                 "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts");
  ret.insert("f",                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews/hosts");
  ret.insert("fg",                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling/hosts");
  ret.insert("fgp",                  "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn/hosts");
  ret.insert("fgps",                 "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-porn-social/hosts");
  ret.insert("fgs",                  "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-gambling-social/hosts");
  ret.insert("fp",                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-porn/hosts");
  ret.insert("fps",                  "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-porn-social/hosts");
  ret.insert("fs",                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews-social/hosts");
  ret.insert("g",                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling/hosts");
  ret.insert("gp",                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-porn/hosts");
  ret.insert("gps",                  "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-porn-social/hosts");
  ret.insert("gs",                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling-social/hosts");
  ret.insert("p",                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn/hosts");
  ret.insert("ps",                   "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn-social/hosts");
  ret.insert("s",                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/social/hosts");
  ret.insert("adaway",               "https://raw.githubusercontent.com/AdAway/adaway.github.io/master/hosts.txt");
  ret.insert("add2o7net",            "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.2o7Net/hosts");
  ret.insert("adddead",              "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Dead/hosts");
  ret.insert("addrisk",              "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Risk/hosts");
  ret.insert("addspam",              "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Spam/hosts");
  ret.insert("adguard",              "https://raw.githubusercontent.com/AdguardTeam/cname-trackers/master/combined_disguised_trackers_justdomains.txt");
  ret.insert("baddboyz",             "https://raw.githubusercontent.com/mitchellkrogza/Badd-Boyz-Hosts/master/hosts");
  ret.insert("clefspear",            "https://raw.githubusercontent.com/Clefspeare13/pornhosts/master/0.0.0.0/hosts");
  ret.insert("digitalside",          "https://raw.githubusercontent.com/davidonzo/Threat-Intel/master/lists/latestdomains.piHole.txt");
  ret.insert("fakenews",             "https://raw.githubusercontent.com/marktron/fakenews/master/fakenews");
  ret.insert("hostsvn",              "https://raw.githubusercontent.com/bigdargon/hostsVN/master/option/hosts-VN");
  ret.insert("kadhosts",             "https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt");
  ret.insert("metamask",             "https://raw.githubusercontent.com/MetaMask/eth-phishing-detect/master/src/hosts.txt");
  ret.insert("mvps",                 "https://winhelp2002.mvps.org/hosts.txt");
  ret.insert("orca",                 "https://orca.pet/notonmyshift/hosts.txt");
  ret.insert("shady",                "https://raw.githubusercontent.com/shreyasminocha/shady-hosts/main/hosts");
  ret.insert("sinfonietta-gambling", "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/gambling-hosts");
  ret.insert("sinfonietta-porn",     "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/pornography-hosts");
  ret.insert("sinfonietta-snuff",    "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/snuff-hosts");
  ret.insert("sinfonietta-social",   "https://raw.githubusercontent.com/Sinfonietta/hostfiles/master/social-hosts");
  ret.insert("someonewhocares",      "https://someonewhocares.org/hosts/zero/hosts");
  ret.insert("stevenblack",          "https://raw.githubusercontent.com/StevenBlack/hosts/master/data/StevenBlack/hosts");
  ret.insert("tiuxo-porn",           "https://raw.githubusercontent.com/tiuxo/hosts/master/porn");
  ret.insert("tiuxo-social",         "https://raw.githubusercontent.com/tiuxo/hosts/master/social");
  ret.insert("tiuxo",                "https://raw.githubusercontent.com/tiuxo/hosts/master/ads");
  ret.insert("uncheckyads",          "https://raw.githubusercontent.com/FadeMind/hosts.extras/master/UncheckyAds/hosts");
  ret.insert("urlhaus",              "https://urlhaus.abuse.ch/downloads/hostfile/");
  ret.insert("yoyo",                 "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext&useip=0.0.0.0");
  ret
}