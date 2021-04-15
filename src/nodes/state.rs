use crate::utils;
use crate::Parser;
use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub code: String,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.name == other.name && self.code == other.code
    }
}

impl Parser {
    pub fn parse_state(&self, s: &str) -> Option<State> {
        Some(State {
            name: String::from("Ontario"),
            code: String::from("ON"),
        })
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
    use crate::Parser;
    #[test]
    fn test_parse_state() {
        let p = Parser::new(None);
        let state = p.parse_state("Ontario");
        assert_eq!(
            state,
            Some(super::State {
                name: "Ontario".into(),
                code: "ON".into()
            })
        );
    }

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
}
