use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::{item_prefab::Color, location_type::CharacterTeamType},
    submarine_info::Vector2,
    util::NodeExp,
};

use super::pickable::PickableComponent;

#[derive(Debug)]
pub struct IdCardComponent {
    pub pickable: PickableComponent,

    pub team_id: Option<CharacterTeamType>,
    pub submarine_specific_id: Option<u32>,
    pub owner_tags: Option<String>,
    pub description: Option<String>,
    pub owner_name: Option<String>,
    pub owner_name_localized: Option<String>,
    pub owner_job_id: String,
    pub owner_hair_index: Option<u32>,
    pub owner_beard_index: Option<u32>,
    pub owner_moustache_index: Option<u32>,
    pub owner_face_attachment_index: Option<u32>,
    pub owner_hair_color: Color,
    pub owner_facial_hair_color: Color,
    pub owner_skin_color: Color,
    pub owner_sheet_index: Vec2,
}

impl IdCardComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            pickable: PickableComponent::from_xml(element),

            team_id: element
                .attribute_ignore_ascii_case("teamid")
                .map(|v| v.parse().unwrap()),
            submarine_specific_id: element
                .attribute_ignore_ascii_case("submarinespecificid")
                .map(|v| v.parse().unwrap()),
            owner_tags: element
                .attribute_ignore_ascii_case("ownertags")
                .map(|v| v.parse().unwrap()),
            description: element
                .attribute_ignore_ascii_case("description")
                .map(|v| v.parse().unwrap()),
            owner_name: element
                .attribute_ignore_ascii_case("ownername")
                .map(|v| v.to_owned()),
            owner_name_localized: element
                .attribute_ignore_ascii_case("ownernamelocalized")
                .map(|v| v.to_owned()),
            owner_job_id: element
                .attribute_ignore_ascii_case("ownerjobid")
                .map(|v| v.to_owned())
                .unwrap(),
            owner_hair_index: element
                .attribute_ignore_ascii_case("ownerhairindex")
                .map(|v| v.parse().unwrap()),
            owner_beard_index: element
                .attribute_ignore_ascii_case("ownerbeardindex")
                .map(|v| v.parse().unwrap()),
            owner_moustache_index: element
                .attribute_ignore_ascii_case("ownermoustacheindex")
                .map(|v| v.parse().unwrap()),
            owner_face_attachment_index: element
                .attribute_ignore_ascii_case("ownerfaceattachmentindex")
                .map(|v| v.parse().unwrap()),
            owner_hair_color: element
                .attribute_ignore_ascii_case("ownerhaircolor")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            owner_facial_hair_color: element
                .attribute_ignore_ascii_case("ownerfacialhaircolor")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            owner_skin_color: element
                .attribute_ignore_ascii_case("ownerskincolor")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            owner_sheet_index: element
                .attribute_ignore_ascii_case("ownersheetindex")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
        }
    }
}
