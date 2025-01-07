use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct ArithmeticComponent {
    pub item: ItemComponent,

    pub clamp_max: f32,
    pub clamp_min: f32,
    pub time_frame: f32,
}

impl ArithmeticComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            clamp_max: element
                .attribute_ignore_ascii_case("clampmax")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            clamp_min: element
                .attribute_ignore_ascii_case("clampmin")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            time_frame: element
                .attribute_ignore_ascii_case("timeframe")
                .map(|v| v.parse().unwrap())
                .unwrap(),
        }
    }
}
