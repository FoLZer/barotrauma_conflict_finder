use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct FabricatorComponent {
    pub powered: PoweredComponent,

    pub fabrication_speed: f32,
    pub skill_requirement_multiplier: f32,
    pub amount_to_fabricate: u32,
}

impl FabricatorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            fabrication_speed: element
                .attribute_ignore_ascii_case("fabricationspeed")
                .map_or(1.0, |v| v.parse().unwrap()),
            skill_requirement_multiplier: element
                .attribute_ignore_ascii_case("skillrequirementmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            amount_to_fabricate: element
                .attribute_ignore_ascii_case("amounttofabricate")
                .map_or(1, |v| v.parse().unwrap()),
        }
    }
}
