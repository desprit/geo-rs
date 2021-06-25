use super::{Address, City, Country, State, Zipcode};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref COMMAS: Regex = Regex::new(r"(, ){2,5}").unwrap();
}

#[derive(Debug, Clone, Hash, Eq)]
pub struct Location {
    pub city: Option<City>,
    pub state: Option<State>,
    pub country: Option<Country>,
    pub zipcode: Option<Zipcode>,
    pub address: Option<Address>,
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.city == other.city
            && self.state == other.state
            && self.country == other.country
            && self.zipcode == other.zipcode
            && self.address == other.address
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let city = self
            .city
            .to_owned()
            .map(|c| format!("{}", c))
            .unwrap_or(String::from(""));
        let state = self
            .state
            .to_owned()
            .map(|s| format!("{}", s))
            .unwrap_or(String::from(""));
        let country = self
            .country
            .to_owned()
            .map(|c| format!("{}", c))
            .unwrap_or(String::from(""));
        let zipcode = self
            .zipcode
            .to_owned()
            .map(|z| format!("{}", z))
            .unwrap_or(String::from(""));
        let address = self
            .address
            .to_owned()
            .map(|a| format!("{}", a))
            .unwrap_or(String::from(""));
        let mut location = format!("{}, {}, {}, {}, {}", city, state, country, zipcode, address);
        location = COMMAS
            .replace_all(&location, ", ")
            .trim()
            .trim_end_matches(",")
            .trim_start_matches(", ")
            .trim()
            .to_string();
        write!(f, "{}", location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::{CANADA, UNITED_STATES};
    use env_logger;

    #[test]
    fn test_location_display() {
        env_logger::init();
        let location = Location {
            city: Some(City {
                name: String::from("Toronto"),
            }),
            state: Some(State {
                code: String::from("ON"),
                name: String::from("Ontario"),
            }),
            country: Some(CANADA.clone()),
            zipcode: None,
            address: None,
        };
        assert_eq!(format!("{}", location), "Toronto, ON, CA");
        let location = Location {
            city: Some(City {
                name: String::from("Toronto"),
            }),
            state: None,
            country: None,
            zipcode: None,
            address: None,
        };
        assert_eq!(format!("{}", location), "Toronto");
        let location = Location {
            city: Some(City {
                name: String::from("Sausalito"),
            }),
            state: None,
            country: Some(UNITED_STATES.clone()),
            zipcode: None,
            address: None,
        };
        assert_eq!(format!("{}", location), "Sausalito, US");
        let location = Location {
            city: Some(City {
                name: String::from("Toronto"),
            }),
            state: None,
            country: None,
            zipcode: Some(Zipcode {
                zipcode: String::from("90E 717"),
            }),
            address: None,
        };
        assert_eq!(format!("{}", location), "Toronto, 90E717");
    }
}
