use std::str::FromStr;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::{
        item_prefab::DoesNotExistError, level_object_prefab::LimbType, structure_prefab::Direction,
    },
    submarine_info::Vector2,
    util::NodeExp,
};

use super::ItemComponent;

#[derive(Debug)]
pub struct ControllerComponent {
    pub item: ItemComponent,

    pub is_toggle: bool,
    pub output: String,
    pub false_output: String,
    pub state: bool,
    pub hide_hud: bool,
    pub usable_in: UseEnvironment,
    pub draw_user_behind: bool,
    pub allow_selecting_when_selected_by_other: bool,
    pub allow_selecting_when_selected_by_bot: bool,
    pub non_interactable_when_flipped_x: bool,
    pub non_interactable_when_flipped_y: bool,
    pub is_secondary_item: bool,

    pub user_pos: Option<Vec2>,
    pub direction: Option<Direction>,
    pub limb_positions: Vec<LimbPosition>,
}

impl ControllerComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            is_toggle: element
                .attribute_ignore_ascii_case("istoggle")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            output: element
                .attribute_ignore_ascii_case("output")
                .map_or("1".to_owned(), |v| v.to_owned()),
            false_output: element
                .attribute_ignore_ascii_case("falseoutput")
                .map_or("0".to_owned(), |v| v.to_owned()),
            state: element
                .attribute_ignore_ascii_case("state")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            hide_hud: element
                .attribute_ignore_ascii_case("hidehud")
                .map_or(true, |v| v.parse().unwrap()),
            usable_in: element
                .attribute_ignore_ascii_case("usablein")
                .map_or(UseEnvironment::Both, |v| v.parse().unwrap()),
            draw_user_behind: element
                .attribute_ignore_ascii_case("drawuserbehind")
                .map_or(false, |v| v.parse().unwrap()),
            allow_selecting_when_selected_by_other: element
                .attribute_ignore_ascii_case("allowselectingwhenselectedbyother")
                .map_or(true, |v| v.parse().unwrap()),
            allow_selecting_when_selected_by_bot: element
                .attribute_ignore_ascii_case("allowselectingwhenselectedbybot")
                .map_or(true, |v| v.parse().unwrap()),
            non_interactable_when_flipped_x: element
                .attribute_ignore_ascii_case("noninteractablewhenflippedx")
                .map_or(false, |v| v.parse().unwrap()),
            non_interactable_when_flipped_y: element
                .attribute_ignore_ascii_case("noninteractablewhenflippedy")
                .map_or(false, |v| v.parse().unwrap()),
            is_secondary_item: element
                .attribute_ignore_ascii_case("issecondaryitem")
                .map_or(false, |v| v.parse().unwrap()),
            user_pos: element
                .attribute_ignore_ascii_case("userpos")
                .map(|v| v.parse::<Vector2>().unwrap().0),
            direction: element
                .attribute_ignore_ascii_case("direction")
                .map(|v| v.parse().unwrap()),
            limb_positions: {
                let mut limb_positions = Vec::new();

                for child in element
                    .children()
                    .filter(Node::is_element)
                    .filter(|v| v.tag_name().name().eq_ignore_ascii_case("limbposition"))
                {
                    let limb_type = child
                        .attribute_ignore_ascii_case("limb")
                        .map(|v| v.parse::<LimbType>().unwrap())
                        .unwrap();
                    let position = child
                        .attribute_ignore_ascii_case("position")
                        .map(|v| v.parse::<Vector2>().unwrap().0)
                        .unwrap();
                    let allow_using_limb = child
                        .attribute_ignore_ascii_case("allowusinglimb")
                        .map(|v| v.parse::<bool>().unwrap())
                        .unwrap();
                    limb_positions.push(LimbPosition {
                        limb_type,
                        position,
                        allow_using_limb,
                    });
                }

                limb_positions
            },
        }
    }
}

#[derive(Debug)]
pub enum UseEnvironment {
    Air,
    Water,
    Both,
}

impl FromStr for UseEnvironment {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "air" => Ok(Self::Air),
            "water" => Ok(Self::Water),
            "both" => Ok(Self::Both),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct LimbPosition {
    pub limb_type: LimbType,
    pub position: Vec2,
    pub allow_using_limb: bool,
}
