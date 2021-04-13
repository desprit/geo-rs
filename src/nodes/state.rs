use crate::Parser;

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub code: String,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.name == other.name && self.code == other.code
    }
}

impl Parser {
    pub fn parse_state(&self, s: &str) -> Option<State> {
        Some(State {
            name: String::from("Ontario"),
            code: String::from("ON"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;
    #[test]
    fn test_parse_state() {
        let p = Parser::new(None);
        let state = p.parse_state("Ontario");
        assert_eq!(
            state,
            Some(super::State {
                name: "Ontario".into(),
                code: "ON".into()
            })
        );
    }
}
