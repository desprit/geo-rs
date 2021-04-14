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
    pub fn find_zipcode(&self, s: &str) -> Option<Zipcode> {
        let us_regex = Regex::new(US_PATTERN).unwrap();
        let ca_regex = Regex::new(CA_PATTERN).unwrap();
        if let Some(m) = us_regex.find(&s) {
            return Some(Zipcode {
                zipcode: s[m.start()..m.end()].replace(" ", "").to_string(),
                country: String::from("US"),
            });
        }
        if let Some(m) = ca_regex.find(&s) {
            return Some(Zipcode {
                zipcode: s[m.start()..m.end()].replace(" ", "").to_string(),
                country: String::from("CA"),
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Regex, Zipcode, CA_PATTERN, US_PATTERN};
    use crate::Parser;
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
