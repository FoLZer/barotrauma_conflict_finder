use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct PoweredComponent {
    pub item: ItemComponent,

    pub min_voltage: f32,
    pub power_consumption: f32,
    pub is_active: bool,
    pub curr_power_consumption: f32,
    pub voltage: f32,
    pub vulnerable_to_emp: bool,
}

impl PoweredComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            min_voltage: element
                .attribute_ignore_ascii_case("minvoltage")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            power_consumption: element
                .attribute_ignore_ascii_case("powerconsumption")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            is_active: element
                .attribute_ignore_ascii_case("isactive")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            curr_power_consumption: element
                .attribute_ignore_ascii_case("currpowerconsumption")
                .map_or(0.0, |v| v.parse().unwrap()),
            voltage: element
                .attribute_ignore_ascii_case("voltage")
                .map_or(0.0, |v| v.parse().unwrap()),
            vulnerable_to_emp: element
                .attribute_ignore_ascii_case("vulnerabletoemp")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
