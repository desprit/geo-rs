use crate::utils;
use crate::{Country, Parser, State};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Hash, Eq)]
pub struct City {
    pub name: String,
    pub state: Option<String>,
}

impl PartialEq for City {
    fn eq(&self, other: &City) -> bool {
        self.name == other.name && self.state == other.state
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

    pub fn find_special_case_city(&self, s: &str) -> Option<City> {
        if s.to_lowercase().contains("district of columbia") {
            return Some(City {
                name: String::from("Washington"),
                state: Some(String::from("DC")),
            });
        }
        if s.to_lowercase().contains("d.c.") {
            return Some(City {
                name: String::from("Washington"),
                state: Some(String::from("DC")),
            });
        }
        None
    }

    pub fn find_city(&self, s: &str, state: &State, country: &Option<Country>) -> Option<City> {
        if let Some(ct) = self.find_special_case_city(s) {
            return Some(ct);
        }
        let as_lowercase = &s.to_lowercase().to_string();
        let countries = utils::get_countries(country);
        for c in &countries {
            let country_cities = &self.cities.get(&c.code).unwrap().cities_by_state;
            let state_cities = country_cities.get(&state.code).unwrap();
            for city in state_cities.into_iter() {
                if as_lowercase.contains(&city.to_lowercase().as_str()) {
                    return Some(City {
                        name: city.clone(),
                        state: Some(state.code.clone()),
                    });
                }
            }
        }
        None
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
/// let states = read_states();
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
                        state_cities.push(parts[1].to_string());
                    }
                    None => {
                        cities_by_state.insert(parts[0].to_string(), vec![parts[1].to_string()]);
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
    fn test_find_special_case_city() {
        let mut cities: HashMap<&str, Option<City>> = HashMap::new();
        cities.insert(
            "United States-District of Columbia-washington-20340-DCCL",
            Some(City {
                name: String::from("Washington"),
                state: Some(String::from("DC")),
            }),
        );
        cities.insert(
            "United States-washington d.c.-20340-DCCL",
            Some(City {
                name: String::from("Washington"),
                state: Some(String::from("DC")),
            }),
        );
        let parser = Parser::new();
        for (input, city) in cities {
            let output = parser.find_special_case_city(&input);
            assert_eq!(output, city);
        }
    }

    #[test]
    fn test_find_city() {
        let mut cities: HashMap<&str, (Option<City>, State, Option<Country>)> = HashMap::new();
        cities.insert(
            "United States-District of Columbia-washington-20340-DCCL",
            (
                Some(City {
                    name: String::from("Washington"),
                    state: Some(String::from("DC")),
                }),
                State {
                    code: String::from("DC"),
                    name: String::from("District of Columbia"),
                },
                Some(Country {
                    code: String::from("US"),
                    name: String::from("United States"),
                }),
            ),
        );
        cities.insert(
            "Jacksonville, Florida, USA",
            (
                Some(City {
                    name: String::from("Jacksonville"),
                    state: Some(String::from("FL")),
                }),
                State {
                    code: String::from("FL"),
                    name: String::from("Florida"),
                },
                Some(Country {
                    code: String::from("US"),
                    name: String::from("United States"),
                }),
            ),
        );
        cities.insert(
            "New Westminster, British Columbia, Canada",
            (
                Some(City {
                    name: String::from("New Westminster"),
                    state: Some(String::from("BC")),
                }),
                State {
                    code: String::from("BC"),
                    name: String::from("British Columbia"),
                },
                Some(Country {
                    code: String::from("CA"),
                    name: String::from("Canada"),
                }),
            ),
        );
        cities.insert(
            "New Westminster, British Columbia, Canada",
            (
                None,
                State {
                    code: String::from("ON"),
                    name: String::from("Ontario"),
                },
                Some(Country {
                    code: String::from("CA"),
                    name: String::from("Canada"),
                }),
            ),
        );
        let parser = Parser::new();
        for (input, (city, state, country)) in cities {
            let output = parser.find_city(&input, &state, &country);
            assert_eq!(output, city);
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
                    state: Some(String::from("MI")),
                },
                "MI, US, 48911",
            ),
        );
        cities.insert(
            "Toronto, ON, Canada",
            (
                City {
                    name: String::from("Toronto"),
                    state: Some(String::from("ON")),
                },
                "ON, Canada",
            ),
        );
        cities.insert(
            "United States-California-San Diego-US CA San Diego",
            (
                City {
                    name: String::from("San Diego"),
                    state: Some(String::from("CA")),
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
