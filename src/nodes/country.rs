use crate::utils;
use crate::Parser;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Country {
    pub name: String,
    pub code: String,
}

impl PartialEq for Country {
    fn eq(&self, other: &Country) -> bool {
        self.name == other.name && self.code == other.code
    }
}

#[derive(Debug)]
pub struct CountriesMap {
    code_to_name: HashMap<String, String>,
    name_to_code: HashMap<String, String>,
}

impl Parser {
    pub fn remove_country(&self, s: &mut String, country: &Country) -> String {
        let mut matched_country = "";
        let case_insensitive_parts: Vec<&str> = match country.code.as_str() {
            "US" => vec!["united states of america", "united states"],
            "CA" => vec!["canada"],
            _ => vec![],
        };
        for part in &case_insensitive_parts {
            if let Some(start) = s.to_lowercase().find(part) {
                s.replace_range(start..part.chars().count() + start, "");
                if case_insensitive_parts.contains(&"canada") {
                    matched_country = "CA";
                } else {
                    matched_country = "US";
                }
            }
        }
        let case_sensitive_parts: Vec<&str> = match country.code.as_str() {
            "US" if matched_country != "CA" => vec!["USA", "US"],
            "CA" if matched_country != "US" => vec!["CA"],
            _ => vec![],
        };
        for part in case_sensitive_parts {
            *s = s.replace(part, "");
        }
        utils::clean(s);
        s.to_owned()
    }

    pub fn find_country(&self, s: &str) -> Option<Country> {
        let as_lowercase = &s.to_lowercase().to_string();
        let parts = utils::split(as_lowercase);
        for part in &parts {
            if vec!["usa", "us"].contains(&part) {
                return Some(Country {
                    name: "United States".to_string(),
                    code: "US".to_string(),
                });
            }
            if vec!["canada"].contains(&part) {
                return Some(Country {
                    name: "Canada".to_string(),
                    code: "CA".to_string(),
                });
            }
        }
        if as_lowercase.contains("united states") {
            return Some(Country {
                name: "United States".to_string(),
                code: "US".to_string(),
            });
        }
        if parts.contains(&"ca") {
            let canada: Vec<&String> = self.states.get("CA").unwrap().code_to_name.keys().collect();
            if let Some(_) = parts.iter().find(|x| canada.contains(&&x.to_uppercase())) {
                return Some(Country {
                    name: "Canada".to_string(),
                    code: "CA".to_string(),
                });
            }
            let us_cities = self.cities.get("US").unwrap();
            let califoria: &Vec<String> = us_cities.cities_by_state.get("CA").unwrap();
            if let Some(_) = califoria
                .iter()
                .find(|x| as_lowercase.contains(&x.to_lowercase()))
            {
                return Some(Country {
                    name: "United States".to_string(),
                    code: "US".to_string(),
                });
            }
        }
        // TODO: check if string contains states (?)
        None
    }
}

/// Read US and CA states GEO data and create a map between
/// state names and state abbreviations and vice-versa.
///
/// # Examples
///
/// ```
/// let countries = read_countries();
/// ```
pub fn read_countries() -> CountriesMap {
    let mut name_to_code: HashMap<String, String> = HashMap::new();
    let mut code_to_name: HashMap<String, String> = HashMap::new();
    for line in utils::read_lines("countries.txt") {
        if let Ok(s) = line {
            let parts: Vec<&str> = s.split(";").collect();
            name_to_code.insert(parts[1].to_string(), parts[0].to_string());
            code_to_name.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    CountriesMap {
        name_to_code,
        code_to_name,
    }
}

#[cfg(test)]
mod tests {
    use super::{Country, Parser};
    use std::collections::HashMap;

    #[test]
    fn test_find_country() {
        let mut countries: HashMap<&str, Option<Country>> = HashMap::new();
        countries.insert(
            "Lansing, MI, US, 48911",
            Some(Country {
                name: String::from("United States"),
                code: String::from("US"),
            }),
        );
        countries.insert(
            "Jacksonville, Florida, USA",
            Some(Country {
                name: String::from("United States"),
                code: String::from("US"),
            }),
        );
        countries.insert(
            "manati, pr, us",
            Some(Country {
                name: String::from("United States"),
                code: String::from("US"),
            }),
        );
        countries.insert(
            "United States-Alaska-Shemya/Eareckson Air Station",
            Some(Country {
                name: String::from("United States"),
                code: String::from("US"),
            }),
        );
        countries.insert(
            "New Westminster, British Columbia, Canada",
            Some(Country {
                name: String::from("Canada"),
                code: String::from("CA"),
            }),
        );
        countries.insert(
            "Sherwood Park, AB, CA, T8A 3H9",
            Some(Country {
                name: String::from("Canada"),
                code: String::from("CA"),
            }),
        );
        countries.insert(
            "Los Angeles, CA",
            Some(Country {
                name: String::from("United States"),
                code: String::from("US"),
            }),
        );
        countries.insert(
            "ON, CA",
            Some(Country {
                name: String::from("Canada"),
                code: String::from("CA"),
            }),
        );
        let parser = Parser::new(None);
        for (k, v) in countries {
            let country = parser.find_country(&k);
            assert_eq!(country, v, "{}", k);
        }
    }

    #[test]
    fn test_remove_country() {
        let mut countries: HashMap<&str, &str> = HashMap::new();
        countries.insert("Lansing, MI, US, 48911", "Lansing, MI, 48911");
        countries.insert("Toronto, ON, Canada", "Toronto, ON");
        countries.insert(
            "United States-California-San Diego-US CA San Diego",
            "California, San Diego, CA San Diego",
        );
        let parser = Parser::new(None);
        for (k, v) in countries {
            let country = parser.find_country(&k).unwrap();
            let remainder = parser.remove_country(&mut k.to_string(), &country);
            assert_eq!(remainder, v, "{}", k);
        }
    }
}
