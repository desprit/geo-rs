use crate::utils;
use crate::{Location, Parser};
use std::collections::HashMap;
use std::fmt;
use titlecase::titlecase;

#[derive(Debug, Clone, Hash, Eq)]
pub struct City {
    pub name: String,
}

impl PartialEq for City {
    fn eq(&self, other: &City) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.trim())
    }
}

impl Parser {
    pub fn remove_city(&self, s: &mut String, city: &City) {
        *s = s.replace(&city.name, "");
        utils::clean(s);
    }

    /// TODO
    pub fn find_special_case_city(&self, s: &str) -> Option<City> {
        if s.to_lowercase().contains("district of columbia") {
            return Some(City {
                name: String::from("Washington"),
            });
        }
        if s.to_lowercase().contains("d.c.") {
            return Some(City {
                name: String::from("Washington"),
            });
        }
        None
    }

    /// Parse location string and try to extract city out of it.
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
    ///     state: Some(State { code: String::from("ON"), name: String::from("Ontario") }),
    ///     country: Some(Country { code: String::from("CA"), name: String::from("Canada") }),
    ///     zipcode: None,
    ///     address: None,
    /// };
    /// parser.fill_city(&mut location, "Toronto, ON, CA");
    /// let city = location.state.unwrap();
    /// assert_eq!(city.name, String::from("Toronto"));
    /// ```
    pub fn fill_city(&self, location: &mut Location, input: &str) {
        if location.state.is_some() & location.country.is_none() {
            self.fill_country_from_state(location);
        }
        let input_cleaned = input
            .to_lowercase()
            .split(",")
            .next()
            .unwrap_or("")
            .to_string();
        for c in utils::get_countries(&location.country) {
            let states = match &location.state {
                Some(s) => vec![&s.code],
                None => match self.states.get(&c.code) {
                    Some(country_states) => {
                        country_states.code_to_name.keys().collect::<Vec<&String>>()
                    }
                    None => vec![],
                },
            };
            if let Some(country_cities) = &self.cities.get(&c.code) {
                let mut candidates: Vec<(String, String)> = vec![];
                for s in states {
                    if let Some(state_cities) = country_cities.cities_by_state.get(s) {
                        if state_cities.contains(&input_cleaned.to_string()) {
                            candidates.push((s.clone(), input_cleaned.clone()))
                        }
                    }
                }
                if candidates.len() == 1 {
                    location.city = Some(City {
                        name: String::from(titlecase(candidates.first().unwrap().1.as_str())),
                    });
                } else {
                    debug!("Found 2+ candidates for a city {:?}", input_cleaned);
                }
            }
        }
        if let Some(ct) = self.find_special_case_city(&input_cleaned) {
            location.city = Some(City {
                name: String::from(titlecase(&ct.name)),
            });
        }
    }
}

#[derive(Debug)]
pub struct CitiesMap {
    pub cities_by_state: HashMap<String, Vec<String>>,
    pub state_of_city: HashMap<String, String>,
}

pub type CountryCities = HashMap<String, CitiesMap>;

/// Read US and CA states GEO data and create a map between
/// state names and state abbreviations and vice-versa.
///
/// # Examples
///
/// ```
/// use geo_rs;
/// let states = geo_rs::nodes::read_states();
/// ```
pub fn read_cities() -> HashMap<String, CitiesMap> {
    let mut data: HashMap<String, CitiesMap> = HashMap::new();
    for country in ["US", "CA"].iter() {
        let filename = format!("{}/{}.txt", &country, "cities");
        let mut cities_by_state: HashMap<String, Vec<String>> = HashMap::new();
        let mut state_of_city: HashMap<String, String> = HashMap::new();
        for line in utils::read_lines(&filename) {
            if let Ok(s) = line {
                let parts: Vec<&str> = s.split(";").collect();
                match cities_by_state.get_mut(parts[0]) {
                    Some(state_cities) => {
                        state_cities.push(parts[1].to_lowercase().to_string());
                    }
                    None => {
                        cities_by_state.insert(
                            parts[0].to_string(),
                            vec![parts[1].to_lowercase().to_string()],
                        );
                    }
                }
                state_of_city.insert(parts[1].to_string(), parts[0].to_string());
            }
        }
        data.insert(
            country.to_string(),
            CitiesMap {
                cities_by_state,
                state_of_city,
            },
        );
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;

    #[test]
    fn test_read_cities() {
        let cities = super::read_cities();
        assert!(cities.get("US").is_some());
        assert!(cities.get("CA").is_some());
        let us_cities = cities.get("US").unwrap();
        assert!(us_cities.state_of_city.get("New York").is_some());
        let ca_cities = cities.get("CA").unwrap();
        assert!(ca_cities.cities_by_state.get("ON").is_some());
        assert!(ca_cities.state_of_city.get("Toronto").is_some());
        let ca_state_cities = ca_cities.cities_by_state.get("ON").unwrap();
        assert!(ca_state_cities.contains(&"Toronto".to_string()));
        let us_state_cities = us_cities.cities_by_state.get("NY").unwrap();
        assert!(us_state_cities.contains(&"New York".to_string()));
    }

    #[test]
    fn test_california_cities() {
        let parser = Parser::new();
        parser
            .cities
            .get("US")
            .unwrap()
            .cities_by_state
            .get("CA")
            .unwrap();
    }

    #[test]
    fn test_find_special_case_city() {
        let mut cities: HashMap<&str, Option<City>> = HashMap::new();
        cities.insert(
            "United States-District of Columbia-washington-20340-DCCL",
            Some(City {
                name: String::from("Washington"),
            }),
        );
        cities.insert(
            "United States-washington d.c.-20340-DCCL",
            Some(City {
                name: String::from("Washington"),
            }),
        );
        let parser = Parser::new();
        for (input, city) in cities {
            let output = parser.find_special_case_city(&input);
            assert_eq!(output, city);
        }
    }

    #[test]
    fn test_fill_city() {
        let parser = Parser::new();
        for (input, output) in mocks::get_mocks() {
            let mut location = Location {
                city: None,
                state: output.1,
                country: output.2,
                zipcode: output.3,
                address: None,
            };
            let mut input_string = String::from(input);
            if let Some(z) = &location.zipcode {
                parser.remove_zipcode(&z, &mut input_string);
            }
            if let Some(c) = &location.country {
                parser.remove_country(&c, &mut input_string);
            }
            if let (Some(s), Some(c)) = (&location.state, &location.country) {
                parser.remove_state(&s, &c, &mut input_string);
            }
            parser.fill_city(&mut location, input_string.as_str());
            assert_eq!(location.city, output.0, "input: {}", input);
        }
    }

    #[test]
    fn test_remove_city() {
        let mut cities: HashMap<&str, (City, &str)> = HashMap::new();
        cities.insert(
            "Lansing, MI, US, 48911",
            (
                City {
                    name: String::from("Lansing"),
                },
                "MI, US, 48911",
            ),
        );
        cities.insert(
            "Toronto, ON, Canada",
            (
                City {
                    name: String::from("Toronto"),
                },
                "ON, Canada",
            ),
        );
        cities.insert(
            "United States-California-San Diego-US CA San Diego",
            (
                City {
                    name: String::from("San Diego"),
                },
                "United States-California-US CA",
            ),
        );
        let parser = Parser::new();
        for (k, (city, output)) in cities {
            let mut input = k.to_string();
            parser.remove_city(&mut input, &city);
            assert_eq!(input, output);
        }
    }
}
