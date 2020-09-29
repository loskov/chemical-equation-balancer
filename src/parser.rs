use regex::Regex;
use std::collections::HashSet;
use crate::{
    element::Element,
    entity::Entity,
    equation::Equation,
    item::Item,
    group::Group,
    parser_error::ParserError,
    regular_expression::RegularExpression,
};

pub struct Parser<'a> {
    equation: &'a str,
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

        let regular_expression = Regex::new(RegularExpression::Token.get_pattern()).unwrap();

        match regular_expression.captures(self.get_substring()) {
            Some(x) => Ok(Some(x[0].to_string())),
            None => Err(ParserError::InvalidSymbol { start_index: self.position }),
        }
    }

    /// Takes a token.
    fn take_token(&mut self) -> Result<String, ParserError> {
        let next_token = match self.get_next_token() {
            Ok(x) => match x {
                Some(x) => x,
                None => return Err(ParserError::AdvancingBeyondLastToken),
            },
            Err(e) => return Err(e),
        };
        self.position += next_token.chars().count();
        self.skip_spaces();
        Ok(next_token)
    }

    /// Consumes a string.
    fn consume(&mut self, string: &str) -> Result<(), ParserError> {
        match self.take_token() {
            Ok(x) => if x != string { Err(ParserError::TokenMismatch) } else { Ok(()) },
            Err(e) => Err(e),
        }
    }

    /// Skips the spaces.
    fn skip_spaces(&mut self) {
        let regular_expression = Regex::new(RegularExpression::Spaces.get_pattern()).unwrap();

        if let Some(x) = regular_expression.captures(self.get_substring()) {
            self.position += x[0].chars().count();
        }
    }

    /// Parses an optional number.
    fn parse_optional_number(&mut self) -> Result<u8, ParserError> {
        match self.get_next_token() {
            Ok(x) => match x {
                Some(x) => {
                    let regular_expression = Regex::new(
                        RegularExpression::Digits.get_pattern()
                    ).unwrap();

                    if regular_expression.is_match(x.as_str()) {
                        match self.take_token() {
                            Ok(x) => match x.parse::<u8>() {
                                Ok(x) => Ok(x),
                                Err(_) => Err(ParserError::TooBigNumber),
                            },
                            Err(e ) => Err(e),
                        }
                    } else {
                        Ok(1)
                    }
                },
                None => Ok(1),
            },
            Err(e) => Err(e),
        }
    }

    /// Parses an element.
    fn parse_element(&mut self) -> Result<Element, ParserError> {
        let token = match self.take_token() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        if Regex::new(RegularExpression::Symbol.get_pattern()).unwrap().is_match(&token) {
            let optional_number = match self.parse_optional_number() {
                Ok(x) => x,
                Err(e) => return Err(e),
            };
            Ok(Element::new(token, optional_number))
        } else {
            Err(ParserError::ElementIsNotParsed)
        }
    }

    /// Parses a group.
    fn parse_group(&mut self) -> Result<Group, ParserError> {
        let start_position = self.position;
        let mut items: Vec<Box<dyn Item>> = vec![];

        if let Err(e) = self.consume("(") {
            return Err(e);
        }

        let regular_expression = Regex::new(RegularExpression::Symbol.get_pattern()).unwrap();

        loop {
            let next_token = match self.get_next_token() {
                Ok(x) => match x {
                    Some(x) => x,
                    None => return Err(ParserError::ElementGroupOrClosingParenthesesIsExpected {
                        start_index: self.position,
                    }),
                },
                Err(e) => return Err(e),
            };

            if next_token == "(" {
                let group = match self.parse_group() {
                    Ok(x) => x,
                    Err(e) => return Err(e),
                };
                items.push(Box::new(group));
            } else if regular_expression.is_match(&next_token) {
                let element = match self.parse_element() {
                    Ok(x) => x,
                    Err(e) => return Err(e),
                };
                items.push(Box::new(element));
            } else if next_token == ")" {
                if let Err(e) = self.consume(&next_token) {
                    return Err(e);
                }

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

        let optional_number = match self.parse_optional_number() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        Ok(Group::new(items, optional_number))
    }

    /// Parses an entity.
    fn parse_entity(&mut self) -> Result<Entity, ParserError> {
        let start_position = self.position;
        let mut items: Vec<Box<dyn Item>> = vec![];
        let mut is_electron = false;
        let regular_expression_for_symbol = Regex::new(
            RegularExpression::Symbol.get_pattern()
        ).unwrap();
        let regular_expression_for_digits = Regex::new(
            RegularExpression::Digits.get_pattern()
        ).unwrap();

        loop {
            let next_token = match self.get_next_token() {
                Ok(x) => match x {
                    Some(x) => x,
                    None => break,
                },
                Err(e) => return Err(e),
            };

            if next_token == "(" {
                let group = match self.parse_group() {
                    Ok(x) => x,
                    Err(e) => return Err(e),
                };
                items.push(Box::new(group));
            } else if next_token == "e" {
                if let Err(e) = self.consume(&next_token) {
                    return Err(e);
                }

                is_electron = true;
            } else if regular_expression_for_symbol.is_match(&next_token) {
                let element = match self.parse_element() {
                    Ok(x) => x,
                    Err(e) => return Err(e),
                };
                items.push(Box::new(element));
            } else if regular_expression_for_digits.is_match(&next_token) {
                return Err(ParserError::NumberNotExpected { start_index: self.position });
            } else {
                break;
            }
        }

        let mut charge: Option<i8> = None;
        match self.get_next_token() {
            Ok(x) => if let Some(x) = x {
                if x == "{" {
                    if let Err(e) = self.consume(&x) {
                        return Err(e);
                    }

                    match self.get_next_token() {
                        Ok(x) => match x {
                            Some(_) => {},
                            None => return Err(ParserError::ChargeOrChargeSignExpected {
                                start_index: self.position,
                            }),
                        },
                        Err(e) => return Err(e),
                    }
                    charge = match self.parse_optional_number() {
                        Ok(x) => Some(x as i8),
                        Err(e) => return Err(e),
                    };
                    match self.get_next_token() {
                        Ok(x) => if let Some(x) = x {
                            if x == "-" {
                                charge = Some(-charge.unwrap());
                            } else if x != "+" {
                                return Err(ParserError::ChargeSignExpected {
                                    start_index: self.position,
                                });
                            }
                        },
                        Err(e) => return Err(e),
                    };
                    match self.take_token() {
                        Ok(_) => {},
                        Err(e) => return Err(e),
                    };
                    match self.get_next_token() {
                        Ok(x) => if let Some(x) = x {
                            if x == "}" {
                                match self.consume(&x) {
                                    Ok(_) => {},
                                    Err(e) => return Err(e),
                                };
                            } else {
                                return Err(ParserError::ClosingParenthesisAfterChargeExpected {
                                    start_index: self.position,
                                });
                            }
                        },
                        Err(e) => return Err(e),
                    };
                }
            },
            Err(e) => return Err(e),
        };
        let mut elements_names = HashSet::new();

        for x in &items {
            x.add_to_elements_names(&mut elements_names);
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
                    start_index: start_position,
                    end_index: self.position,
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
        let entity = match self.parse_entity() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        reactants.push(entity);

        loop {
            match self.get_next_token() {
                Ok(x) => if let Some(x) = x {
                    match x.as_str() {
                        "+" => {
                            if let Err(e) = self.consume(&x) {
                                return Err(e);
                            }

                            let entity = match self.parse_entity() {
                                Ok(x) => x,
                                Err(e) => return Err(e),
                            };
                            reactants.push(entity);
                        },
                        "=" => {
                            if let Err(e) = self.consume(&x) {
                                return Err(e);
                            }

                            break;
                        },
                        _ => return Err(ParserError::PlusOrEqualSignExpected {
                            start_index: self.position,
                        }),
                    }
                },
                Err(e) => return Err(e),
            };
        }

        let mut products = vec![];
        let entity = match self.parse_entity() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        products.push(entity);

        loop {
            match self.get_next_token() {
                Ok(x) => match x {
                    Some(x) => match x.as_str() {
                        "+" => {
                            if let Err(e) = self.consume(&x) {
                                return Err(e)
                            }

                            let entity = match self.parse_entity() {
                                Ok(x) => x,
                                Err(e) => return Err(e),
                            };
                            products.push(entity);
                        }
                        _ => return Err(ParserError::PlusOrEndExpected {
                            start_index: self.position,
                        }),
                    },
                    None => break,
                },
                Err(e) => return Err(e),
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
