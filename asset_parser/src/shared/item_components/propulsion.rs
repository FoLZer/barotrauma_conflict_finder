use std::str::FromStr;

use roxmltree::Node;

use crate::shared::{prefabs::item_prefab::DoesNotExistError, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct PropulsionComponent {
    pub item: ItemComponent,

    pub usable_in: UseEnvironment,
    pub force: f32,
    pub apply_to_hands: bool,
    pub particles: Option<String>,
}

impl PropulsionComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            usable_in: element
                .attribute_ignore_ascii_case("usablein")
                .map_or(UseEnvironment::Both, |v| v.parse().unwrap()),
            force: element
                .attribute_ignore_ascii_case("force")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            apply_to_hands: element
                .attribute_ignore_ascii_case("applytohands")
                .map_or(true, |v| v.parse().unwrap()),
            particles: element
                .attribute_ignore_ascii_case("particles")
                .map(|v| v.parse().unwrap()),
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
