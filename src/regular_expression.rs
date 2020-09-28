use regex::Regex;

#[derive(Debug)]
pub enum RegularExpression {
    Digits,
    Element,
    Spaces,
    Token,
}

impl RegularExpression {
    pub fn new(regular_expression: RegularExpression) -> Regex {
        Regex::new(regular_expression.get_pattern()).unwrap()
    }

    fn get_pattern(&self) -> &str {
        match &self {
            RegularExpression::Digits => r"^\d+",
            RegularExpression::Element => r"^[A-Z][a-z]*",
            RegularExpression::Spaces => r"^\s+",
            RegularExpression::Token => r"^([A-Z][a-z]*|\d+|[e+-=(){}])",
        }
    }
}
