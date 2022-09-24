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


// //! This crate give you all the combinations of values in a vec
// Source: https://github.com/meltinglava/uniquecombinations/blob/master/src/lib.rs

// #[deny(missing_docs)]

/// Combinations selects all the combinations that can be selected.
///
/// Inlife example: You have two decks of cards. You want to draw 5 random cards.
/// What are all the hands you can possibly draw?
///
/// Some info:
/// * The order of the selected does not matter (if you want all orders of all combinations, you should probably use [permutohedron](https://crates.io/crates/permutohedron) for the orders, and this crate for the combinations)
/// * Equality of values matter. if 2, 2 is input and you want len 1, the only given solution is 2 once.

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
