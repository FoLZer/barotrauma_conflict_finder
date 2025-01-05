use roxmltree::Node;

use super::arithmetic_component::ArithmeticComponent;

#[derive(Debug)]
pub struct AdderComponent {
    pub arithmetic_component: ArithmeticComponent,
}

impl AdderComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            arithmetic_component: ArithmeticComponent::from_xml(element),
        }
    }
}
