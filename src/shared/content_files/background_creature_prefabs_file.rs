use roxmltree::Node;

use crate::shared::{
    prefabs::background_creature_prefab::BackgroundCreaturePrefab, util::XmlContentFile,
};

#[derive(Debug)]
pub struct BackgroundCreaturePrefabsFile {
    pub background_creature_prefabs: Vec<BackgroundCreaturePrefab>,
}

impl BackgroundCreaturePrefabsFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            background_creature_prefabs: Vec::new(),
        };

        if element.tag_name().name().eq_ignore_ascii_case("override") {
            for child in element
                .children()
                .find(Node::is_element)
                .unwrap()
                .children()
                .filter(Node::is_element)
            {
                r.background_creature_prefabs
                    .push(BackgroundCreaturePrefab::new(child));
            }
        } else {
            for child in element.children().filter(Node::is_element) {
                r.background_creature_prefabs
                    .push(BackgroundCreaturePrefab::new(child));
            }
        }

        r.background_creature_prefabs.shrink_to_fit();

        r
    }
}

impl XmlContentFile for BackgroundCreaturePrefabsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
