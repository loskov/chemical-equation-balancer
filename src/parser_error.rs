#[derive(Debug)]
pub enum ParserError {
    AdvancingBeyondLastToken,
    ChargeOrChargeSignExpected { start_index: usize },
    ChargeSignExpected { start_index: usize },
    ClosingParenthesisAfterChargeExpected { start_index: usize },
    ElectronNeedsToStandAlone { start_index: usize, end_index: usize },
    ElementGroupOrClosingParenthesesIsExpected { start_index: usize },
    ElementIsNotParsed,
    EmptyGroup { start_index: usize, end_index: usize },
    EntityExpected { start_index: usize, end_index: usize },
    InvalidChargeForElectron { start_index: usize, end_index: usize },
    InvalidSymbol { start_index: usize },
    NumberNotExpected { start_index: usize },
    PlusOrEndExpected { start_index: usize },
    PlusOrEqualSignExpected { start_index: usize },
    TokenMismatch,
    TooBigNumber,
}

impl ParserError {
    /// Returns the description
    pub fn get_description(&self) -> &str {
        match self {
            Self::AdvancingBeyondLastToken { .. } => "Продвижение за пределы последнего ключа.",
            Self::ChargeOrChargeSignExpected { .. } => "Ожидается заряд или знак заряда.",
            Self::ChargeSignExpected { .. } => "Ожидается знак заряда.",
            Self::ClosingParenthesisAfterChargeExpected { .. } =>
                "Ожидается закрывающая скобка после заряда.",
            Self::ElectronNeedsToStandAlone { .. } => "Электрон должен стоять один.",
            Self::ElementGroupOrClosingParenthesesIsExpected { .. } =>
                "Ожидается элемент, группа или закрывающая скобка.",
            Self::ElementIsNotParsed => "Элемент не разобран.",
            Self::EmptyGroup { .. } => "Пустая группа",
            Self::EntityExpected { .. } => "Пропущено вещество.",
            Self::InvalidChargeForElectron { .. } => "Неверный заряд электрона.",
            Self::InvalidSymbol { .. } => "Неверный символ.",
            Self::NumberNotExpected { .. } => "Число не ожидалось.",
            Self::PlusOrEndExpected { .. } => "Ожидается плюс или завершение.",
            Self::PlusOrEqualSignExpected { .. } => "Ожидается плюс или знак равенства.",
            Self::TokenMismatch => "Ключ не совпадает со строкой.",
            Self::TooBigNumber => "Слишком большое число.",
        }
    }
}
