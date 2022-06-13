use super::nodes::{Address, City, Country, State, Zipcode};
use std::collections::HashMap;

type Input = &'static str;
type ParseCityResult = Option<City>;
type ParseStateResult = Option<State>;
type ParseCountryResult = Option<Country>;
type ParseZipcodeResult = Option<Zipcode>;
type ParseAddressResult = Option<Address>;
type FormatLocationResult = &'static str;
type Output = (
    ParseCityResult,
    ParseStateResult,
    ParseCountryResult,
    ParseZipcodeResult,
    ParseAddressResult,
    FormatLocationResult,
);

pub fn get_mocks() -> HashMap<Input, Output> {
    let mut locations: HashMap<Input, Output> = HashMap::new();
    locations.insert(
        "Kenogami Mill , Quebec, Canada",
        (
            None,
            Some(State {
                code: String::from("QC"),
                name: String::from("Quebec"),
            }),
            Some(Country {
                code: String::from("CA"),
                name: String::from("Canada"),
            }),
            None,
            None,
            "Kenogami Mill, QC, CA",
        ),
    );
    locations.insert(
        "Washington D.C.",
        (
            Some(City {
                name: String::from("Washington"),
            }),
            Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            }),
            None,
            None,
            None,
            "Washington, DC, US",
        ),
    );
    locations.insert(
        "BUFFALO, New York, US",
        (
            Some(City {
                name: String::from("Buffalo"),
            }),
            Some(State {
                code: String::from("NY"),
                name: String::from("New York"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "Buffalo, NY, US",
        ),
    );
    locations.insert(
        "Sausalito",
        (
            Some(City {
                name: String::from("Sausalito"),
            }),
            None,
            None,
            None,
            None,
            "Sausalito, CA, US",
        ),
    );
    locations.insert(
        "United States-District of Columbia-washington-20340-DCCL",
        (
            Some(City {
                name: String::from("Washington"),
            }),
            Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            Some(Zipcode {
                zipcode: String::from("20340"),
            }),
            None,
            "Washington, DC, US, 20340",
        ),
    );
    locations.insert(
        "ON CA",
        (
            None,
            Some(State {
                code: String::from("ON"),
                name: String::from("Ontario"),
            }),
            Some(Country {
                code: String::from("CA"),
                name: String::from("Canada"),
            }),
            None,
            None,
            "ON, CA",
        ),
    );
    locations.insert(
        "Los Angeles, CA",
        (
            Some(City {
                name: String::from("Los Angeles"),
            }),
            Some(State {
                name: String::from("California"),
                code: String::from("CA"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "Los Angeles, CA, US",
        ),
    );
    locations.insert(
        "Saint-Lin-Laurentides, QC J5M 0G3",
        (
            Some(City {
                name: String::from("Saint-Lin-Laurentides"),
            }),
            Some(State {
                code: String::from("QC"),
                name: String::from("Quebec"),
            }),
            None,
            Some(Zipcode {
                zipcode: String::from("J5M 0G3"),
            }),
            None,
            "Saint-Lin-Laurentides, QC, CA, J5M0G3",
        ),
    );
    locations.insert(
        "Saint-Lin-Laurentides, QC 11111111",
        (
            Some(City {
                name: String::from("Saint-Lin-Laurentides"),
            }),
            Some(State {
                code: String::from("QC"),
                name: String::from("Quebec"),
            }),
            None,
            None,
            None,
            "Saint-Lin-Laurentides, QC, CA",
        ),
    );
    locations.insert(
        "Saint-Lin-Laurentides, QC",
        (
            Some(City {
                name: String::from("Saint-Lin-Laurentides"),
            }),
            Some(State {
                code: String::from("QC"),
                name: String::from("Quebec"),
            }),
            None,
            None,
            None,
            "Saint-Lin-Laurentides, QC, CA",
        ),
    );
    locations.insert(
        "Saint-Lin-Laurentides, QC J5MM 0G3",
        (
            Some(City {
                name: String::from("Saint-Lin-Laurentides"),
            }),
            Some(State {
                code: String::from("QC"),
                name: String::from("Quebec"),
            }),
            None,
            None,
            None,
            "Saint-Lin-Laurentides, QC, CA",
        ),
    );
    locations.insert(
        "Lansing, US",
        (
            None,
            None,
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "Lansing, US",
        ),
    );
    locations.insert(
        "Sausalito, US",
        (
            Some(City {
                name: String::from("Sausalito"),
            }),
            None,
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "Sausalito, CA, US",
        ),
    );
    locations.insert(
        "Lansing, MI, US, 48911",
        (
            Some(City {
                name: String::from("Lansing"),
            }),
            Some(State {
                code: String::from("MI"),
                name: String::from("Michigan"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            Some(Zipcode {
                zipcode: String::from("48911"),
            }),
            None,
            "Lansing, MI, US, 48911",
        ),
    );
    locations.insert(
        "Toronto, ON, CA",
        (
            Some(City {
                name: String::from("Toronto"),
            }),
            Some(State {
                code: String::from("ON"),
                name: String::from("Ontario"),
            }),
            Some(Country {
                code: String::from("CA"),
                name: String::from("Canada"),
            }),
            None,
            None,
            "Toronto, ON, CA",
        ),
    );
    locations.insert(
        "Lansing, MI, US",
        (
            Some(City {
                name: String::from("Lansing"),
            }),
            Some(State {
                code: String::from("MI"),
                name: String::from("Michigan"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "Lansing, MI, US",
        ),
    );
    locations.insert(
        "Lansing, MI, US, 67139037",
        (
            Some(City {
                name: String::from("Lansing"),
            }),
            Some(State {
                code: String::from("MI"),
                name: String::from("Michigan"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "Lansing, MI, US",
        ),
    );
    locations.insert(
        "Lansing, MI, US, 48911",
        (
            Some(City {
                name: String::from("Lansing"),
            }),
            Some(State {
                code: String::from("MI"),
                name: String::from("Michigan"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            Some(Zipcode {
                zipcode: String::from("48911"),
            }),
            None,
            "Lansing, MI, US, 48911",
        ),
    );
    locations.insert(
        "Sherwood Park, AB, CA, T8A3H9",
        (
            Some(City {
                name: String::from("Sherwood Park"),
            }),
            Some(State {
                code: String::from("AB"),
                name: String::from("Alberta"),
            }),
            Some(Country {
                code: String::from("CA"),
                name: String::from("Canada"),
            }),
            Some(Zipcode {
                zipcode: String::from("T8A3H9"),
            }),
            None,
            "Sherwood Park, AB, CA, T8A3H9",
        ),
    );
    locations.insert(
        "Jacksonville, Florida, USA",
        (
            Some(City {
                name: String::from("Jacksonville"),
            }),
            Some(State {
                code: String::from("FL"),
                name: String::from("Florida"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "Jacksonville, FL, US",
        ),
    );
    locations.insert(
        "MANATI, PR, US",
        (
            Some(City {
                name: String::from("Manati"),
            }),
            Some(State {
                code: String::from("PR"),
                name: String::from("Puerto Rico"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "Manati, PR, US",
        ),
    );
    locations.insert(
        "United States-Alaska-Shemya",
        (
            Some(City {
                name: String::from("Shemya"),
            }),
            Some(State {
                code: String::from("AK"),
                name: String::from("Alaska"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "Shemya, AK, US",
        ),
    );
    locations.insert(
        "British Columbia, Canada",
        (
            None,
            Some(State {
                code: String::from("BC"),
                name: String::from("British Columbia"),
            }),
            Some(Country {
                code: String::from("CA"),
                name: String::from("Canada"),
            }),
            None,
            None,
            "BC, CA",
        ),
    );
    locations.insert(
        "New Westminster, British Columbia, Canada",
        (
            Some(City {
                name: String::from("New Westminster"),
            }),
            Some(State {
                code: String::from("BC"),
                name: String::from("British Columbia"),
            }),
            Some(Country {
                code: String::from("CA"),
                name: String::from("Canada"),
            }),
            None,
            None,
            "New Westminster, BC, CA",
        ),
    );
    locations.insert(
        "New York, NY, US",
        (
            Some(City {
                name: String::from("New York"),
            }),
            Some(State {
                code: String::from("NY"),
                name: String::from("New York"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            None,
            None,
            "New York, NY, US",
        ),
    );
    locations.insert(
        "United States-District of Columbia-washington-20340",
        (
            Some(City {
                name: String::from("Washington"),
            }),
            Some(State {
                code: String::from("DC"),
                name: String::from("District Of Columbia"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            Some(Zipcode {
                zipcode: String::from("20340"),
            }),
            None,
            "Washington, DC, US, 20340",
        ),
    );
    locations.insert(
        "Offutt AFB, Nebraska -Offutt AFB, NE 68113 US",
        (
            None,
            Some(State {
                code: String::from("NE"),
                name: String::from("Nebraska"),
            }),
            Some(Country {
                code: String::from("US"),
                name: String::from("United States"),
            }),
            Some(Zipcode {
                zipcode: String::from("68113"),
            }),
            None,
            "Offutt AFB, NE, US, 68113",
        ),
    );
    locations.insert(
        "Barcelona, Barcelona, ES",
        (
            None,
            None,
            Some(Country {
                code: String::from("ES"),
                name: String::from("Spain"),
            }),
            None,
            None,
            "Barcelona, ES",
        ),
    );
    locations
}
