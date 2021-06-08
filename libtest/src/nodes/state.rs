use super::{Country, Location, CANADA, UNITED_STATES};
use crate::{utils, Parser};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Hash, Eq)]
pub struct State {
    pub name: String,
    pub code: String,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.name == other.name && self.code == other.code
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code.trim())
    }
}

impl Parser {
    /// Parse location string and try to extract state out of it.
    ///
    /// # Arguments
    ///
    /// * `location` - Location struct that stores final values
    /// * `input` - Location string to be parsed
    ///
    /// # Examples
    ///
    /// ```
    /// let parser = Parser::new();
    /// let mut location = Location {
    ///     city: None,
    ///     state: None,
    ///     country: None,
    ///     zipcode: None,
    ///     address: None,
    /// };
    /// parser.find_state(&mut location, "Toronto, ON, CA");
    /// assert_eq!(location.state.code, String::from("ON"));
    /// assert_eq!(location.state.name, String::from("Ontario"));
    /// ```
    pub fn find_state(&self, location: &mut Location, input: &str) {
        if input.chars().count() == 0 {
            return;
        }
        self.find_special_case_state(location, &input);
        if location.state.is_some() {
            return;
        }
        let as_lowercase = input.to_lowercase().to_string();
        let parts = utils::split(&as_lowercase);
        let countries = match &location.country {
            Some(c) => vec![c.clone()],
            None => vec![UNITED_STATES.clone(), CANADA.clone()],
        };
        for c in &countries {
            if let Some(states) = self.states.get(&c.code) {
                for (code, name) in &states.code_to_name {
                    if as_lowercase.contains(&name.to_lowercase()) {
                        location.state = Some(State {
                            code: code.clone(),
                            name: name.clone(),
                        });
                        if location.country.is_none() {
                            location.country = Some(c.clone());
                        }
                        return;
                    }
                }
            }
        }
        for c in &countries {
            if let Some(states) = self.states.get(&c.code) {
                for part in &parts {
                    for (code, name) in &states.code_to_name {
                        if code == &part.to_uppercase().to_string() {
                            location.state = Some(State {
                                code: code.clone(),
                                name: name.clone(),
                            });
                            if location.country.is_none() {
                                location.country = Some(c.clone());
                            }
                            return;
                        }
                    }
                }
            };
        }
    }

    /// Remove state from location string.
    ///
    /// # Arguments
    ///
    /// * `state` - State to be removed
    /// * `input` - Location string from which state is removed
    ///
    /// # Examples
    ///
    /// ```
    /// let parser = Parser::new();
    /// let mut location = String::from("New York, NY, US");
    /// let state = State {
    ///     code: String::from("NY"),
    ///     name: String::from("New York"),
    /// })
    /// parser.remove_state(&state, &mut location);
    /// assert_eq!(location, String::from("New York, US"));
    /// ```
    pub fn remove_state(&self, state: &State, country: &Country, input: &mut String) {
        if input.contains(&state.code) {
            *input = input.replace(&state.code, "");
        }
        if input.contains(&state.name) {
            // remove name only if it's not a part of cities
            // for example, if we remove both "CO" and "Colorado"
            // we may accidentally remove part of "Colorado Springs" city
            if let Some(country_cities) = self.cities.get(&country.code) {
                if let Some(state_cities) = country_cities.cities_by_state.get(&state.code) {
                    let cities_as_string = state_cities.join(", ");
                    let words = cities_as_string.split(" ").collect::<Vec<&str>>();
                    if !words.contains(&state.name.as_str()) {
                        *input = input.replace(&state.name, "");
                    }
                }
            }
        }
        utils::clean(input);
        debug!("after removing state: {}", input);
    }

    /// Return a State struct that match the given state code.
    ///
    /// # Arguments
    ///
    /// * `country` - Country of the given state
    /// * `input` - State code, e.g. "CA"
    ///
    /// # Examples
    ///
    /// ```
    /// let parser = Parser::new();
    /// let state_code = "CA";
    /// let country = Country { code: String::from("US"), name: String::from("United States") };
    /// let state = parser.state_from_code(&country, &state_code);
    /// assert_eq!(state.unwrap().code, String::from("CA"));
    /// assert_eq!(state.unwrap().name, String::from("California"));
    /// let state_code = "ON";
    /// let country = None;
    /// let state = parser.state_from_code(&country, &state_code);
    /// assert_eq!(state.unwrap().code, String::from("ON"));
    /// assert_eq!(state.unwrap().name, String::from("Ontario"));
    /// ```
    pub fn state_from_code(&self, country: &Option<Country>, input: &str) -> Option<State> {
        let countries = match country {
            Some(c) => vec![c.clone()],
            None => vec![UNITED_STATES.clone(), CANADA.clone()],
        };
        for c in &countries {
            if let Some(states) = self.states.get(&c.code) {
                for (code, name) in &states.code_to_name {
                    if code.as_str() == input {
                        return Some(State {
                            code: code.clone(),
                            name: name.clone(),
                        });
                    }
                }
            }
        }
        None
    }

    // pub fn find_country_from_state(&self, state: &State) -> Option<Country> {
    //     let countries = utils::get_countries(&None);
    //     for c in &countries {
    //         if let Some(states) = self.states.get(&c.code) {
    //             for state_code in states.code_to_name.keys() {
    //                 if state_code == &state.code {
    //                     return Some(c.clone());
    //                 }
    //             }
    //         };
    //     }
    //     None
    // }

    fn find_special_case_state(&self, location: &mut Location, input: &str) {
        if input.to_lowercase().contains("district of columbia") {
            location.state = Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            });
            if location.country.is_none() {
                location.country = Some(UNITED_STATES.clone())
            }
            return;
        }
        if input.to_lowercase().contains("d.c.") {
            location.state = Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            });
            if location.country.is_none() {
                location.country = Some(UNITED_STATES.clone())
            }
            return;
        }
    }
}

#[derive(Debug)]
pub struct StatesMap {
    pub code_to_name: HashMap<String, String>,
    pub name_to_code: HashMap<String, String>,
}

pub type CountryStates = HashMap<String, StatesMap>;

/// Read US and CA states GEO data and create a map between
/// state names and state abbreviations and vice-versa.
///
/// # Examples
///
/// ```
/// use crate::nodes::read_states;
/// let states = read_states();
/// ```
pub fn read_states() -> HashMap<String, StatesMap> {
    let mut data: HashMap<String, StatesMap> = HashMap::new();
    for country in ["US", "CA"].iter() {
        let filename = format!("{}/{}.txt", &country, "states");
        let mut name_to_code: HashMap<String, String> = HashMap::new();
        let mut code_to_name: HashMap<String, String> = HashMap::new();
        for line in utils::read_lines(&filename) {
            if let Ok(s) = line {
                let parts: Vec<&str> = s.split(";").collect();
                name_to_code.insert(parts[1].to_string(), parts[0].to_string());
                code_to_name.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
        data.insert(
            country.to_string(),
            StatesMap {
                name_to_code,
                code_to_name,
            },
        );
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_states() {
        let states = super::read_states();
        assert!(states.get("US").is_some());
        assert!(states.get("CA").is_some());
        let us_states = states.get("US").unwrap();
        let ca_states = states.get("CA").unwrap();
        assert!(ca_states.code_to_name.get("ON").is_some());
        assert!(ca_states.name_to_code.get("Ontario").is_some());
        assert!(us_states.code_to_name.get("CA").is_some());
        assert!(us_states.name_to_code.get("California").is_some());
    }

    #[test]
    fn test_ca_states() {
        let parser = Parser::new();
        parser.states.get("CA").unwrap();
    }

    #[test]
    fn test_us_states() {
        let parser = Parser::new();
        parser.states.get("US").unwrap();
    }

    #[test]
    fn test_state_display() {
        let state = State {
            code: String::from(" ON "),
            name: String::from("Ontario"),
        };
        assert_eq!(format!("{}", state), "ON");
    }

    fn get_states() -> HashMap<&'static str, Option<State>> {
        let mut states: HashMap<&str, Option<State>> = HashMap::new();
        states.insert(
            "Jacksonville, Florida, USA",
            Some(State {
                code: String::from("FL"),
                name: String::from("Florida"),
            }),
        );
        states.insert(
            "Lansing, MI, US, 48911",
            Some(State {
                code: String::from("MI"),
                name: String::from("Michigan"),
            }),
        );
        states.insert(
            "New Westminster, British Columbia, Canada",
            Some(State {
                code: String::from("BC"),
                name: String::from("British Columbia"),
            }),
        );
        states.insert(
            "New York, NY, US",
            Some(State {
                code: String::from("NY"),
                name: String::from("New York"),
            }),
        );
        states.insert(
            "United States-District of Columbia-washington-20340-DCCL",
            Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            }),
        );
        states
    }

    #[test]
    fn test_find_state() {
        let parser = Parser::new();
        for (input, output) in get_states() {
            let mut location = Location {
                city: None,
                state: None,
                country: None,
                zipcode: None,
                address: None,
            };
            parser.find_state(&mut location, &input);
            assert_eq!(location.state, output);
        }
    }

    #[test]
    fn test_find_special_case_state() {
        let parser = Parser::new();
        let input = "United States-District of Columbia-washington-20340-DCCL";
        let mut location = Location {
            city: None,
            state: None,
            country: None,
            zipcode: None,
            address: None,
        };
        parser.find_special_case_state(&mut location, &input);
        assert_eq!(location.state.unwrap().code, String::from("DC"));
        assert_eq!(location.country.unwrap().code, String::from("US"));
        let input = "United States-washington d.c.-20340-DCCL";
        let mut location = Location {
            city: None,
            state: None,
            country: None,
            zipcode: None,
            address: None,
        };
        parser.find_special_case_state(&mut location, &input);
        assert_eq!(location.state.unwrap().code, String::from("DC"));
        assert_eq!(location.country.unwrap().code, String::from("US"));
    }

    #[test]
    fn test_remove_state() {
        let parser = Parser::new();
        let state = State {
            code: String::from("AB"),
            name: String::from("Alberta"),
        };
        let mut location = String::from("Sherwood Park, AB, CA");
        parser.remove_state(&state, &CANADA.clone(), &mut location);
        assert_eq!(location, String::from("Sherwood Park, CA"));
        let state = State {
            code: String::from("ON"),
            name: String::from("Ontario"),
        };
        let mut location = String::from("Toronto, ON, CA");
        parser.remove_state(&state, &CANADA.clone(), &mut location);
        assert_eq!(location, String::from("Toronto, CA"));
        let state = State {
            code: String::from("CA"),
            name: String::from("California"),
        };
        let mut location = String::from("United States-California-San Diego-US CA San Diego");
        parser.remove_state(&state, &UNITED_STATES.clone(), &mut location);
        assert_eq!(
            location,
            String::from("United States-California-San Diego-US San Diego")
        );
        let state = State {
            code: String::from("CO"),
            name: String::from("Colorado"),
        };
        let mut location = String::from("Colorado Springs, CO, US");
        parser.remove_state(&state, &UNITED_STATES.clone(), &mut location);
        assert_eq!(location, String::from("Colorado Springs, US"));
    }

    #[test]
    fn test_state_from_code() {
        let parser = Parser::new();
        let state_code = "CA";
        let country = Some(UNITED_STATES.clone());
        let state = parser.state_from_code(&country, state_code).unwrap();
        assert_eq!(state.code, String::from("CA"));
        assert_eq!(state.name, String::from("California"));
        let state_code = "BC";
        let country = None;
        let state = parser.state_from_code(&country, state_code).unwrap();
        assert_eq!(state.code, String::from("BC"));
        assert_eq!(state.name, String::from("British Columbia"));
    }

    // #[test]
    // fn test_find_country_from_state() {
    //     let mut states: HashMap<State, Option<Country>> = HashMap::new();
    //     states.insert(
    //         State {
    //             code: String::from("CA"),
    //             name: String::from("California"),
    //         },
    //         Some(Country {
    //             code: String::from("US"),
    //             name: String::from("United States"),
    //         }),
    //     );
    //     states.insert(
    //         State {
    //             code: String::from("ON"),
    //             name: String::from("Ontario"),
    //         },
    //         Some(Country {
    //             code: String::from("CA"),
    //             name: String::from("Canada"),
    //         }),
    //     );
    //     states.insert(
    //         State {
    //             code: String::from("ZZ"),
    //             name: String::from("Wrong State"),
    //         },
    //         None,
    //     );
    //     let parser = Parser::new();
    //     for (state, country) in states {
    //         let output = parser.find_country_from_state(&state);
    //         assert_eq!(output, country);
    //     }
    // }

    /// cargo test benchmark_find_state -- --nocapture --ignored
    #[test]
    #[ignore]
    fn benchmark_find_state() {
        let n = 250;
        let parser = Parser::new();
        let before = std::time::Instant::now();
        for _ in 0..n {
            for state in get_states().keys() {
                let mut location = Location {
                    city: None,
                    state: None,
                    country: None,
                    zipcode: None,
                    address: None,
                };
                parser.find_state(&mut location, &state);
            }
        }
        println!(
            "Elapsed time: {:.2?}, {:.2?} each",
            before.elapsed(),
            before.elapsed() / (n * get_states().len() as u32)
        );
    }
}
