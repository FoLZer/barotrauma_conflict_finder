use std::str::FromStr;

use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::item_prefab::{BarotraumaSprite, Color, DoesNotExistError};

#[derive(Debug)]
pub struct OrderCategoryIcon {
    pub identifier: String,
    pub category: OrderCategory,
    pub sprite: BarotraumaSprite,
    pub color: Color,
}

impl OrderCategoryIcon {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("category")
            .map(|v| v.to_owned())
            .unwrap();
        let category = identifier.parse().unwrap();
        let sprite = BarotraumaSprite::new(
            element
                .children()
                .filter(Node::is_element)
                .find(|child| child.tag_name().name().eq_ignore_ascii_case("sprite"))
                .unwrap(),
        );
        let color = element.attribute_ignore_ascii_case("color").map_or(
            Color::Simple {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            |v| v.parse().unwrap(),
        );

        Self {
            identifier,
            category,
            sprite,
            color,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub enum OrderCategory {
    Emergency,
    Movement,
    Power,
    Maintenance,
    Operate,
}
impl FromStr for OrderCategory {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "emergency" => Ok(Self::Emergency),
            "movement" => Ok(Self::Movement),
            "power" => Ok(Self::Power),
            "maintenance" => Ok(Self::Maintenance),
            "operate" => Ok(Self::Operate),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct OrderPrefab {
    pub identifier: String,
    pub target_item_type: Option<String>,
    pub can_type_be_subclass: bool,
    pub color: Option<Color>,
    pub fade_out_time: f32,
    pub use_controller: bool,
    pub controller_tags: Option<Vec<String>>,
    pub target_all_characters: bool,
    pub appropriate_jobs: Option<Vec<String>>,
    pub traitor_mode_only: bool,
    pub preferred_jobs: Option<Vec<String>>,
    pub options: Option<Vec<String>>,
    pub hidden_options: Option<Vec<String>>,
    pub option_target_items: Option<Vec<Vec<String>>>,
    pub target_items: Option<Vec<String>>,
    pub require_items: Option<Vec<String>>,
    pub category: Option<OrderCategory>,
    pub must_set_target: bool,
    pub can_be_generalized: bool,
    pub appropriate_skill: Option<String>,
    pub hidden: bool,
    pub ignore_at_outpost: bool,
    pub must_manually_assign: bool,
    pub draw_icon_when_contained: bool,
    pub auto_dismiss: bool,
    pub assignment_priority: u32,
    pub colored_when_controlling_giver: bool,
    pub display_giver_in_tooltip: bool,
    pub symbol_sprite: Option<BarotraumaSprite>,
    pub option_sprites: Option<Vec<BarotraumaSprite>>,
}
impl OrderPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let target_item_type = element
            .attribute_ignore_ascii_case("targetitemtype")
            .map(|v| v.to_owned());
        let can_type_be_subclass = element
            .attribute_ignore_ascii_case("cantypebesubclass")
            .map_or(false, |v| v.parse().unwrap());
        let color = element
            .attribute_ignore_ascii_case("color")
            .map(|v| v.parse::<Color>().unwrap());
        let fade_out_time = element
            .attribute_ignore_ascii_case("fadeouttime")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let use_controller = element
            .attribute_ignore_ascii_case("usecontroller")
            .map_or(false, |v| v.parse().unwrap());
        let controller_tags = element
            .attribute_ignore_ascii_case("controllertags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let target_all_characters = element
            .attribute_ignore_ascii_case("targetallcharacters")
            .map_or(false, |v| v.parse().unwrap());
        let appropriate_jobs = element
            .attribute_ignore_ascii_case("appropriatejobs")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let traitor_mode_only = element
            .attribute_ignore_ascii_case("TraitorModeOnly")
            .map_or(false, |v| v.parse().unwrap());
        let preferred_jobs = element
            .attribute_ignore_ascii_case("preferredjobs")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let options = element
            .attribute_ignore_ascii_case("options")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let hidden_options = element
            .attribute_ignore_ascii_case("hiddenoptions")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());

        let mut option_target_items = None;
        let mut target_items = None;
        if let Some(s) = element.attribute_ignore_ascii_case("targetitems") {
            if s.contains(';') {
                let mut option_target_items_v = Vec::new();
                let mut all_target_items = Vec::new();
                for v in s
                    .split(';')
                    .map(|s| s.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
                {
                    all_target_items.extend_from_slice(&v);
                    option_target_items_v.push(v);
                }
                target_items = Some(all_target_items);
                option_target_items = Some(option_target_items_v);
            } else {
                target_items = Some(s.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
            }
        };
        let require_items = element
            .attribute_ignore_ascii_case("requireitems")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let category = element
            .attribute_ignore_ascii_case("category")
            .map(|v| v.parse::<OrderCategory>().unwrap());
        let must_set_target = element
            .attribute_ignore_ascii_case("mustsettarget")
            .map_or(false, |v| v.parse().unwrap());
        let can_be_generalized = !must_set_target
            && element
                .attribute_ignore_ascii_case("canbegeneralized")
                .map_or(true, |v| v.parse().unwrap());
        let appropriate_skill = element
            .attribute_ignore_ascii_case("appropriateskill")
            .map(|v| v.to_owned());
        let hidden = element
            .attribute_ignore_ascii_case("hidden")
            .map_or(false, |v| v.parse().unwrap());
        let ignore_at_outpost = element
            .attribute_ignore_ascii_case("ignoreatoutpost")
            .map_or(false, |v| v.parse().unwrap());
        let must_manually_assign = element
            .attribute_ignore_ascii_case("mustmanuallyassign")
            .map_or(false, |v| v.parse().unwrap());
        let draw_icon_when_contained = element
            .attribute_ignore_ascii_case("displayiconwhencontained")
            .map_or(false, |v| v.parse().unwrap());
        let auto_dismiss = element.attribute_ignore_ascii_case("autodismiss").map_or(
            category
                .as_ref()
                .is_some_and(|v| matches!(v, OrderCategory::Operate | OrderCategory::Movement)),
            |v| v.parse().unwrap(),
        );
        let assignment_priority = element
            .attribute_ignore_ascii_case("assignmentpriority")
            .map_or(100, |v| v.parse::<u32>().unwrap().clamp(0, 100));
        let colored_when_controlling_giver = element
            .attribute_ignore_ascii_case("coloredwhencontrollinggiver")
            .map_or(false, |v| v.parse().unwrap());
        let display_giver_in_tooltip = element
            .attribute_ignore_ascii_case("displaygiverintooltip")
            .map_or(false, |v| v.parse().unwrap());

        let symbol_sprite = element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("sprite"))
            .map(|child| BarotraumaSprite::new(child));

        let option_sprites = element
            .children()
            .filter(Node::is_element)
            .find(|child| {
                child
                    .tag_name()
                    .name()
                    .eq_ignore_ascii_case("optionsprites")
            })
            .map(|child| {
                child
                    .children()
                    .filter(Node::is_element)
                    .filter(|child| child.tag_name().name().eq_ignore_ascii_case("sprite"))
                    .map(|child| BarotraumaSprite::new(child))
                    .collect::<Vec<_>>()
            });
        Self {
            identifier,
            target_item_type,
            can_type_be_subclass,
            color,
            fade_out_time,
            use_controller,
            controller_tags,
            target_all_characters,
            appropriate_jobs,
            traitor_mode_only,
            preferred_jobs,
            options,
            hidden_options,
            option_target_items,
            target_items,
            require_items,
            category,
            must_set_target,
            can_be_generalized,
            appropriate_skill,
            hidden,
            ignore_at_outpost,
            must_manually_assign,
            draw_icon_when_contained,
            auto_dismiss,
            assignment_priority,
            colored_when_controlling_giver,
            display_giver_in_tooltip,
            symbol_sprite,
            option_sprites,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
