use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct PowerTransfer {
    pub powered: PoweredComponent,

    pub can_be_overloaded: bool,
    pub overload_voltage: f32,
    pub fire_probability: f32,
    pub overload: bool,
}

impl PowerTransfer {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            can_be_overloaded: element
                .attribute_ignore_ascii_case("canbeoverloaded")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            overload_voltage: element
                .attribute_ignore_ascii_case("overloadvoltage")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            fire_probability: element
                .attribute_ignore_ascii_case("fireprobability")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            overload: element
                .attribute_ignore_ascii_case("overload")
                .map_or(false, |v| v.parse().unwrap()),
        }
    }
}
