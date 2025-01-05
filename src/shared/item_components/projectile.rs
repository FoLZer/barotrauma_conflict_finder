use roxmltree::Node;

use crate::shared::{prefabs::level_object_prefab::Attack, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct ProjectileComponent {
    pub item: ItemComponent,

    pub launch_impulse: f32,
    pub impulse_spread: f32,
    pub launch_rotation: f32,
    pub does_stick: bool,
    pub stick_to_characters: bool,
    pub stick_to_structures: bool,
    pub stick_to_items: bool,
    pub stick_to_doors: bool,
    pub stick_to_deflective: bool,
    pub stick_to_light_targets: bool,
    pub hitscan: bool,
    pub hitscan_count: u32,
    pub max_targets_to_hit: u32,
    pub remove_on_hit: bool,
    pub spread: f32,
    pub static_spread: bool,
    pub friendly_fire: bool,
    pub deactivation_time: f32,
    pub stick_duration: f32,
    pub max_joint_translation: Option<f32>,
    pub prismatic: bool,
    pub ignore_projectiles_while_active: bool,
    pub damage_doors: bool,
    pub damage_user: bool,

    pub attack: Option<Attack>,
}

impl ProjectileComponent {
    pub fn from_xml(element: &Node) -> Self {
        let mut attack = None;
        for child in element.children().filter(Node::is_element) {
            let tag_name = child.tag_name().name();
            match tag_name {
                "attack" => {
                    attack = Some(Attack::new(child));
                }
                _ => {
                    panic!("Unexpected tag name in ProjectileComponent: {}", tag_name);
                }
            }
        }

        Self {
            item: ItemComponent::from_xml(element),

            launch_impulse: element
                .attribute_ignore_ascii_case("launchimpulse")
                .map_or(10.0, |v| v.parse().unwrap()),
            impulse_spread: element
                .attribute_ignore_ascii_case("impulsespread")
                .map_or(0.0, |v| v.parse().unwrap()),
            launch_rotation: element
                .attribute_ignore_ascii_case("launchrotation")
                .map_or(0.0, |v| v.parse().unwrap()),
            does_stick: element
                .attribute_ignore_ascii_case("doesstick")
                .map_or(false, |v| v.parse().unwrap()),
            stick_to_characters: element
                .attribute_ignore_ascii_case("sticktocharacters")
                .map_or(false, |v| v.parse().unwrap()),
            stick_to_structures: element
                .attribute_ignore_ascii_case("sticktostructures")
                .map_or(false, |v| v.parse().unwrap()),
            stick_to_items: element
                .attribute_ignore_ascii_case("sticktoitems")
                .map_or(false, |v| v.parse().unwrap()),
            stick_to_doors: element
                .attribute_ignore_ascii_case("sticktodoors")
                .map_or(false, |v| v.parse().unwrap()),
            stick_to_deflective: element
                .attribute_ignore_ascii_case("sticktodeflective")
                .map_or(false, |v| v.parse().unwrap()),
            stick_to_light_targets: element
                .attribute_ignore_ascii_case("sticktolighttargets")
                .map_or(false, |v| v.parse().unwrap()),
            hitscan: element
                .attribute_ignore_ascii_case("hitscan")
                .map_or(false, |v| v.parse().unwrap()),
            hitscan_count: element
                .attribute_ignore_ascii_case("hitscancount")
                .map_or(1, |v| v.parse().unwrap()),
            max_targets_to_hit: element
                .attribute_ignore_ascii_case("maxtargetstohit")
                .map_or(1, |v| v.parse().unwrap()),
            remove_on_hit: element
                .attribute_ignore_ascii_case("removeonhit")
                .map_or(false, |v| v.parse().unwrap()),
            spread: element
                .attribute_ignore_ascii_case("spread")
                .map_or(0.0, |v| v.parse().unwrap()),
            static_spread: element
                .attribute_ignore_ascii_case("staticspread")
                .map_or(false, |v| v.parse().unwrap()),
            friendly_fire: element
                .attribute_ignore_ascii_case("friendlyfire")
                .map_or(false, |v| v.parse().unwrap()),
            deactivation_time: element
                .attribute_ignore_ascii_case("deactivationtime")
                .map_or(0.0, |v| v.parse().unwrap()),
            stick_duration: element
                .attribute_ignore_ascii_case("stickduration")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_joint_translation: element
                .attribute_ignore_ascii_case("maxjointtranslation")
                .map(|v| v.parse().unwrap()),
            prismatic: element
                .attribute_ignore_ascii_case("prismatic")
                .map_or(true, |v| v.parse().unwrap()),
            ignore_projectiles_while_active: element
                .attribute_ignore_ascii_case("ignoreprojectileswhileactive")
                .map_or(false, |v| v.parse().unwrap()),
            damage_doors: element
                .attribute_ignore_ascii_case("damagedoors")
                .map_or(false, |v| v.parse().unwrap()),
            damage_user: element
                .attribute_ignore_ascii_case("damageuser")
                .map_or(false, |v| v.parse().unwrap()),
            attack,
        }
    }
}
