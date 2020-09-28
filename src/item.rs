use std::collections::HashSet;
use std::collections::hash_map::RandomState;

pub trait Item {
    /// Adds element name to set of elements' names.
    fn add_to_elements_names(&self, elements_names: &mut HashSet<String, RandomState>);
    /// Counts the number of atoms of element by its name.
    fn count_element_by_name(&self, element_name: &str) -> u32;
    /// Formats an item.
    fn format(&self) -> String;
}
