use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::talent_prefab::TalentPrefab,
        util::{NodeExp, Overridable},
    },
};

#[derive(Debug)]
pub struct TalentsFile {
    pub items: Vec<Overridable<TalentPrefab>>,
}

impl TalentsFile {
    pub fn new(element: Node) -> Self {
        let mut items = Self::load_from_node(element, false);
        items.shrink_to_fit();
        Self { items }
    }

    fn matches_singular(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("talent")
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("talents")
    }

    fn load_from_node(element: Node, overriding: bool) -> Vec<Overridable<TalentPrefab>> {
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
                    value: TalentPrefab::new(element),
                    is_override: overriding,
                }]
            } else {
                println!(
                    "Failed to create an TalentPrefab since it doesn't contain an identifier."
                );
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

impl XmlContentFile for TalentsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
