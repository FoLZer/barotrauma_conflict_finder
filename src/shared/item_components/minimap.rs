use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct MiniMapComponent {
    pub powered: PoweredComponent,

    pub require_water_detectors: bool,
    pub require_oxygen_detectors: bool,
    pub show_hull_integrity: bool,
    pub enable_hull_status: bool,
    pub enable_electrical_view: bool,
    pub enable_item_finder: bool,
}

impl MiniMapComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            require_water_detectors: element
                .attribute_ignore_ascii_case("requirewaterdetectors")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            require_oxygen_detectors: element
                .attribute_ignore_ascii_case("requireoxygendetectors")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            show_hull_integrity: element
                .attribute_ignore_ascii_case("showhullintegrity")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            enable_hull_status: element
                .attribute_ignore_ascii_case("enablehullstatus")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            enable_electrical_view: element
                .attribute_ignore_ascii_case("enableelectricalview")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            enable_item_finder: element
                .attribute_ignore_ascii_case("enableitemfinder")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
