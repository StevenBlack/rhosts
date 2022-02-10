/// Utilities and functions
///
use std::net::{IpAddr};
use psl::List;
use addr::{parse_domain_name, parse_dns_name};
use addr::parser::{DnsName, DomainName};

pub fn is_ip_address(s: &str) -> bool {
    use std::str::FromStr;
    // let addr = IpAddr::from_str(s);
    // addr.is_ok()
    IpAddr::from_str(s).is_ok()
}

#[test]
fn test_ip_test() {
    assert_eq!(is_ip_address("127.0.0.1"), true);
    assert_eq!(is_ip_address("599.0.0.1"), false);
    assert_eq!(is_ip_address("192.168.0.1"), true);
    assert_eq!(is_ip_address("192.168"), false);
    assert_eq!(is_ip_address(" 192.168.0.1 "), false);
}

pub fn is_domain(s: &str) -> bool {
    // parse_dns_name(s).is_ok()
    if ! s.contains(".") {
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
    assert_eq!(is_domain("a23456789012345678901234567890123456789012345678901234567890123.com"), true);
    // this one is too long (> 64)
    assert_eq!(is_domain("a2345678901234567890123456789012345678901234567890123456789012345.com"), false);
}

pub fn sep(n: usize) {
    println!("{}", "-".repeat(n));
}

#[allow(dead_code)]
pub fn print_type_of<T>(_: &T) {
    println!("===> {}", std::any::type_name::<T>())
}

pub fn vtrim(v: &mut Vec<String>) -> &mut Vec<String> {
    v.iter_mut()
    .for_each(
        |line| {
            *line = norm_string(line.as_str())
        }
    );
    v
}

#[test]
fn test_vtrim() {
    let mut v = vec![];
    v.push("      Line 1".to_string());
    v.push("   Line 2   ".to_string());
    v.push("Line 3   ".to_string());
    // embedded spaces and tabs
    v.push("  127.0.0.1	10iski.com   ".to_string());

    let foo = vtrim(&mut v);
    assert_eq!(foo[0], "Line 1".to_string());
    assert_eq!(foo[1], "Line 2".to_string());
    assert_eq!(foo[2], "Line 3".to_string());
    assert_eq!(foo[3], "127.0.0.1 10iski.com".to_string());
}

pub fn stripblanklines(v: &mut Vec<String>) -> &mut Vec<String> {
    let mut trimmed = vtrim(v);
    trimmed.retain(|line | line.chars().count() > 0);
    v
}

#[test]
fn test_stripblanklines() {
    // assert_eq!(2 + 2, 4);
    let mut v = vec![];
    v.push("      Line 1".to_string());
    v.push("   Line 2   ".to_string());
    v.push("     ".to_string());
    v.push("Line 3   ".to_string());
    let foo = stripblanklines(&mut v);
    assert_eq!(foo[0], "Line 1".to_string());
    assert_eq!(foo[1], "Line 2".to_string());
    assert_eq!(foo[2], "Line 3".to_string());
}

pub fn norm_string(passed: &str) -> String {
    let x: Vec<_> = passed.trim().split_ascii_whitespace().collect();
    x.join(" ").to_string()
}

#[test]
fn test_norm_string() {
    assert_eq!(norm_string("   Hello           World  "), "Hello World".to_string());
}

#[test]
fn test_lf() {
    assert!("xx\nxx".contains("\n"));
    assert_eq!("xx\nxx".to_string().split("\n").map(|l| l.to_string()).collect::<Vec<String>>().len(), 2);
}