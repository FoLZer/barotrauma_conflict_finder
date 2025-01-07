use roxmltree::Node;

use super::arithmetic_component::ArithmeticComponent;

#[derive(Debug)]
pub struct DivideComponent {
    pub arithmetic_component: ArithmeticComponent,
}

impl DivideComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            arithmetic_component: ArithmeticComponent::from_xml(element),
        }
    }
}
