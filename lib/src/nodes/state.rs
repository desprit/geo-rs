use super::{Country, Location, CANADA, UNITED_STATES};
use crate::nodes::CitiesMap;
use crate::{utils, Parser};
use std::collections::{HashMap, HashSet};
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
        let as_lowercase = input.to_lowercase();
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
            let default = CitiesMap::default();
            let country_cities = self.cities.get(&c.code).unwrap_or(&default);
            if let Some(states) = self.states.get(&c.code) {
                for (code, name) in &states.code_to_name {
                    // check if state name isn't a city
                    if country_cities.city_names_set.contains(&name.to_lowercase()) {
                        continue;
                    }
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
                        if code.as_str() == *part {
                            let state = State {
                                code: code.clone(),
                                name: name.clone(),
                            };
                            candidates.push((state, c.clone()));
                        }
                    }
                    if let Some(name_lower) = states.code_to_name_lower.get(code.as_str()) {
                        if name_lower.split_whitespace().all(|s| parts_lowercase.contains(&s)) {
                            let state = State {
                                code: code.clone(),
                                name: name.clone(),
                            };
                            candidates.push((state, c.clone()));
                        }
                    }
                }
            };
        }
        let mut candidates_deduped: Vec<(State, Country)> = vec![];
        for (state, country) in &candidates {
            if !candidates_deduped.contains(&(state.clone(), country.clone())) {
                candidates_deduped.push((state.clone(), country.clone()));
            }
        }
        let country_codes: Vec<String> = self.countries.code_to_name.keys().cloned().collect();
        // When analyzing locations such as `Sherwood Park, AB, CA`
        // we may end up having more than one state, in that case
        // use the one that doesn't look like a country
        match candidates_deduped.len() {
            0 => (),
            1 => {
                let s = candidates_deduped.first().unwrap().0.clone();
                let c = candidates_deduped.first().unwrap().1.clone();
                location.state = Some(s);
                if location.country.is_none() {
                    location.country = Some(c);
                }
            }
            _ => {
                let first_candidate_state = candidates_deduped.first().unwrap().0.clone();
                let first_candidate_country = candidates_deduped.first().unwrap().1.clone();

                let mut filtered_candidates: Vec<(State, Country)> = match &location.country {
                    Some(_) => candidates_deduped.clone(),
                    None => candidates_deduped
                        .into_iter()
                        .filter(|(x, _)| !country_codes.contains(&x.code))
                        .collect(),
                };
                // [(State { name: "Washington", code: "WA" }, Country { name: "United States", code: "US" }), (State { name: "Pennsylvania", code: "PA" }, Country { name: "United States", code: "US" })]
                // Iterate over candidates and choose more likely state: if one candidate has name in the input string and
                // another candidate has code in the input string pick the second one because state is usually written as code
                filtered_candidates.sort_by(|a, b| {
                    let a_state_code_in_str = as_lowercase.contains(&a.0.code.to_lowercase());
                    let b_state_code_in_str = as_lowercase.contains(&b.0.code.to_lowercase());

                    if a_state_code_in_str && !b_state_code_in_str {
                        return std::cmp::Ordering::Less;
                    }
                    if !a_state_code_in_str && b_state_code_in_str {
                        return std::cmp::Ordering::Greater;
                    }

                    let a_state_name_in_str = as_lowercase.contains(&a.0.name.to_lowercase());
                    let b_state_name_in_str = as_lowercase.contains(&b.0.name.to_lowercase());

                    if a_state_name_in_str && !b_state_name_in_str {
                        return std::cmp::Ordering::Greater;
                    }
                    if !a_state_name_in_str && b_state_name_in_str {
                        return std::cmp::Ordering::Less;
                    }

                    std::cmp::Ordering::Equal
                });

                if filtered_candidates.len() == 1 {
                    location.state = Some(filtered_candidates.first().unwrap().0.clone());
                    if location.country.is_none() {
                        location.country = Some(filtered_candidates.first().unwrap().1.clone());
                    }
                }
                if filtered_candidates.len() == 0 {
                    // pick first candidate
                    location.state = Some(first_candidate_state);
                    if location.country.is_none() {
                        location.country = Some(first_candidate_country);
                    }
                }
                if filtered_candidates.len() > 1 {
                    let first_candidate = filtered_candidates.first().unwrap();
                    location.state = Some(first_candidate.0.clone());
                    if location.country.is_none() {
                        location.country = Some(first_candidate.1.clone());
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

        let input_lowercase = input.to_lowercase();
        let state_name_lowercase = state.name.to_lowercase();

        // Find all occurrences of the state name in the input (case-insensitive)
        // and collect those that have proper word boundaries
        let mut valid_positions = Vec::new();
        let mut search_start = 0;
        while let Some(p) = input_lowercase[search_start..].find(&state_name_lowercase) {
            let absolute_pos = search_start + p;
            // Check if this is a complete word/phrase, not part of another word
            // by verifying the character before and after are not alphanumeric
            let is_start_boundary = absolute_pos == 0 || {
                input_lowercase[..absolute_pos]
                    .chars()
                    .last()
                    .map(|c| !c.is_alphanumeric())
                    .unwrap_or(true)
            };
            let end_pos = absolute_pos + state_name_lowercase.len();
            let is_end_boundary = end_pos >= input_lowercase.len() || {
                input_lowercase[end_pos..]
                    .chars()
                    .next()
                    .map(|c| !c.is_alphanumeric())
                    .unwrap_or(true)
            };

            if is_start_boundary && is_end_boundary {
                valid_positions.push(absolute_pos);
            }
            search_start = absolute_pos + 1;
        }

        // Remove the state name if we found valid occurrences
        if !valid_positions.is_empty() {
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
                            // Remove state name occurrences from right to left to maintain valid positions
                            for p in valid_positions.iter().rev() {
                                let state_name_len = state.name.len();
                                input.replace_range(*p..*p + state_name_len, "");
                            }
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
    // key: uppercase state code (e.g. "CA"), value: lowercase state name (e.g. "california")
    pub code_to_name_lower: HashMap<String, String>,
    // all lowercase state names, for O(1) lookup
    pub name_lower_set: HashSet<String>,
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
    static US_STATES: &str = include_str!("../data/US/states.txt");
    static CA_STATES: &str = include_str!("../data/CA/states.txt");

    let mut data: HashMap<String, StatesMap> = HashMap::new();
    for (country, content) in [("US", US_STATES), ("CA", CA_STATES)] {
        let mut name_to_code: HashMap<String, String> = HashMap::new();
        let mut code_to_name: HashMap<String, String> = HashMap::new();
        let mut code_to_name_lower: HashMap<String, String> = HashMap::new();
        let mut name_lower_set: HashSet<String> = HashSet::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() < 2 { continue; }
            let code = parts[0].to_string();
            let name = parts[1].to_string();
            let name_lower = name.to_lowercase();
            code_to_name_lower.insert(code.clone(), name_lower.clone());
            name_lower_set.insert(name_lower);
            name_to_code.insert(name.clone(), code.clone());
            code_to_name.insert(code, name);
        }
        data.insert(country.to_string(), StatesMap {
            name_to_code, code_to_name, code_to_name_lower, name_lower_set,
        });
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
    fn test_fill_state() {
        let parser = Parser::new();
        let input = String::from("Northwood, ND, 104 ND-15");
        let mut location = Location {
            city: None,
            state: None,
            country: None,
            zipcode: None,
            address: None,
        };
        parser.fill_state(&mut location, &input);
        assert_eq!(location.state.unwrap().code, String::from("ND"));
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
        let state = State {
            code: String::from("IN"),
            name: String::from("Indiana"),
        };
        let mut location = String::from("Indianapolis, Indiana");
        parser.remove_state(&state, &UNITED_STATES.clone(), &mut location);
        assert_eq!(location, String::from("Indianapolis"));
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

    #[test]
    fn test_states_map_lower_fields() {
        let states = read_states();
        let us = states.get("US").unwrap();
        assert_eq!(us.code_to_name_lower.get("CA"), Some(&"california".to_string()));
        assert_eq!(us.code_to_name_lower.get("NY"), Some(&"new york".to_string()));
        assert!(us.name_lower_set.contains("california"));
        assert!(us.name_lower_set.contains("new york"));
        let ca = states.get("CA").unwrap();
        assert_eq!(ca.code_to_name_lower.get("ON"), Some(&"ontario".to_string()));
        assert!(ca.name_lower_set.contains("ontario"));
    }

    /// cargo test benchmark_fill_state -- --nocapture --ignored
    #[test]
    #[ignore]
    fn benchmark_fill_state() {
        let n = 250;
        let parser = Parser::new();
        let mocks = mocks::get_mocks();
        let before = std::time::Instant::now();
        for _ in 0..n {
            for input in mocks.keys() {
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
            before.elapsed() / (n * mocks.len() as u32)
        );
    }
}
