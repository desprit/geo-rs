use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Read data from a given path and return as a HashMap
///
/// # Examples
///
/// ```
/// let countries = read_data("countries.txt")
/// ```
pub fn read_file(filename: &str) -> HashMap<String, String> {
    let data_path = format!("{}/src/data", env!("CARGO_MANIFEST_DIR"));
    let file_path = Path::new(&data_path).join(&filename);
    let mut data: HashMap<String, String> = HashMap::new();
    let file = File::open(file_path).unwrap();
    for line in io::BufReader::new(file).lines() {
        if let Ok(s) = line {
            let parts: Vec<&str> = s.split(";").collect();
            data.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    data
}

/// Read specific type of GEO data for a given country from file
///
/// # Arguments
///
/// * `country` - Name of the country, e.g. `US`
/// * `filename` - Type of GEO data to read, e.g. `cities`
///
/// # Examples
///
/// ```
/// read_country_data("US", "states");
/// ```
pub fn read_country_data(
    country: &str,
    filename: &str,
) -> HashMap<String, HashMap<String, String>> {
    let filename = format!("{}/{}.txt", &country.to_uppercase(), &filename);
    let mut data = HashMap::new();
    data.insert(country.to_string(), read_file(&filename));
    data
}

/// Read specific type of GEO data for all countries
///
/// # Arguments
///
/// * `filename` - Type of GEO data to read, e.g. `cities`
///
/// # Examples
///
/// ```
/// read_all_countries("cities");
/// ```
pub fn read_all_countries(filename: &str) -> HashMap<String, HashMap<String, String>> {
    let data_path = format!("{}/src/data", env!("CARGO_MANIFEST_DIR"));
    let mut data = HashMap::new();
    for path in Path::new(&data_path).read_dir().expect("couldn't read dir") {
        if let Ok(p) = path {
            if p.path().is_dir() {
                if let Ok(c) = p.file_name().into_string() {
                    let country_data_file = format!("{}/{}.txt", &c.as_str(), &filename);
                    data.insert(c.to_string(), read_file(&country_data_file));
                }
            }
        }
    }
    data
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_data_should_work() {
        let countries = super::read_file("countries.txt");
        assert_eq!(countries.get("Italy"), Some(&String::from("IT")));
    }

    #[test]
    fn test_read_country_data_should_work() {
        let states = super::read_country_data("US", "states");
        let us_states = states.get("US").unwrap();
        assert_eq!(us_states.get("AL"), Some(&String::from("Alabama")));
    }

    #[test]
    fn test_read_all_countries() {
        let states = super::read_all_countries("states");
        let ca_states = states.get("CA").unwrap();
        assert_eq!(ca_states.get("ON"), Some(&String::from("Ontario")));
        let us_states = states.get("US").unwrap();
        assert_eq!(us_states.get("CA"), Some(&String::from("California")));
    }
}
