use log::warn;
use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::item_prefab::ItemPrefab,
        util::{NodeExp, Overridable},
    },
};

#[derive(Debug)]
pub struct ItemFile {
    pub items: Vec<Overridable<ItemPrefab>>,
}

impl ItemFile {
    pub fn new(element: Node) -> Self {
        let mut items = Self::load_from_node(element, false);
        items.shrink_to_fit();
        Self { items }
    }

    fn matches_singular(identifier: &str) -> bool {
        !Self::matches_plural(identifier) //The guy who did that is a :/
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("items")
    }

    fn load_from_node(element: Node, overriding: bool) -> Vec<Overridable<ItemPrefab>> {
        let elem_name = element.tag_name().name();
        if elem_name.eq_ignore_ascii_case("override") {
            element
                .children()
                .filter(Node::is_element)
                .flat_map(|child| Self::load_from_node(child, true))
                .collect()
        } else if elem_name.eq_ignore_ascii_case("clear") {
            todo!();
            //self.prefabs.add_override_file(OverrideFile {
            //    hash: self.hash.clone(),
            //    content_package_index: todo!(),
            //});
        } else if Self::matches_singular(elem_name) {
            if element.has_attribute_ignore_ascii_case("identifier") {
                vec![Overridable {
                    value: ItemPrefab::new(element),
                    is_override: overriding,
                }]
            } else {
                warn!("Failed to create an ItemPrefab since it doesn't contain an identifier.");
                vec![]
            }
        } else if Self::matches_plural(elem_name) {
            element
                .children()
                .filter(Node::is_element)
                .flat_map(|child| Self::load_from_node(child, overriding))
                .collect()
        } else {
            dbg!(elem_name);
            panic!() //TODO:
        }
    }
}

impl XmlContentFile for ItemFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
