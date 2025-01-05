use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct RepairableComponent {
    pub item: ItemComponent,

    pub deterioration_speed: f32,
    pub min_deterioration_delay: f32,
    pub max_deterioration_delay: f32,
    pub min_deterioration_condition: f32,
    pub min_sabotage_condition: f32,
    pub sabotage_deterioration_duration: f32,
    pub repair_threshold: f32,
    pub max_stress_deterioration_multiplier: f32,
    pub stress_deterioration_threshold: f32,
    pub stress_deterioration_increase_speed: f32,
    pub stress_deterioration_decrease_speed: f32,
    pub fix_duration_low_skill: f32,
    pub fix_duration_high_skill: f32,
    pub skill_requirement_multiplier: f32,

    pub header: Option<String>,
    pub name: Option<String>,
}

impl RepairableComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            deterioration_speed: element
                .attribute_ignore_ascii_case("deteriorationspeed")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            min_deterioration_delay: element
                .attribute_ignore_ascii_case("mindeteriorationdelay")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            max_deterioration_delay: element
                .attribute_ignore_ascii_case("maxdeteriorationdelay")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            min_deterioration_condition: element
                .attribute_ignore_ascii_case("mindeteriorationcondition")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            min_sabotage_condition: element
                .attribute_ignore_ascii_case("minsabotagecondition")
                .map_or(0.0, |v| v.parse().unwrap()),
            sabotage_deterioration_duration: element
                .attribute_ignore_ascii_case("sabotagedeteriorationduration")
                .map_or(60.0, |v| v.parse().unwrap()),
            repair_threshold: element
                .attribute_ignore_ascii_case("repairthreshold")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            max_stress_deterioration_multiplier: element
                .attribute_ignore_ascii_case("maxstressdeteriorationmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            stress_deterioration_threshold: element
                .attribute_ignore_ascii_case("stressdeteriorationthreshold")
                .map_or(0.5, |v| v.parse().unwrap()),
            stress_deterioration_increase_speed: element
                .attribute_ignore_ascii_case("stressdeteriorationincreasespeed")
                .map_or(0.1, |v| v.parse().unwrap()),
            stress_deterioration_decrease_speed: element
                .attribute_ignore_ascii_case("stressdeteriorationdecreasespeed")
                .map_or(0.1, |v| v.parse().unwrap()),
            fix_duration_low_skill: element
                .attribute_ignore_ascii_case("fixdurationlowskill")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            fix_duration_high_skill: element
                .attribute_ignore_ascii_case("fixdurationhighskill")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            skill_requirement_multiplier: element
                .attribute_ignore_ascii_case("skillrequirementmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            header: element
                .attribute_ignore_ascii_case("header")
                .map(|v| v.to_owned()),
            name: element
                .attribute_ignore_ascii_case("name")
                .map(|v| v.to_owned()),
        }
    }
}
