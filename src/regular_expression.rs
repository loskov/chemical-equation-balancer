use regex::Regex;

#[derive(Debug)]
pub enum RegularExpression {
    Digits,
    Spaces,
    Symbol,
    Token,
}

impl RegularExpression {
    /// Returns a pattern.
    fn get_pattern(&self) -> &str {
        match self {
            Self::Digits => r"^\d+",
            Self::Spaces => r"^\s+",
            Self::Symbol => "^[A-Z][a-z]*",
            Self::Token => r"^([A-Z][a-z]*|\d+|[e+-=(){}])",
        }
    }

    /// Returns the Regex instance.
    pub fn get_regex(self) -> Regex {
        Regex::new(self.get_pattern()).unwrap()
    }
}
