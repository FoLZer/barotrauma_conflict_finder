use roxmltree::Node;

use super::ItemComponent;

#[derive(Debug)]
pub struct OxygenDetectorComponent {
    pub item: ItemComponent,
}

impl OxygenDetectorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),
        }
    }
}
