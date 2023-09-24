use std::convert::TryFrom;
use crate::{
    element::Element,
    entity::Entity,
    equation::Equation,
    group::Group,
    item::Item,
    parser_error::ParserError,
    regular_expression::RegularExpression,
};

pub struct Parser<'eq> {
    equation: &'eq str,
    position: usize,
}

impl Parser<'_> {
    /// Parser constructor.
    pub fn new(equation: &str) -> Parser {
        Parser { equation, position: 0 }
    }

    /// Returns a substring.
    fn get_substring(&self) -> &str {
        &self.equation[self.position..]
    }

    /// Returns the next token.
    fn get_next_token(&self) -> Result<Option<String>, ParserError> {
        if self.position == self.equation.chars().count() {
            return Ok(None);
        }

        match RegularExpression::Token.get_regex().captures(self.get_substring()) {
            Some(x) => Ok(Some(x[0].to_string())),
            None => Err(ParserError::InvalidSymbol { start_index: self.position }),
        }
    }

    /// Takes a token.
    fn take_token(&mut self) -> Result<String, ParserError> {
        let next_token = self.get_next_token()?.ok_or(ParserError::AdvancingBeyondLastToken)?;
        self.position += next_token.chars().count();
        self.skip_spaces();

        Ok(next_token)
    }

    /// Consumes a string.
    fn consume(&mut self, string: &str) -> Result<(), ParserError> {
        if self.take_token()? == string { Ok(()) } else { Err(ParserError::TokenMismatch) }
    }

    /// Skips the spaces.
    fn skip_spaces(&mut self) {
        if let Some(x) = RegularExpression::Spaces.get_regex().captures(self.get_substring()) {
            self.position += x[0].chars().count();
        }
    }

    /// Parses an optional number.
    fn parse_optional_number(&mut self) -> Result<u8, ParserError> {
        match self.get_next_token()? {
            Some(x) => {
                if RegularExpression::Digits.get_regex().is_match(&x) {
                    self.take_token()?.parse::<u8>().map_err(|_e| ParserError::TooBigNumber)
                } else {
                    Ok(1)
                }
            },
            None => Ok(1),
        }
    }

    /// Parses an element.
    fn parse_element(&mut self) -> Result<Element, ParserError> {
        let token = self.take_token()?;

        if RegularExpression::Symbol.get_regex().is_match(&token) {
            Ok(Element::new(token, self.parse_optional_number()?))
        } else {
            Err(ParserError::ElementIsNotParsed)
        }
    }

    /// Parses a group.
    fn parse_group(&mut self) -> Result<Group, ParserError> {
        let start_position = self.position;
        let mut items: Vec<Box<dyn Item>> = vec![];

        self.consume("(")?;

        let regex_for_symbol = RegularExpression::Symbol.get_regex();

        loop {
            let next_token = self.get_next_token()?
                .ok_or(ParserError::ElementGroupOrClosingParenthesesIsExpected {
                    start_index: self.position
                })?;

            if next_token == "(" {
                items.push(Box::new(self.parse_group()?));
            } else if regex_for_symbol.is_match(&next_token) {
                items.push(Box::new(self.parse_element()?));
            } else if next_token == ")" {
                self.consume(&next_token)?;

                if items.is_empty() {
                    return Err(ParserError::EmptyGroup {
                        start_index: start_position, end_index: self.position,
                    });
                }

                break;
            } else {
                return Err(ParserError::ElementGroupOrClosingParenthesesIsExpected {
                    start_index: self.position,
                });
            }
        }

        Ok(Group::new(items, self.parse_optional_number()?))
    }

    /// Parses an entity.
    fn parse_entity(&mut self) -> Result<Entity, ParserError> {
        let start_position = self.position;
        let mut items: Vec<Box<dyn Item>> = vec![];
        let mut is_electron = false;
        let regex_for_symbol = RegularExpression::Symbol.get_regex();
        let regex_for_digits = RegularExpression::Digits.get_regex();

        while let Some(x) = self.get_next_token()? {
            if x == "(" {
                items.push(Box::new(self.parse_group()?));
            } else if x == "e" {
                self.consume(&x)?;

                is_electron = true;
            } else if regex_for_symbol.is_match(&x) {
                items.push(Box::new(self.parse_element()?));
            } else if regex_for_digits.is_match(&x) {
                return Err(ParserError::NumberNotExpected { start_index: self.position });
            } else {
                break;
            }
        }

        let mut charge: Option<i8> = None;

        if let Some(x) = self.get_next_token()? {
            if x == "{" {
                self.consume(&x)?;

                self.get_next_token()?.ok_or(ParserError::ChargeOrChargeSignExpected {
                    start_index: self.position,
                })?;

                charge = Some(i8::try_from(self.parse_optional_number()?).unwrap());

                if let Some(x) = self.get_next_token()? {
                    if x == "-" {
                        charge = Some(-charge.unwrap());
                    } else if x != "+" {
                        return Err(ParserError::ChargeSignExpected { start_index: self.position });
                    }
                }

                self.take_token()?;

                if let Some(x) = self.get_next_token()? {
                    if x == "}" {
                        self.consume(&x)?;
                    } else {
                        return Err(ParserError::ClosingParenthesisAfterChargeExpected {
                            start_index: self.position,
                        });
                    }
                }
            }
        }

        if is_electron {
            if !items.is_empty() {
                return Err(ParserError::ElectronNeedsToStandAlone {
                    start_index: start_position, end_index: self.position,
                });
            }

            if charge.is_none() {
                charge = Some(-1);
            }

            if charge != Some(-1) {
                return Err(ParserError::InvalidChargeForElectron {
                    start_index: start_position, end_index: self.position,
                });
            }
        } else {
            if items.is_empty() {
                return Err(ParserError::EntityExpected {
                    start_index: start_position, end_index: self.position,
                });
            }

            if charge.is_none() {
                charge = Some(0);
            }
        }

        Ok(Entity::new(items, charge.unwrap()))
    }

    /// Parses an equation.
    pub fn parse_equation(&mut self) -> Result<Equation, ParserError> {
        self.skip_spaces();

        let mut reactants = vec![];
        reactants.push(self.parse_entity()?);

        loop {
            let next_token = self.get_next_token()?
                .ok_or(ParserError::PlusOrEqualSignExpected { start_index: self.position })?;

            match next_token.as_str() {
                "+" => {
                    self.consume(&next_token)?;

                    reactants.push(self.parse_entity()?);
                },
                "=" => {
                    self.consume(&next_token)?;

                    break;
                },
                _ => return Err(ParserError::PlusOrEqualSignExpected {
                    start_index: self.position,
                }),
            }
        }

        let mut products = vec![];
        products.push(self.parse_entity()?);

        while let Some(x) = self.get_next_token()? {
            match x.as_str() {
                "+" => {
                    self.consume(&x)?;

                    products.push(self.parse_entity()?);
                }
                _ => return Err(ParserError::PlusOrEndExpected { start_index: self.position }),
            }
        }

        Ok(Equation::new(reactants, products))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::item::Item;

    #[test]
    fn test_new() {
        let parser = Parser::new("H2 + O2 = H2O");
        assert_eq!(parser.equation, "H2 + O2 = H2O");
        assert_eq!(parser.position, 0);
    }

    #[test]
    fn test_get_substring() {
        let parser = Parser::new("H2 + O2 = H2O");
        assert_eq!(parser.get_substring(), "H2 + O2 = H2O");
    }

    #[test]
    fn test_get_next_token() {
        let mut parser = Parser::new("H2 + O2 = H2O");
        assert_eq!(parser.get_next_token().unwrap(), Some("H".to_string()));

        let _ = parser.consume("H");
        assert_eq!(parser.get_next_token().unwrap(), Some("2".to_string()));
    }

    #[test]
    fn test_take_token() {
        let mut parser = Parser::new("H2 + O2 = H2O");
        assert_eq!(parser.take_token().unwrap(), "H");
        assert_eq!(parser.position, 1);
        assert_eq!(parser.take_token().unwrap(), "2");
        assert_eq!(parser.position, 3);
        assert_eq!(parser.take_token().unwrap(), "+");
        assert_eq!(parser.position, 5);
    }

    #[test]
    fn test_consume() {
        let mut parser = Parser::new("H2 + O2 = H2O");
        let _ = parser.consume("H");
        assert_eq!(parser.position, 1);
    }

    #[test]
    fn test_skip_spaces() {
        let mut parser = Parser::new("H2 + O2 = H2O");
        let _ = parser.consume("H");
        let _ = parser.consume("2");
        parser.skip_spaces();
        assert_eq!(parser.position, 3);
    }

    #[test]
    fn test_parse_optional_number() {
        let mut parser = Parser::new("H2 + O2 = H2O");
        let _ = parser.consume("H");
        assert_eq!(parser.parse_optional_number().unwrap(), 2);
    }

    #[test]
    fn test_parse_element() {
        let mut parser = Parser::new("H2 + O2 = H2O");
        assert_eq!(parser.parse_element().unwrap().format(), "H2");
    }

    #[test]
    fn test_parse_group() {
        let mut parser = Parser::new("Al2(SO4)3 = Al2O3 + SO3");
        let _ = parser.consume("Al");
        let _ = parser.consume("2");
        assert_eq!(parser.parse_group().unwrap().format(), "(SO4)3");
    }

    #[test]
    fn test_parse_entity() {
        let mut parser = Parser::new("Al2(SO4)3 = Al2O3 + SO3");
        assert_eq!(parser.parse_entity().unwrap().format(), "Al2(SO4)3");
    }

    #[test]
    fn test_parse_equation() {
        let mut parser = Parser::new("H2 + O2 = H2O");
        assert_eq!(
            parser.parse_equation().unwrap().format(&[2, 1, 2]),
            "2\u{a0}H2 + O2 = 2\u{a0}H2O"
        );
    }
}
