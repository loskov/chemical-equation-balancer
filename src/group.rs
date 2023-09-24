use std::collections::{HashSet, hash_map::RandomState};
use crate::item::Item;

pub struct Group {
    /// Items
    pub items: Vec<Box<dyn Item>>,
    /// Count
    pub count: u8,
}

impl Group {
    /// Group constructor.
    pub fn new(items: Vec<Box<dyn Item>>, count: u8) -> Self {
        Self { items, count }
    }
}

impl Item for Group {
    fn add_to_elements_names(&self, elements_names: &mut HashSet<String, RandomState>) {
        for x in &self.items {
            x.add_to_elements_names(elements_names);
        }
    }

    fn count_element_by_name(&self, element_name: &str) -> u32 {
        u32::from(self.count)
            * self.items.iter().fold(0, |sum, x| sum + x.count_element_by_name(element_name))
    }

    fn format(&self) -> String {
        let mut result = "(".to_string();

        for x in &self.items {
            result += &x.format();
        }

        result += ")";

        if self.count != 1 {
            result += &self.count.to_string();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{element::Element, group::Group, item::Item};

    #[test]
    fn test_new() {
        let element = Element::new("H".to_string(), 2);
        let items: Vec<Box<dyn Item>> = vec![Box::new(element)];
        let group = Group::new(items, 1);
        assert_eq!(group.count, 1);
    }

    #[test]
    fn test_add_to_elements_names() {
        let mut elements_names = HashSet::new();
        let mut result = HashSet::new();
        let element = Element::new("H".to_string(), 2);
        let items: Vec<Box<dyn Item>> = vec![Box::new(element)];
        let group = Group::new(items, 1);
        group.add_to_elements_names(&mut elements_names);
        result.insert("H".to_string());
        assert_eq!(elements_names, result);

        // TODO
    }

    #[test]
    fn test_count_element_by_name() {
        let element = Element::new("H".to_string(), 2);
        let items: Vec<Box<dyn Item>> = vec![Box::new(element)];
        let group = Group::new(items, 1);
        assert_eq!(group.count_element_by_name("H"), 2);

        let element = Element::new("H".to_string(), 2);
        let items: Vec<Box<dyn Item>> = vec![Box::new(element)];
        let group = Group::new(items, 2);
        assert_eq!(group.count_element_by_name("H"), 4);

        // TODO
    }

    #[test]
    fn test_format() {
        let element = Element::new("H".to_string(), 2);
        let items: Vec<Box<dyn Item>> = vec![Box::new(element)];
        let group = Group::new(items, 1);
        assert_eq!(group.format(), "(H2)");

        let element = Element::new("H".to_string(), 2);
        let items: Vec<Box<dyn Item>> = vec![Box::new(element)];
        let group = Group::new(items, 2);
        assert_eq!(group.format(), "(H2)2");

        // TODO
    }
}
