use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct CircuitBoxComponent {
    pub item: ItemComponent,

    pub locked: bool,
}

impl CircuitBoxComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            locked: element
                .attribute_ignore_ascii_case("locked")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
        }
    }
}
