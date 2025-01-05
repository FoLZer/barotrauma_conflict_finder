use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::level_object_prefab::Attack, submarine_info::Vector2, util::NodeExp,
};

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct EngineComponent {
    pub powered: PoweredComponent,

    pub max_force: f32,
    pub propeller_pos: Vec2,
    pub disable_propeller_damage: bool,

    pub propeller_damage: Option<Attack>,
}

impl EngineComponent {
    pub fn from_xml(element: &Node) -> Self {
        let mut propeller_damage = None;

        for child in element.children().filter(Node::is_element) {
            let tag_name = child.tag_name().name();
            match tag_name {
                "propellerdamage" => {
                    propeller_damage = Some(Attack::new(child));
                }
                _ => {
                    panic!("Unexpected tag name in EngineComponent: {}", tag_name);
                }
            }
        }

        Self {
            powered: PoweredComponent::from_xml(element),

            max_force: element
                .attribute_ignore_ascii_case("maxforce")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            propeller_pos: element
                .attribute_ignore_ascii_case("propellerpos")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            disable_propeller_damage: element
                .attribute_ignore_ascii_case("disablepropellerdamage")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            propeller_damage,
        }
    }
}
