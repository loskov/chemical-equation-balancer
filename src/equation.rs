use std::collections::HashSet;
use crate::{entity::Entity, item::Item};

pub struct Equation {
    /// Reactants
    pub reactants: Vec<Entity>,
    /// Products
    pub products: Vec<Entity>,
}

impl Equation {
    /// Equation constructor.
    pub fn new(reactants: Vec<Entity>, products: Vec<Entity>) -> Equation {
        Equation { reactants, products }
    }

    /// Returns the elements names.
    pub fn get_elements_names(&self) -> Vec<String> {
        let mut elements_names = HashSet::new();

        for x in &self.reactants {
            x.add_to_elements_names(&mut elements_names);
        }

        for x in &self.products {
            x.add_to_elements_names(&mut elements_names);
        }

        elements_names.into_iter().collect()
    }

    /// Formats the entities.
    fn format_entities(&self, coefficients: &[i32], entities: &[Entity]) -> String {
        let mut result = String::new();
        let mut is_head = true;

        for i in 0..entities.len() {
            let coefficient = if i < coefficients.len() { coefficients[i] } else { 1 };

            if coefficient != 0 {
                if is_head {
                    is_head = false;
                } else {
                    result += " + ";
                }

                if coefficient != 1 {
                    result += &[coefficient.to_string().as_str(), "\u{a0}"].join("");
                }

                result += &entities[i].format();
            }
        }

        result
    }

    /// Formats an equation.
    pub fn format(&self, coefficients: &[i32]) -> String {
        [
            &self.format_entities(coefficients, &self.reactants),
            " = ",
            &self.format_entities(
                &coefficients[self.reactants.len()..coefficients.len()],
                &self.products
            ),
        ].join("")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        // TODO
    }

    #[test]
    fn test_get_elements_names() {
        // TODO
    }

    #[test]
    fn test_format_entities() {
        // TODO
    }

    #[test]
    fn test_format() {
        // TODO
    }
}
