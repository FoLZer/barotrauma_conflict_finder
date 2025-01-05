use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{prefabs::mission_prefab::MissionPrefab, util::Overridable},
};

#[derive(Debug)]
pub struct MissionsFile {
    pub mission_prefabs: Vec<Overridable<MissionPrefab>>,
}

impl MissionsFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            mission_prefabs: Vec::new(),
        };

        r.load_from_node(element, false);
        r.mission_prefabs.shrink_to_fit();

        r
    }

    fn matches_singular(identifier: &str) -> bool {
        !Self::matches_plural(identifier)
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("missions")
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
            self.mission_prefabs.push(Overridable {
                value: MissionPrefab::new(element),
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

impl XmlContentFile for MissionsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
