use roxmltree::Node;

use crate::shared::{util::NodeExp, version::Version};

use super::item_prefab::{BarotraumaSprite, Color};

#[derive(Debug)]
pub struct TalentPrefab {
    pub identifier: String,
    pub ability_effects_stack_with_same_talent: bool,
    pub name_identifier: Option<String>,
    pub color_override: Option<Color>,
    pub migrations: Vec<TalentMigration>,
    pub icon: Option<BarotraumaSprite>,
    pub description: Option<Description>,
}

impl TalentPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let ability_effects_stack_with_same_talent = element
            .attribute_ignore_ascii_case("abilityeffectsstackwithsametalent")
            .map_or(true, |v| v.parse().unwrap());
        let name_identifier = element
            .attribute_ignore_ascii_case("nameidentifier")
            .map(std::borrow::ToOwned::to_owned);
        let color_override = element
            .attribute_ignore_ascii_case("coloroverride")
            .map(|v| v.parse::<Color>().unwrap());

        let mut migrations = Vec::new();
        let mut icon = None;
        let mut description = None;
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "icon" => {
                    icon = Some(BarotraumaSprite::new(child));
                }
                "description" => {
                    let tag = child
                        .attribute_ignore_ascii_case("tag")
                        .or(child.attribute_ignore_ascii_case("linebreak").map(|_| "\n"))
                        .map(std::borrow::ToOwned::to_owned);
                    let mut replacements = Vec::new();
                    for child in child
                        .children()
                        .filter(Node::is_element)
                        .filter(|v| v.tag_name().name().eq_ignore_ascii_case("replace"))
                    {
                        let variable_tag = child
                            .attribute_ignore_ascii_case("tag")
                            .map(std::borrow::ToOwned::to_owned);
                        let values = child.attribute_ignore_ascii_case("value").map(|v| {
                            v.split(',')
                                .map(std::borrow::ToOwned::to_owned)
                                .collect::<Vec<_>>()
                        });
                        let color = child
                            .attribute_ignore_ascii_case("color")
                            .map(|v| v.parse::<Color>().unwrap());
                        replacements.push(DescriptionReplacement {
                            tag: variable_tag.unwrap(),
                            values: values.unwrap(),
                            color,
                        });
                    }
                    description = Some(Description {
                        tag: tag.unwrap(),
                        replacements,
                    });
                }
                "migrations" => {
                    for child in child.children().filter(Node::is_element) {
                        migrations.push(TalentMigration::new(child));
                    }
                }
                _ => (),
            }
        }

        Self {
            identifier,
            ability_effects_stack_with_same_talent,
            name_identifier,
            color_override,
            migrations,
            icon,
            description,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct Description {
    pub tag: String,
    pub replacements: Vec<DescriptionReplacement>,
}

#[derive(Debug)]
pub struct DescriptionReplacement {
    pub tag: String,
    pub values: Vec<String>,
    pub color: Option<Color>,
}

#[derive(Debug)]
pub struct TalentMigration {
    pub version: Version,
    pub name: String,
}

impl TalentMigration {
    pub fn new(element: Node) -> Self {
        let version = element
            .attribute_ignore_ascii_case("version")
            .map(|v| v.parse::<Version>().unwrap())
            .unwrap();
        let name = element.tag_name().name().to_owned();
        Self { version, name }
    }
}
