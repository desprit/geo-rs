use crate::Country;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

lazy_static! {
    static ref RE_BRACKETS: Regex = Regex::new(r"\(.*?\)").unwrap();
    static ref RE_LEADING: Regex = Regex::new(r"^[\s\-,;:_\.\?!/]*").unwrap();
    static ref RE_TRAILING: Regex = Regex::new(r"[\s\-,;:_\.\?!/]*$").unwrap();
    static ref RE_SPLITTER1: Regex = Regex::new(r"[^a-zA-Z0-9\s-]").unwrap();
    static ref RE_SPLITTER2: Regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    static ref RE_SPACES: Regex = Regex::new(r"\s+").unwrap();
    static ref RE_ABBREVIATIONS: Regex =
        Regex::new(r"\b(?:[A-Z]{3,5}\b|(?:[A-Za-z]\.){3,})\s*").unwrap();
}

/// Read file with the given name from `src/data` folder and return `std::io::Lines`
///
/// # Arguments
///
/// * `filename` - Name of the file to read
///
/// # Examples
///
/// ```
/// use geo_rs;
/// let lines = geo_rs::utils::read_lines("countries.txt");
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
/// use geo_rs;
/// let mut s = String::from("!(#3)Toronto ,");
/// geo_rs::utils::clean(&mut s);
/// assert_eq!(s, String::from("Toronto"));
/// ```
pub fn clean(s: &mut String) {
    *s = RE_ABBREVIATIONS.replace_all(&s, "").to_string();
    *s = RE_BRACKETS.replace_all(&s, "").to_string();
    *s = RE_LEADING.replace_all(&s, "").to_string();
    *s = RE_TRAILING.replace_all(&s, "").to_string();
    *s = RE_SPLITTER1
        .split(&s)
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&str>>()
        .join(", ");
    *s = RE_SPACES.replace_all(&s, " ").to_string();
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
/// use geo_rs;
/// let parts = geo_rs::utils::split("a-b.c");
/// assert_eq!(parts, vec!["a", "b", "c"]);
/// ```
pub fn split(s: &str) -> Vec<&str> {
    RE_SPLITTER2.split(&s).filter(|&x| !x.is_empty()).collect()
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
/// use geo_rs;
/// let countries = geo_rs::utils::get_countries(&None);
/// assert_eq!(countries[0].code, "US".to_string());
/// assert_eq!(countries[1].code, "CA".to_string());
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
        let mut s = "United States-District of Columbia-washington-20340-DCCL".to_string();
        clean(&mut s);
        assert_eq!(
            s,
            "United States-District of Columbia-washington-20340".to_string()
        );
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
}
