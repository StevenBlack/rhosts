/// Utilities and functions
///
use addr::parser::DnsName;
use psl::List;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Tests if a string is a valid domain.
pub fn is_domain(s: &str) -> bool {
    // parse_dns_name(s).is_ok()
    if !s.contains('.') {
        return false;
    }
    List.parse_dns_name(s).is_ok()
}

#[test]
fn test_domains() {
    assert_eq!(is_domain("com"), false);
    assert_eq!(is_domain("github"), false);
    assert_eq!(is_domain("github.com"), true);
    assert_eq!(is_domain("www.github.com"), true);
    // this one is max length (63)
    assert_eq!(
        is_domain("a23456789012345678901234567890123456789012345678901234567890123.com"),
        true
    );
    // this one is too long (> 64)
    assert_eq!(
        is_domain("a2345678901234567890123456789012345678901234567890123456789012345.com"),
        false
    );
}

#[allow(dead_code)]
pub fn print_type_of<T>(_: &T) {
    println!("===> {}", std::any::type_name::<T>())
}

pub fn trim_inline_comments(s: String) -> String {
    if let Some(result) = s.find("#") {
        if let Some(inner) = s.get(..result) {
            return inner.trim().to_string();
        }
    }
    s
}

pub fn norm_string(passed: &str) -> String {
    let x: Vec<_> = passed.trim().split_ascii_whitespace().collect();
    x.join(" ")
}

#[test]
fn test_norm_string() {
    assert_eq!(
        norm_string("   Hello           World  "),
        "Hello World".to_string()
    );
}

#[test]
fn test_lf() {
    assert!("xx\nxx".contains("\n"));
    assert_eq!(
        "xx\nxx"
            .to_string()
            .split("\n")
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .len(),
        2
    );
}

pub fn hash<T>(obj: T) -> String
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

#[test]
fn test_hashing_string() {
    assert_eq!(hash("hosts".to_string()),"b6e6d131fe41b528".to_string());
    assert_eq!(hash("domains".to_string()),"5ae5d5636edd71d4".to_string());
}

// pub fn vtrim(v: &mut Vec<String>) -> &mut Vec<String> {
//     v.iter_mut()
//         .for_each(|line| *line = norm_string(line.as_str()));
//     v
// }

// #[test]
// fn test_vtrim() {
//     let mut v = vec![];
//     v.push("      Line 1".to_string());
//     v.push("   Line 2   ".to_string());
//     v.push("Line 3   ".to_string());
//     // embedded spaces and tabs
//     v.push("  127.0.0.1	10iski.com   ".to_string());

//     let foo = vtrim(&mut v);
//     assert_eq!(foo[0], "Line 1".to_string());
//     assert_eq!(foo[1], "Line 2".to_string());
//     assert_eq!(foo[2], "Line 3".to_string());
//     assert_eq!(foo[3], "127.0.0.1 10iski.com".to_string());
// }
