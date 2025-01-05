use roxmltree::Node;

use super::ItemComponent;

#[derive(Debug)]
pub struct LadderComponent {
    pub item: ItemComponent,
}

impl LadderComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),
        }
    }
}
