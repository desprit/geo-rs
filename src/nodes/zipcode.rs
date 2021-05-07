use crate::utils;
use crate::{Country, Parser};
use regex::Regex;
use std::fmt;

const US_PATTERN: &str = r"\d{5}(?:[-\s]\d{4})?";
const CA_PATTERN: &str =
    r"[ABCEGHJKLMNPRSTVXY][0-9][ABCEGHJKLMNPRSTVWXYZ] ?[0-9][ABCEGHJKLMNPRSTVWXYZ][0-9]";

#[derive(Debug, Clone, Hash, Eq)]
pub struct Zipcode {
    pub zipcode: String,
    pub country: Country,
}

impl PartialEq for Zipcode {
    fn eq(&self, other: &Zipcode) -> bool {
        self.zipcode == other.zipcode
    }
}

impl fmt::Display for Zipcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.zipcode.replace(" ", ""))
    }
}

impl Parser {
    pub fn find_zipcode(&self, s: &str) -> Option<Zipcode> {
        let us_regex = Regex::new(US_PATTERN).unwrap();
        let ca_regex = Regex::new(CA_PATTERN).unwrap();
        if let Some(m) = ca_regex.find(&s) {
            return Some(Zipcode {
                zipcode: s[m.start()..m.end()].to_string(),
                country: Country {
                    code: String::from("CA"),
                    name: String::from("Canada"),
                },
            });
        }
        for part in utils::split(&s) {
            let has_correct_len = vec![5, 9, 10].contains(&part.chars().count());
            let has_correct_chars = &part.chars().all(|c| {
                c.is_numeric()
                    || c.to_string() == "-".to_string()
                    || c.to_string() == " ".to_string()
            });
            if has_correct_len & has_correct_chars {
                if let Some(m) = us_regex.find(&s) {
                    return Some(Zipcode {
                        zipcode: s[m.start()..m.end()].replace(" ", "").to_string(),
                        country: Country {
                            code: String::from("US"),
                            name: String::from("United States"),
                        },
                    });
                }
            }
        }
        None
    }

    pub fn remove_zipcode(&self, s: &mut String, zipcode: &Zipcode) {
        *s = s.replace(&zipcode.zipcode, "");
        utils::clean(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_find_zipcode() {
        let mut zipcodes: HashMap<&str, Option<Zipcode>> = HashMap::new();
        zipcodes.insert("Saint-Lin-Laurentides, QC J5MM 0G3", None);
        zipcodes.insert("Saint-Lin-Laurentides, QC", None);
        zipcodes.insert("Saint-Lin-Laurentides, QC 11111111", None);
        zipcodes.insert(
            "Lansing, MI, US, 48911",
            Some(Zipcode {
                zipcode: String::from("48911"),
                country: Country {
                    code: String::from("US"),
                    name: String::from("United States"),
                },
            }),
        );
        zipcodes.insert(
            "Saint-Lin-Laurentides, QC J5M 0G3",
            Some(Zipcode {
                zipcode: String::from("J5M 0G3"),
                country: Country {
                    code: String::from("CA"),
                    name: String::from("Canada"),
                },
            }),
        );
        zipcodes.insert(
            "Sherwood Park, AB, CA, T8A3H9",
            Some(Zipcode {
                zipcode: String::from("T8A3H9"),
                country: Country {
                    code: String::from("CA"),
                    name: String::from("Canada"),
                },
            }),
        );
        zipcodes.insert("Lansing, MI, US", None);
        zipcodes.insert("Lansing, MI, US, 67139037", None);
        let parser = Parser::new();
        for (k, v) in zipcodes {
            let zipcode = parser.find_zipcode(&k);
            assert_eq!(zipcode, v);
        }
    }

    #[test]
    fn test_remove_zipcode() {
        let mut zipcodes: HashMap<&str, &str> = HashMap::new();
        zipcodes.insert("Lansing, MI, US, 48911", "Lansing, MI, US");
        zipcodes.insert(
            "Saint-Lin-Laurentides, QC J5M 0G3",
            "Saint-Lin-Laurentides, QC",
        );
        let parser = Parser::new();
        for (k, v) in zipcodes {
            let mut input = k.to_string();
            let zipcode = parser.find_zipcode(k).unwrap();
            parser.remove_zipcode(&mut input, &zipcode);
            assert_eq!(input, v);
        }
    }

    #[test]
    fn test_regex_patterns_can_compile() {
        Regex::new(US_PATTERN).unwrap();
        Regex::new(CA_PATTERN).unwrap();
    }

    #[test]
    fn test_zipcode_display() {
        let zipcode = Zipcode {
            zipcode: String::from("J5M 0G3"),
            country: Country {
                code: String::from("US"),
                name: String::from("United States"),
            },
        };
        assert_eq!(format!("{}", zipcode), "J5M0G3");
    }
}
