use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct PowerContainerComponent {
    pub powered: PoweredComponent,

    pub indicator_position: Vec2,
    pub indicator_size: Vec2,
    pub is_horizontal: bool,
    pub max_output: f32,
    pub capacity: f32,
    pub charge: f32,
    pub max_recharge_speed: f32,
    pub recharge_speed: f32,
    pub exponential_recharge_speed: bool,
    pub effeciency: f32,
    pub flip_indicator: bool,
}

impl PowerContainerComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            indicator_position: element
                .attribute_ignore_ascii_case("indicatorposition")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            indicator_size: element
                .attribute_ignore_ascii_case("indicatorsize")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            is_horizontal: element
                .attribute_ignore_ascii_case("ishorizontal")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            max_output: element
                .attribute_ignore_ascii_case("maxoutput")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            capacity: element
                .attribute_ignore_ascii_case("capacity")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            charge: element
                .attribute_ignore_ascii_case("charge")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            max_recharge_speed: element
                .attribute_ignore_ascii_case("maxrechargespeed")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            recharge_speed: element
                .attribute_ignore_ascii_case("rechargespeed")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            exponential_recharge_speed: element
                .attribute_ignore_ascii_case("exponentialrechargespeed")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            effeciency: element
                .attribute_ignore_ascii_case("effeciency")
                .map_or(0.95, |v| v.parse().unwrap()),
            flip_indicator: element
                .attribute_ignore_ascii_case("flipindicator")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
