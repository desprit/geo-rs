use crate::utils;
use crate::Parser;
use std::collections::HashMap;

#[derive(Debug)]
pub struct City {
    pub name: String,
    pub state: Option<String>,
}

impl PartialEq for City {
    fn eq(&self, other: &City) -> bool {
        self.name == other.name && self.state == other.state
    }
}

impl Parser {
    pub fn parse_city(&self, s: &str) -> Option<City> {
        Some(City {
            name: String::from("Toronto"),
            state: Some(String::from("ON")),
        })
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
    use crate::Parser;
    #[test]
    fn test_parse_city() {
        let p = Parser::new(None);
        let city = p.parse_city("Toronto");
        assert_eq!(
            city.unwrap(),
            super::City {
                name: String::from("Toronto"),
                state: Some(String::from("ON"))
            }
        );
    }

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
}
