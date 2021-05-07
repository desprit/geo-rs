use crate::Country;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const RE_BRACKETS: &str = r"\(.*?\)";
const RE_LEADING: &str = r"^[\s\-,;:_\.\?!/]*";
const RE_TRAILING: &str = r"[\s\-,;:_\.\?!/]*$";
const RE_SPLITTER1: &str = r"[^a-zA-Z0-9\s-]";
const RE_SPLITTER2: &str = r"[^a-zA-Z0-9]";
const RE_SPACES: &str = r"\s+";

/// Read file with the given name from `src/data` folder and return `std::io::Lines`
///
/// # Arguments
///
/// * `filename` - Name of the file to read
///
/// # Examples
///
/// ```
/// let lines = read_lines("countries.txt");
/// ```
pub fn read_lines(filename: &str) -> std::io::Lines<BufReader<File>> {
    let data_path = format!("{}/src/data", env!("CARGO_MANIFEST_DIR"));
    let file_path = Path::new(&data_path).join(&filename);
    let file = File::open(file_path).unwrap();
    io::BufReader::new(file).lines()
}

/// Remove useless garbage from the given string, e.g. trailing commas, values in brackets, etc.
///
/// # Arguments
///
/// * `s` - String to be cleaned
///
/// # Examples
///
/// ```
/// let mut s = "!(#3)Toronto ,".to_string();
/// clean(&mut s);
/// assert_eq!(s, "Toronto".to_string());
/// ```
pub fn clean(s: &mut String) {
    *s = Regex::new(RE_BRACKETS)
        .unwrap()
        .replace_all(&s, "")
        .to_string();
    *s = Regex::new(RE_LEADING)
        .unwrap()
        .replace_all(&s, "")
        .to_string();
    *s = Regex::new(RE_TRAILING)
        .unwrap()
        .replace_all(&s, "")
        .to_string();
    *s = Regex::new(RE_SPLITTER1)
        .unwrap()
        .split(&s)
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&str>>()
        .join(", ");
    *s = Regex::new(RE_SPACES)
        .unwrap()
        .replace_all(&s, " ")
        .to_string();
    *s = s
        .replace("- ", "-")
        .replace(", , ", ", ")
        .replace("--", "-");
}

/// Split given string by non alphanumeric symbol and return a `Vec<&str>`
///
/// # Arguments
///
/// * `s` - An input string that should be split
///
/// # Examples
///
/// ```
/// let parts = split("a-b.c")
/// assert_eq(parts, vec!["a", "b", "c"])
/// ```
pub fn split(s: &str) -> Vec<&str> {
    let split_regex = Regex::new(RE_SPLITTER2).unwrap();
    split_regex.split(&s).filter(|&x| !x.is_empty()).collect()
}

/// Return a `Vec` of CA and US countries or a single country `Vec`
///
/// # Arguments
///
/// * `country` - An optional `Country`
///
/// # Examples
///
/// ```
/// let countries = get_countries(None);
/// assert_eq(countries[0].code, "US".to_string());
/// assert_eq(countries[1].code, "CA".to_string());
/// ```
pub fn get_countries(country: &Option<Country>) -> Vec<Country> {
    let us = Country {
        code: "US".to_string(),
        name: "United States".to_string(),
    };
    let ca = Country {
        code: "CA".to_string(),
        name: "Canada".to_string(),
    };
    match country {
        Some(c) => vec![c.clone()],
        _ => vec![us, ca],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean() {
        let mut s = "canada,".to_string();
        clean(&mut s);
        assert_eq!(s, "canada".to_string());
        s = "!--?(invalid)Toronto/".to_string();
        clean(&mut s);
        assert_eq!(s, "Toronto".to_string());
        let mut s = "Dundas St W (Store# 04278)".to_string();
        clean(&mut s);
        assert_eq!(s, "Dundas St W".to_string());
    }

    #[test]
    fn test_split() {
        let s = "s - s !! test";
        let parts = split(&s);
        assert_eq!(parts, vec!["s", "s", "test"])
    }

    #[test]
    fn test_get_countries() {
        let countries = get_countries(&None);
        assert_eq!(countries.len(), 2);
        assert_eq!(countries[0].code, "US".to_string());
        assert_eq!(countries[1].code, "CA".to_string());
        let countries = get_countries(&Some(Country {
            code: "US".to_string(),
            name: "United States".to_string(),
        }));
        assert_eq!(countries.len(), 1);
        assert_eq!(countries[0].code, "US".to_string());
        let countries = get_countries(&Some(Country {
            code: "CA".to_string(),
            name: "Canada".to_string(),
        }));
        assert_eq!(countries.len(), 1);
        assert_eq!(countries[0].code, "CA".to_string());
    }

    #[test]
    fn test_regex_patterns_can_compile() {
        Regex::new(RE_BRACKETS).unwrap();
        Regex::new(RE_LEADING).unwrap();
        Regex::new(RE_TRAILING).unwrap();
        Regex::new(RE_SPLITTER1).unwrap();
        Regex::new(RE_SPLITTER2).unwrap();
    }
}
