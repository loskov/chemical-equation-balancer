use std::collections::{HashSet, hash_map::RandomState};
use crate::item::Item;

#[derive(Debug)]
pub struct Element {
    /// Name
    pub name: String,
    /// Count
    pub count: u8,
}

impl Element {
    /// Element constructor.
    pub fn new(name: String, count: u8) -> Self {
        Self { name, count }
    }
}

impl Item for Element {
    fn add_to_elements_names(&self, elements_names: &mut HashSet<String, RandomState>) {
        elements_names.insert(self.name.clone());
    }

    fn count_element_by_name(&self, element_name: &str) -> u32 {
        if self.name == element_name { u32::from(self.count) } else { 0 }
    }

    fn format(&self) -> String {
        let mut result = self.name.clone();

        if self.count != 1 {
            result += &self.count.to_string();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{element::Element, item::Item};

    #[test]
    fn test_new() {
        let element = Element::new("H".to_string(), 2);
        assert_eq!(element.name, "H");
        assert_eq!(element.count, 2);
    }

    #[test]
    fn test_add_to_elements_names() {
        let mut elements_names = HashSet::new();
        let mut result = HashSet::new();
        let element = Element::new("H".to_string(), 2);
        element.add_to_elements_names(&mut elements_names);
        result.insert("H".to_string());
        assert_eq!(elements_names, result);

        element.add_to_elements_names(&mut elements_names);
        assert_eq!(elements_names, result);

        let element = Element::new("O".to_string(), 1);
        element.add_to_elements_names(&mut elements_names);
        result.insert("O".to_string());
        assert_eq!(elements_names, result);
    }

    #[test]
    fn test_count_element_by_name() {
        let element = Element::new("H".to_string(), 2);
        assert_eq!(element.count_element_by_name("H"), 2);
        assert_eq!(element.count_element_by_name("O"), 0);
    }

    #[test]
    fn test_format() {
        assert_eq!(Element::new("H".to_string(), 2).format(), "H2");
        assert_eq!(Element::new("O".to_string(), 1).format(), "O");
    }
}
