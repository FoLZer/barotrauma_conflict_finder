use roxmltree::Node;

use super::ItemComponent;

#[derive(Debug)]
pub struct OutpostTerminalComponent {
    pub item: ItemComponent,
}

impl OutpostTerminalComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),
        }
    }
}
