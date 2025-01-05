use std::str::FromStr;

use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::item_prefab::{BarotraumaSprite, DoesNotExistError};

#[derive(Debug)]
pub struct TutorialPrefab {
    pub identifier: String,
    pub order: Option<u32>,
    pub disable_bot_conversations: bool,
    pub allow_character_switch: bool,
    pub submarine_path: String,
    pub outpost_path: String,
    pub level_seed: String,
    pub level_params: String,
    pub tutorial_character: Option<TutorialCharacter>,
    pub banner: Option<BarotraumaSprite>,
    pub event_identifier: String,
    pub end_type: EndType,
    pub next_tutorial_identifier: Option<String>,
}
impl TutorialPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let order = element
            .attribute_ignore_ascii_case("order")
            .map(|v| v.parse::<u32>().unwrap());
        let disable_bot_conversations = element
            .attribute_ignore_ascii_case("disablebotconversations")
            .map_or(true, |v| v.parse().unwrap());
        let allow_character_switch = element
            .attribute_ignore_ascii_case("allowcharacterswitch")
            .map_or(false, |v| v.parse().unwrap());
        let submarine_path = element
            .attribute_ignore_ascii_case("submarinepath")
            .map_or("Content/Tutorials/Dugong_Tutorial.sub".to_owned(), |v| {
                v.parse().unwrap()
            });
        let outpost_path = element
            .attribute_ignore_ascii_case("outpostpath")
            .map_or("Content/Tutorials/TutorialOutpost.sub".to_owned(), |v| {
                v.parse().unwrap()
            });
        let level_seed = element
            .attribute_ignore_ascii_case("levelseed")
            .map_or("nLoZLLtza".to_owned(), |v| v.parse().unwrap());
        let level_params = element
            .attribute_ignore_ascii_case("levelparams")
            .map_or("ColdCavernsTutorial".to_owned(), |v| v.parse().unwrap());

        let tutorial_character = element
            .children()
            .filter(Node::is_element)
            .find(|child| {
                child
                    .tag_name()
                    .name()
                    .eq_ignore_ascii_case("characterinfo")
            })
            .map(|child| TutorialCharacter::new(child));
        let banner = element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("banner"))
            .map(|child| BarotraumaSprite::new(child));
        let event_identifier = element
            .children()
            .filter(Node::is_element)
            .find(|child| {
                child
                    .tag_name()
                    .name()
                    .eq_ignore_ascii_case("scriptedevent")
            })
            .unwrap()
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let end_message_element = element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("endmessage"))
            .unwrap();
        let end_type = end_message_element
            .attribute_ignore_ascii_case("type")
            .map_or(EndType::None, |v| v.parse().unwrap());
        let next_tutorial_identifier = end_message_element
            .attribute_ignore_ascii_case("nexttutorial")
            .map(|v| v.to_owned());

        Self {
            identifier,
            order,
            disable_bot_conversations,
            allow_character_switch,
            submarine_path,
            outpost_path,
            level_seed,
            level_params,
            tutorial_character,
            banner,
            event_identifier,
            end_type,
            next_tutorial_identifier,
        }
    }
}

#[derive(Debug)]
pub enum EndType {
    None,
    Continue,
    Restart,
}
impl FromStr for EndType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "continue" => Ok(Self::Continue),
            "restart" => Ok(Self::Restart),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct TutorialCharacter {
    pub starting_item_tags: Vec<String>,
    pub species_name: String,
    pub job_prefab_identifier: String,
    pub job_variant: u32,
    pub skills: Vec<(String, f32)>,
}

impl TutorialCharacter {
    pub fn new(element: Node) -> Self {
        let starting_item_tags = element
            .attribute_ignore_ascii_case("startingitemtags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
            .unwrap();
        let species_name = element
            .attribute_ignore_ascii_case("speciesname")
            .map_or("human".to_owned(), |v| v.to_owned());
        let job_prefab_identifier = element
            .attribute_ignore_ascii_case("jobidentifier")
            .map_or("assistant".to_owned(), |v| v.to_owned());
        let job_variant = element
            .attribute_ignore_ascii_case("variant")
            .map_or(0, |v| v.parse::<u32>().unwrap());

        let skills = element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("skill"))
            .map(|child| {
                let skill_identifier = child
                    .attribute_ignore_ascii_case("identifier")
                    .map(|v| v.to_owned())
                    .unwrap();
                let level = child
                    .attribute_ignore_ascii_case("level")
                    .map_or(0.0, |v| v.parse::<f32>().unwrap());
                (skill_identifier, level)
            })
            .collect::<Vec<_>>();

        Self {
            starting_item_tags,
            species_name,
            job_prefab_identifier,
            job_variant,
            skills,
        }
    }
}
