use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct DeconstructorComponent {
    pub powered: PoweredComponent,

    pub deconstruct_items_simultaneously: bool,
    pub deconstruction_speed: f32,
}

impl DeconstructorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            deconstruct_items_simultaneously: element
                .attribute_ignore_ascii_case("deconstructitemssimultaneously")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            deconstruction_speed: element
                .attribute_ignore_ascii_case("deconstructionspeed")
                .map(|v| v.parse().unwrap())
                .unwrap(),
        }
    }
}
