use roxmltree::Node;

use crate::shared::{content_files::level_generation_parameters_file::LevelType, util::NodeExp};

use super::item_prefab::{BarotraumaSprite, Color};

#[derive(Debug)]
pub struct FactionPrefab {
    pub identifier: String,
    pub menu_order: u32,
    pub start_outpost: bool,
    pub min_reputation: i32,
    pub max_reputation: i32,
    pub initial_reputation: i32,
    pub controlled_outpost_percentage: f32,
    pub secondary_controlled_outpost_percentage: f32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub opposing_faction: Option<String>,
    pub icon: Option<BarotraumaSprite>,
    pub icon_color: Option<Color>,
    pub icon_small: Option<BarotraumaSprite>,
    pub background_portrait: Option<BarotraumaSprite>,
    pub hireable_characters: Vec<HireableCharacter>,
    pub automatic_missions: Vec<AutomaticMission>,
}
impl FactionPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let menu_order = element
            .attribute_ignore_ascii_case("menuorder")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let start_outpost = element
            .attribute_ignore_ascii_case("startoutpost")
            .map_or(false, |v| v.parse().unwrap());
        let min_reputation = element
            .attribute_ignore_ascii_case("menuorder")
            .map_or(-100, |v| v.parse::<i32>().unwrap());
        let max_reputation = element
            .attribute_ignore_ascii_case("menuorder")
            .map_or(100, |v| v.parse::<i32>().unwrap());
        let initial_reputation = element
            .attribute_ignore_ascii_case("initialreputation")
            .map_or(0, |v| v.parse::<i32>().unwrap());
        let controlled_outpost_percentage = element
            .attribute_ignore_ascii_case("controlledoutpostpercentage")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let secondary_controlled_outpost_percentage = element
            .attribute_ignore_ascii_case("secondarycontrolledoutpostpercentage")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(|v| v.to_owned());
        let description = element
            .attribute_ignore_ascii_case("description")
            .map(|v| v.to_owned());
        let short_description = element
            .attribute_ignore_ascii_case("shortdescription")
            .map(|v| v.to_owned());
        let opposing_faction = element
            .attribute_ignore_ascii_case("opposingfaction")
            .map(|v| v.to_owned());

        let mut icon = None;
        let mut icon_color = None;
        let mut icon_small = None;
        let mut background_portrait = None;
        let mut hireable_characters = Vec::new();
        let mut automatic_missions = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "icon" => {
                    icon = Some(BarotraumaSprite::new(child));
                    icon_color = Some(child.attribute_ignore_ascii_case("color").map_or(
                        Color::Simple {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        },
                        |v| v.parse().unwrap(),
                    ));
                }
                "iconsmall" => {
                    icon_small = Some(BarotraumaSprite::new(child));
                }
                "portrait" => {
                    background_portrait = Some(BarotraumaSprite::new(child));
                }
                "hireable" => {
                    hireable_characters.push(HireableCharacter::new(child));
                }
                "mission" | "automaticmission" => {
                    automatic_missions.push(AutomaticMission::new(child))
                }
                _ => (),
            }
        }

        Self {
            identifier,
            menu_order,
            start_outpost,
            min_reputation,
            max_reputation,
            initial_reputation,
            controlled_outpost_percentage,
            secondary_controlled_outpost_percentage,
            name,
            description,
            short_description,
            opposing_faction,
            icon,
            icon_color,
            icon_small,
            background_portrait,
            hireable_characters,
            automatic_missions,
        }
    }
}

#[derive(Debug)]
pub struct HireableCharacter {
    pub npc_set_identifier: String,
    pub npc_identifier: String,
    pub min_reputation: f32,
}
impl HireableCharacter {
    pub fn new(element: Node) -> Self {
        let npc_set_identifier = element
            .attribute_ignore_ascii_case("from")
            .or(element.attribute_ignore_ascii_case("npcsetidentifier"))
            .map(|v| v.to_owned())
            .unwrap();
        let npc_identifier = element
            .attribute_ignore_ascii_case("identifier")
            .or(element.attribute_ignore_ascii_case("npcidentifier"))
            .map(|v| v.to_owned())
            .unwrap();
        let min_reputation = element
            .attribute_ignore_ascii_case("minreputation")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());

        Self {
            npc_set_identifier,
            npc_identifier,
            min_reputation,
        }
    }
}

#[derive(Debug)]
pub struct AutomaticMission {
    pub mission_tag: String,
    pub level_type: LevelType,
    pub min_reputation: f32,
    pub max_reputation: f32,
    pub min_probability: f32,
    pub max_probability: f32,
    pub max_distance_from_faction_outpost: Option<u32>,
    pub disallow_between_other_faction_outposts: bool,
}
impl AutomaticMission {
    pub fn new(element: Node) -> Self {
        let mission_tag = element
            .attribute_ignore_ascii_case("missiontag")
            .map(|v| v.to_owned())
            .unwrap();
        let level_type = element
            .attribute_ignore_ascii_case("leveltype")
            .map_or(LevelType::LocationConnection, |v| v.parse().unwrap());
        let min_reputation = element
            .attribute_ignore_ascii_case("minreputation")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let max_reputation = element
            .attribute_ignore_ascii_case("maxreputation")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let probability = element
            .attribute_ignore_ascii_case("probability")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let min_probability = element
            .attribute_ignore_ascii_case("minprobability")
            .map_or(probability, |v| v.parse::<f32>().unwrap());
        let max_probability = element
            .attribute_ignore_ascii_case("maxprobability")
            .map_or(probability, |v| v.parse::<f32>().unwrap());
        let max_distance_from_faction_outpost = element
            .attribute_ignore_ascii_case("maxdistancefromfactionoutpost")
            .map(|v| v.parse::<u32>().unwrap());
        let disallow_between_other_faction_outposts = element
            .attribute_ignore_ascii_case("disallowbetweenotherfactionoutposts")
            .map_or(false, |v| v.parse().unwrap());

        Self {
            mission_tag,
            level_type,
            min_reputation,
            max_reputation,
            min_probability,
            max_probability,
            max_distance_from_faction_outpost,
            disallow_between_other_faction_outposts,
        }
    }
}
