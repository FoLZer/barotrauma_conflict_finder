use roxmltree::Node;

use crate::shared::{
    prefabs::npc_set::NPCSet,
    util::{NodeExp, Overridable, XmlContentFile},
};

#[derive(Debug)]
pub struct NPCSetsFile {
    pub sets: Vec<Overridable<NPCSet>>,
}

impl NPCSetsFile {
    pub fn new(element: Node) -> Self {
        let mut sets = Self::load_from_node(element, false);
        sets.shrink_to_fit();
        Self { sets }
    }

    fn matches_singular(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("npcset")
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("npcsets")
    }

    fn load_from_node(element: Node, overriding: bool) -> Vec<Overridable<NPCSet>> {
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
                    value: NPCSet::new(element),
                    is_override: overriding,
                }]
            } else {
                println!("Failed to create an NPCSet since it doesn't contain an identifier.");
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

impl XmlContentFile for NPCSetsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
