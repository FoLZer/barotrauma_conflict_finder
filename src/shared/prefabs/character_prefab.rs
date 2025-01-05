use std::collections::HashMap;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::level_object_prefab::RoundSound;

#[derive(Debug)]
pub struct CharacterPrefab {
    pub variant_of: Option<String>,
    pub character_info_prefab: Option<CharacterInfoPrefab>,
    pub sounds: Vec<RoundSound>,
    pub is_humanoid: bool,
}

impl CharacterPrefab {
    pub fn new(element: Node) -> Self {
        let variant_of = element
            .attribute_ignore_ascii_case("inherit")
            .or(element.attribute_ignore_ascii_case("variantof"))
            .map(std::borrow::ToOwned::to_owned);

        let heads_element = element
            .children()
            .find(|v| v.tag_name().name().eq_ignore_ascii_case("heads"));
        let vars_element = element
            .children()
            .find(|v| v.tag_name().name().eq_ignore_ascii_case("vars"));
        let menu_category_element = element
            .children()
            .find(|v| v.tag_name().name().eq_ignore_ascii_case("menucategory"));
        let pronouns_element = element
            .children()
            .find(|v| v.tag_name().name().eq_ignore_ascii_case("pronouns"));

        let character_info_prefab = heads_element.map(|heads_element| {
            CharacterInfoPrefab::new(
                heads_element,
                vars_element,
                menu_category_element,
                pronouns_element,
            )
        });

        let sounds = element
            .children()
            .filter(Node::is_element)
            .filter(|v| v.tag_name().name().eq_ignore_ascii_case("sound"))
            .map(|child| RoundSound::new(child))
            .collect::<Vec<_>>();
        let is_humanoid = element
            .attribute_ignore_ascii_case("humanoid")
            .is_some_and(|v| v.to_lowercase().parse::<bool>().unwrap());

        Self {
            variant_of,
            character_info_prefab,
            sounds,
            is_humanoid,
        }
    }
}

#[derive(Debug)]
pub struct FilenameNotFound;

#[derive(Debug)]
pub struct CharacterInfoPrefab {
    pub heads: Vec<HeadPreset>,
    pub var_tags: HashMap<String, Vec<String>>,
    pub menu_category_var: String,
    pub pronouns: String,
}

impl CharacterInfoPrefab {
    pub fn new(
        heads_element: Node,
        vars_element: Option<Node>,
        menu_category_element: Option<Node>,
        pronouns_element: Option<Node>,
    ) -> Self {
        let heads = heads_element
            .children()
            .filter(Node::is_element)
            .map(|child| HeadPreset::new(child))
            .collect::<Vec<_>>();

        let var_tags = if let Some(vars_element) = vars_element {
            vars_element
                .children()
                .filter(Node::is_element)
                .map(|child| {
                    (
                        child
                            .attribute_ignore_ascii_case("var")
                            .map(std::borrow::ToOwned::to_owned)
                            .unwrap(),
                        child
                            .attribute_ignore_ascii_case("tags")
                            .map(|v| {
                                v.split(',')
                                    .map(std::borrow::ToOwned::to_owned)
                                    .collect::<Vec<_>>()
                            })
                            .unwrap(),
                    )
                })
                .collect::<HashMap<_, _>>()
        } else {
            let mut map = HashMap::new();
            map.insert("GENDER".to_owned(), vec![
                "female".to_owned(),
                "male".to_owned(),
            ]);
            map
        };

        let menu_category_var = menu_category_element.map_or("GENDER".to_owned(), |v| {
            v.attribute_ignore_ascii_case("var")
                .map(std::borrow::ToOwned::to_owned)
                .unwrap()
        });
        let pronouns = pronouns_element.map_or("GENDER".to_owned(), |v| {
            v.attribute_ignore_ascii_case("var")
                .map(std::borrow::ToOwned::to_owned)
                .unwrap()
        });

        Self {
            heads,
            var_tags,
            menu_category_var,
            pronouns,
        }
    }
}

#[derive(Debug)]
pub struct HeadPreset {
    pub tags: Vec<String>,
    pub sheet_index: Vec2,
}

impl HeadPreset {
    pub fn new(element: Node) -> Self {
        let tags = element
            .attribute_ignore_ascii_case("tags")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let sheet_index = element
            .attribute_ignore_ascii_case("sheetindex")
            .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0);

        Self { tags, sheet_index }
    }
}
