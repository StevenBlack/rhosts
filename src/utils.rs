#![allow(dead_code)]
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
fn test_is_domaain_function_handles_good_and_bad_domains() {
    assert_eq!(is_domain("localhost"), false);
    assert_eq!(is_domain("com"), false);
    assert_eq!(is_domain("github"), false);
    assert_eq!(is_domain("github.com"), true);
    // assert_eq!(is_domain("github-.com"), false);
    // assert_eq!(is_domain("-github.com"), false);
    assert_eq!(is_domain("www.github.com"), true);
    assert_eq!(is_domain("123.com"), true);
}

#[test]
fn test_is_domaain_function_handles_labels_that_are_too_long() {

    // Each element of a domain name separated by [.] is called a “label.”
    // The maximum length of each label is 63 characters, and a full domain
    // name can have a maximum of 253 characters. Alphanumeric characters and
    // hyphens can be used in labels, but a domain name must not commence
    // or end with a hyphen.


    // this label is max length (63)
    assert_eq!(
        is_domain(("a".repeat(63) + ".com").as_str()),
        true
    );
    // this label is too long (>= 64)
    assert_eq!(
        is_domain(("a".repeat(64) + ".com").as_str()),
        false
    );
}

#[test]
fn test_is_domaain_function_handles_domains_that_are_too_long() {

    // Each element of a domain name separated by [.] is called a “label.”
    // The maximum length of each label is 63 characters, and a full domain
    // name can have a maximum of 253 characters. Alphanumeric characters and
    // hyphens can be used in labels, but a domain name must not commence
    // or end with a hyphen.

    // this domain is max length (253)
    assert_eq!(
        // 61 * 4 = 244
        is_domain((("a".repeat(60) + ".").repeat(4) + "56789.com").as_str()),
        true
    );
    // this domain too long (length > 253)
    assert_eq!(
        // 61 * 4 = 244
        is_domain((("a".repeat(60) + ".").repeat(4) + "56789x.com").as_str()),
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
fn test_string_with_excessive_spaces_is_trimmed_and_normalized() {
    assert_eq!(
        norm_string("   Hello           World  "),
        "Hello World".to_string()
    );
}

#[test]
fn test_line_feeds_are_properly_handled() {
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
fn test_hashing_strings_returns_expected_values() {
    assert_eq!(hash("hosts".to_string()),"b6e6d131fe41b528".to_string());
    assert_eq!(hash("domains".to_string()),"5ae5d5636edd71d4".to_string());
}

pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

pub struct Combinations<T>
where
    T: Ord + Clone,
{
    original: Vec<T>,
    possition: Vec<usize>,
    len: usize,
    started: bool,
}

impl<T> Combinations<T>
where
    T: Ord + Clone,
{
    /// Initializes the setup for the permutation.
    /// `original`: `Vec` of the values to permutate over (example: vec![1, 2, 2, 3])
    /// `len`: The length of the returned length (number of draws, attempts)
    /// ```
    /// use combinations::Combinations;
    ///
    /// let computed: Vec<_> = Combinations::new(vec![1, 2, 2, 3, 4], 3).collect();
    /// let expected = vec![
    ///     vec![1, 2, 2],
    ///     vec![1, 2, 3],
    ///     vec![1, 2, 4],
    ///     vec![1, 3, 4],
    ///     vec![2, 2, 3],
    ///     vec![2, 2, 4],
    ///     vec![2, 3, 4],
    /// ];
    /// assert!(computed == expected)
    /// ```
    /// Note: This sorts the original vector as the algorithm requires this.
    pub fn new(mut original: Vec<T>, len: usize) -> Self {
        if original.len() >= len && len >= 1 {
            original.sort_unstable();
            Self {
                original,
                possition: (0..len).collect(),
                len,
                started: false,
            }
        } else {
            panic!("the length has to be smaller then the datasets len");
        }
    }

    #[inline]
    fn insert(&self, col: &mut Vec<T>) {
        col.clear();
        self.possition
            .iter()
            .enumerate()
            .for_each(|(p, n)| col.insert(p, self.original[*n].clone()))
    }


    /// clears the contents of the comb vector and inserts the next combination into the vec.
    /// This is usefull if you do not need the data from the previous iteration.
    /// Note: LLVM might do this for you for normal iterations?.
    // need to check the note in comment
    pub fn next_combination(&mut self, mut comb: &mut Vec<T>) -> bool {
        if !self.started {
            // first pass throught
            self.started = true;
            self.insert(&mut comb);
            true
        } else {
            let org_len = self.original.len();
            // check if we cant bump the back number
            if self.original[self.possition[self.len - 1]] == self.original[org_len - 1] {
                // locate the number closest behind that needs to be bumped
                for i in 2..=self.len {
                    if self.original[self.possition[self.len - i]] < self.original[org_len - i] {
                        //find the value of the
                        let lastpos = self.possition[self.len - i];
                        let val = &self.original[lastpos];
                        for j in lastpos + 1..org_len {
                            if *val < self.original[j] {
                                for k in 0..i {
                                    self.possition[self.len - i + k] = j + k;
                                }
                                self.insert(&mut comb);
                                return true;
                            }
                        }
                    }
                }
                false
            } else {
                let mut i = self.possition[self.len - 1];
                let current = &self.original[i];
                let mut next = current;
                while current == next {
                    i += 1;
                    next = &self.original[i];
                }
                self.possition[self.len - 1] = i;
                self.insert(&mut comb);
                true
            }
        }
    }
}

impl<T> Iterator for Combinations<T>
where
    T: Ord + Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vals = Vec::with_capacity(self.len);
        if self.next_combination(&mut vals) {
            Some(vals)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals() {
        assert!(Combinations::new(vec![2, 2, 2], 2).next().unwrap() == vec![2, 2])
    }

    #[test]
    fn t_123() {
        assert!(
            dbg!(Combinations::new(vec![1, 2, 3], 2)
                 .take(10)
                 .collect::<Vec<_>>())
                == vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        )
    }

    #[test]
    fn test_complicated() {
        let actual: Vec<_> = Combinations::new(vec![1, 2, 2, 3, 4], 3).collect();
        let expected = vec![
            vec![1, 2, 2],
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 2, 3],
            vec![2, 2, 4],
            vec![2, 3, 4],
        ];
        assert!(actual == expected)
    }

    #[test]
    fn test_str() {
        let actual: Vec<_> = Combinations::new(vec!["1", "2", "2", "3", "4"], 3).collect();
        let expected = vec![
            vec!["1", "2", "2"],
            vec!["1", "2", "3"],
            vec!["1", "2", "4"],
            vec!["1", "3", "4"],
            vec!["2", "2", "3"],
            vec!["2", "2", "4"],
            vec!["2", "3", "4"],
        ];
        assert!(actual == expected)
    }
}
