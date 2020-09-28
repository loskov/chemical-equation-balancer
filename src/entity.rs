use num::abs;
use std::collections::{HashSet, hash_map::RandomState};
use crate::item::Item;

pub struct Entity {
    /// Items
    pub items: Vec<Box<dyn Item>>,
    /// Charge
    pub charge: i8,
}

impl Entity {
    /// Entity constructor.
    pub fn new(items: Vec<Box<dyn Item>>, charge: i8) -> Entity {
        debug_assert!(!items.is_empty() || charge == -1, "Invalid entity.");

        Entity { items, charge }
    }
}

impl Item for Entity {
    fn add_to_elements_names(&self, elements_names: &mut HashSet<String, RandomState>) {
        elements_names.insert("e".to_string());

        for x in &self.items {
            x.add_to_elements_names(elements_names);
        }
    }

    fn count_element_by_name(&self, element_name: &str) -> u32 {
        if element_name == "e" {
            return -self.charge as u32;
        }

        self.items.iter().fold(0, |sum, x| sum + x.count_element_by_name(element_name))
    }

    fn format(&self) -> String {
        if self.items.is_empty() && self.charge == -1 {
            return "e\u{2212}".to_string();
        }

        let mut result = String::new();

        for x in &self.items {
            result += &x.format();
        }

        if self.charge != 0 {
            let charge_without_sign = abs(self.charge);

            if charge_without_sign != 1 {
                result += &[
                    "{",
                    charge_without_sign.to_string().as_str(),
                    if self.charge > 0 { "+" } else { "\u{2212}" },
                    "}",
                ].join("");
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{entity::Entity, element::Element, item::Item};

    #[test]
    fn test_new() {
        let element = Element::new("H".to_string(), 2);
        let items: Vec<Box<dyn Item>> = vec![Box::new(element)];
        let entity = Entity::new(items, 1);
        assert_eq!(entity.charge, 1);
    }

    #[test]
    fn test_add_to_elements_names() {
        // TODO
    }

    #[test]
    fn test_count_element_by_name() {
        // TODO
    }

    #[test]
    fn test_format() {
        // TODO
    }
}
