use crate::utils;
use crate::Parser;
use std::fmt;

#[derive(Debug, Clone, Hash, Eq)]
pub struct Address {
    pub address: String,
}

impl PartialEq for Address {
    fn eq(&self, other: &Address) -> bool {
        self.address == other.address
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.address.trim())
    }
}

impl Parser {
    pub fn remove_address(&self, s: &mut String, address: &Address) {
        *s = s.replace(&address.address, "");
        utils::clean(s);
    }

    pub fn find_address(&self, s: &str) -> Option<Address> {
        Some(Address {
            address: s.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_fmt_address() {
        let address = Address {
            address: String::from("test address  "),
        };
        assert_eq!(format!("{}", address), String::from("test address"))
    }

    #[test]
    fn test_find_address() {
        let mut addresses: HashMap<&str, Option<Address>> = HashMap::new();
        addresses.insert(
            "Kent Atholville 44",
            Some(Address {
                address: String::from("Kent Atholville 44"),
            }),
        );
        let parser = Parser::new();
        for (input, address) in addresses {
            let output = parser.find_address(&input);
            assert_eq!(output, address);
        }
    }

    #[test]
    fn test_remove_address() {
        let mut addresses: HashMap<&str, (Address, &str)> = HashMap::new();
        addresses.insert(
            "Atholville, New Brunswick, Canada, Kent Atholville 44",
            (
                Address {
                    address: String::from("Kent Atholville 44"),
                },
                "Atholville, New Brunswick, Canada",
            ),
        );
        let parser = Parser::new();
        for (k, (address, output)) in addresses {
            let mut input = k.to_string();
            parser.remove_address(&mut input, &address);
            assert_eq!(input, output);
        }
    }
}
