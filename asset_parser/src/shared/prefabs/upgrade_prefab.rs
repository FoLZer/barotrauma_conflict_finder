use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    ops::RangeInclusive,
    str::FromStr,
};

use roxmltree::Node;

use crate::shared::{submarine_info::SubmarineClass, util::NodeExp};

use super::{item_prefab::BarotraumaSprite, structure_prefab::DecorativeSprite};

#[derive(Debug)]
pub struct UpgradeCategory {
    pub identifier: String,
    pub self_item_tags: Option<Vec<String>>,
    pub name: Option<String>,
    pub is_wall_upgrade: bool,
    pub name_identifier: Option<String>,
}

impl UpgradeCategory {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let self_item_tags = element
            .attribute_ignore_ascii_case("items")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(|v| v.to_owned());
        let is_wall_upgrade = element
            .attribute_ignore_ascii_case("wallupgrade")
            .map_or(false, |v| v.parse().unwrap());
        let name_identifier = element
            .attribute_ignore_ascii_case("nameidentifier")
            .map(|v| v.to_owned());

        Self {
            identifier,
            self_item_tags,
            name,
            is_wall_upgrade,
            name_identifier,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct UpgradePrefab {
    pub identifier: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_level: u32,
    pub suppress_warnings: bool,
    pub hide_in_menus: bool,
    pub name_identifier: Option<String>,
    pub description_identifier: Option<String>,
    pub increase_on_tooltip: Option<f32>,
    pub upgrade_category_identifiers: HashSet<String>,
    pub price: Option<UpgradePrice>,
    pub max_levels: Vec<UpgradeMaxLevelMod>,
    pub resource_costs: Vec<UpgradeResourceCost>,
    pub decorative_sprites: Vec<DecorativeSprite>,
    pub sprite: Option<BarotraumaSprite>,
    pub target_properties: HashMap<String, Vec<String>>,
}

impl UpgradePrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(|v| v.to_owned());
        let description = element
            .attribute_ignore_ascii_case("description")
            .map(|v| v.to_owned());
        let max_level = element
            .attribute_ignore_ascii_case("maxlevel")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let suppress_warnings = element
            .attribute_ignore_ascii_case("supresswarnings")
            .map_or(false, |v| v.parse().unwrap());
        let hide_in_menus = element
            .attribute_ignore_ascii_case("hideinmenus")
            .map_or(false, |v| v.parse().unwrap());
        let name_identifier = element
            .attribute_ignore_ascii_case("nameidentifier")
            .map(|v| v.to_owned());
        let description_identifier = element
            .attribute_ignore_ascii_case("descriptionidentifier")
            .map(|v| v.to_owned());
        let increase_on_tooltip = element
            .attribute_ignore_ascii_case("increaseontooltip")
            .map(|v| v.parse::<f32>().unwrap());
        let upgrade_category_identifiers = element
            .attribute_ignore_ascii_case("categories")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<HashSet<_>>())
            .unwrap();

        let mut price = None;
        let mut max_levels = Vec::new();
        let mut resource_costs = Vec::new();
        let mut decorative_sprites = Vec::new();
        let mut sprite = None;
        let mut target_properties = HashMap::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "price" => {
                    price = Some(UpgradePrice::new(child));
                }
                "maxlevel" => {
                    max_levels.push(UpgradeMaxLevelMod::new(child));
                }
                "resourcecost" => {
                    resource_costs.push(UpgradeResourceCost::new(child));
                }
                "decorativesprite" => {
                    decorative_sprites.push(DecorativeSprite::new(child));
                }
                "sprite" => {
                    sprite = Some(BarotraumaSprite::new(child));
                }
                _ => {
                    let properties = child
                        .attributes()
                        .map(|v| v.name().to_owned())
                        .collect::<Vec<_>>();
                    target_properties.insert(child.tag_name().name().to_owned(), properties);
                }
            }
        }

        Self {
            identifier,
            name,
            description,
            max_level,
            suppress_warnings,
            hide_in_menus,
            name_identifier,
            description_identifier,
            increase_on_tooltip,
            upgrade_category_identifiers,
            price,
            max_levels,
            resource_costs,
            decorative_sprites,
            sprite,
            target_properties,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct UpgradePrice {
    pub increase_low: u32,
    pub increase_high: u32,
    pub base_price: u32,
}

impl UpgradePrice {
    pub fn new(element: Node) -> Self {
        let increase_low = element
            .attribute_ignore_ascii_case("increaselow")
            .map(|v| v.parse::<Percentage>().unwrap().0)
            .unwrap();
        let increase_high = element
            .attribute_ignore_ascii_case("increasehigh")
            .map(|v| v.parse::<Percentage>().unwrap().0)
            .unwrap();
        let base_price = element
            .attribute_ignore_ascii_case("baseprice")
            .map(|v| v.parse::<u32>().unwrap())
            .unwrap();

        Self {
            increase_low,
            increase_high,
            base_price,
        }
    }
}

pub struct Percentage(pub u32);
impl FromStr for Percentage {
    type Err = ParseIntError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.strip_prefix('+').unwrap_or(s);
        s = s.strip_suffix('%').unwrap_or(s);
        Ok(Percentage(s.parse()?))
    }
}

#[derive(Debug)]
pub struct UpgradeMaxLevelMod {
    pub sub_class: Option<SubmarineClass>,
    pub tier: Option<u32>,
    pub ty: MaxLevelModType,
    pub value: i32,
}

impl UpgradeMaxLevelMod {
    pub fn new(element: Node) -> Self {
        let sub_class = element
            .attribute_ignore_ascii_case("class")
            .map(|v| v.parse::<SubmarineClass>().unwrap());
        let tier = element
            .attribute_ignore_ascii_case("tier")
            .map(|v| v.parse::<u32>().unwrap());

        let mut v = element.attribute_ignore_ascii_case("level").unwrap();
        let first_char = v.chars().next().unwrap();
        let ty = if first_char == '+' {
            v = &v[1..];
            MaxLevelModType::Increase
        } else if first_char == '-' {
            MaxLevelModType::Increase
        } else {
            MaxLevelModType::Set
        };
        let value = v.parse::<i32>().unwrap();

        Self {
            sub_class,
            tier,
            ty,
            value,
        }
    }
}

#[derive(Debug)]
pub enum MaxLevelModType {
    Increase,
    Set,
}

#[derive(Debug)]
pub struct UpgradeResourceCost {
    pub amount: u32,
    pub target_items: Vec<String>,
    pub target_levels: RangeInclusive<u32>,
}

impl UpgradeResourceCost {
    pub fn new(element: Node) -> Self {
        let amount = element
            .attribute_ignore_ascii_case("amount")
            .map(|v| v.parse::<u32>().unwrap())
            .unwrap();
        let target_items = element
            .attribute_ignore_ascii_case("item")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
            .unwrap();
        let target_levels = element
            .attribute_ignore_ascii_case("levels")
            .map(|v| v.parse::<Range>().unwrap().0)
            .unwrap();

        Self {
            amount,
            target_items,
            target_levels,
        }
    }
}

pub struct Range(RangeInclusive<u32>);
impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split('-');
        let from = spl.next().unwrap().parse::<u32>()?;
        match spl.next() {
            Some(to) => {
                let to = to.parse::<u32>()?;
                assert!(from < to);
                Ok(Range(from..=to))
            }
            None => Ok(Range(from..=from)),
        }
    }
}
