use roxmltree::Node;

use super::boolean_operator::BooleanOperatorComponent;

#[derive(Debug)]
pub struct OrComponent {
    pub boolean_operator: BooleanOperatorComponent,
}

impl OrComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            boolean_operator: BooleanOperatorComponent::from_xml(element),
        }
    }
}
