# General information

`geo-rs` is a parser for canadian and united states' geo locations. It attempts to extract geo entities such as City, State, Country, Zipcode and Address. This a work in progress so expect API changes, bugs and lack of optimization.


# Supported input formats

  - "BUFFALO,New York,United States"
  - "Sausalito, US"
  - "US-DE-Wilmington"
  - "Lansing, MI, US, 48911"
  - "Colleretto Giacosa"
  - "Mercer Island, WA"
  - "Lees Summit, Missouri"
  - "BULLHEAD CITY FORT MOHAVE, Arizona, 86426"
  - "Manati, PR, US"
  - "OR, Beaverton, 3485 SW Ceder Hills BLVD Ste 170"
  - "15 McKenna Rd  Arden, North Carolina"
  - "Atholville, New Brunswick, Canada, Kent Atholville 44"
  - "Jacksonville, Florida, USA"
  - "CA, Cupertino - Stevens Creek"
  - "Saint-Lin-Laurentides, QC J5M 0G3"
  - "VA-Christiansburg-24073"
  - "Colorado Springs, CO, 80907, US"
  - "B - USA - FL - JACKSONVILLE - 9985 PRITCHARD RD"
  - "Kelowna, BC, CA V1Z 2S9"
  - "410 - Wichita  - Kansas"
  - "United States-California-San Diego-US CA San Diego - W. Brdway"
  - "CA-ON-Oakville-3235 Dundas St W (Store# 04278)"
  - "600778 Wilton, NY - Route 50"
  - "Toronto (Toronto Eaton Center (ON)), ON, Canada"
  - "United States-Alaska-Shemya/Eareckson Air Station"
  - "United States-District of Columbia-washington-20340-DCCL"
  - "01713-Mall At Greece Ridge Center"
  - "New Westminster, British Columbia, Canada"
  - "MI-Commerce Township"
  - "Sherwood Park, AB, CA, T8A 3H9"

# Usage

```sh
extern crate geo_rs;

let parser = geo_rs::Parser::new();
let location_string = "Toronto, ON, CA, M4E 3J1";
let location_parsed = parser.parse_location(&location_string);
assert_eq!(location_parsed.city.unwrap().name, String::from("Toronto"));
assert_eq!(location_parsed.state.unwrap().code, String::from("ON"));
assert_eq!(location_parsed.state.unwrap().name, String::from("Ontario"));
assert_eq!(location_parsed.zipcode.unwrap().zipcode, String::from("M4E 3J1"));

let location_string = "CA-ON-Oakville-3235 Dundas St W (Store# 04278)";
let location_parsed = parser.parse_location(&location_string);
assert_eq!(format!("{}", location_parsed), String::from("Oakville, ON, CA"))
```

# TODO

  - Extract street address part
  - Support other countries
  - Be able to specify in Cargo.toml a list of countries
