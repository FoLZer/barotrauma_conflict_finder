use roxmltree::Node;

use super::equals_component::EqualsComponent;

#[derive(Debug)]
pub struct GreaterComponent {
    pub equals_component: EqualsComponent,
}

impl GreaterComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            equals_component: EqualsComponent::from_xml(element),
        }
    }
}
