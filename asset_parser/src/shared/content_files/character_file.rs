use roxmltree::Node;

use crate::{shared::prefabs::character_prefab::CharacterPrefab, shared::util::XmlContentFile};

#[derive(Debug)]
pub struct CharacterFile {
    pub character: CharacterPrefab,
}

impl CharacterFile {
    pub fn new(element: Node) -> Self {
        Self {
            character: CharacterPrefab::new(element),
        }
    }
}

impl XmlContentFile for CharacterFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
