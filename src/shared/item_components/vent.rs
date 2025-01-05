use roxmltree::Node;

use super::ItemComponent;

#[derive(Debug)]
pub struct VentComponent {
    pub item: ItemComponent,
}

impl VentComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),
        }
    }
}
