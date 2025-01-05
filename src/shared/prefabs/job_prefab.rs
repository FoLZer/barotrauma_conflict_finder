use std::ops::RangeInclusive;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::{
    human_prefab::BehaviorType,
    item_prefab::{BarotraumaSprite, Color},
};

#[derive(Debug)]
pub struct ItemRepairPriority {
    pub identifier: String,
    pub priority: f32,
}

impl ItemRepairPriority {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("tag")
            .map(|v| v.to_owned())
            .unwrap();
        let priority = element
            .attribute_ignore_ascii_case("priority")
            .map(|v| v.parse::<f32>().unwrap())
            .unwrap();

        Self {
            identifier,
            priority,
        }
    }
}

#[derive(Debug)]
pub struct JobPrefab {
    pub identifier: String,
    pub ui_color: Color,
    pub idle_behavior: BehaviorType,
    pub only_job_specific_dialog: bool,
    pub initial_count: u32,
    pub allow_always: bool,
    pub max_number: u32,
    pub min_number: u32,
    pub min_karma: f32,
    pub price_multiplier: f32,
    pub vitality_modifier: f32,
    pub hidden_job: bool,
    pub item_sets: Vec<JobItemSet>,
    pub skills: Vec<SkillPrefab>,
    pub autonomous_objectives: Vec<AutonomousObjective>,
    pub appropriate_orders: Vec<String>,
    pub icon: Option<BarotraumaSprite>,
    pub icon_small: Option<BarotraumaSprite>,
}

impl JobPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let ui_color = element.attribute_ignore_ascii_case("uicolor").map_or(
            Color::Simple {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            |v| v.parse().unwrap(),
        );
        let idle_behavior = element
            .attribute_ignore_ascii_case("idlebehavior")
            .map_or(BehaviorType::Passive, |v| v.parse().unwrap());
        let only_job_specific_dialog = element
            .attribute_ignore_ascii_case("onlyjobspecificdialog")
            .map_or(false, |v| v.parse().unwrap());
        let initial_count = element
            .attribute_ignore_ascii_case("initialcount")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let allow_always = element
            .attribute_ignore_ascii_case("allowalways")
            .map_or(false, |v| v.parse().unwrap());
        let max_number = element
            .attribute_ignore_ascii_case("maxnumber")
            .map_or(100, |v| v.parse::<u32>().unwrap());
        let min_number = element
            .attribute_ignore_ascii_case("minnumber")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let min_karma = element
            .attribute_ignore_ascii_case("minkarma")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let price_multiplier = element
            .attribute_ignore_ascii_case("pricemultiplier")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let vitality_modifier = element
            .attribute_ignore_ascii_case("vitalitymodifier")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let hidden_job = element
            .attribute_ignore_ascii_case("hiddenjob")
            .map_or(false, |v| v.parse().unwrap());

        let mut item_sets = Vec::new();
        let mut skills = Vec::new();
        let mut autonomous_objectives = Vec::new();
        let mut appropriate_orders = Vec::new();
        let mut icon = None;
        let mut icon_small = None;
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "itemset" => {
                    item_sets.push(JobItemSet {
                        items: child
                            .children()
                            .filter(Node::is_element)
                            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("item"))
                            .map(|child| JobItem::new(child))
                            .collect::<Vec<_>>(),
                        outfit_preview: child
                            .children()
                            .filter(Node::is_element)
                            .find(|child| {
                                child
                                    .tag_name()
                                    .name()
                                    .eq_ignore_ascii_case("PreviewSprites")
                            })
                            .map(|child| OutfitPreview::new(child)),
                    });
                }
                "skills" => {
                    skills.extend(
                        child
                            .children()
                            .filter(Node::is_element)
                            .map(|child| SkillPrefab::new(child)),
                    );
                }
                "autonomousobjectives" => {
                    autonomous_objectives.extend(
                        child
                            .children()
                            .filter(Node::is_element)
                            .map(|child| AutonomousObjective::new(child)),
                    );
                }
                "approprtiateobjectives" | "appropriateorders" => {
                    appropriate_orders.extend(child.children().filter(Node::is_element).map(
                        |child| {
                            child
                                .attribute_ignore_ascii_case("identifier")
                                .map(|v| v.to_owned())
                                .unwrap()
                        },
                    ));
                }
                "jobicon" => {
                    icon = Some(BarotraumaSprite::new(
                        child.children().find(Node::is_element).unwrap(),
                    ))
                }
                "jobiconsmall" => {
                    icon_small = Some(BarotraumaSprite::new(
                        child.children().find(Node::is_element).unwrap(),
                    ))
                }
                _ => (),
            }
        }

        Self {
            identifier,
            ui_color,
            idle_behavior,
            only_job_specific_dialog,
            initial_count,
            allow_always,
            max_number,
            min_number,
            min_karma,
            price_multiplier,
            vitality_modifier,
            hidden_job,
            item_sets,
            skills,
            autonomous_objectives,
            appropriate_orders,
            icon,
            icon_small,
        }
    }
}

#[derive(Debug)]
pub struct AutonomousObjective {
    pub identifier: String,
    pub option: Option<String>,
    pub priority_modifier: f32,
    pub ignore_at_outpost: bool,
}

impl AutonomousObjective {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let option = element
            .attribute_ignore_ascii_case("option")
            .map(|v| v.to_owned());
        let priority_modifier = element
            .attribute_ignore_ascii_case("prioritymodifier")
            .map_or(1.0, |v| v.parse::<f32>().unwrap().max(0.0));
        let ignore_at_outpost = element
            .attribute_ignore_ascii_case("ignoreatoutpost")
            .map_or(false, |v| v.parse::<bool>().unwrap());

        Self {
            identifier,
            option,
            priority_modifier,
            ignore_at_outpost,
        }
    }
}

#[derive(Debug)]
pub struct SkillPrefab {
    pub identifier: String,
    pub price_modifier: f32,
    pub level_range: RangeInclusive<f32>,
    pub is_primary_skill: bool,
}

impl SkillPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let price_modifier = element
            .attribute_ignore_ascii_case("pricemodifier")
            .map_or(25.0, |v| v.parse::<f32>().unwrap());
        let level_range = element
            .attribute_ignore_ascii_case("level")
            .map(|v| {
                if v.contains(',') {
                    let v = v.parse::<Vector2>().unwrap().0;
                    v.x..=v.y
                } else {
                    let v = v.parse::<f32>().unwrap();
                    v..=v
                }
            })
            .unwrap();
        let is_primary_skill = element
            .attribute_ignore_ascii_case("primary")
            .map_or(false, |v| v.parse::<bool>().unwrap());

        Self {
            identifier,
            price_modifier,
            level_range,
            is_primary_skill,
        }
    }
}

#[derive(Debug)]

pub struct JobItem {
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub equip: bool,
    pub child_items: Vec<JobItem>,
}

impl JobItem {
    pub fn new(element: Node) -> Self {
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(|v| v.to_owned());
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned());
        assert!(name.is_some() || identifier.is_some());
        let equip = element
            .attribute_ignore_ascii_case("equip")
            .map_or(false, |v| v.parse().unwrap());
        let child_items = element
            .children()
            .filter(Node::is_element)
            .map(|child| JobItem::new(child))
            .collect::<Vec<_>>();

        Self {
            name,
            identifier,
            equip,
            child_items,
        }
    }
}

#[derive(Debug)]
pub struct OutfitPreview {
    pub dimensions: Vec2,
    pub sprites: Vec<BarotraumaSpriteWithOffset>,
}

impl OutfitPreview {
    pub fn new(element: Node) -> Self {
        let dimensions = element
            .attribute_ignore_ascii_case("dims")
            .map_or(Vec2::ONE, |v| v.parse::<Vector2>().unwrap().0);
        let sprites = element
            .children()
            .filter(Node::is_element)
            .map(|child| BarotraumaSpriteWithOffset {
                sprite: BarotraumaSprite::new(child),
                offset: child
                    .attribute_ignore_ascii_case("offset")
                    .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            })
            .collect::<Vec<_>>();

        Self {
            dimensions,
            sprites,
        }
    }
}

#[derive(Debug)]
pub struct BarotraumaSpriteWithOffset {
    pub sprite: BarotraumaSprite,
    pub offset: Vec2,
}

#[derive(Debug)]
pub struct JobItemSet {
    pub items: Vec<JobItem>,
    pub outfit_preview: Option<OutfitPreview>,
}
