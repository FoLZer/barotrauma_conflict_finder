use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct ScannerComponent {
    pub item: ItemComponent,

    pub scan_duration: f32,
    pub scan_timer: f32,
    pub scan_radius: f32,
    pub always_display_progress_bar: bool,
}

impl ScannerComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            scan_duration: element
                .attribute_ignore_ascii_case("scanduration")
                .map_or(1.0, |v| v.parse().unwrap()),
            scan_timer: element
                .attribute_ignore_ascii_case("scantimer")
                .map_or(0.0, |v| v.parse().unwrap()),
            scan_radius: element
                .attribute_ignore_ascii_case("scanradius")
                .map_or(1.0, |v| v.parse().unwrap()),
            always_display_progress_bar: element
                .attribute_ignore_ascii_case("alwaysdisplayprogressbar")
                .map_or(true, |v| v.parse().unwrap()),
        }
    }
}
