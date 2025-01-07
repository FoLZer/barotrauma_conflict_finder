use roxmltree::Node;

use crate::shared::{prefabs::item_assembly_prefab::Rect, util::NodeExp};

use super::pickable::PickableComponent;

#[derive(Debug)]
pub struct DoorComponent {
    pub pickable: PickableComponent,

    pub is_stuck: bool,
    pub stuck: f32,
    pub opening_speed: f32,
    pub closing_speed: f32,
    pub toggle_cooldown: f32,
    pub window: Option<Rect>,
    pub is_open: bool,
    pub has_integrated_buttons: bool,
    pub toggle_when_clicked: bool,
    pub impassable: bool,
    pub use_between_output_modules: bool,
    pub bots_should_keep_open: bool,
}

impl DoorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            pickable: PickableComponent::from_xml(element),

            is_stuck: element
                .attribute_ignore_ascii_case("isstuck")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            stuck: element
                .attribute_ignore_ascii_case("stuck")
                .map_or(0.0, |v| v.parse().unwrap()),
            opening_speed: element
                .attribute_ignore_ascii_case("openingspeed")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            closing_speed: element
                .attribute_ignore_ascii_case("closingspeed")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            toggle_cooldown: element
                .attribute_ignore_ascii_case("togglecooldown")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            window: element
                .attribute_ignore_ascii_case("window")
                .map(|v| Rect::from_str(v, false).unwrap()),
            is_open: element
                .attribute_ignore_ascii_case("isopen")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            has_integrated_buttons: element
                .attribute_ignore_ascii_case("hasintegratedbuttons")
                .map_or(false, |v| v.parse().unwrap()),
            toggle_when_clicked: element
                .attribute_ignore_ascii_case("togglewhenclicked")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            impassable: element
                .attribute_ignore_ascii_case("impassable")
                .map_or(false, |v| v.parse().unwrap()),
            use_between_output_modules: element
                .attribute_ignore_ascii_case("usebetweenoutputmodules")
                .map_or(true, |v| v.parse().unwrap()),
            bots_should_keep_open: element
                .attribute_ignore_ascii_case("botsshouldkeepopen")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
        }
    }
}
