use roxmltree::Node;

use crate::shared::util::NodeExp;

#[derive(Debug)]
pub struct NPCPersonalityTrait {
    pub identifier: String,
    pub name: Option<String>,
    pub allowed_dialog_tags: Vec<String>,
    pub commonness: f32,
}

impl NPCPersonalityTrait {
    pub fn new(element: Node) -> Self {
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(|v| v.to_owned());
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .or(name.clone())
            .unwrap();
        let allowed_dialog_tags = element
            .attribute_ignore_ascii_case("alloweddialogtags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
            .unwrap();
        let commonness = element
            .attribute_ignore_ascii_case("commonness")
            .map_or(1.0, |v| v.parse().unwrap());

        Self {
            identifier,
            name,
            allowed_dialog_tags,
            commonness,
        }
    }
}
