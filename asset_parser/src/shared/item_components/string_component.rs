use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct StringComponent {
    pub item: ItemComponent,

    pub time_frame: Option<f32>,
}

impl StringComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            time_frame: element
                .attribute_ignore_ascii_case("timeframe")
                .map(|v| v.parse().unwrap()),
        }
    }
}
