#[derive(Debug)]
pub enum RegularExpression {
    Digits,
    Spaces,
    Symbol,
    Token,
}

impl RegularExpression {
    pub fn get_pattern(&self) -> &str {
        match &self {
            RegularExpression::Digits => r"^\d+",
            RegularExpression::Spaces => r"^\s+",
            RegularExpression::Symbol => r"^[A-Z][a-z]*",
            RegularExpression::Token => r"^([A-Z][a-z]*|\d+|[e+-=(){}])",
        }
    }
}
