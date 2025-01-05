use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::level_object_prefab::LevelObjectPrefab,
        util::{NodeExp, Overridable},
    },
};

#[derive(Debug)]
pub struct LevelObjectPrefabsFile {
    pub prefabs: Vec<Overridable<LevelObjectPrefab>>,
}

impl LevelObjectPrefabsFile {
    pub fn new(element: Node) -> Self {
        let mut prefabs = Self::load_from_node(element, false);
        prefabs.shrink_to_fit();
        Self { prefabs }
    }

    fn matches_singular(identifier: &str) -> bool {
        !Self::matches_plural(identifier)
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("levelobjects")
    }

    fn load_from_node(element: Node, overriding: bool) -> Vec<Overridable<LevelObjectPrefab>> {
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
                    value: LevelObjectPrefab::new(element),
                    is_override: overriding,
                }]
            } else {
                println!(
                    "Failed to create an LevelObjectPrefab since it doesn't contain an identifier."
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

impl XmlContentFile for LevelObjectPrefabsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
