use std::str::FromStr;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::item_prefab::DoesNotExistError, submarine_info::Vector2, util::NodeExp,
};

use super::ItemComponent;

#[derive(Debug)]
pub struct RepairToolComponent {
    pub item: ItemComponent,

    pub usable_in: UseEnvironment,
    pub range: f32,
    pub spread: f32,
    pub unskilled_spread: f32,
    pub structure_fix_amount: f32,
    pub fire_damage: Option<f32>,
    pub level_wall_fix_amount: f32,
    pub extinguish_amount: Option<f32>,
    pub water_amount: Option<f32>,
    pub barrel_pos: Vec2,
    pub repair_through_walls: bool,
    pub repair_multiple: bool,
    pub repair_multiple_walls: bool,
    pub repair_through_holes: bool,
    pub max_overlapping_wall_dist: f32,
    pub deattach_speed: f32,
    pub hit_items: bool,
    pub hit_broken_doors: bool,
    pub ignore_characters: bool,
    pub fire_probability: f32,
    pub target_force: Option<f32>,
    pub barrel_rotation: f32,

    pub require_aim_to_use: bool,

    pub fixable_entities: Vec<String>,
    pub non_fixable_entities: Vec<String>,
}

impl RepairToolComponent {
    pub fn from_xml(element: &Node) -> Self {
        let mut fixable_entities = Vec::new();
        let mut non_fixable_entities = Vec::new();

        for child in element.children().filter(Node::is_element) {
            let tag_name = child.tag_name().name();
            match tag_name {
                "fixable" => {
                    fixable_entities.extend(
                        child
                            .attribute_ignore_ascii_case("identifier")
                            .map(|v| v.split(',').map(|v| v.to_owned()))
                            .unwrap(),
                    );
                }
                "nonfixable" => {
                    non_fixable_entities.extend(
                        child
                            .attribute_ignore_ascii_case("identifier")
                            .map(|v| v.split(',').map(|v| v.to_owned()))
                            .unwrap(),
                    );
                }
                "requireditem" => (), //handled in ItemComponent
                _ => {
                    panic!("Unexpected tag name in RepairToolComponent: {}", tag_name);
                }
            }
        }

        Self {
            item: ItemComponent::from_xml(element),

            usable_in: element
                .attribute_ignore_ascii_case("usablein")
                .map_or(UseEnvironment::Both, |v| v.parse().unwrap()),
            range: element
                .attribute_ignore_ascii_case("range")
                .map_or(0.0, |v| v.parse().unwrap()),
            spread: element
                .attribute_ignore_ascii_case("spread")
                .map_or(0.0, |v| v.parse().unwrap()),
            unskilled_spread: element
                .attribute_ignore_ascii_case("unskilledspread")
                .map_or(0.0, |v| v.parse().unwrap()),
            structure_fix_amount: element
                .attribute_ignore_ascii_case("structurefixamount")
                .map_or(0.0, |v| v.parse().unwrap()),
            fire_damage: element
                .attribute_ignore_ascii_case("firedamage")
                .map(|v| v.parse().unwrap()),
            level_wall_fix_amount: element
                .attribute_ignore_ascii_case("levelwallfixamount")
                .map_or(0.0, |v| v.parse().unwrap()),
            extinguish_amount: element
                .attribute_ignore_ascii_case("extinguishamount")
                .map(|v| v.parse().unwrap()),
            water_amount: element
                .attribute_ignore_ascii_case("wateramount")
                .map(|v| v.parse().unwrap()),
            barrel_pos: element
                .attribute_ignore_ascii_case("barrelpos")
                .map_or(Vec2 { x: 0.0, y: 0.0 }, |v| v.parse::<Vector2>().unwrap().0),
            repair_through_walls: element
                .attribute_ignore_ascii_case("repairthroughwalls")
                .map_or(false, |v| v.parse().unwrap()),
            repair_multiple: element
                .attribute_ignore_ascii_case("repairmultiple")
                .map_or(false, |v| v.parse().unwrap()),
            repair_multiple_walls: element
                .attribute_ignore_ascii_case("repairmultiplewalls")
                .map_or(true, |v| v.parse().unwrap()),
            repair_through_holes: element
                .attribute_ignore_ascii_case("repairthroughholes")
                .map_or(false, |v| v.parse().unwrap()),
            max_overlapping_wall_dist: element
                .attribute_ignore_ascii_case("max_overlappingwalldist")
                .map_or(100.0, |v| v.parse().unwrap()),
            deattach_speed: element
                .attribute_ignore_ascii_case("deattachspeed")
                .map_or(1.0, |v| v.parse().unwrap()),
            hit_items: element
                .attribute_ignore_ascii_case("hititems")
                .map_or(true, |v| v.parse().unwrap()),
            hit_broken_doors: element
                .attribute_ignore_ascii_case("hitbrokendoors")
                .map_or(false, |v| v.parse().unwrap()),
            ignore_characters: element
                .attribute_ignore_ascii_case("ignorecharacters")
                .map_or(false, |v| v.parse().unwrap()),
            fire_probability: element
                .attribute_ignore_ascii_case("fireprobability")
                .map_or(0.0, |v| v.parse().unwrap()),
            target_force: element
                .attribute_ignore_ascii_case("targetforce")
                .map(|v| v.parse().unwrap()),
            barrel_rotation: element
                .attribute_ignore_ascii_case("barrelrotation")
                .map(|v| v.parse().unwrap())
                .unwrap(),

            require_aim_to_use: element
                .parent()
                .unwrap()
                .attribute_ignore_ascii_case("requireaimtouse")
                .map_or(true, |v| v.parse().unwrap()),

            fixable_entities,
            non_fixable_entities,
        }
    }
}

#[derive(Debug)]
pub enum UseEnvironment {
    Air,
    Water,
    Both,
    None,
}

impl FromStr for UseEnvironment {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "air" => Ok(Self::Air),
            "water" => Ok(Self::Water),
            "both" => Ok(Self::Both),
            "none" => Ok(Self::None),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
