#![allow(dead_code)]
#[macro_use]
extern crate log;
mod mocks;
pub mod nodes;
pub mod utils;
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
    /// use geo_rs;
    /// let parser = geo_rs::Parser::new();
    /// let location = parser.parse_location("Toronto, ON, CA");
    /// assert_eq!(location.city.unwrap().name, String::from("Toronto"));
    /// assert_eq!(location.state.unwrap().code, String::from("ON"));
    /// assert_eq!(location.country.unwrap().code, String::from("CA"));
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
        self.fill_zipcode(&mut output, &remainder);
        if let Some(z) = &output.zipcode {
            self.remove_zipcode(z, &mut remainder);
            if let Some(c) = &output.country {
                self.remove_country(c, &mut remainder);
            }
        }
        self.fill_country(&mut output, &remainder);
        if let Some(c) = &output.country {
            self.remove_country(c, &mut remainder);
        }
        self.fill_state(&mut output, &remainder);
        if let (Some(s), Some(c)) = (&output.state, &output.country) {
            self.remove_state(s, c, &mut remainder);
            self.remove_country(c, &mut remainder);
        }
        self.fill_city(&mut output, &remainder);
        if let Some(c) = output.city {
            output.city = Some(c.clone());
            self.remove_city(&mut remainder, &c);
        }
        debug!("output value: {}, remainder: {}", output, remainder);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;
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

    #[test]
    fn test_parse_location() {
        let parser = Parser::new();
        for (input, (_, _, _, _, _, output)) in mocks::get_mocks() {
            let location = parser.parse_location(input);
            assert_eq!(location.to_string(), output);
        }
    }

    /// cargo test benchmark_parse_location -- --nocapture --ignored
    /// 9.5ms -> 3.77ms -> ~1ms -> ~1.8ms -> 0.7ms (laptop) -> 1ms (laptop)
    #[test]
    #[ignore]
    fn benchmark_parse_location() {
        let n = 250;
        let parser = Parser::new();
        let before = std::time::Instant::now();
        for _ in 0..n {
            for input in mocks::get_mocks().keys() {
                parser.parse_location(input);
            }
        }
        println!(
            "Elapsed time: {:.2?}, {:.2?} each",
            before.elapsed(),
            before.elapsed() / (n * mocks::get_mocks().len() as u32)
        );
    }
}
