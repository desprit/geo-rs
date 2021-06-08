#![allow(dead_code)]
#[macro_use]
extern crate log;
mod nodes;
mod utils;
use nodes::{
    read_cities, read_countries, read_states, CountriesMap, Country, CountryCities, CountryStates,
    Location, State,
};

#[derive(Debug)]
pub struct Parser {
    cities: CountryCities,
    states: CountryStates,
    countries: CountriesMap,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            cities: read_cities(),
            states: read_states(),
            countries: read_countries(),
        }
    }

    /// Parse location string and try to extract geo parts out of it.
    ///
    /// # Arguments
    ///
    /// * `input` - Location string that's gonna be parsed
    ///
    /// # Examples
    ///
    /// ```
    /// let parser = Parser::new();
    /// let location = parser.parse_location("Toronto, ON, CA");
    /// assert_eq(location.city.name, String::from("Toronto"));
    /// assert_eq(location.state.code, String::from("ON"));
    /// assert_eq(location.country.code, String::from("CA"));
    /// ```
    pub fn parse_location(&self, input: &str) -> Location {
        let mut output = Location {
            city: None,
            state: None,
            country: None,
            zipcode: None,
            address: None,
        };
        let mut input_copy = input.to_string();
        utils::clean(&mut input_copy);
        let mut remainder = input_copy.clone();
        debug!("input value: {}", remainder);
        self.find_zipcode(&mut output, &remainder);
        if let Some(z) = &output.zipcode {
            self.remove_zipcode(z, &mut remainder);
            if let Some(c) = &output.country {
                self.remove_country(c, &mut remainder);
            }
        }
        self.find_country(&mut output, &remainder);
        if let Some(c) = &output.country {
            self.remove_country(c, &mut remainder);
        }
        self.find_state(&mut output, &remainder);
        if let (Some(s), Some(c)) = (&output.state, &output.country) {
            self.remove_state(s, c, &mut remainder);
            self.remove_country(c, &mut remainder);
        }
        let city = self.find_city(&remainder, &output.state, &output.country);
        if let Some(c) = city.as_ref() {
            output.city = city.clone();
            self.remove_city(&mut remainder, &c);
        }
        debug!("output value: {}, remainder: {}", output, remainder);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nodes::{City, Country, State, Zipcode};
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
        // locations.insert("Sausalito, US", "Sausalito, CA, US");
        // locations.insert("Lee's Summit, Missouri", "MO, US");
        locations.insert("BUFFALO,New York,United States", "Buffalo, NY, US");
        locations.insert("US-DE-Wilmington", "Wilmington, DE, US");
        locations.insert("Lansing, MI, US, 48911", "Lansing, MI, US, 48911");
        locations.insert("Colleretto Giacosa", "");
        locations.insert("Mercer Island, WA", "Mercer Island, WA, US");
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
        // locations.insert("01713-Mall At Greece Ridge Center", "US, 01713");
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
            println!("{:?}", output);
            assert_eq!(output.to_string(), v.to_string());
        }
    }

    fn get_locations() -> HashMap<&'static str, Location> {
        let mut locations: HashMap<&str, Location> = HashMap::new();
        locations.insert(
            "BUFFALO, New York, US",
            Location {
                city: Some(City {
                    name: String::from("Buffalo"),
                    state: Some(String::from("NY")),
                }),
                state: Some(State {
                    code: String::from("NY"),
                    name: String::from("New York"),
                }),
                country: Some(Country {
                    code: String::from("US"),
                    name: String::from("United States"),
                }),
                zipcode: None,
                address: None,
            },
        );
        locations.insert(
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
                }),
                address: None,
            },
        );
        // locations.insert(
        //     "Sausalito, US",
        //     Location {
        //         city: Some(City {
        //             name: String::from("Sausalito"),
        //             state: Some(String::from("CA")),
        //         }),
        //         state: Some(State {
        //             code: String::from("CA"),
        //             name: String::from("California"),
        //         }),
        //         country: Some(Country {
        //             code: String::from("US"),
        //             name: String::from("United States"),
        //         }),
        //         zipcode: None,
        //         address: None,
        //     },
        // );
        locations.insert(
            "Toronto, ON, CA",
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
        // locations.insert(
        //     "Lansing, US",
        //     Location {
        //         city: None,
        //         state: None,
        //         country: Some(Country {
        //             code: String::from("US"),
        //             name: String::from("United States"),
        //         }),
        //         zipcode: None,
        //         address: None,
        //     },
        // );
        locations
    }

    #[test]
    fn test_parse_location() {
        let parser = Parser::new();
        for (k, v) in get_locations() {
            let location = parser.parse_location(&k);
            assert_eq!(location, v, "{}", k);
        }
    }

    /// cargo test benchmark_parse_location -- --nocapture --ignored
    /// 9.5ms -> 3.77ms -> ~1ms -> ~1.8ms
    #[test]
    #[ignore]
    fn benchmark_parse_location() {
        let n = 250;
        let parser = Parser::new();
        let before = std::time::Instant::now();
        for _ in 0..n {
            for location_string in get_locations().keys() {
                parser.parse_location(&location_string);
            }
        }
        println!(
            "Elapsed time: {:.2?}, {:.2?} each",
            before.elapsed(),
            before.elapsed() / (n * get_locations().len() as u32)
        );
    }
}
