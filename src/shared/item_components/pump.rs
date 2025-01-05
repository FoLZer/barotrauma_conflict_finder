use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct PumpComponent {
    pub powered: PoweredComponent,

    pub flow_percentage: f32,
    pub max_flow: f32,
    pub is_on: bool,
}

impl PumpComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            flow_percentage: element
                .attribute_ignore_ascii_case("flowpercentage")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_flow: element
                .attribute_ignore_ascii_case("maxflow")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            is_on: element
                .attribute_ignore_ascii_case("ison")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
