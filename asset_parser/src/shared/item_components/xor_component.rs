use roxmltree::Node;

use super::boolean_operator::BooleanOperatorComponent;

#[derive(Debug)]
pub struct XorComponent {
    pub boolean_operator: BooleanOperatorComponent,
}

impl XorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            boolean_operator: BooleanOperatorComponent::from_xml(element),
        }
    }
}
