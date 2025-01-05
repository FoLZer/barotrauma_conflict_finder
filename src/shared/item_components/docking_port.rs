use std::str::FromStr;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::item_prefab::DoesNotExistError, submarine_info::Vector2, util::NodeExp,
};

use super::ItemComponent;

#[derive(Debug)]
pub struct DockingPortComponent {
    pub item: ItemComponent,

    pub distance_tolerance: Vec2,
    pub docked_distance: f32,
    pub is_horizontal: bool,
    pub main_docking_port: bool,
    pub apply_effects_on_docking: bool,
    pub force_docking_direction: Option<DirectionType>,
}

impl DockingPortComponent {
    pub fn from_xml(element: &Node) -> Self {
        //TODO: sprite
        Self {
            item: ItemComponent::from_xml(element),

            distance_tolerance: element
                .attribute_ignore_ascii_case("distancetolerance")
                .map_or(Vec2 { x: 32.0, y: 32.0 }, |v| {
                    v.parse::<Vector2>().unwrap().0
                }),
            docked_distance: element
                .attribute_ignore_ascii_case("dockeddistance")
                .map_or(32.0, |v| v.parse().unwrap()),
            is_horizontal: element
                .attribute_ignore_ascii_case("ishorizontal")
                .map_or(true, |v| v.parse().unwrap()),
            main_docking_port: element
                .attribute_ignore_ascii_case("maindockingport")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            apply_effects_on_docking: element
                .attribute_ignore_ascii_case("applyeffectsondocking")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            force_docking_direction: element
                .attribute_ignore_ascii_case("forcedockingdirection")
                .filter(|v| !v.eq_ignore_ascii_case("none")) //use Option instead of including None in DirectionType
                .map(|v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub enum DirectionType {
    Top,
    Bottom,
    Left,
    Right,
}

impl FromStr for DirectionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
