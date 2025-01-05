use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct NotComponent {
    pub item: ItemComponent,

    pub continuous_output: bool,
}

impl NotComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            continuous_output: element
                .attribute_ignore_ascii_case("continuousoutput")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
