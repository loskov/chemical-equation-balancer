#[derive(Debug)]
pub enum ParserError {
    AdvancingBeyondLastToken,
    ChargeOrChargeSignIsExpected { start_index: usize },
    ChargeSignIsExpected { start_index: usize },
    ClosingBracketAfterChargeIsExpected { start_index: usize },
    ElectronNeedsToStandAlone { start_index: usize, end_index: usize },
    ElementGroupOrClosingBracketIsExpected { start_index: usize },
    ElementIsNotParsed,
    EmptyGroup { start_index: usize, end_index: usize },
    EntityIsExpected { start_index: usize, end_index: usize },
    InvalidChargeForElectron { start_index: usize, end_index: usize },
    InvalidSymbol { start_index: usize },
    NumberIsNotExpected { start_index: usize },
    NumberIsTooLarge,
    PlusSignOrEndIsExpected { start_index: usize },
    PlusSignOrEqualSignIsExpected { start_index: usize },
    TokenDoesNotMatchString,
}

impl ParserError {
    /// Returns the description
    pub fn get_description(&self) -> &str {
        match self {
            Self::AdvancingBeyondLastToken { .. } => "Advancing beyond the last token.",
            Self::ChargeOrChargeSignIsExpected { .. } => "The charge or charge sign is expected.",
            Self::ChargeSignIsExpected { .. } => "The charge sign is expected.",
            Self::ClosingBracketAfterChargeIsExpected { .. } =>
                "The closing bracket after the charge is expected.",
            Self::ElectronNeedsToStandAlone { .. } => "An electron needs to stand alone.",
            Self::ElementGroupOrClosingBracketIsExpected { .. } =>
                "The element, group, or closing bracket is expected.",
            Self::ElementIsNotParsed => "The element is not parsed.",
            Self::EmptyGroup { .. } => "Empty group.",
            Self::EntityIsExpected { .. } => "The entity is expected.",
            Self::InvalidChargeForElectron { .. } => "Invalid charge for an electron.",
            Self::InvalidSymbol { .. } => "Invalid symbol.",
            Self::NumberIsNotExpected { .. } => "The number is not expected.",
            Self::NumberIsTooLarge => "The number is too large.",
            Self::PlusSignOrEndIsExpected { .. } => "The plus sign or end is expected.",
            Self::PlusSignOrEqualSignIsExpected { .. } =>
                "The plus sign or equal sign is expected.",
            Self::TokenDoesNotMatchString => "The token does not match the string.",
        }
    }
}
