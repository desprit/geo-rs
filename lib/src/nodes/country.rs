use super::Location;
use crate::utils;
use crate::Parser;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Hash, Eq)]
pub struct Country {
    pub name: String,
    pub code: String,
}

lazy_static! {
    pub static ref UNITED_STATES: Country = Country {
        code: String::from("US"),
        name: String::from("United States"),
    };
    pub static ref CANADA: Country = Country {
        code: String::from("CA"),
        name: String::from("Canada"),
    };
}

impl PartialEq for Country {
    fn eq(&self, other: &Country) -> bool {
        self.name == other.name && self.code == other.code
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code.trim())
    }
}

#[derive(Debug)]
pub struct CountriesMap {
    pub code_to_name: HashMap<String, String>,
    pub name_to_code: HashMap<String, String>,
}

impl Parser {
    /// Parse location string and try to extract country out of it.
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
    /// parser.fill_country(&mut location, "Toronto, ON, CA");
    /// assert_eq!(location.country, Some(geo_rs::nodes::CANADA.clone()));
    /// ```
    pub fn fill_country(&self, location: &mut Location, input: &str) {
        if input.chars().count() == 0 {
            return;
        }
        if location.country.is_some() {
            return;
        }
        let as_lowercase = input.to_lowercase().to_string();
        let parts = utils::split(&as_lowercase);
        for part in &parts {
            if vec!["usa", "us"].contains(&part) {
                location.country = Some(UNITED_STATES.clone());
                return;
            }
            if vec!["canada"].contains(&part) {
                location.country = Some(CANADA.clone());
                return;
            }
        }
        if as_lowercase.contains("united states") {
            location.country = Some(UNITED_STATES.clone());
            return;
        }
        if parts.contains(&"ca") {
            let ca_states = self.states.get("CA").unwrap();
            let codes: Vec<&String> = ca_states.code_to_name.keys().collect();
            let names: Vec<&String> = ca_states.name_to_code.keys().collect();
            if parts
                .iter()
                .find(|x| codes.contains(&&x.to_uppercase()))
                .is_some()
            {
                location.country = Some(CANADA.clone());
                return;
            }
            if parts
                .iter()
                .find(|x| names.contains(&&x.to_string()))
                .is_some()
            {
                location.country = Some(CANADA.clone());
                return;
            }
            let ca_cities: Vec<&String> = self
                .cities
                .get("CA")
                .unwrap()
                .cities_by_state
                .values()
                .flatten()
                .collect();
            let us_cities = self.cities.get("US").unwrap();
            let california_cities = us_cities.cities_by_state.get("CA").unwrap();
            if california_cities
                .iter()
                .find(|x| {
                    // Check whether input string has a California city in it
                    if !as_lowercase.contains(&x.to_lowercase()) {
                        return false;
                    }
                    // Make sure that California city is not also a Canadian city
                    if ca_cities.contains(x) {
                        return false;
                    }
                    return true;
                })
                .is_some()
            {
                return;
            }
        }
        if input.contains("US") {
            location.country = Some(UNITED_STATES.clone());
        }
        if input.contains("CA") {
            location.country = Some(CANADA.clone());
        }
        // Search fill country name in the input string, ignore country if code is also US or CA state,
        // For example, ignore country code PA (Panama) because it's also Pennsylvania
        for (country_name, country_code) in self.countries.name_to_code.iter() {
            if as_lowercase.contains(&country_name.to_lowercase()) {
                if let Some(us_states) = self.states.get("US") {
                    if us_states
                        .name_to_code
                        .keys()
                        .find(|name| name == &country_name)
                        .is_some()
                    {
                        continue;
                    }
                }
                if let Some(ca_states) = self.states.get("CA") {
                    if ca_states
                        .name_to_code
                        .keys()
                        .find(|name| name == &country_name)
                        .is_some()
                    {
                        continue;
                    }
                }
                location.country = Some(Country {
                    name: String::from(country_name),
                    code: String::from(country_code),
                });
                return;
            }
        }
        // Search country code in the input string, ignore country if code is also US or CA state,
        // For example, ignore country code PA (Panama) because it's also Pennsylvania
        for (country_name, country_code) in self.countries.name_to_code.iter() {
            if let Some(us_states) = self.states.get("US") {
                if us_states.code_to_name.contains_key(country_code) {
                    continue;
                }
            }
            if let Some(ca_states) = self.states.get("CA") {
                if ca_states.code_to_name.contains_key(country_code) {
                    continue;
                }
            }
            if utils::split(&input.to_string()).contains(&country_code.as_str()) {
                location.country = Some(Country {
                    code: country_code.clone(),
                    name: country_name.clone(),
                });
                return;
            }
        }
    }

    /// Remove country from location string.
    ///
    /// # Arguments
    ///
    /// * `country` - Country to be removed
    /// * `input` - Location string from which country is removed
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_rs;
    /// let parser = geo_rs::Parser::new();
    /// let mut location = String::from("New York, NY, US");
    /// let country = geo_rs::nodes::Country {
    ///     code: String::from("US"),
    ///     name: String::from("United States"),
    /// };
    /// parser.remove_country(&country, &mut location);
    /// assert_eq!(location, String::from("New York, NY"));
    /// ```
    pub fn remove_country(&self, country: &Country, input: &mut String) {
        let case_insensitive_parts: Vec<String> = match country.code.as_str() {
            "US" => vec![
                String::from("united states of america"),
                String::from("united states"),
            ],
            "CA" => vec![String::from("canada")],
            _ => vec![country.name.to_lowercase()],
        };
        let case_sensitive_parts: Vec<String> = match country.code.as_str() {
            "US" => vec![String::from("USA"), String::from("US")],
            "CA" => vec![String::from("CA")],
            _ => vec![country.code.clone()],
        };
        for part in &case_insensitive_parts {
            if let Some(start) = input.to_lowercase().find(part) {
                input.replace_range(start..part.chars().count() + start, "");
            }
        }
        for part in case_sensitive_parts {
            *input = input.replace(&part, "");
        }
        utils::clean(input);
        debug!("after removing country: {}", input);
    }
}

/// Read US and CA states GEO data and create a map between
/// state names and state abbreviations and vice-versa.
///
/// # Examples
///
/// ```
/// use geo_rs;
/// let countries = geo_rs::nodes::read_countries();
/// ```
pub fn read_countries() -> CountriesMap {
    let mut name_to_code: HashMap<String, String> = HashMap::new();
    let mut code_to_name: HashMap<String, String> = HashMap::new();
    for line in utils::read_lines("countries.txt") {
        if let Ok(s) = line {
            let parts: Vec<&str> = s.split(";").collect();
            code_to_name.insert(parts[1].to_string(), parts[0].to_string());
            name_to_code.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    CountriesMap {
        name_to_code,
        code_to_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;

    #[test]
    fn test_ca() {
        let parser = Parser::new();
        parser.countries.code_to_name.get("CA").unwrap();
    }

    #[test]
    fn test_us() {
        let parser = Parser::new();
        parser.countries.code_to_name.get("US").unwrap();
    }

    #[test]
    fn test_country_display() {
        let country = Country {
            code: String::from(" US "),
            name: String::from("United States"),
        };
        assert_eq!(format!("{}", country), "US");
    }

    #[test]
    fn test_remove_country() {
        let parser = Parser::new();
        let country = CANADA.clone();
        let mut location = String::from("Sherwood Park, AB, CA");
        parser.remove_country(&country, &mut location);
        assert_eq!(location, String::from("Sherwood Park, AB"));
        let country = CANADA.clone();
        let mut location = String::from("Toronto, ON, Canada");
        parser.remove_country(&country, &mut location);
        assert_eq!(location, String::from("Toronto, ON"));
        let country = UNITED_STATES.clone();
        let mut location = String::from("United States-California-San Diego-US CA San Diego");
        parser.remove_country(&country, &mut location);
        assert_eq!(location, String::from("California-San Diego-CA San Diego"));
        let country = UNITED_STATES.clone();
        let mut location = String::from("Lansing, MI, US");
        parser.remove_country(&country, &mut location);
        assert_eq!(location, String::from("Lansing, MI"));
        let country = Country {
            code: String::from("ES"),
            name: String::from("Spain"),
        };
        let mut location = String::from("Barcelona, ES");
        parser.remove_country(&country, &mut location);
        assert_eq!(location, String::from("Barcelona"));
    }

    /// cargo test benchmark_fill_country -- --nocapture --ignored
    #[test]
    #[ignore]
    fn benchmark_fill_country() {
        let n = 250;
        let parser = Parser::new();
        let before = std::time::Instant::now();
        for _ in 0..n {
            for country in mocks::get_mocks().keys() {
                let mut location = Location {
                    city: None,
                    state: None,
                    country: None,
                    zipcode: None,
                    address: None,
                };
                parser.fill_country(&mut location, &country);
            }
        }
        println!(
            "Elapsed time: {:.2?}, {:.2?} each",
            before.elapsed(),
            before.elapsed() / (n * mocks::get_mocks().len() as u32)
        );
    }
}
