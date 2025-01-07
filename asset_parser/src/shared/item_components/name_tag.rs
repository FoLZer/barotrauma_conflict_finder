use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct NameTagComponent {
    pub item: ItemComponent,

    pub written_name: String,
}

impl NameTagComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            written_name: element
                .attribute_ignore_ascii_case("writtenname")
                .map(|v| v.to_owned())
                .unwrap(),
        }
    }
}
