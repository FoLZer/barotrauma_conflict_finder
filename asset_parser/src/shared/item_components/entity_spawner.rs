use std::str::FromStr;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::item_prefab::DoesNotExistError, submarine_info::Vector2, util::NodeExp,
};

use super::ItemComponent;

#[derive(Debug)]
pub struct EntitySpawnerComponent {
    pub item: ItemComponent,

    pub item_identifier: Option<String>,
    pub species_name: Option<String>,
    pub only_spawn_when_crew_in_range: bool,
    pub crew_area_shape: AreaShape,
    pub crew_area_bounds: Vec2,
    pub crew_area_radius: f32,
    pub crew_area_offset: Vec2,
    pub spawn_area_shape: AreaShape,
    pub spawn_area_offset: Vec2,
    pub spawn_timer_range: Vec2,
    pub spawn_amount_range: Vec2,
    pub maximum_amount: u32,
    pub maximum_amount_in_area: u32,
    pub maximum_amount_range_padding: f32,
    pub can_spawn: bool,
    pub preload_character: bool,
}

impl EntitySpawnerComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            item_identifier: element
                .attribute_ignore_ascii_case("itemidentifier")
                .map(|v| v.to_owned()),
            species_name: element
                .attribute_ignore_ascii_case("speciesname")
                .map(|v| v.to_owned()),
            only_spawn_when_crew_in_range: element
                .attribute_ignore_ascii_case("onlyspawnwhencrewinrange")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            crew_area_shape: element
                .attribute_ignore_ascii_case("crewareashape")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            crew_area_bounds: element
                .attribute_ignore_ascii_case("crewareabounds")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            crew_area_radius: element
                .attribute_ignore_ascii_case("crewarearadius")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            crew_area_offset: element
                .attribute_ignore_ascii_case("crewareaoffset")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            spawn_area_shape: element
                .attribute_ignore_ascii_case("spawnareashape")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            spawn_area_offset: element
                .attribute_ignore_ascii_case("spawnareaoffset")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            spawn_timer_range: element
                .attribute_ignore_ascii_case("spawntimerrange")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            spawn_amount_range: element
                .attribute_ignore_ascii_case("spawnamountrange")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            maximum_amount: element
                .attribute_ignore_ascii_case("maximumamount")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            maximum_amount_in_area: element
                .attribute_ignore_ascii_case("maximumamountinarea")
                .map_or(8, |v| v.parse().unwrap()),
            maximum_amount_range_padding: element
                .attribute_ignore_ascii_case("maximumamountrangepadding")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            can_spawn: element
                .attribute_ignore_ascii_case("canspawn")
                .map_or(true, |v| v.parse().unwrap()),
            preload_character: element
                .attribute_ignore_ascii_case("preloadcharacter")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub enum AreaShape {
    Rectangle,
    Circle,
}

impl FromStr for AreaShape {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rectangle" => Ok(Self::Rectangle),
            "circle" => Ok(Self::Circle),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
