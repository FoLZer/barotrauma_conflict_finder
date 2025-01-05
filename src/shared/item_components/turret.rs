use glam::Vec2;
use roxmltree::Node;

use crate::shared::{prefabs::item_prefab::Color, submarine_info::Vector2, util::NodeExp};

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct TurretComponent {
    pub powered: PoweredComponent,

    pub barrel_pos: Option<Vec2>,
    pub firing_offset: Option<Vec2>,
    pub alternating_firing_offset: bool,
    pub launch_impulse: f32,
    pub damage_multiplier: f32,
    pub projectile_count: u32,
    pub launch_without_projectile: bool,
    pub spread: f32,
    pub firing_rotation_speed_modifier: f32,
    pub single_charged_shot: bool,
    pub base_rotation: f32,
    pub ai_range: f32,
    pub max_angle_offset: f32,
    pub ai_current_target_priority_multiplier: f32,
    pub max_active_projectiles: Option<u32>,
    pub max_charge_time: Option<f32>,
    pub reload: f32,
    pub shots_per_burst: u32,
    pub delay_between_bursts: f32,
    pub retraction_duration_multiplier: f32,
    pub recoil_time: f32,
    pub retraction_delay: f32,
    pub rotation_limits: Vec2,
    pub spring_stiffness_low_skill: f32,
    pub spring_stiffness_high_skill: f32,
    pub spring_damping_low_skill: f32,
    pub spring_damping_high_skill: f32,
    pub rotation_speed_low_skill: f32,
    pub rotation_speed_high_skill: f32,
    pub hud_tint: Color,
    pub auto_operate: bool,
    pub allow_auto_operate_with_wiring: bool,
    pub random_aim_amount: Option<f32>,
    pub random_aim_min_time: f32,
    pub random_aim_max_time: f32,
    pub random_movement: bool,
    pub aim_delay: bool,
    pub target_characters: bool,
    pub target_monsters: bool,
    pub target_humans: bool,
    pub target_submarines: bool,
    pub target_items: bool,
    pub friendly_tag: Option<String>,
}

impl TurretComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            barrel_pos: element
                .attribute_ignore_ascii_case("barrelpos")
                .map(|v| v.parse::<Vector2>().unwrap().0),
            firing_offset: element
                .attribute_ignore_ascii_case("firingoffset")
                .map(|v| v.parse::<Vector2>().unwrap().0),
            alternating_firing_offset: element
                .attribute_ignore_ascii_case("alternatingfiringoffset")
                .map_or(false, |v| v.parse().unwrap()),
            launch_impulse: element
                .attribute_ignore_ascii_case("launchimpulse")
                .map_or(0.0, |v| v.parse().unwrap()),
            damage_multiplier: element
                .attribute_ignore_ascii_case("damagemultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            projectile_count: element
                .attribute_ignore_ascii_case("projectilecount")
                .map_or(1, |v| v.parse().unwrap()),
            launch_without_projectile: element
                .attribute_ignore_ascii_case("launchwithoutprojectile")
                .map_or(false, |v| v.parse().unwrap()),
            spread: element
                .attribute_ignore_ascii_case("spread")
                .map_or(0.0, |v| v.parse().unwrap()),
            firing_rotation_speed_modifier: element
                .attribute_ignore_ascii_case("firingrotationspeedmodifier")
                .map_or(1.0, |v| v.parse().unwrap()),
            single_charged_shot: element
                .attribute_ignore_ascii_case("singlechargedshot")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            base_rotation: element
                .attribute_ignore_ascii_case("baserotation")
                .map_or(0.0, |v| v.parse().unwrap()),
            ai_range: element
                .attribute_ignore_ascii_case("airange")
                .map_or(3500.0, |v| v.parse().unwrap()),
            max_angle_offset: element
                .attribute_ignore_ascii_case("maxangleoffset")
                .map_or(10.0, |v| v.parse().unwrap()),
            ai_current_target_priority_multiplier: element
                .attribute_ignore_ascii_case("aicurrenttargetprioritymultiplier")
                .map_or(1.1, |v| v.parse().unwrap()),
            max_active_projectiles: element
                .attribute_ignore_ascii_case("maxactiveprojectiles")
                .map(|v| v.parse().unwrap()),
            max_charge_time: element
                .attribute_ignore_ascii_case("maxchargetime")
                .map(|v| v.parse().unwrap()),
            reload: element
                .attribute_ignore_ascii_case("reload")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            shots_per_burst: element
                .attribute_ignore_ascii_case("shotsperburst")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            delay_between_bursts: element
                .attribute_ignore_ascii_case("delaybetweenbursts")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            retraction_duration_multiplier: element
                .attribute_ignore_ascii_case("retractiondurationmultiplier")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            recoil_time: element
                .attribute_ignore_ascii_case("recoiltime")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            retraction_delay: element
                .attribute_ignore_ascii_case("retractiondelay")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            rotation_limits: element
                .attribute_ignore_ascii_case("rotationlimits")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            spring_stiffness_low_skill: element
                .attribute_ignore_ascii_case("springstiffnesslowskill")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            spring_stiffness_high_skill: element
                .attribute_ignore_ascii_case("springstiffnesshighskill")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            spring_damping_low_skill: element
                .attribute_ignore_ascii_case("springdampinglowskill")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            spring_damping_high_skill: element
                .attribute_ignore_ascii_case("springdampinghighskill")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            rotation_speed_low_skill: element
                .attribute_ignore_ascii_case("rotationspeedlowskill")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            rotation_speed_high_skill: element
                .attribute_ignore_ascii_case("rotationspeedhighskill")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            hud_tint: element
                .attribute_ignore_ascii_case("hudtint")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            auto_operate: element
                .attribute_ignore_ascii_case("autooperate")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            allow_auto_operate_with_wiring: element
                .attribute_ignore_ascii_case("allowautooperatewithwiring")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            random_aim_amount: element
                .attribute_ignore_ascii_case("randomaimamount")
                .map(|v| v.parse().unwrap()),
            random_aim_min_time: element
                .attribute_ignore_ascii_case("randomaimmintime")
                .map_or(0.0, |v| v.parse().unwrap()),
            random_aim_max_time: element
                .attribute_ignore_ascii_case("randomaimmaxtime")
                .map_or(0.0, |v| v.parse().unwrap()),
            random_movement: element
                .attribute_ignore_ascii_case("randommovement")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            aim_delay: element
                .attribute_ignore_ascii_case("aimdelay")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            target_characters: element
                .attribute_ignore_ascii_case("targetcharacters")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            target_monsters: element
                .attribute_ignore_ascii_case("targetmonsters")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            target_humans: element
                .attribute_ignore_ascii_case("targethumans")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            target_submarines: element
                .attribute_ignore_ascii_case("targetsubmarines")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            target_items: element
                .attribute_ignore_ascii_case("targetitems")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            friendly_tag: element
                .attribute_ignore_ascii_case("friendlytag")
                .map(|v| v.parse().unwrap()),
        }
    }
}
