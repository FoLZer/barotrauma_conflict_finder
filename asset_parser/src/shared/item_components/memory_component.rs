use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct MemoryComponent {
    pub item: ItemComponent,

    pub max_value_length: u32,
    pub value: String,
    pub writeable: bool,
}

impl MemoryComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            max_value_length: element
                .attribute_ignore_ascii_case("maxvaluelength")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            value: element
                .attribute_ignore_ascii_case("value")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            writeable: element
                .attribute_ignore_ascii_case("writeable")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
