use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct RangedWeaponComponent {
    pub item: ItemComponent,

    pub barrel_pos: Vec2,
    pub reload: f32,
    pub reload_skill_requirement: f32,
    pub reload_no_skill: f32,
    pub hold_trigger: bool,
    pub projectile_count: u32,
    pub spread: f32,
    pub unskilled_spread: f32,
    pub launch_impulse: f32,
    pub penetration: f32,
    pub weapon_damage_modifier: f32,
    pub max_charge_time: f32,
    pub dual_wield_reload_time_penalty_multiplier: f32,
    pub dual_wield_accuracy_penalty: f32,
    pub suitable_projectiles: Vec<String>,

    pub require_aim_to_use: bool,
}

impl RangedWeaponComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            barrel_pos: element
                .attribute_ignore_ascii_case("barrelpos")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            reload: element
                .attribute_ignore_ascii_case("reload")
                .map_or(1.0, |v| v.parse().unwrap()),
            reload_skill_requirement: element
                .attribute_ignore_ascii_case("reloadskillrequirement")
                .map_or(0.0, |v| v.parse().unwrap()),
            reload_no_skill: element
                .attribute_ignore_ascii_case("reloadnoskill")
                .map_or(1.0, |v| v.parse().unwrap()),
            hold_trigger: element
                .attribute_ignore_ascii_case("holdtrigger")
                .map_or(false, |v| v.parse().unwrap()),
            projectile_count: element
                .attribute_ignore_ascii_case("projectilecount")
                .map_or(1, |v| v.parse().unwrap()),
            spread: element
                .attribute_ignore_ascii_case("spread")
                .map_or(0.0, |v| v.parse().unwrap()),
            unskilled_spread: element
                .attribute_ignore_ascii_case("unskilledspread")
                .map_or(0.0, |v| v.parse().unwrap()),
            launch_impulse: element
                .attribute_ignore_ascii_case("launchimpulse")
                .map_or(0.0, |v| v.parse().unwrap()),
            penetration: element
                .attribute_ignore_ascii_case("penetration")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            weapon_damage_modifier: element
                .attribute_ignore_ascii_case("weapondamagemodifier")
                .map_or(1.0, |v| v.parse().unwrap()),
            max_charge_time: element
                .attribute_ignore_ascii_case("maxchargetime")
                .map_or(0.0, |v| v.parse().unwrap()),
            dual_wield_reload_time_penalty_multiplier: element
                .attribute_ignore_ascii_case("dualwieldreloadtimepenaltymultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            dual_wield_accuracy_penalty: element
                .attribute_ignore_ascii_case("dualwieldaccuracypenalty")
                .map_or(0.0, |v| v.parse().unwrap()),
            suitable_projectiles: element
                .attribute_ignore_ascii_case("suitableprojectiles")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap_or_default(),

            require_aim_to_use: element
                .parent()
                .unwrap()
                .attribute_ignore_ascii_case("requireaimtouse")
                .map_or(true, |v| v.parse().unwrap()),
        }
    }
}
