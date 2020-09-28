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
    pub fn get_description(&self) -> &'static str {
        match &self {
            ParserError::AdvancingBeyondLastToken { .. } =>
                "Продвижение за пределы последнего ключа.",
            ParserError::ChargeOrChargeSignExpected { .. } => "Ожидается заряд или знак заряда.",
            ParserError::ChargeSignExpected { .. } => "Ожидается знак заряда.",
            ParserError::ClosingParenthesisAfterChargeExpected { .. } =>
                "Ожидается закрывающая скобка после заряда.",
            ParserError::ElectronNeedsToStandAlone { .. } => "Электрон должен стоять один.",
            ParserError::ElementGroupOrClosingParenthesesIsExpected { .. } =>
                "Ожидается элемент, группа или закрывающая скобка.",
            ParserError::ElementIsNotParsed => "Элемент не разобран.",
            ParserError::EmptyGroup { .. } => "Пустая группа",
            ParserError::EntityExpected { .. } => "Пропущено вещество.",
            ParserError::InvalidChargeForElectron { .. } => "Неверный заряд электрона.",
            ParserError::InvalidSymbol { .. } => "Неверный символ.",
            ParserError::NumberNotExpected { .. } => "Число не ожидалось.",
            ParserError::PlusOrEndExpected { .. } => "Ожидается плюс или завершение.",
            ParserError::PlusOrEqualSignExpected { .. } => "Ожидается плюс или знак равенства.",
            ParserError::TokenMismatch => "Ключ не совпадает со строкой.",
            ParserError::TooBigNumber => "Слишком большое число.",
        }
    }
}
