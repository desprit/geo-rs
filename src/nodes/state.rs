use crate::utils;
use crate::{Country, Parser};
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
    pub fn remove_state(&self, s: &mut String, state: &State) {
        *s = s.replace(&state.name, "");
        *s = s.replace(&state.code, "");
        utils::clean(s);
    }

    pub fn find_special_case_state(&self, s: &str) -> Option<State> {
        if s.to_lowercase().contains("district of columbia") {
            return Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            });
        }
        if s.to_lowercase().contains("d.c.") {
            return Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            });
        }
        None
    }

    pub fn find_state(&self, s: &str, country: &Option<Country>) -> Option<State> {
        if let Some(st) = self.find_special_case_state(s) {
            return Some(st);
        }
        let as_lowercase = &s.to_lowercase().to_string();
        let parts = utils::split(as_lowercase);
        let countries = utils::get_countries(&country);
        for c in &countries {
            let states = &self.states.get(&c.code).unwrap().code_to_name;
            for (k, v) in states {
                if as_lowercase.contains(&v.to_lowercase()) {
                    return Some(State {
                        name: v.into(),
                        code: k.into(),
                    });
                }
            }
        }
        for c in &countries {
            let states = &self.states.get(&c.code).unwrap().code_to_name;
            for part in &parts {
                for (k, v) in states {
                    if k == &part.to_uppercase().to_string() {
                        return Some(State {
                            name: v.into(),
                            code: k.into(),
                        });
                    }
                }
            }
        }
        None
    }

    pub fn find_country_from_state(&self, state: &State) -> Option<Country> {
        let countries = utils::get_countries(&None);
        for c in &countries {
            let states = &self.states.get(&c.code).unwrap().code_to_name;
            for state_code in states.keys() {
                if state_code == &state.code {
                    return Some(c.clone());
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
    fn test_find_special_case_state() {
        let mut states: HashMap<&str, Option<State>> = HashMap::new();
        states.insert(
            "United States-District of Columbia-washington-20340-DCCL",
            Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            }),
        );
        states.insert(
            "United States-washington d.c.-20340-DCCL",
            Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            }),
        );
        let parser = Parser::new();
        for (input, state) in states {
            let output = parser.find_special_case_state(&input);
            assert_eq!(output, state);
        }
    }

    #[test]
    fn test_find_state() {
        let mut states: HashMap<&str, Option<State>> = HashMap::new();
        states.insert(
            "Jacksonville, Florida, USA",
            Some(State {
                name: String::from("Florida"),
                code: String::from("FL"),
            }),
        );
        states.insert(
            "Lansing, MI, US, 48911",
            Some(State {
                name: String::from("Michigan"),
                code: String::from("MI"),
            }),
        );
        states.insert(
            "New Westminster, British Columbia, Canada",
            Some(State {
                name: String::from("British Columbia"),
                code: String::from("BC"),
            }),
        );
        let parser = Parser::new();
        for (k, v) in states {
            let state = parser.find_state(&k, &None);
            assert_eq!(state, v, "{}", k);
        }
    }

    #[test]
    fn test_find_state_with_country() {
        let parser = Parser::new();
        let state = parser.find_state(
            "Jacksonville, Florida, USA",
            &Some(Country {
                name: String::from("United States"),
                code: String::from("US"),
            }),
        );
        assert_eq!(
            state,
            Some(State {
                code: String::from("FL"),
                name: String::from("Florida")
            })
        );
        let state = parser.find_state(
            "Jacksonville, Florida, USA",
            &Some(Country {
                name: String::from("Canada"),
                code: String::from("CA"),
            }),
        );
        assert_eq!(state, None);
    }

    #[test]
    fn test_remove_state() {
        let mut states: HashMap<&str, &str> = HashMap::new();
        states.insert("Lansing, MI, US, 48911", "Lansing, US, 48911");
        states.insert("Toronto, ON, Canada", "Toronto, Canada");
        states.insert(
            "New Westminster, British Columbia, Canada",
            "New Westminster, Canada",
        );
        states.insert(
            "United States-California-San Diego-US CA San Diego",
            "United States-San Diego-US San Diego",
        );
        let parser = Parser::new();
        for (k, v) in states {
            let mut input = k.to_string();
            let state = parser.find_state(&k, &None).unwrap();
            parser.remove_state(&mut input, &state);
            assert_eq!(input, v, "{}", k);
        }
    }

    #[test]
    fn test_find_country_from_state() {
        let mut states: HashMap<State, Option<Country>> = HashMap::new();
        states.insert(
            State {
                code: String::from("CA"),
                name: String::from("California"),
            },
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
        );
        states.insert(
            State {
                code: String::from("ON"),
                name: String::from("Ontario"),
            },
            Some(Country {
                code: String::from("CA"),
                name: String::from("Canada"),
            }),
        );
        states.insert(
            State {
                code: String::from("ZZ"),
                name: String::from("Wrong State"),
            },
            None,
        );
        let parser = Parser::new();
        for (state, country) in states {
            let output = parser.find_country_from_state(&state);
            assert_eq!(output, country);
        }
    }
}
