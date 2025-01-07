use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct OxygenGeneratorComponent {
    pub powered: PoweredComponent,

    pub generated_amount: f32,
}

impl OxygenGeneratorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            generated_amount: element
                .attribute_ignore_ascii_case("generatedamount")
                .map(|v| v.parse().unwrap())
                .unwrap(),
        }
    }
}
