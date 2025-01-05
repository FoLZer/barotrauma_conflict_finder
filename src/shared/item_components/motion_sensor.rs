use std::str::FromStr;

use bitfield_struct::bitfield;
use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::item_prefab::DoesNotExistError, submarine_info::Vector2, util::NodeExp,
};

use super::ItemComponent;

#[derive(Debug)]
pub struct MotionSensorComponent {
    pub item: ItemComponent,

    pub motion_detected: bool,
    pub target: TargetType,
    pub ignore_dead: bool,
    pub range_x: f32,
    pub range_y: f32,
    pub detect_offset: Vec2,
    pub max_output_length: u32,
    pub output: String,
    pub false_output: String,
    pub minimum_velocity: f32,
    pub detect_own_motion: bool,
}

impl MotionSensorComponent {
    pub fn from_xml(element: &Node) -> Self {
        let (range_x, range_y) = if let Some(range) = element
            .attribute_ignore_ascii_case("range")
            .map(|v| v.parse::<f32>().unwrap())
        {
            (range, range)
        } else {
            (
                element
                    .attribute_ignore_ascii_case("rangex")
                    .map(|v| v.parse::<f32>().unwrap())
                    .unwrap(),
                element
                    .attribute_ignore_ascii_case("rangey")
                    .map(|v| v.parse::<f32>().unwrap())
                    .unwrap(),
            )
        };
        Self {
            item: ItemComponent::from_xml(element),

            motion_detected: element
                .attribute_ignore_ascii_case("motiondetected")
                .map_or(false, |v| v.parse().unwrap()),
            target: element
                .attribute_ignore_ascii_case("target")
                .map(|v| {
                    v.split(',')
                        .map(|v| v.parse::<TargetType>().unwrap())
                        .reduce(|acc, v| TargetType::from_bits(acc.into_bits() | v.into_bits()))
                        .unwrap()
                })
                .unwrap(),
            ignore_dead: element
                .attribute_ignore_ascii_case("ignoredead")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            range_x,
            range_y,
            detect_offset: element
                .attribute_ignore_ascii_case("detectoffset")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            max_output_length: element
                .attribute_ignore_ascii_case("maxoutputlength")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            output: element
                .attribute_ignore_ascii_case("output")
                .map(|v| v.to_owned())
                .unwrap(),
            false_output: element
                .attribute_ignore_ascii_case("falseoutput")
                .map(|v| v.to_owned())
                .unwrap(),
            minimum_velocity: element
                .attribute_ignore_ascii_case("minimumvelocity")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            detect_own_motion: element
                .attribute_ignore_ascii_case("detectownmotion")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
        }
    }
}

#[bitfield(u8)]

pub struct TargetType {
    pub human: bool,
    pub monster: bool,
    pub wall: bool,
    pub pet: bool,
    #[bits(4)]
    _unused: u8,
}

impl FromStr for TargetType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "human" => Ok(Self::new().with_human(true)),
            "monster" => Ok(Self::new().with_monster(true)),
            "wall" => Ok(Self::new().with_wall(true)),
            "pet" => Ok(Self::new().with_pet(true)),
            "any" => Ok(Self::new()
                .with_human(true)
                .with_monster(true)
                .with_wall(true)
                .with_pet(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
