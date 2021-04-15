use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const RE_BRACKETS: &str = r"\(.*?\)";
const RE_LEADING: &str = r"^[\s\-,;:_\.\?!/]*";
const RE_TRAILING: &str = r"[\s\-,;:_\.\?!/]*$";
const RE_SPLITTER1: &str = r"[^a-zA-Z0-9\s]";
const RE_SPLITTER2: &str = r"[^a-zA-Z0-9]";
const RE_SPACES: &str = r"\s+";

/// Read data from a given path and return as a HashMap
///
/// # Examples
///
/// ```
/// use crate::utils::read_data;
/// let countries = read_data("countries.txt")
/// ```
pub fn read_file(filename: &str) -> HashMap<String, String> {
    let data_path = format!("{}/src/data", env!("CARGO_MANIFEST_DIR"));
    let file_path = Path::new(&data_path).join(&filename);
    let mut data: HashMap<String, String> = HashMap::new();
    let file = File::open(file_path).unwrap();
    for line in io::BufReader::new(file).lines() {
        if let Ok(s) = line {
            let parts: Vec<&str> = s.split(";").collect();
            data.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    data
}

pub fn read_lines(filename: &str) -> std::io::Lines<BufReader<File>> {
    let data_path = format!("{}/src/data", env!("CARGO_MANIFEST_DIR"));
    let file_path = Path::new(&data_path).join(&filename);
    let file = File::open(file_path).unwrap();
    io::BufReader::new(file).lines()
}

/// Read specific type of GEO data for a given country from file
///
/// # Arguments
///
/// * `country` - Name of the country, e.g. `US`
/// * `filename` - Type of GEO data to read, e.g. `cities`
///
/// # Examples
///
/// ```
/// use crate::utils::read_country_data;
/// read_country_data("US", "states");
/// ```
pub fn read_country_data(
    country: &str,
    filename: &str,
) -> HashMap<String, HashMap<String, String>> {
    let filename = format!("{}/{}.txt", &country.to_uppercase(), &filename);
    let mut data = HashMap::new();
    data.insert(country.to_string(), read_file(&filename));
    data
}

/// Read specific type of GEO data for all countries
///
/// # Arguments
///
/// * `filename` - Type of GEO data to read, e.g. `cities`
///
/// # Examples
///
/// ```
/// use crate::utils::read_all_countries;
/// read_all_countries("cities");
/// ```
pub fn read_all_countries(filename: &str) -> HashMap<String, HashMap<String, String>> {
    let data_path = format!("{}/src/data", env!("CARGO_MANIFEST_DIR"));
    let mut data = HashMap::new();
    for path in Path::new(&data_path).read_dir().expect("couldn't read dir") {
        if let Ok(p) = path {
            if p.path().is_dir() {
                if let Ok(c) = p.file_name().into_string() {
                    let country_data_file = format!("{}/{}.txt", &c.as_str(), &filename);
                    data.insert(c.to_string(), read_file(&country_data_file));
                }
            }
        }
    }
    data
}

/// Remove useless garbage from the given string, e.g. trailing commas, values in brackets, etc.
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
    *s = s.replace(",,", "").replace(", , ", ", ");
}

pub fn split(s: &str) -> Vec<&str> {
    let split_regex = Regex::new(RE_SPLITTER2).unwrap();
    split_regex.split(&s).filter(|&x| !x.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_data_should_work() {
        let countries = super::read_file("countries.txt");
        assert_eq!(countries.get("Italy"), Some(&String::from("IT")));
    }

    #[test]
    fn test_read_country_data_should_work() {
        let states = super::read_country_data("US", "states");
        let us_states = states.get("US").unwrap();
        assert_eq!(us_states.get("AL"), Some(&String::from("Alabama")));
    }

    #[test]
    fn test_read_all_countries() {
        let states = super::read_all_countries("states");
        let ca_states = states.get("CA").unwrap();
        assert_eq!(ca_states.get("ON"), Some(&String::from("Ontario")));
        let us_states = states.get("US").unwrap();
        assert_eq!(us_states.get("CA"), Some(&String::from("California")));
    }

    #[test]
    fn test_clean() {
        let mut s = "canada,".to_string();
        clean(&mut s);
        assert_eq!(s, "canada".to_string());
        s = "!--?(invalid)Toronto/".to_string();
        clean(&mut s);
        assert_eq!(s, "Toronto".to_string());
    }

    #[test]
    fn test_split() {
        let s = "s - s !! test";
        let parts = split(&s);
        assert_eq!(parts, vec!["s", "s", "test"])
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
