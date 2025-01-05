use std::collections::HashMap;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{prefabs::human_prefab::StatType, submarine_info::Vector2, util::NodeExp};

use super::pickable::PickableComponent;

#[derive(Debug)]
pub struct HoldableComponent {
    pub pickable: PickableComponent,

    pub can_push: bool,
    pub attached: bool,
    pub aimable: bool,
    pub control_pose: bool,
    pub use_hand_rotation_for_hold_angle: bool,
    pub attachable: bool,
    pub reattachable: bool,
    pub limited_attachable: bool,
    pub attached_by_default: bool,
    pub hold_pos: Vec2,
    pub aim_pos: Vec2,
    pub hold_angle: f32,
    pub aim_angle: f32,
    pub swing_amount: Option<Vec2>,
    pub swing_speed: f32,
    pub swing_when_holding: bool,
    pub swing_when_aiming: bool,
    pub swing_when_using: bool,
    pub disable_head_rotation: bool,
    pub disable_when_ranged_weapon_equipped: bool,
    pub sprite_depth_when_dropped: f32,

    pub blocks_players: bool,
    pub handle_pos: [Vec2; 2],
    pub character_usable: bool,
    pub stat_values: HashMap<StatType, f32>,
}

impl HoldableComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            pickable: PickableComponent::from_xml(element),

            can_push: element
                .attribute_ignore_ascii_case("canpush")
                .map_or(true, |v| v.parse().unwrap()),
            attached: element
                .attribute_ignore_ascii_case("attached")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            aimable: element
                .attribute_ignore_ascii_case("aimable")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            control_pose: element
                .attribute_ignore_ascii_case("controlpose")
                .map_or(false, |v| v.parse().unwrap()),
            use_hand_rotation_for_hold_angle: element
                .attribute_ignore_ascii_case("usehandrotationforholdangle")
                .map_or(false, |v| v.parse().unwrap()),
            attachable: element
                .attribute_ignore_ascii_case("attachable")
                .map_or(false, |v| v.parse().unwrap()),
            reattachable: element
                .attribute_ignore_ascii_case("reattachable")
                .map_or(false, |v| v.parse().unwrap()),
            limited_attachable: element
                .attribute_ignore_ascii_case("limitedattachable")
                .map_or(false, |v| v.parse().unwrap()),
            attached_by_default: element
                .attribute_ignore_ascii_case("attachedbydefault")
                .map_or(false, |v| v.parse().unwrap()),
            hold_pos: element
                .attribute_ignore_ascii_case("holdpos")
                .map_or(Vec2 { x: 0.0, y: 0.0 }, |v| v.parse::<Vector2>().unwrap().0),
            aim_pos: element
                .attribute_ignore_ascii_case("aimpos")
                .map_or(Vec2 { x: 0.0, y: 0.0 }, |v| v.parse::<Vector2>().unwrap().0),
            hold_angle: element
                .attribute_ignore_ascii_case("holdangle")
                .map_or(0.0, |v| v.parse().unwrap()),
            aim_angle: element
                .attribute_ignore_ascii_case("aimangle")
                .map_or(0.0, |v| v.parse().unwrap()),
            swing_amount: element
                .attribute_ignore_ascii_case("swingamount")
                .map(|v| v.parse::<Vector2>().unwrap().0),
            swing_speed: element
                .attribute_ignore_ascii_case("swingspeed")
                .map_or(0.0, |v| v.parse().unwrap()),
            swing_when_holding: element
                .attribute_ignore_ascii_case("swingwhenholding")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            swing_when_aiming: element
                .attribute_ignore_ascii_case("swingwhenaiming")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            swing_when_using: element
                .attribute_ignore_ascii_case("swingwhenusing")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            disable_head_rotation: element
                .attribute_ignore_ascii_case("disableheadrotation")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            disable_when_ranged_weapon_equipped: element
                .attribute_ignore_ascii_case("disablewhenrangedweaponequipped")
                .map_or(false, |v| v.parse().unwrap()),
            sprite_depth_when_dropped: element
                .attribute_ignore_ascii_case("spritedepthwhendropped")
                .map_or(0.55, |v| v.parse().unwrap()),

            blocks_players: element
                .attribute_ignore_ascii_case("blocksplayers")
                .map_or(false, |v| v.parse().unwrap()),
            handle_pos: {
                let handle1 = element
                    .attribute_ignore_ascii_case("handle1")
                    .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0);
                let handle2 = element
                    .attribute_ignore_ascii_case("handle2")
                    .map_or(handle1, |v| v.parse::<Vector2>().unwrap().0);
                [handle1, handle2]
            },
            character_usable: element
                .attribute_ignore_ascii_case("blocksplayers")
                .map_or(true, |v| v.parse().unwrap()),
            stat_values: {
                let mut stat_values = HashMap::new();

                for child in element
                    .children()
                    .filter(Node::is_element)
                    .filter(|v| v.tag_name().name().eq_ignore_ascii_case("statvalue"))
                {
                    let stat_type = child
                        .attribute_ignore_ascii_case("stattype")
                        .map(|v| v.parse().unwrap())
                        .unwrap();
                    let stat_value = child
                        .attribute_ignore_ascii_case("value")
                        .map(|v| v.parse().unwrap())
                        .unwrap();
                    match stat_values.entry(stat_type) {
                        std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                            *occupied_entry.get_mut() += stat_value;
                        }
                        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(stat_value);
                        }
                    }
                }

                stat_values
            },
        }
    }
}
