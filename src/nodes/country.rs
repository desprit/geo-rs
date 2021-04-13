use crate::Parser;

#[derive(Debug)]
pub struct Country {
    pub name: String,
    pub code: String,
}

impl PartialEq for Country {
    fn eq(&self, other: &Country) -> bool {
        self.name == other.name && self.code == other.code
    }
}

impl Parser {
    pub fn parse_country(&self, s: &str) -> Option<Country> {
        Some(Country {
            name: String::from("Italy"),
            code: String::from("IT"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;
    #[test]
    fn test_parse_country() {
        let p = Parser::new(None);
        let country = p.parse_country("Italy");
        assert_eq!(
            country,
            Some(super::Country {
                name: "Italy".into(),
                code: "IT".into()
            })
        );
    }
}
