use roxmltree::Node;

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct SonarTransducerComponent {
    pub powered: PoweredComponent,
}

impl SonarTransducerComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),
        }
    }
}
