use super::{Location, CANADA, UNITED_STATES};
use crate::utils;
use crate::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

lazy_static! {
    static ref US_PATTERN: Regex = Regex::new(r"\d{5}(?:[-\s]\d{4})?").unwrap();
    static ref CA_PATTERN: Regex = Regex::new(
        r"[ABCEGHJKLMNPRSTVXY][0-9][ABCEGHJKLMNPRSTVWXYZ] ?[0-9][ABCEGHJKLMNPRSTVWXYZ][0-9]"
    )
    .unwrap();
}

#[derive(Debug, Clone, Hash, Eq)]
pub struct Zipcode {
    pub zipcode: String,
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
    /// Parse location string and try to extract zipcode out of it.
    /// Add zipcode and it's country to the location struct on success.
    ///
    /// # Arguments
    ///
    /// * `location` - Location struct that stores final values
    /// * `input` - Location string to be parsed
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_rs;
    /// let parser = geo_rs::Parser::new();
    /// let mut location = geo_rs::nodes::Location {
    ///     city: None,
    ///     state: None,
    ///     country: None,
    ///     zipcode: None,
    ///     address: None,
    /// };
    /// parser.find_zipcode(&mut location, "Saint-Lin-Laurentides, QC J5M 0G3");
    /// assert_eq!(location.zipcode.unwrap().zipcode, String::from("J5M 0G3"));
    /// assert_eq!(location.country.unwrap().code, String::from("CA"));
    /// ```
    pub fn find_zipcode(&self, location: &mut Location, input: &str) {
        if input.chars().count() == 0 {
            return;
        }
        if let Some(zipcode) = CA_PATTERN.find(&input) {
            location.zipcode = Some(Zipcode {
                zipcode: input[zipcode.start()..zipcode.end()].to_string(),
            });
            location.country = Some(CANADA.clone());
            return;
        }
        for part in utils::split(&input) {
            let has_correct_len = vec![5, 9, 10].contains(&part.chars().count());
            let has_correct_chars = &part.chars().all(|c| {
                c.is_numeric()
                    || c.to_string() == "-".to_string()
                    || c.to_string() == " ".to_string()
            });
            if has_correct_len & has_correct_chars {
                if let Some(zipcode) = US_PATTERN.find(&input) {
                    location.zipcode = Some(Zipcode {
                        zipcode: input[zipcode.start()..zipcode.end()].to_string(),
                    });
                    location.country = Some(UNITED_STATES.clone());
                    return;
                }
            }
        }
    }

    /// Remove zipcode from location string.
    ///
    /// # Arguments
    ///
    /// * `zipcode` - Zipcode to be removed
    /// * `input` - Location string from which zipcode is removed
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_rs;
    /// let parser = geo_rs::Parser::new();
    /// let mut location = String::from("QC J5MM 0G3");
    /// let zipcode = geo_rs::nodes::Zipcode { zipcode: String::from("J5MM 0G3") };
    /// parser.remove_zipcode(&zipcode, &mut location);
    /// assert_eq!(location, String::from("QC"));
    /// ```
    pub fn remove_zipcode(&self, zipcode: &Zipcode, input: &mut String) {
        *input = input.replace(&zipcode.zipcode, "");
        utils::clean(input);
        debug!("after removing zipcode: {}", input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;

    #[test]
    fn test_find_zipcode() {
        let parser = Parser::new();
        for (input, output) in mocks::get_mocks() {
            let mut location = Location {
                city: None,
                state: None,
                country: None,
                zipcode: None,
                address: None,
            };
            parser.find_zipcode(&mut location, &input);
            assert_eq!(location.zipcode, output.3, "input: {}", input);
        }
    }

    #[test]
    fn test_remove_zipcode() {
        let parser = Parser::new();
        let zipcode = Zipcode {
            zipcode: String::from("T8A3H9"),
        };
        let mut location = String::from("Sherwood Park, AB, CA, T8A3H9");
        parser.remove_zipcode(&zipcode, &mut location);
        assert_eq!(location, String::from("Sherwood Park, AB, CA"));
        let zipcode = Zipcode {
            zipcode: String::from("J5M 0G3"),
        };
        let mut location = String::from("Montreal, QC J5M 0G3");
        parser.remove_zipcode(&zipcode, &mut location);
        assert_eq!(location, String::from("Montreal, QC"));
    }

    #[test]
    fn test_zipcode_display() {
        let zipcode = Zipcode {
            zipcode: String::from("J5M 0G3"),
        };
        assert_eq!(format!("{}", zipcode), "J5M0G3");
    }

    /// cargo test benchmark_find_zipcode -- --nocapture --ignored
    #[test]
    #[ignore]
    fn benchmark_find_zipcode() {
        let n = 250;
        let parser = Parser::new();
        let before = std::time::Instant::now();
        for _ in 0..n {
            for zipcode in mocks::get_mocks().keys() {
                let mut location = Location {
                    city: None,
                    state: None,
                    country: None,
                    zipcode: None,
                    address: None,
                };
                parser.find_zipcode(&mut location, &zipcode);
            }
        }
        println!(
            "Elapsed time: {:.2?}, {:.2?} each",
            before.elapsed(),
            before.elapsed() / (n * mocks::get_mocks().len() as u32)
        );
    }
}
