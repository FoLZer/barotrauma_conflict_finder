use roxmltree::Node;

use crate::shared::{prefabs::level_object_prefab::Attack, util::NodeExp};

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct ElectricalDischargerComponent {
    pub powered: PoweredComponent,

    pub range: f32,
    pub range_multiplier_in_walls: f32,
    pub raycast_range: Option<f32>,
    pub duration: f32,
    pub reload: f32,
    pub outdoors_only: bool,
    pub ignore_user: bool,

    pub attack: Option<Attack>,
}

impl ElectricalDischargerComponent {
    pub fn from_xml(element: &Node) -> Self {
        let mut attack = None;
        for child in element.children().filter(Node::is_element) {
            let tag_name = child.tag_name().name();
            match tag_name {
                "attack" => attack = Some(Attack::new(child)),
                _ => {
                    panic!(
                        "Unexpected tag name in ElectricalDischargerComponent: {}",
                        tag_name
                    );
                }
            }
        }

        Self {
            powered: PoweredComponent::from_xml(element),

            range: element
                .attribute_ignore_ascii_case("range")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            range_multiplier_in_walls: element
                .attribute_ignore_ascii_case("rangemultiplierinwalls")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            raycast_range: element
                .attribute_ignore_ascii_case("raycastrange")
                .map(|v| v.parse().unwrap()),
            duration: element
                .attribute_ignore_ascii_case("duration")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            reload: element
                .attribute_ignore_ascii_case("reload")
                .map_or(0.25, |v| v.parse().unwrap()),
            outdoors_only: element
                .attribute_ignore_ascii_case("outdoorsonly")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            ignore_user: element
                .attribute_ignore_ascii_case("ignoreuser")
                .map_or(false, |v| v.parse().unwrap()),
            attack,
        }
    }
}
