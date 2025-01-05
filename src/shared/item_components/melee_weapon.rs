use glam::Vec2;
use roxmltree::Node;

use crate::shared::{prefabs::level_object_prefab::Attack, submarine_info::Vector2, util::NodeExp};

use super::holdable::HoldableComponent;

#[derive(Debug)]
pub struct MeleeWeaponComponent {
    pub holdable: HoldableComponent,

    pub range: f32,
    pub reload: f32,
    pub allow_hit_multiple: bool,
    pub hit_only_characters: bool,
    pub swing: bool,
    pub swing_pos: Vec2,
    pub swing_force: Vec2,

    pub require_aim_to_use: bool,
    pub preferred_contained_items: Vec<String>,

    pub attack: Option<Attack>,
}

impl MeleeWeaponComponent {
    pub fn from_xml(element: &Node) -> Self {
        let mut attack = None;
        for child in element.children().filter(Node::is_element) {
            let tag_name = child.tag_name().name();
            match tag_name {
                "attack" => {
                    attack = Some(Attack::new(child));
                }
                //holdable
                "requireditem" => {}
                _ => {
                    panic!("Unexpected tag name in MeleeWeaponComponent: {}", tag_name);
                }
            }
        }

        Self {
            holdable: HoldableComponent::from_xml(element),

            range: element
                .attribute_ignore_ascii_case("range")
                .map_or(0.0, |v| v.parse().unwrap()),
            reload: element
                .attribute_ignore_ascii_case("reload")
                .map_or(0.5, |v| v.parse().unwrap()),
            allow_hit_multiple: element
                .attribute_ignore_ascii_case("allowhitmultiple")
                .map_or(false, |v| v.parse().unwrap()),
            hit_only_characters: element
                .attribute_ignore_ascii_case("hitonlycharacters")
                .map_or(false, |v| v.parse().unwrap()),
            swing: element
                .attribute_ignore_ascii_case("swing")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            swing_pos: element
                .attribute_ignore_ascii_case("swingpos")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            swing_force: element
                .attribute_ignore_ascii_case("swingforce")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),

            require_aim_to_use: element
                .parent()
                .unwrap()
                .attribute_ignore_ascii_case("requireaimtouse")
                .map_or(true, |v| v.parse().unwrap()),
            preferred_contained_items: element
                .attribute_ignore_ascii_case("preferredcontaineditems")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap_or_default(),

            attack,
        }
    }
}
