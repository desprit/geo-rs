use crate::utils;
use crate::Parser;
use regex::Regex;

const US_PATTERN: &str = r"\d{5}(?:[-\s]\d{4})?";
const CA_PATTERN: &str =
    r"[ABCEGHJKLMNPRSTVXY][0-9][ABCEGHJKLMNPRSTVWXYZ] ?[0-9][ABCEGHJKLMNPRSTVWXYZ][0-9]";

#[derive(Debug)]
pub struct Zipcode {
    pub zipcode: String,
    pub country: String,
}

impl PartialEq for Zipcode {
    fn eq(&self, other: &Zipcode) -> bool {
        self.zipcode == other.zipcode
    }
}

impl Parser {
    pub fn remove_zipcode(&self, s: &mut String, zipcode: &Zipcode) -> String {
        utils::clean(s);
        s.to_owned()
    }

    pub fn find_zipcode(&self, s: &str) -> Option<Zipcode> {
        let us_regex = Regex::new(US_PATTERN).unwrap();
        let ca_regex = Regex::new(CA_PATTERN).unwrap();
        if let Some(m) = ca_regex.find(&s) {
            return Some(Zipcode {
                zipcode: s[m.start()..m.end()].replace(" ", "").to_string(),
                country: String::from("CA"),
            });
        }
        for part in utils::split(&s) {
            let has_correct_len = vec![5, 9, 10].contains(&part.chars().count());
            let has_correct_chars = &part.chars().all(|c| {
                c.is_numeric()
                    || c.to_string() == "-".to_string()
                    || c.to_string() == " ".to_string()
            });
            if !has_correct_len || !has_correct_chars {
                continue;
            }
            if let Some(m) = us_regex.find(&s) {
                return Some(Zipcode {
                    zipcode: s[m.start()..m.end()].replace(" ", "").to_string(),
                    country: String::from("US"),
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_find_zipcode() {
        let mut zipcodes: HashMap<&str, Option<Zipcode>> = HashMap::new();
        zipcodes.insert(
            "Lansing, MI, US, 48911",
            Some(Zipcode {
                zipcode: String::from("48911"),
                country: String::from("US"),
            }),
        );
        zipcodes.insert(
            "Saint-Lin-Laurentides, QC J5M 0G3",
            Some(Zipcode {
                zipcode: String::from("J5M0G3"),
                country: String::from("CA"),
            }),
        );
        zipcodes.insert(
            "Sherwood Park, AB, CA, T8A3H9",
            Some(Zipcode {
                zipcode: String::from("T8A3H9"),
                country: String::from("CA"),
            }),
        );
        zipcodes.insert("Lansing, MI, US", None);
        zipcodes.insert("Lansing, MI, US, 67139037", None);
        let parser = Parser::new(None);
        for (k, v) in zipcodes {
            let zipcode = parser.find_zipcode(&k);
            assert_eq!(zipcode, v);
        }
    }

    #[test]
    fn test_regex_patterns_can_compile() {
        Regex::new(US_PATTERN).unwrap();
        Regex::new(CA_PATTERN).unwrap();
    }
}
