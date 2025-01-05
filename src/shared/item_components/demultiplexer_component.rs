use roxmltree::Node;

use super::connection_selector_component::ConnectionSelectorComponent;

#[derive(Debug)]
pub struct DemultiplexerComponent {
    pub connection_selector_component: ConnectionSelectorComponent,
}

impl DemultiplexerComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            connection_selector_component: ConnectionSelectorComponent::from_xml(element),
        }
    }
}
