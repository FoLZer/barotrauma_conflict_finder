use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct LevelResourceComponent {
    pub item: ItemComponent,

    pub deattach_duration: f32,
    pub deattach_timer: f32,
    pub random_offset_from_wall: f32,
}

impl LevelResourceComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            deattach_duration: element
                .attribute_ignore_ascii_case("deattachduration")
                .map_or(1.0, |v| v.parse().unwrap()),
            deattach_timer: element
                .attribute_ignore_ascii_case("deattachtimer")
                .map_or(0.0, |v| v.parse().unwrap()),
            random_offset_from_wall: element
                .attribute_ignore_ascii_case("randomoffsetfromwall")
                .map_or(1.0, |v| v.parse().unwrap()),
        }
    }
}
