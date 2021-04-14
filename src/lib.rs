#![allow(dead_code)]
mod nodes;
mod utils;
use nodes::Country;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Location {
    city: Option<String>,
    state: Option<String>,
    country: Option<Country>,
    zipcode: Option<String>,
    address: Option<String>,
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
    fn to_string(&self) -> Option<String> {
        let mut as_string = String::new();
        if let Some(city) = &self.city {
            as_string.push_str(", ");
            as_string.push_str(city.as_str());
        }
        if let Some(state) = &self.state {
            as_string.push_str(", ");
            as_string.push_str(state.as_str());
        }
        if let Some(country) = &self.country {
            as_string.push_str(", ");
            as_string.push_str(country.name.as_str());
        }
        if let Some(zipcode) = &self.zipcode {
            as_string.push_str(", ");
            as_string.push_str(zipcode.as_str());
        }
        if let Some(address) = &self.address {
            as_string.push_str(", ");
            as_string.push_str(address.as_str());
        }
        if as_string.chars().count() > 0 {
            as_string = as_string.trim().to_string();
            return Some(as_string.trim_start_matches(",").to_string());
        }
        None
    }
}

#[derive(Debug)]
pub struct Parser {
    countries: HashMap<String, String>,
    states: HashMap<String, HashMap<String, String>>,
    cities: HashMap<String, HashMap<String, String>>,
}

impl Parser {
    fn new(country: Option<&str>) -> Self {
        Self {
            countries: utils::read_file("countries.txt"),
            states: match country {
                Some(c) => utils::read_country_data(&c, "states"),
                None => utils::read_all_countries("states"),
            },
            cities: match country {
                Some(c) => utils::read_country_data(&c, "cities"),
                None => utils::read_all_countries("cities"),
            },
        }
    }

    pub fn parse_location(&self, s: &str) -> Location {
        Location {
            city: None,
            state: None,
            country: self.parse_country(s),
            zipcode: None,
            address: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use phf::{phf_map, Map};

    const LOCATIONS: Map<&'static str, &'static str> = phf_map! {
        "Pune Maharashtra India" => "Pune Maharashtra, IN",
        "US-DE-Wilmington" => "Wilmington, DE, US",
        "Lansing, MI, US, 48911" => "Lansing, MI, US, 48911",
        "Colleretto Giacosa" => "Colleretto Giacosa",
        "Mercer Island, WA" => "Mercer Island, WA, US",
        "Lee's Summit, Missouri" => "Lee's Summit, MO, US",
        "BULLHEAD CITY FORT MOHAVE, Arizona, 86426" => "Bullhead City, AZ, US, 86426",
        "Manati, PR, US" => "Manati, PR, US",
        "OR, Beaverton, 3485 SW Ceder Hills BLVD Ste 170" => "Beaverton, OR, US, 3485 SW Ceder Hills BLVD Ste 170",
        "15 McKenna Rd  Arden, North Carolina" => "Arden, NC, US, 15 McKenna Rd",
        "Atholville, New Brunswick, Canada, Kent Atholville 44" => "Atholville, NB, CA, Kent Atholville 44",
        "Jacksonville, Florida, USA" => "Jacksonville, FL, US",
        "CA, Cupertino - Stevens Creek" => "Cupertino, CA, US",
        "Saint-Lin-Laurentides, QC J5M 0G3" => "Saint-Lin-Laurentides, QC, CA, J5M0G3",
        "VA-Christiansburg-24073" => "Christiansburg, VA, US, 24073",
        "Colorado Springs, CO, 80907, US" => "Colorado Springs, CO, US, 80907",
        "Wilkes-Barre, Pennsylvania (PA)" => "Wilkes Barre, PA, US",
        "B - USA - FL - JACKSONVILLE - 9985 PRITCHARD RD" => "Jacksonville, FL, US",
        "Kelowna, BC, CA V1Z 2S9" => "Kelowna, BC, CA, V1Z2S9",
        "410 - Wichita  - Kansas" => "Wichita, KS, US",
        "United States-California-San Diego-US CA San Diego - W. Brdway" => "San Diego, CA, US",
        "CA-ON-Oakville-3235 Dundas St W (Store# 04278)" => "Oakville, ON, CA, 3235 Dundas St W",
        "600778 Wilton, NY - Route 50" => "Wilton, NY, US, Route 50",
        "Toronto (Toronto Eaton Center (ON)), ON, Canada" => "Toronto, ON, CA",
        "United States-Alaska-Shemya/Eareckson Air Station" => "Shemya/Eareckson Air Station, AK, US",
        "United States-District of Columbia-washington-20340-DCCL" => "Washington, DC, US, 20340",
        "01713-Mall At Greece Ridge Center" => "Mall At Greece Ridge Center, US, 01713",
        "New Westminster, British Columbia, Canada" => "New Westminster, BC, CA",
        "MI-Commerce Township" => "Commerce Township, MI, US",
        "Sherwood Park, AB, CA, T8A 3H9" => "Sherwood Park, AB, CA, T8A3H9",
    };

    #[test]
    fn test_can_create_parser() {
        super::Parser::new(None);
    }

    #[test]
    fn test_format_location() {
        let parser = super::Parser::new(None);
        for k in LOCATIONS.keys() {
            let location = parser.parse_location(&k);
            assert_eq!(
                location.to_string(),
                Some("Pune Maharashtra, IN".to_string())
            )
        }
    }
}
