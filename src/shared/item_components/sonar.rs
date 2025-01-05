use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::powered::PoweredComponent;

const DEFAULT_SONAR_RANGE: f32 = 10000.0;

#[derive(Debug)]
pub struct SonarComponent {
    pub powered: PoweredComponent,

    pub range: f32,
    pub detect_submarine_walls: bool,
    pub use_transducers: bool,
    pub center_on_transducers: bool,
    pub has_mineral_scanner: bool,
    pub use_mineral_scanner: bool,
}

impl SonarComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            range: element
                .attribute_ignore_ascii_case("range")
                .map_or(DEFAULT_SONAR_RANGE, |v| v.parse().unwrap()),
            detect_submarine_walls: element
                .attribute_ignore_ascii_case("detectsubmarinewalls")
                .map_or(false, |v| v.parse().unwrap()),
            use_transducers: element
                .attribute_ignore_ascii_case("usetransducers")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            center_on_transducers: element
                .attribute_ignore_ascii_case("centerontransducers")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            has_mineral_scanner: element
                .attribute_ignore_ascii_case("hasmineralscanner")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            use_mineral_scanner: element
                .attribute_ignore_ascii_case("usemineralscanner")
                .map_or(true, |v| v.parse().unwrap()),
        }
    }
}
