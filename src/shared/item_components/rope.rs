use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct RopeComponent {
    pub item: ItemComponent,

    pub snap_anim_duration: f32,
    pub projectile_pull_force: f32,
    pub target_pull_force: f32,
    pub source_pull_force: f32,
    pub max_length: f32,
    pub min_pull_distance: f32,
    pub max_angle: f32,
    pub snap_on_collision: bool,
    pub snap_when_not_aimed: bool,
    pub snap_when_weapon_fired_again: bool,
    pub barrel_length_multiplier: f32,
    pub target_min_mass: f32,
    pub lerp_forces: bool,
    pub increase_force_for_escaping_targets: bool,
    //TODO: sprites
}

impl RopeComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            snap_anim_duration: element
                .attribute_ignore_ascii_case("snapanimduration")
                .map_or(1.0, |v| v.parse().unwrap()),
            projectile_pull_force: element
                .attribute_ignore_ascii_case("projectilepullforce")
                .map_or(0.0, |v| v.parse().unwrap()),
            target_pull_force: element
                .attribute_ignore_ascii_case("targetpullforce")
                .map_or(0.0, |v| v.parse().unwrap()),
            source_pull_force: element
                .attribute_ignore_ascii_case("sourcepullforce")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_length: element
                .attribute_ignore_ascii_case("maxlength")
                .map_or(1000.0, |v| v.parse().unwrap()),
            min_pull_distance: element
                .attribute_ignore_ascii_case("minpulldistance")
                .map_or(200.0, |v| v.parse().unwrap()),
            max_angle: element
                .attribute_ignore_ascii_case("maxangle")
                .map_or(360.0, |v| v.parse().unwrap()),
            snap_on_collision: element
                .attribute_ignore_ascii_case("snaponcollision")
                .map_or(true, |v| v.parse().unwrap()),
            snap_when_not_aimed: element
                .attribute_ignore_ascii_case("snapwhennotaimed")
                .map_or(true, |v| v.parse().unwrap()),
            snap_when_weapon_fired_again: element
                .attribute_ignore_ascii_case("snapwhenweaponfiredagain")
                .map_or(true, |v| v.parse().unwrap()),
            barrel_length_multiplier: element
                .attribute_ignore_ascii_case("barrellengthmultiplier")
                .map_or(0.9, |v| v.parse().unwrap()),
            target_min_mass: element
                .attribute_ignore_ascii_case("targetminmass")
                .map_or(30.0, |v| v.parse().unwrap()),
            lerp_forces: element
                .attribute_ignore_ascii_case("lerpforces")
                .map_or(false, |v| v.parse().unwrap()),
            increase_force_for_escaping_targets: element
                .attribute_ignore_ascii_case("increaseforceforescapingtargets")
                .map_or(true, |v| v.parse().unwrap()),
        }
    }
}
