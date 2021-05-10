#![allow(dead_code)]
mod nodes;
mod utils;
use log::debug;
use nodes::{
    read_cities, read_countries, read_states, Address, City, CountriesMap, Country, CountryCities,
    CountryStates, State, Zipcode,
};

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

impl Location {
    pub fn to_string(&self) -> String {
        let mut as_string = String::new();
        if let Some(city) = &self.city {
            as_string.push_str(", ");
            as_string.push_str(format!("{}", city).as_str());
        }
        if let Some(state) = &self.state {
            as_string.push_str(", ");
            as_string.push_str(format!("{}", state).as_str());
        }
        if let Some(country) = &self.country {
            as_string.push_str(", ");
            as_string.push_str(format!("{}", country).as_str());
        }
        if let Some(zipcode) = &self.zipcode {
            as_string.push_str(", ");
            as_string.push_str(format!("{}", zipcode).as_str());
        }
        if let Some(address) = &self.address {
            as_string.push_str(", ");
            as_string.push_str(address.address.as_str());
        }
        if as_string.chars().count() > 0 {
            as_string = as_string.trim().trim_start_matches(",").trim().to_string();
        }
        as_string
    }
}

#[derive(Debug)]
pub struct Parser {
    countries: CountriesMap,
    states: CountryStates,
    cities: CountryCities,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            countries: read_countries(),
            states: read_states(),
            cities: read_cities(),
        }
    }

    pub fn parse_location(&self, input: &str) -> Location {
        let mut input_copy = input.to_string().clone();
        utils::clean(&mut input_copy);
        let mut remainder = input_copy.to_string();

        let mut zipcode = self.find_zipcode(&input_copy);
        if let Some(z) = zipcode.as_ref() {
            self.remove_zipcode(&mut remainder, &z);
        }
        // TODO: don't search for location if zipcode is available
        debug!("After removing zipcode: {}", remainder);

        let mut country = self.find_country(&input_copy);
        if let Some(ct) = country.as_ref() {
            self.remove_country(&mut remainder, &ct);
        }
        if let Some(zp) = zipcode.as_ref() {
            if let Some(ct) = country.as_ref() {
                if &zp.country != ct {
                    zipcode = None;
                }
            } else {
                country = Some(zp.country.clone());
            }
        }
        debug!("After removing country: {}", remainder);

        let state = self.find_state(&input_copy, &country);
        if let Some(c) = state.as_ref() {
            self.remove_state(&mut remainder, &c);
        }
        if let (None, Some(v)) = (&country, &state) {
            country = self.find_country_from_state(&v);
        }
        debug!("After removing state: {}", remainder);

        let city = match state.as_ref() {
            Some(v) => self.find_city(&input_copy, &v, &country),
            None => None,
        };
        if let Some(c) = city.as_ref() {
            self.remove_city(&mut remainder, &c);
        }
        debug!("After removing city: {}", remainder);
        Location {
            city,
            state,
            country,
            zipcode,
            address: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_can_create_parser() {
        super::Parser::new();
    }

    #[test]
    fn test_format_location() {
        let mut locations: HashMap<&str, &str> = HashMap::new();
        // locations.insert("Moscow, Russia", "Moscow, RU");
        // locations.insert("Pune Maharashtra India", "Pune Maharashtra, IN");
        // locations.insert("Wilkes-Barre, Pennsylvania (PA)", "Wilkes Barre, PA, US");
        locations.insert("US-DE-Wilmington", "Wilmington, DE, US");
        locations.insert("Lansing, MI, US, 48911", "Lansing, MI, US, 48911");
        locations.insert("Colleretto Giacosa", "");
        locations.insert("Mercer Island, WA", "Mercer Island, WA, US");
        locations.insert("Lee's Summit, Missouri", "MO, US");
        locations.insert("Lees Summit, Missouri", "Lees Summit, MO, US");
        locations.insert(
            "BULLHEAD CITY FORT MOHAVE, Arizona, 86426",
            "Bullhead City, AZ, US, 86426",
        );
        locations.insert("Manati, PR, US", "Manati, PR, US");
        locations.insert(
            "OR, Beaverton, 3485 SW Ceder Hills BLVD Ste 170",
            "Beaverton, OR, US",
        );
        locations.insert("15 McKenna Rd  Arden, North Carolina", "Arden, NC, US");
        locations.insert(
            "Atholville, New Brunswick, Canada, Kent Atholville 44",
            "Atholville, NB, CA",
        );
        locations.insert("Jacksonville, Florida, USA", "Jacksonville, FL, US");
        locations.insert("CA, Cupertino - Stevens Creek", "Cupertino, CA, US");
        locations.insert(
            "Saint-Lin-Laurentides, QC J5M 0G3",
            "Saint-Lin-Laurentides, QC, CA, J5M0G3",
        );
        locations.insert("VA-Christiansburg-24073", "Christiansburg, VA, US, 24073");
        locations.insert(
            "Colorado Springs, CO, 80907, US",
            "Colorado Springs, CO, US, 80907",
        );
        locations.insert(
            "B - USA - FL - JACKSONVILLE - 9985 PRITCHARD RD",
            "Jacksonville, FL, US",
        );
        locations.insert("Kelowna, BC, CA V1Z 2S9", "Kelowna, BC, CA, V1Z2S9");
        locations.insert("410 - Wichita  - Kansas", "Wichita, KS, US");
        locations.insert(
            "United States-California-San Diego-US CA San Diego - W. Brdway",
            "San Diego, CA, US",
        );
        locations.insert(
            "CA-ON-Oakville-3235 Dundas St W (Store# 04278)",
            "Oakville, ON, CA",
        );
        locations.insert("600778 Wilton, NY - Route 50", "Wilton, NY, US");
        locations.insert(
            "Toronto (Toronto Eaton Center (ON)), ON, Canada",
            "Toronto, ON, CA",
        );
        locations.insert(
            "United States-Alaska-Shemya/Eareckson Air Station",
            "AK, US",
        );
        locations.insert(
            "United States-District of Columbia-washington-20340-DCCL",
            "Washington, DC, US, 20340",
        );
        locations.insert("01713-Mall At Greece Ridge Center", "US, 01713");
        locations.insert(
            "New Westminster, British Columbia, Canada",
            "New Westminster, BC, CA",
        );
        locations.insert("MI-Commerce Township", "Commerce Township, MI, US");
        locations.insert(
            "Sherwood Park, AB, CA, T8A 3H9",
            "Sherwood Park, AB, CA, T8A3H9",
        );
        let parser = super::Parser::new();
        for (k, v) in locations {
            let output = parser.parse_location(&k);
            assert_eq!(output.to_string(), v.to_string());
        }
    }

    #[test]
    fn test_parse_location() {
        let mut states: HashMap<&str, Location> = HashMap::new();
        states.insert(
            "Lansing, MI, US, 48911",
            Location {
                city: Some(City {
                    name: String::from("Lansing"),
                    state: Some(String::from("MI")),
                }),
                state: Some(State {
                    code: String::from("MI"),
                    name: String::from("Michigan"),
                }),
                country: Some(Country {
                    code: String::from("US"),
                    name: String::from("United States"),
                }),
                zipcode: Some(Zipcode {
                    zipcode: String::from("48911"),
                    country: Country {
                        code: String::from("US"),
                        name: String::from("United States"),
                    },
                }),
                address: None,
            },
        );
        states.insert(
            "Toronto, ON, CA, 48911",
            Location {
                city: Some(City {
                    name: String::from("Toronto"),
                    state: Some(String::from("ON")),
                }),
                state: Some(State {
                    code: String::from("ON"),
                    name: String::from("Ontario"),
                }),
                country: Some(Country {
                    code: String::from("CA"),
                    name: String::from("Canada"),
                }),
                zipcode: None,
                address: None,
            },
        );
        states.insert(
            "Lansing, US",
            Location {
                city: None,
                state: None,
                country: Some(Country {
                    code: String::from("US"),
                    name: String::from("United States"),
                }),
                zipcode: None,
                address: None,
            },
        );
        let parser = Parser::new();
        for (k, v) in states {
            let location = parser.parse_location(&k);
            assert_eq!(location, v, "{}", k);
        }
    }

    #[test]
    fn test_location_to_string() {
        let mut locations: HashMap<Location, &str> = HashMap::new();
        locations.insert(
            Location {
                city: Some(City {
                    name: String::from("Toronto"),
                    state: Some(String::from("ON")),
                }),
                state: Some(State {
                    code: String::from("ON"),
                    name: String::from("Ontario"),
                }),
                country: Some(Country {
                    code: String::from("CA"),
                    name: String::from("Canada"),
                }),
                zipcode: Some(Zipcode {
                    zipcode: String::from("M4E3H8"),
                    country: Country {
                        code: String::from("CA"),
                        name: String::from("Canada"),
                    },
                }),
                address: Some(Address {
                    address: String::from("119 Yonge St"),
                }),
            },
            "Toronto, ON, CA, M4E3H8, 119 Yonge St",
        );
        for (location, output) in locations {
            assert_eq!(location.to_string(), output);
        }
    }
}
