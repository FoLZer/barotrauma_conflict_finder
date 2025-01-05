use roxmltree::Node;

use crate::shared::util::NodeExp;

#[derive(Debug)]
pub struct CPRSettings {
    pub revive_chance_per_skill: f32,
    pub revive_chance_exponent: f32,
    pub revive_chance_min: f32,
    pub revive_chance_max: f32,
    pub stabilization_per_skill: f32,
    pub stabilization_min: f32,
    pub stabilization_max: f32,
    pub damage_skill_threshold: f32,
    pub damage_skill_multiplier: f32,
    pub insufficient_skill_affliction_identifier: String,
}

impl CPRSettings {
    pub fn new(element: Node) -> Self {
        let revive_chance_per_skill = element
            .attribute_ignore_ascii_case("revivechanceperskill")
            .map_or(0.01, |v| v.parse::<f32>().unwrap().max(0.0));
        let revive_chance_exponent = element
            .attribute_ignore_ascii_case("revivechanceexponent")
            .map_or(2.0, |v| v.parse::<f32>().unwrap().max(0.0));
        let revive_chance_min = element
            .attribute_ignore_ascii_case("revivechancemin")
            .map_or(0.05, |v| v.parse::<f32>().unwrap().clamp(0.0, 1.0));
        let revive_chance_max = element
            .attribute_ignore_ascii_case("revivechancemax")
            .map_or(0.9, |v| {
                v.parse::<f32>().unwrap().clamp(revive_chance_min, 1.0)
            });
        let stabilization_per_skill = element
            .attribute_ignore_ascii_case("stabilizationperskill")
            .map_or(0.01, |v| v.parse::<f32>().unwrap().max(0.0));
        let stabilization_min = element
            .attribute_ignore_ascii_case("stabilizationmin")
            .map_or(0.05, |v| v.parse::<f32>().unwrap().max(0.0));
        let stabilization_max = element
            .attribute_ignore_ascii_case("stabilizationmax")
            .map_or(2.0, |v| v.parse::<f32>().unwrap().max(stabilization_min));
        let damage_skill_threshold = element
            .attribute_ignore_ascii_case("damageskillthreshold")
            .map_or(40.0, |v| v.parse::<f32>().unwrap().clamp(0.0, 100.0));
        let damage_skill_multiplier = element
            .attribute_ignore_ascii_case("damageskillmultiplier")
            .map_or(0.1, |v| v.parse::<f32>().unwrap().clamp(0.0, 100.0));
        let insufficient_skill_affliction_identifier = element
            .attribute_ignore_ascii_case("insufficientskillaffliction")
            .map(|v| v.to_owned())
            .unwrap();
        Self {
            revive_chance_per_skill,
            revive_chance_exponent,
            revive_chance_min,
            revive_chance_max,
            stabilization_per_skill,
            stabilization_min,
            stabilization_max,
            damage_skill_threshold,
            damage_skill_multiplier,
            insufficient_skill_affliction_identifier,
        }
    }
}
