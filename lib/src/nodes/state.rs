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
    /// use geo_rs;
    /// let parser = geo_rs::Parser::new();
    /// let mut location = geo_rs::nodes::Location {
    ///     city: None,
    ///     state: None,
    ///     country: None,
    ///     zipcode: None,
    ///     address: None,
    /// };
    /// parser.fill_state(&mut location, "Toronto, ON, CA");
    /// let state = location.state.unwrap();
    /// assert_eq!(state.code, String::from("ON"));
    /// assert_eq!(state.name, String::from("Ontario"));
    /// ```
    pub fn fill_state(&self, location: &mut Location, input: &str) {
        if input.chars().count() == 0 {
            return;
        }
        if location.state.is_some() {
            return;
        }
        let as_lowercase = input.to_lowercase().to_string();
        let mut parts = utils::split(input);
        parts.dedup();
        let mut parts_lowercase = utils::split(&as_lowercase);
        parts_lowercase.dedup();
        let countries = match &location.country {
            Some(c) => vec![c.clone()],
            None => vec![UNITED_STATES.clone(), CANADA.clone()],
        };
        // Search by a full match of input and state name
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
        // Search by input containing state code or state name
        let mut candidates: Vec<(State, Country)> = vec![];
        for c in &countries {
            if let Some(states) = self.states.get(&c.code) {
                for (code, name) in &states.code_to_name {
                    for part in &parts {
                        if code == &part.to_string() {
                            let state = State {
                                code: code.clone(),
                                name: name.clone(),
                            };
                            candidates.push((state, c.clone()));
                        }
                    }
                    if name.split_whitespace().all(|s| {
                        return parts_lowercase.contains(&s.to_lowercase().as_str());
                    }) {
                        let state = State {
                            code: code.clone(),
                            name: name.clone(),
                        };
                        candidates.push((state, c.clone()));
                    }
                }
            };
        }
        let country_codes: Vec<String> = self.countries.code_to_name.keys().cloned().collect();
        // When analyzing locations such as `Sherwood Park, AB, CA`
        // we may end up having more than one state, in that case
        // use the one that doesn't look like a country
        match candidates.len() {
            0 => (),
            1 => {
                let s = candidates.first().unwrap().0.clone();
                let c = candidates.first().unwrap().1.clone();
                location.state = Some(s);
                if location.country.is_none() {
                    location.country = Some(c);
                }
                // if !country_codes.contains(&s.code) || location.country == Some(c.clone()) {
                //     location.state = Some(s);
                //     if location.country.is_none() {
                //         location.country = Some(c);
                //     }
                // }
            }
            _ => {
                let filtered_candidates: Vec<(State, Country)> = candidates
                    .into_iter()
                    .filter(|(x, _)| !country_codes.contains(&x.code))
                    .collect();
                if filtered_candidates.len() == 1 {
                    location.state = Some(filtered_candidates.first().unwrap().0.clone());
                    if location.country.is_none() {
                        location.country = Some(filtered_candidates.first().unwrap().1.clone());
                    }
                }
            }
        }
        utils::decode(location);
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
    /// use geo_rs;
    /// let parser = geo_rs::Parser::new();
    /// let mut location = String::from("Los Angeles, CA, US");
    /// let state = geo_rs::nodes::State {
    ///     code: String::from("CA"),
    ///     name: String::from("California"),
    /// };
    /// let country = geo_rs::nodes::Country {
    ///     code: String::from("US"),
    ///     name: String::from("United States"),
    /// };
    /// parser.remove_state(&state, &country, &mut location);
    /// assert_eq!(location, String::from("Los Angeles, US"));
    /// ```
    pub fn remove_state(&self, state: &State, country: &Country, input: &mut String) {
        let input_raw = input.clone();
        // first of all, remove state code from the input string
        // make sure to not remove parts, e.g. for location
        // Washington-20340-DCCL we want to keep DCCL untouched
        // without removing DC out of it
        *input = input
            .split_whitespace()
            .filter(|s| s != &state.code.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        if let Some(p) = input.to_lowercase().find(&state.name.to_lowercase()) {
            // Easy cases with the same state and city "New York, NY, US"
            if !utils::split(&input_raw).contains(&state.code.as_str()) {
                // remove state name only if it's not a part of cities
                // for example, when we parse "Colorado Springs, CO, US"
                // we want to remove "CO" but not "Colorado" because it's a city
                if let Some(country_cities) = self.cities.get(&country.code) {
                    if let Some(state_cities) = country_cities.cities_by_state.get(&state.code) {
                        if state_cities.iter().all(|s| {
                            let parts = s.split_whitespace().collect::<Vec<_>>();
                            state
                                .name
                                .to_lowercase()
                                .split_whitespace()
                                .all(|s| !parts.contains(&s))
                        }) || !input.starts_with(&state.name)
                        {
                            input.replace_range(p..p + state.name.chars().count(), "");
                        }
                    }
                }
            }
        }
        if utils::split(input).contains(&state.code.as_str()) {
            if let Some(p) = input.find(&state.code) {
                input.replace_range(p..p + state.code.chars().count(), "");
            }
        }
        utils::clean(input);
        debug!("after removing state: {}", input);
    }

    pub fn fill_country_from_state(&self, location: &mut Location) {
        if let Some(s) = &location.state {
            for country in utils::get_countries(&None) {
                if let Some(country_states) = self.states.get(&country.code) {
                    if country_states.code_to_name.get(&s.code).is_some() {
                        location.country = Some(country.clone());
                    }
                }
            }
        }
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
    /// use geo_rs;
    /// let parser = geo_rs::Parser::new();
    /// let state_code = "CA";
    /// let country = Some(geo_rs::nodes::Country { code: String::from("US"), name: String::from("United States") });
    /// let state = parser.state_from_code(&country, &state_code).unwrap();
    /// assert_eq!(state.code, String::from("CA"));
    /// assert_eq!(state.name, String::from("California"));
    /// let state_code = "ON";
    /// let country = None;
    /// let state = parser.state_from_code(&country, &state_code).unwrap();
    /// assert_eq!(state.code, String::from("ON"));
    /// assert_eq!(state.name, String::from("Ontario"));
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
/// use geo_rs;
/// let states = geo_rs::nodes::read_states();
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
    use crate::mocks;

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
        let mut location = String::from("United States-San Diego-US CA San Diego");
        parser.remove_state(&state, &UNITED_STATES.clone(), &mut location);
        assert_eq!(
            location,
            String::from("United States-San Diego-US San Diego")
        );
        let state = State {
            code: String::from("CO"),
            name: String::from("Colorado"),
        };
        let mut location = String::from("Colorado Springs, CO, US");
        parser.remove_state(&state, &UNITED_STATES.clone(), &mut location);
        assert_eq!(location, String::from("Colorado Springs, US"));
        let state = State {
            code: String::from("NY"),
            name: String::from("New York"),
        };
        let mut location = String::from("New York, NY, US");
        parser.remove_state(&state, &UNITED_STATES.clone(), &mut location);
        assert_eq!(location, String::from("New York, US"));
        let state = State {
            code: String::from("DC"),
            name: String::from("District Of Columbia"),
        };
        let mut location = String::from("United States-District of Columbia-washington-20340-DCCL");
        parser.remove_state(&state, &UNITED_STATES.clone(), &mut location);
        assert_eq!(location, String::from("United States-washington-20340"));
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

    #[test]
    fn test_fill_country_from_state() {
        let parser = Parser::new();
        let mut location = Location {
            city: None,
            state: Some(State {
                code: String::from("CA"),
                name: String::from("California"),
            }),
            country: None,
            zipcode: None,
            address: None,
        };
        parser.fill_country_from_state(&mut location);
        assert_eq!(location.country.unwrap(), UNITED_STATES.clone());
        let mut location = Location {
            city: None,
            state: Some(State {
                code: String::from("ON"),
                name: String::from("Ontario"),
            }),
            country: None,
            zipcode: None,
            address: None,
        };
        parser.fill_country_from_state(&mut location);
        assert_eq!(location.country.unwrap(), CANADA.clone());
    }

    /// cargo test benchmark_fill_state -- --nocapture --ignored
    #[test]
    #[ignore]
    fn benchmark_fill_state() {
        let n = 250;
        let parser = Parser::new();
        let before = std::time::Instant::now();
        for _ in 0..n {
            for input in mocks::get_mocks().keys() {
                let mut location = Location {
                    city: None,
                    state: None,
                    country: None,
                    zipcode: None,
                    address: None,
                };
                parser.fill_state(&mut location, &input);
            }
        }
        println!(
            "Elapsed time: {:.2?}, {:.2?} each",
            before.elapsed(),
            before.elapsed() / (n * mocks::get_mocks().len() as u32)
        );
    }
}
