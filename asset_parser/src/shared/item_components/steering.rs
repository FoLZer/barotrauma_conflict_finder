use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct SteeringComponent {
    pub powered: PoweredComponent,

    pub neutral_ballast_level: f32,
    pub docking_assist_threshold: f32,
}

impl SteeringComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            neutral_ballast_level: element
                .attribute_ignore_ascii_case("neutralballastlevel")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            docking_assist_threshold: element
                .attribute_ignore_ascii_case("dockingassistthreshold")
                .map_or(1000.0, |v| v.parse().unwrap()),
        }
    }
}
