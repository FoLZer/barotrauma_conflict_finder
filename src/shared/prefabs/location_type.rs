use std::{collections::HashMap, str::FromStr};

use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::{
    item_prefab::{BarotraumaSprite, Color, DoesNotExistError},
    map_generation_params::Point,
};

#[derive(Debug)]
pub struct LocationType {
    pub identifier: String,
    pub beacon_station_chance: f32,
    pub use_portrait_in_random_loading_screens: bool,
    pub has_outpost: bool,
    pub is_enterable: bool,
    pub allow_as_biome_gate: bool,
    pub allow_in_random_levels: bool,
    pub faction: Option<String>,
    pub secondary_faction: Option<String>,
    pub show_sonar_marker: bool,
    pub mission_identifiers: Option<Vec<String>>,
    pub mission_tags: Option<Vec<String>>,
    pub hide_entity_subcategories: Option<Vec<String>>,
    pub replace_in_radiation: Option<String>,
    pub force_outpost_generation_params_identifier: Option<String>,
    pub ignore_generic_events: bool,
    pub outpost_team: CharacterTeamType,
    pub force_location_name: Option<String>,
    pub name_files: Option<Vec<String>>,
    pub name_identifiers: Option<Vec<String>>,
    pub commonness_per_zone: Option<HashMap<u32, f32>>,
    pub min_count_per_zone: Option<HashMap<u32, u32>>,
    pub hireable_jobs: Vec<(String, f32)>,
    pub total_hireable_weight: f32,
    pub sprite: Option<BarotraumaSprite>,
    pub sprite_color: Option<Color>,
    pub radiation_sprite: Option<BarotraumaSprite>,
    pub can_change_to: Vec<LocationTypeChange>,
    pub portraits: Vec<BarotraumaSprite>,
    pub store_max_reputation_modifier: f32,
    pub store_sell_price_modifier: f32,
    pub daily_special_price_modifier: f32,
    pub request_good_price_modifier: f32,
    pub store_initial_balance: u32,
    pub store_price_modifier_range: u32,
    pub daily_specials_count: u32,
    pub requested_goods_count: u32,
}

impl LocationType {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let beacon_station_chance = element
            .attribute_ignore_ascii_case("beaconstationchance")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let use_portrait_in_random_loading_screens = element
            .attribute_ignore_ascii_case("useportraitinrandomloadingscreens")
            .map_or(true, |v| v.parse().unwrap());
        let has_outpost = element
            .attribute_ignore_ascii_case("hasoutpost")
            .map_or(true, |v| v.parse().unwrap());
        let is_enterable = element
            .attribute_ignore_ascii_case("isenterable")
            .map_or(has_outpost, |v| v.parse().unwrap());
        let allow_as_biome_gate = element
            .attribute_ignore_ascii_case("allowasbiomegate")
            .map_or(true, |v| v.parse().unwrap());
        let allow_in_random_levels = element
            .attribute_ignore_ascii_case("allowinrandomlevels")
            .map_or(true, |v| v.parse().unwrap());
        let faction = element
            .attribute_ignore_ascii_case("faction")
            .map(|v| v.to_owned());
        let secondary_faction = element
            .attribute_ignore_ascii_case("secondaryfaction")
            .map(|v| v.to_owned());
        let show_sonar_marker = element
            .attribute_ignore_ascii_case("showsonarmarker")
            .map_or(true, |v| v.parse().unwrap());
        let mission_identifiers = element
            .attribute_ignore_ascii_case("missionidentifiers")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let mission_tags = element
            .attribute_ignore_ascii_case("missiontags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let hide_entity_subcategories = element
            .attribute_ignore_ascii_case("hideentitysubcategories")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let replace_in_radiation = element
            .attribute_ignore_ascii_case("replaceinradiation")
            .map(|v| v.to_owned());
        let force_outpost_generation_params_identifier = element
            .attribute_ignore_ascii_case("forceoutpostgenerationparamsidentifier")
            .map(|v| v.to_owned());
        let ignore_generic_events = element
            .attribute_ignore_ascii_case("ignoregenericevents")
            .map_or(false, |v| v.parse().unwrap());
        let outpost_team = element
            .attribute_ignore_ascii_case("outpostteam")
            .map_or(CharacterTeamType::FriendlyNPC, |v| v.parse().unwrap());
        let (force_location_name, name_files, name_identifiers) =
            if let Some(name) = element.attribute_ignore_ascii_case("name") {
                (Some(name.to_owned()), None, None)
            } else if let Some(name_files) = element
                .attribute_ignore_ascii_case("namefile")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
            {
                (None, Some(name_files), None)
            } else {
                (
                    None,
                    None,
                    Some(
                        element
                            .attribute_ignore_ascii_case("nameidentifiers")
                            .map_or(vec![identifier.clone()], |v| {
                                v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>()
                            }),
                    ),
                )
            };
        let commonness_per_zone = element
            .attribute_ignore_ascii_case("commonnessperzone")
            .map(|v| {
                v.split(',')
                    .map(|v| {
                        v.split_once(':')
                            .map(|v| {
                                (
                                    v.0.trim().parse::<u32>().unwrap(),
                                    v.1.trim().parse::<f32>().unwrap(),
                                )
                            })
                            .unwrap()
                    })
                    .collect::<HashMap<_, _>>()
            });
        let min_count_per_zone = element
            .attribute_ignore_ascii_case("mincountperzone")
            .map(|v| {
                v.split(',')
                    .map(|v| {
                        v.split_once(':')
                            .map(|v| {
                                (
                                    v.0.trim().parse::<u32>().unwrap(),
                                    v.1.trim().parse::<u32>().unwrap(),
                                )
                            })
                            .unwrap()
                    })
                    .collect::<HashMap<_, _>>()
            });

        let mut hireable_jobs = Vec::new();
        let mut total_hireable_weight = 0.0;
        let mut sprite = None;
        let mut sprite_color = None;
        let mut radiation_sprite = None;
        let mut can_change_to = Vec::new();
        let mut portraits = Vec::new();
        let mut store_max_reputation_modifier = 0.1;
        let mut store_sell_price_modifier = 0.3;
        let mut daily_special_price_modifier = 0.5;
        let mut request_good_price_modifier = 2.0;
        let mut store_initial_balance = 5000;
        let mut store_price_modifier_range = 5;
        let mut daily_specials_count = 1;
        let mut requested_goods_count = 1;
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "hireable" => {
                    let job_identifier = child
                        .attribute_ignore_ascii_case("identifier")
                        .map(|v| v.to_owned())
                        .unwrap();
                    let job_commonness = child
                        .attribute_ignore_ascii_case("commonness")
                        .map_or(1.0, |v| v.parse::<f32>().unwrap());
                    total_hireable_weight += job_commonness;
                    hireable_jobs.push((job_identifier, job_commonness))
                }
                "symbol" => {
                    sprite = Some(BarotraumaSprite::new(child));
                    sprite_color = Some(child.attribute_ignore_ascii_case("color").map_or(
                        Color::Simple {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        },
                        |v| v.parse().unwrap(),
                    ))
                }
                "radiationsymbol" => {
                    radiation_sprite = Some(BarotraumaSprite::new(child));
                }
                "changeto" => {
                    can_change_to.push(LocationTypeChange::new(child, &identifier));
                }
                "portrait" => {
                    portraits.push(BarotraumaSprite::new(child));
                }
                "store" => {
                    store_max_reputation_modifier = child
                        .attribute_ignore_ascii_case("maxreputationmodifier")
                        .map_or(store_max_reputation_modifier, |v| v.parse::<f32>().unwrap());
                    store_sell_price_modifier = child
                        .attribute_ignore_ascii_case("sellpricemodifier")
                        .map_or(store_sell_price_modifier, |v| v.parse::<f32>().unwrap());
                    daily_special_price_modifier = child
                        .attribute_ignore_ascii_case("dailyspecialpricemodifier")
                        .map_or(daily_special_price_modifier, |v| v.parse::<f32>().unwrap());
                    request_good_price_modifier = child
                        .attribute_ignore_ascii_case("requestgoodpricemodifier")
                        .map_or(request_good_price_modifier, |v| v.parse::<f32>().unwrap());
                    store_initial_balance = child
                        .attribute_ignore_ascii_case("initialbalance")
                        .map_or(store_initial_balance, |v| v.parse::<u32>().unwrap());
                    store_price_modifier_range = child
                        .attribute_ignore_ascii_case("pricemodifierrange")
                        .map_or(store_price_modifier_range, |v| v.parse::<u32>().unwrap());
                    daily_specials_count = child
                        .attribute_ignore_ascii_case("dailyspecialscount")
                        .map_or(daily_specials_count, |v| v.parse::<u32>().unwrap());
                    requested_goods_count = child
                        .attribute_ignore_ascii_case("requestedgoodscount")
                        .map_or(requested_goods_count, |v| v.parse::<u32>().unwrap());
                }
                _ => (),
            }
        }

        Self {
            identifier,
            beacon_station_chance,
            use_portrait_in_random_loading_screens,
            has_outpost,
            is_enterable,
            allow_as_biome_gate,
            allow_in_random_levels,
            faction,
            secondary_faction,
            show_sonar_marker,
            mission_identifiers,
            mission_tags,
            hide_entity_subcategories,
            replace_in_radiation,
            force_outpost_generation_params_identifier,
            ignore_generic_events,
            outpost_team,
            force_location_name,
            name_files,
            name_identifiers,
            commonness_per_zone,
            min_count_per_zone,
            hireable_jobs,
            total_hireable_weight,
            sprite,
            sprite_color,
            radiation_sprite,
            can_change_to,
            portraits,
            store_max_reputation_modifier,
            store_sell_price_modifier,
            daily_special_price_modifier,
            request_good_price_modifier,
            store_initial_balance,
            store_price_modifier_range,
            daily_specials_count,
            requested_goods_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CharacterTeamType {
    None,
    Team1,
    Team2,
    FriendlyNPC,
}
impl FromStr for CharacterTeamType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "team1" => Ok(Self::Team1),
            "team2" => Ok(Self::Team2),
            "friendlynpc" => Ok(Self::FriendlyNPC),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

impl TryFrom<u8> for CharacterTeamType {
    type Error = DoesNotExistError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::None,
            1 => Self::Team1,
            2 => Self::Team2,
            3 => Self::FriendlyNPC,
            _ => return Err(DoesNotExistError(value.to_string())),
        })
    }
}

#[derive(Debug)]
pub struct LocationTypeChange {
    pub change_to_type: String,
    pub require_discovered: bool,
    pub disallowed_adjacent_locations: Option<Vec<String>>,
    pub disallowed_proximity: u32,
    pub required_duration_range: Point,
    pub probability: f32,
    pub cooldown_after_change: u32,
    pub message_tag: String,
    pub requirements: Vec<Requirement>,
}

impl LocationTypeChange {
    pub fn new(element: Node, current_type: &str) -> Self {
        let change_to_type = element
            .attribute_ignore_ascii_case("type")
            .or(element.attribute_ignore_ascii_case("to"))
            .map(|v| v.to_owned())
            .unwrap();
        let require_discovered = element
            .attribute_ignore_ascii_case("requirediscovered")
            .map_or(false, |v| v.parse().unwrap());
        let disallowed_adjacent_locations = element
            .attribute_ignore_ascii_case("disallowedadjacentlocations")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let disallowed_proximity = element
            .attribute_ignore_ascii_case("disallowedproximity")
            .map_or(1, |v| v.parse::<u32>().unwrap().max(1));
        let mut required_duration_range = element
            .attribute_ignore_ascii_case("requireddurationrange")
            .map_or(Point { x: 0, y: 0 }, |v| v.parse::<Point>().unwrap());
        let probability = element
            .attribute_ignore_ascii_case("probability")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let cooldown_after_change = element
            .attribute_ignore_ascii_case("cooldownafterchange")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let message_tag = element.attribute_ignore_ascii_case("messagetag").map_or(
            format!(
                "LocationChange.{}.ChangeTo.{}",
                current_type, change_to_type
            ),
            |v| v.to_owned(),
        );

        if let Some(required_duration) = element
            .attribute_ignore_ascii_case("requiredduration")
            .map(|v| v.parse::<u32>().unwrap())
        {
            //backwards compatibility
            required_duration_range = Point {
                x: required_duration as i32,
                y: required_duration as i32,
            };
        }

        let mut requirements = Vec::new();
        if element.has_attribute_ignore_ascii_case("requiredlocations") {
            //backwards compatibility
            requirements.push(Requirement::new(element));
        }

        for child in element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("requirement"))
        {
            requirements.push(Requirement::new(child));
        }

        Self {
            change_to_type,
            require_discovered,
            disallowed_adjacent_locations,
            disallowed_proximity,
            required_duration_range,
            probability,
            cooldown_after_change,
            message_tag,
            requirements,
        }
    }
}

#[derive(Debug)]
pub struct Requirement {
    pub required_locations: Option<Vec<String>>,
    pub required_proximity: u32,
    pub proximity_probability_increase: f32,
    pub required_proximity_for_probability_increase: Option<u32>,
    pub require_beacon_station: bool,
    pub require_hunting_grounds: bool,
    pub function: FunctionType,
    pub probability: f32,
}

impl Requirement {
    pub fn new(element: Node) -> Self {
        let required_locations = element
            .attribute_ignore_ascii_case("requiredlocations")
            .or(element.attribute_ignore_ascii_case("requiredadjacentlocations"))
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let required_proximity = element
            .attribute_ignore_ascii_case("requiredproximity")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let proximity_probability_increase = element
            .attribute_ignore_ascii_case("proximityprobabilityincrease")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let required_proximity_for_probability_increase = element
            .attribute_ignore_ascii_case("requiredproximityforprobabilityincrease")
            .map(|v| v.parse::<u32>().unwrap());
        let require_beacon_station = element
            .attribute_ignore_ascii_case("requirebeaconstation")
            .map_or(false, |v| v.parse().unwrap());
        let require_hunting_grounds = element
            .attribute_ignore_ascii_case("requirehuntinggrounds")
            .map_or(false, |v| v.parse().unwrap());
        let function = element
            .attribute_ignore_ascii_case("function")
            .map_or(FunctionType::Add, |v| v.parse().unwrap());
        let probability = element
            .attribute_ignore_ascii_case("probability")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());

        Self {
            required_locations,
            required_proximity,
            proximity_probability_increase,
            required_proximity_for_probability_increase,
            require_beacon_station,
            require_hunting_grounds,
            function,
            probability,
        }
    }
}

#[derive(Debug)]
pub enum FunctionType {
    Add,
    Multiply,
}
impl FromStr for FunctionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Add" => Ok(Self::Add),
            "Multiply" => Ok(Self::Multiply),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
