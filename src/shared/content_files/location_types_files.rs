use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{prefabs::location_type::LocationType, util::Overridable},
};

#[derive(Debug)]
pub struct LocationTypesFile {
    pub location_types: Vec<Overridable<LocationType>>,
}

impl LocationTypesFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            location_types: Vec::new(),
        };

        r.load_from_node(element, false);
        r.location_types.shrink_to_fit();

        r
    }

    fn matches_singular(identifier: &str) -> bool {
        !Self::matches_plural(identifier)
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("locationtypes")
    }

    fn load_from_node(&mut self, element: Node, overriding: bool) {
        let elem_name = element.tag_name().name();
        if elem_name.eq_ignore_ascii_case("override") {
            element
                .children()
                .filter(Node::is_element)
                .for_each(|child| self.load_from_node(child, true));
        } else if elem_name.eq_ignore_ascii_case("clear") {
            todo!();
            //self.prefabs.add_override_file(OverrideFile {
            //    hash: self.hash.clone(),
            //    content_package_index: todo!(),
            //});
        } else if Self::matches_singular(elem_name) {
            self.location_types.push(Overridable {
                value: LocationType::new(element),
                is_override: overriding,
            });
        } else if Self::matches_plural(elem_name) {
            element
                .children()
                .filter(Node::is_element)
                .for_each(|child| self.load_from_node(child, overriding));
        } else {
            dbg!(elem_name);
            panic!() //TODO:
        }
    }
}

impl XmlContentFile for LocationTypesFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
