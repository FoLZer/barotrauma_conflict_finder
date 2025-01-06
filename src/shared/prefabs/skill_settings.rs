use roxmltree::Node;

use crate::shared::util::NodeExp;

#[derive(Debug)]
pub struct SkillSettings {
    pub single_round_skill_gain_multiplier: f32,
    pub skill_increase_per_repair: f32,
    pub skill_increase_per_sabotage: f32,
    pub skill_increase_per_cpr_revive: f32,
    pub skill_increase_per_repaired_structure_damage: f32,
    pub skill_increase_per_second_when_steering: f32,
    pub skill_increase_per_fabricator_required_skill: f32,
    pub skill_increase_per_hostile_damage: f32,
    pub skill_increase_per_second_when_operating_turret: f32,
    pub skill_increase_per_friendly_healed: f32,
    pub assistant_skill_increase_multiplier: f32,
    pub maximum_skill_with_talents: f32,
}
impl SkillSettings {
    pub fn new(element: Node) -> Self {
        Self {
            single_round_skill_gain_multiplier: element
                .attribute_ignore_ascii_case("singleroundskillgainmultiplier")
                .map_or(4.0, |v| v.parse().unwrap()),
            skill_increase_per_repair: element
                .attribute_ignore_ascii_case("skillincreaseperrepair")
                .map_or(5.0, |v| v.parse().unwrap()),
            skill_increase_per_sabotage: element
                .attribute_ignore_ascii_case("skillincreasepersabotage")
                .map_or(3.0, |v| v.parse().unwrap()),
            skill_increase_per_cpr_revive: element
                .attribute_ignore_ascii_case("skillincreasepercprrevive")
                .map_or(0.5, |v| v.parse().unwrap()),
            skill_increase_per_repaired_structure_damage: element
                .attribute_ignore_ascii_case("skillincreaseperrepairedstructuredamage")
                .map_or(0.0025, |v| v.parse().unwrap()),
            skill_increase_per_second_when_steering: element
                .attribute_ignore_ascii_case("skillincreasepersecondwhensteering")
                .map_or(0.005, |v| v.parse().unwrap()),
            skill_increase_per_fabricator_required_skill: element
                .attribute_ignore_ascii_case("skillincreaseperfabricatorrequiredskill")
                .map_or(0.5, |v| v.parse().unwrap()),
            skill_increase_per_hostile_damage: element
                .attribute_ignore_ascii_case("skillincreaseperhostiledamage")
                .map_or(0.01, |v| v.parse().unwrap()),
            skill_increase_per_second_when_operating_turret: element
                .attribute_ignore_ascii_case("skillincreasepersecondwhenoperatingturret")
                .map_or(0.001, |v| v.parse().unwrap()),
            skill_increase_per_friendly_healed: element
                .attribute_ignore_ascii_case("skillincreaseperfriendlyhealed")
                .map_or(0.001, |v| v.parse().unwrap()),
            assistant_skill_increase_multiplier: element
                .attribute_ignore_ascii_case("assistantskillincreasemultiplier")
                .map_or(1.1, |v| v.parse().unwrap()),
            maximum_skill_with_talents: element
                .attribute_ignore_ascii_case("maximumskillwithtalents")
                .map_or(200.0, |v| v.parse().unwrap()),
        }
    }

    pub fn get_identifier(&self) -> &str {
        "SkillSettings"
    }
}
