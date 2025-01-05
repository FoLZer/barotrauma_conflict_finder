use std::{collections::HashMap, str::FromStr};

use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::item_prefab::DoesNotExistError;

#[derive(Debug)]
pub struct TalentTree {
    pub job_identifier: String,
    pub sub_trees: Vec<TalentSubTree>,
}

impl TalentTree {
    pub fn new(element: Node) -> Self {
        let job_identifier = element
            .attribute_ignore_ascii_case("jobidentifier")
            .unwrap()
            .to_owned();
        let sub_trees = element
            .children()
            .filter(Node::is_element)
            .filter(|v| v.tag_name().name().eq_ignore_ascii_case("subtree"))
            .map(|child| TalentSubTree::new(child))
            .collect::<Vec<_>>();
        Self {
            job_identifier,
            sub_trees,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.job_identifier
    }
}

#[derive(Debug)]
pub struct TalentSubTree {
    pub identifier: String,
    pub name_identifier: Option<String>,
    pub tree_type: TalentTreeType,
    pub required_trees: Option<Vec<String>>,
    pub blocked_trees: Option<Vec<String>>,
    pub talent_option_stages: Vec<TalentOption>,
}

impl TalentSubTree {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let name_identifier = element
            .attribute_ignore_ascii_case("nameidentifier")
            .map(std::borrow::ToOwned::to_owned);
        let tree_type = element
            .attribute_ignore_ascii_case("type")
            .map_or(TalentTreeType::Specialization, |v| {
                v.parse::<TalentTreeType>().unwrap()
            });
        let required_trees = element.attribute_ignore_ascii_case("requires").map(|v| {
            v.split(',')
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<_>>()
        });
        let blocked_trees = element.attribute_ignore_ascii_case("blocks").map(|v| {
            v.split(',')
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<_>>()
        });
        let talent_option_stages = element
            .children()
            .filter(Node::is_element)
            .filter(|v| v.tag_name().name().eq_ignore_ascii_case("talentoptions"))
            .map(|child| TalentOption::new(child))
            .collect();

        Self {
            identifier,
            name_identifier,
            tree_type,
            required_trees,
            blocked_trees,
            talent_option_stages,
        }
    }
}

#[derive(Debug)]
pub struct TalentOption {
    pub max_chosen_talents: u32,
    pub required_talents: u32,
    pub identifiers: Vec<String>,
    pub show_case_talents: HashMap<String, Vec<String>>,
}

impl TalentOption {
    pub fn new(element: Node) -> Self {
        let max_chosen_talents = element
            .attribute_ignore_ascii_case("maxchosentalents")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let required_talents = element
            .attribute_ignore_ascii_case("required_talents")
            .map_or(max_chosen_talents, |v| v.parse::<u32>().unwrap());
        let mut identifiers = Vec::new();
        let mut show_case_talents = HashMap::new();

        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "talentoption" => {
                    identifiers.push(
                        child
                            .attribute_ignore_ascii_case("identifier")
                            .unwrap()
                            .to_owned(),
                    );
                }
                "showcasetalent" => {
                    let show_case_identifier = child
                        .attribute_ignore_ascii_case("identifier")
                        .unwrap()
                        .to_owned();
                    let mut show_case_talent_identifiers = Vec::new();
                    for child in child.children().filter(Node::is_element) {
                        let identifier = child
                            .attribute_ignore_ascii_case("identifier")
                            .unwrap()
                            .to_owned();
                        show_case_talent_identifiers.push(identifier.clone());
                        identifiers.push(identifier);
                    }
                    show_case_talents.insert(show_case_identifier, show_case_talent_identifiers);
                }
                _ => (),
            }
        }

        Self {
            max_chosen_talents,
            required_talents,
            identifiers,
            show_case_talents,
        }
    }
}

#[derive(Debug)]
pub enum TalentTreeType {
    Specialization,
    Primary,
}

impl FromStr for TalentTreeType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "specialization" => Ok(Self::Specialization),
            "primary" => Ok(Self::Primary),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
