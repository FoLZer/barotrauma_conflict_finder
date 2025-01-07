use glam::Vec4;
use roxmltree::Node;

use crate::shared::{
    prefabs::item_prefab::{Color, Vector4},
    util::NodeExp,
};

use super::ItemComponent;

#[derive(Debug)]
pub struct ItemLabelComponent {
    pub item: ItemComponent,

    pub padding: Vec4,
    pub text: String,
    pub ignore_localization: bool,
    pub text_color: Color,
    pub text_scale: f32,
    pub scrollable: bool,
    pub scroll_speed: f32,
}

impl ItemLabelComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            padding: element
                .attribute_ignore_ascii_case("padding")
                .map(|v| v.parse::<Vector4>().unwrap().0)
                .unwrap(),
            text: element
                .attribute_ignore_ascii_case("text")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            ignore_localization: element
                .attribute_ignore_ascii_case("ignorelocalization")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            text_color: element
                .attribute_ignore_ascii_case("textcolor")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            text_scale: element
                .attribute_ignore_ascii_case("textscale")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            scrollable: element
                .attribute_ignore_ascii_case("scrollable")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            scroll_speed: element
                .attribute_ignore_ascii_case("scrollspeed")
                .map_or(20.0, |v| v.parse().unwrap()),
        }
    }
}
