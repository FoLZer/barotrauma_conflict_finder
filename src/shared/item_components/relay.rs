use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::power_transfer::PowerTransfer;

#[derive(Debug)]
pub struct RelayComponent {
    pub power_transfer: PowerTransfer,

    pub max_power: f32,
    pub is_on: bool,
}

impl RelayComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            power_transfer: PowerTransfer::from_xml(element),

            max_power: element
                .attribute_ignore_ascii_case("maxpower")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            is_on: element
                .attribute_ignore_ascii_case("ison")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
