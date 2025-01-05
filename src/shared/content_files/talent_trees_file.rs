use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{prefabs::talent_tree_prefab::TalentTree, util::Overridable},
};

#[derive(Debug)]
pub struct TalentTreesFile {
    pub trees: Vec<Overridable<TalentTree>>,
}

impl TalentTreesFile {
    pub fn new(element: Node) -> Self {
        let mut trees = Self::load_from_node(element, false);
        trees.shrink_to_fit();
        Self { trees }
    }

    fn matches_singular(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("talenttree")
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("talenttrees")
    }

    fn load_from_node(element: Node, overriding: bool) -> Vec<Overridable<TalentTree>> {
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
            vec![Overridable {
                value: TalentTree::new(element),
                is_override: overriding,
            }]
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

impl XmlContentFile for TalentTreesFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
