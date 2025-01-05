use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::human_prefab::HumanPrefab;

#[derive(Debug)]
pub struct NPCSet {
    pub identifier: String,
    pub humans: Vec<HumanPrefab>,
}

impl NPCSet {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let humans = element
            .children()
            .filter(Node::is_element)
            .map(|child| HumanPrefab::new(child, Some(identifier.clone())))
            .collect::<Vec<_>>();

        Self { identifier, humans }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
