use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use bitfield_struct::bitfield;
use rand::{Rng, distributions::Alphanumeric, thread_rng};
use roxmltree::Node;

use crate::shared::{
    content_files::level_generation_parameters_file,
    prefabs::item_prefab::{BarotraumaSprite, Color},
    util::NodeExp,
};

use super::{
    item_prefab::DoesNotExistError,
    level_object_prefab::{ComparisonOperatorType, PropertyConditional},
};

#[derive(Debug)]
pub struct EventPrefab {
    pub identifier: String,
    pub event_type: EventType,
    pub biome_identifier: Option<String>,
    pub faction: Option<String>,
    pub commonness: f32,
    pub probability: f32,
    pub trigger_event_cooldown: bool,
    pub unlock_path_event: bool,
    pub unlock_path_tooltip: Option<String>,
    pub unlock_path_reputation: i32,
}

impl EventPrefab {
    pub fn new(element: Node, fallback_identifier: Option<String>) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .or(fallback_identifier)
            .unwrap();
        let event_type = match element.tag_name().name() {
            "Event" => EventType::Normal,
            "ArtifactEvent" => EventType::Artifact,
            "MalfunctionEvent" => EventType::Malfunction,
            "MonsterEvent" => EventType::Monster,
            "ScriptedEvent" => EventType::Scripted,
            "TraitorEvent" => EventType::Traitor,
            _ => panic!(),
        };
        let biome_identifier = element
            .attribute_ignore_ascii_case("biome")
            .map(|v| v.to_owned());
        let faction = element
            .attribute_ignore_ascii_case("faction")
            .map(|v| v.to_owned());
        let commonness = element
            .attribute_ignore_ascii_case("commonness")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let probability = element
            .attribute_ignore_ascii_case("probability")
            .or(element.attribute_ignore_ascii_case("spawnprobability"))
            .map_or(1.0, |v| v.parse::<f32>().unwrap().clamp(0.0, 1.0));
        let trigger_event_cooldown = element
            .attribute_ignore_ascii_case("triggereventcooldown")
            .map_or(!matches!(event_type, EventType::Scripted), |v| {
                v.parse().unwrap()
            });
        let unlock_path_event = element
            .attribute_ignore_ascii_case("triggereventcooldown")
            .map_or(false, |v| v.parse().unwrap());
        let unlock_path_tooltip = element
            .attribute_ignore_ascii_case("unlockpathtooltip")
            .or(element.attribute_ignore_ascii_case("lockedpathtooltip"))
            .map(|v| v.to_owned());
        let unlock_path_reputation = element
            .attribute_ignore_ascii_case("unlockpathreputation")
            .map_or(0, |v| v.parse::<i32>().unwrap());

        Self {
            identifier,
            event_type,
            biome_identifier,
            faction,
            commonness,
            probability,
            trigger_event_cooldown,
            unlock_path_event,
            unlock_path_tooltip,
            unlock_path_reputation,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub enum EventType {
    Normal,
    Artifact,
    Malfunction,
    Monster,
    Scripted,
    Traitor,
}

#[derive(Debug)]
pub struct TraitorEventPrefab {
    pub event_prefab: EventPrefab,
    pub danger_level: u32,
    pub required_previous_danger_level: u32,
    pub require_previous_danger_level_completed: bool,
    pub min_player_count: u32,
    pub secondary_traitor_amount: u32,
    pub secondary_traitor_percentage: f32,
    pub allow_accusing_secondary_traitor: bool,
    pub money_penalty_for_unfounded_traitor_accusation: u32,
    pub tags: Option<HashSet<String>>,
    pub required_completed_tags: Option<HashSet<String>>,
    pub steal_percentage_of_experience: f32,
    pub is_chainable: bool,
    pub reputation_requirements: Vec<ReputationRequirement>,
    pub mission_requirements: Vec<MissionRequirement>,
    pub level_requirements: Vec<LevelRequirement>,
    pub icon: Option<BarotraumaSprite>,
    pub icon_color: Option<Color>,
}

impl TraitorEventPrefab {
    pub fn new(element: Node) -> Self {
        const MIN_DANGER_LEVEL: u32 = 1;
        const MAX_DANGER_LEVEL: u32 = 3;

        let event_prefab = EventPrefab::new(element, None);
        let danger_level =
            element
                .attribute_ignore_ascii_case("dangerlevel")
                .map_or(MIN_DANGER_LEVEL, |v| {
                    v.parse::<u32>()
                        .unwrap()
                        .clamp(MIN_DANGER_LEVEL, MAX_DANGER_LEVEL)
                });
        let required_previous_danger_level = element
            .attribute_ignore_ascii_case("requiredpreviousdangerlevel")
            .map_or(danger_level - 1, |v| {
                v.parse::<u32>()
                    .unwrap()
                    .clamp(MIN_DANGER_LEVEL - 1, MAX_DANGER_LEVEL - 1)
            });
        let require_previous_danger_level_completed = element
            .attribute_ignore_ascii_case("requirepreviousdangerlevelcompleted")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let min_player_count = element
            .attribute_ignore_ascii_case("minplayercount")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let secondary_traitor_amount = element
            .attribute_ignore_ascii_case("secondarytraitoramount")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let secondary_traitor_percentage = element
            .attribute_ignore_ascii_case("secondarytraitorpercentage")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let allow_accusing_secondary_traitor = element
            .attribute_ignore_ascii_case("allowaccusingsecondarytraitor")
            .map_or(true, |v| v.parse::<bool>().unwrap());
        let money_penalty_for_unfounded_traitor_accusation = element
            .attribute_ignore_ascii_case("moneypenaltyforunfoundedtraitoraccusation")
            .map_or(100, |v| v.parse::<u32>().unwrap());
        let tags = element
            .attribute_ignore_ascii_case("tags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect());
        let required_completed_tags = element
            .attribute_ignore_ascii_case("requiredcompletedtags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect());
        let steal_percentage_of_experience = element
            .attribute_ignore_ascii_case("stealpercentageofexperience")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let is_chainable = element
            .attribute_ignore_ascii_case("ischainable")
            .map_or(true, |v| v.parse::<bool>().unwrap());

        let mut reputation_requirements = Vec::new();
        let mut mission_requirements = Vec::new();
        let mut level_requirements = Vec::new();
        let mut icon = None;
        let mut icon_color = None;
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "reputationrequirement" => {
                    reputation_requirements.push(ReputationRequirement::new(child));
                }
                "missionrequirement" => {
                    mission_requirements.push(MissionRequirement::new(child));
                }
                "levelrequirement" => {
                    level_requirements.push(LevelRequirement::new(child));
                }
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
                _ => (),
            }
        }

        Self {
            event_prefab,
            danger_level,
            required_previous_danger_level,
            require_previous_danger_level_completed,
            min_player_count,
            secondary_traitor_amount,
            secondary_traitor_percentage,
            allow_accusing_secondary_traitor,
            money_penalty_for_unfounded_traitor_accusation,
            tags,
            required_completed_tags,
            steal_percentage_of_experience,
            is_chainable,
            reputation_requirements,
            mission_requirements,
            level_requirements,
            icon,
            icon_color,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.event_prefab.identifier
    }
}

#[derive(Debug)]
pub struct ReputationRequirement {
    pub faction: String,
    pub operator: ComparisonOperatorType,
    pub comparison: ComparisonType,
}

impl ReputationRequirement {
    pub fn new(element: Node) -> Self {
        let faction = element
            .attribute_ignore_ascii_case("faction")
            .map(|v| v.to_owned())
            .unwrap();
        let condition_str = element.attribute_ignore_ascii_case("reputation").unwrap();
        let spl = condition_str.split_once(' ').unwrap();
        let operator = spl.0.parse::<ComparisonOperatorType>().unwrap();
        let comparison = match spl.1.parse::<f32>() {
            Ok(v) => ComparisonType::CompareToValue(v),
            Err(_) => ComparisonType::CompareToFaction(spl.1.to_owned()),
        };

        Self {
            faction,
            operator,
            comparison,
        }
    }
}

#[derive(Debug)]
pub enum ComparisonType {
    CompareToValue(f32),
    CompareToFaction(String),
}

#[derive(Debug)]
pub enum MissionRequirement {
    Identifier(String),
    Tag(String),
    Type(MissionType),
}

impl MissionRequirement {
    pub fn new(element: Node) -> Self {
        let mission_identifier = element.attribute_ignore_ascii_case("missionidentifier");
        if let Some(mission_identifier) = mission_identifier {
            return Self::Identifier(mission_identifier.to_owned());
        }
        let mission_tag = element.attribute_ignore_ascii_case("mission_tag");
        if let Some(mission_tag) = mission_tag {
            return Self::Tag(mission_tag.to_owned());
        }
        let mission_type = element
            .attribute_ignore_ascii_case("missiontype")
            .map_or(MissionType::new(), |v| v.parse().unwrap());

        Self::Type(mission_type)
    }
}

#[bitfield(u16)]

pub struct MissionType {
    pub salvage: bool,
    pub monster: bool,
    pub cargo: bool,
    pub beacon: bool,
    pub nest: bool,
    pub mineral: bool,
    pub combat: bool,
    pub abandoned_outpost: bool,
    pub escort: bool,
    pub pirate: bool,
    pub go_to: bool,
    pub scan_alien_ruins: bool,
    pub eliminate_targets: bool,
    pub end: bool,
    #[bits(2)]
    _unused: u8,
}

impl MissionType {
    pub fn all() -> Self {
        Self::new()
            .with_salvage(true)
            .with_monster(true)
            .with_cargo(true)
            .with_beacon(true)
            .with_nest(true)
            .with_mineral(true)
            .with_combat(true)
            .with_abandoned_outpost(true)
            .with_escort(true)
            .with_pirate(true)
            .with_go_to(true)
            .with_scan_alien_ruins(true)
            .with_eliminate_targets(true)
            .with_end(true)
    }
}

impl FromStr for MissionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::new()),
            "salvage" => Ok(Self::new().with_salvage(true)),
            "monster" => Ok(Self::new().with_monster(true)),
            "cargo" => Ok(Self::new().with_cargo(true)),
            "beacon" => Ok(Self::new().with_beacon(true)),
            "nest" => Ok(Self::new().with_nest(true)),
            "mineral" => Ok(Self::new().with_mineral(true)),
            "combat" => Ok(Self::new().with_combat(true)),
            "abandonedoutpost" => Ok(Self::new().with_abandoned_outpost(true)),
            "escort" => Ok(Self::new().with_escort(true)),
            "pirate" => Ok(Self::new().with_pirate(true)),
            "goto" => Ok(Self::new().with_go_to(true)),
            "scanalienruins" => Ok(Self::new().with_scan_alien_ruins(true)),
            "clearalienruins" => Ok(Self::new().with_eliminate_targets(true)),
            "end" => Ok(Self::new().with_end(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum SingleMissionType {
    Salvage,
    Monster,
    Cargo,
    Beacon,
    Nest,
    Mineral,
    Combat,
    AbandonedOutpost,
    Escort,
    Pirate,
    GoTo,
    ScanAlienRuins,
    EliminateTargets,
    End,
}

impl FromStr for SingleMissionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = if let Some(s) = s.strip_suffix("mission") {
            s
        } else {
            &s
        };
        match s.to_lowercase().as_str() {
            "salvage" => Ok(Self::Salvage),
            "monster" => Ok(Self::Monster),
            "cargo" => Ok(Self::Cargo),
            "beacon" => Ok(Self::Beacon),
            "nest" => Ok(Self::Nest),
            "mineral" => Ok(Self::Mineral),
            "combat" => Ok(Self::Combat),
            "abandonedoutpost" => Ok(Self::AbandonedOutpost),
            "escort" => Ok(Self::Escort),
            "pirate" => Ok(Self::Pirate),
            "goto" => Ok(Self::GoTo),
            "scanalienruins" => Ok(Self::ScanAlienRuins),
            "eliminatetargets" | "clearalienruins" => Ok(Self::EliminateTargets),
            "end" => Ok(Self::End),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct LevelRequirement {
    pub level_type: LevelType,
    pub location_types: Option<Vec<String>>,
    pub min_difficulty: f32,
    pub min_difficulty_in_campaign: f32,
    pub required_item_conditionals: Vec<PropertyConditional>,
}

impl LevelRequirement {
    pub fn new(element: Node) -> Self {
        let level_type = element
            .attribute_ignore_ascii_case("leveltype")
            .map_or(LevelType::Any, |v| v.parse().unwrap());
        let location_types = element
            .attribute_ignore_ascii_case("locationtype")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let min_difficulty = element
            .attribute_ignore_ascii_case("mindifficulty")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let min_difficulty_in_campaign = element
            .attribute_ignore_ascii_case("mindifficultyincampaign")
            .map_or(min_difficulty.max(5.0), |v| v.parse::<f32>().unwrap());
        let required_item_conditionals = element
            .children()
            .filter(Node::is_element)
            .filter(|child| {
                child
                    .tag_name()
                    .name()
                    .eq_ignore_ascii_case("itemconditional")
            })
            .map(|child| PropertyConditional::from_xml(child))
            .collect::<Vec<_>>();

        Self {
            level_type,
            location_types,
            min_difficulty,
            min_difficulty_in_campaign,
            required_item_conditionals,
        }
    }
}

#[derive(Debug)]
pub enum LevelType {
    LocationConnection,
    Outpost,
    Any,
}

impl FromStr for LevelType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LocationConnection" => Ok(Self::LocationConnection),
            "Outpost" => Ok(Self::Outpost),
            "Any" => Ok(Self::Any),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct EventSprite {
    pub identifier: String,
    pub sprite: BarotraumaSprite,
}

impl EventSprite {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let sprite = BarotraumaSprite::new(element);

        Self { identifier, sprite }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]

pub struct EventSet {
    pub identifier: String,
    pub biome_identifier: Option<String>,
    pub min_level_difficulty: f32,
    pub max_level_difficulty: f32,
    pub additive: bool,
    pub level_type: level_generation_parameters_file::LevelType,
    pub faction: Option<String>,
    pub location_type_identifiers: Option<Vec<String>>,
    pub min_intensity: f32,
    pub max_intensity: f32,
    pub choose_random: bool,
    pub event_count: u32,
    pub subset_count: u32,
    pub exhaustible: bool,
    pub min_distance_traveled: f32,
    pub min_mission_time: f32,
    pub allow_at_start: bool,
    pub per_ruin: bool,
    pub per_cave: bool,
    pub per_wreck: bool,
    pub disable_in_hunting_grounds: bool,
    pub ignore_cooldown: bool,
    pub ignore_intensity: bool,
    pub delay_when_crew_away: bool,
    pub once_per_level: bool,
    pub trigger_event_cooldown: bool,
    pub is_campaign_set: bool,
    pub reset_time: f32,
    pub campaign_tutorial_only: bool,
    pub force_at_discovered_number: Option<u32>,
    pub force_at_visited_number: Option<u32>,
    pub default_commonness: f32,
    pub override_commonness: HashMap<String, f32>,
    pub child_sets: Vec<EventSet>,
    pub override_event_count: HashMap<String, u32>,
    pub event_prefabs: Vec<SubEventPrefab>,
}

impl EventSet {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap_or(
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect(),
            ); //TODO: Complex identifier based on where the file is located
        let biome_identifier = element
            .attribute_ignore_ascii_case("biome")
            .map(|v| v.to_owned());
        let min_level_difficulty = element
            .attribute_ignore_ascii_case("minleveldifficulty")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let max_level_difficulty = element
            .attribute_ignore_ascii_case("maxleveldifficulty")
            .map_or(100.0, |v| {
                v.parse::<f32>().unwrap().max(min_level_difficulty)
            });
        let additive = element
            .attribute_ignore_ascii_case("additive")
            .map_or(false, |v| v.parse().unwrap());
        let level_type = element.attribute_ignore_ascii_case("leveltype").map_or(
            level_generation_parameters_file::LevelType::LocationConnection,
            |v| v.parse().unwrap(),
        );
        let faction = element
            .attribute_ignore_ascii_case("faction")
            .map(|v| v.to_owned());
        let location_type_identifiers = element
            .attribute_ignore_ascii_case("locationtype")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let min_intensity = element
            .attribute_ignore_ascii_case("minintensity")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let max_intensity = element
            .attribute_ignore_ascii_case("maxintensity")
            .map_or(100.0, |v| v.parse::<f32>().unwrap().max(min_intensity));
        let choose_random = element
            .attribute_ignore_ascii_case("chooserandom")
            .map_or(false, |v| v.parse().unwrap());
        let event_count = element
            .attribute_ignore_ascii_case("eventcount")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let subset_count = element
            .attribute_ignore_ascii_case("setcount")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let exhaustible = element
            .attribute_ignore_ascii_case("exhaustible")
            .map_or(false, |v| v.parse().unwrap());
        let min_distance_traveled = element
            .attribute_ignore_ascii_case("mindistancetraveled")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let min_mission_time = element
            .attribute_ignore_ascii_case("minmissiontime")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let allow_at_start = element
            .attribute_ignore_ascii_case("allowatstart")
            .map_or(false, |v| v.parse().unwrap());
        let per_ruin = element
            .attribute_ignore_ascii_case("perruin")
            .map_or(false, |v| v.parse().unwrap());
        let per_cave = element
            .attribute_ignore_ascii_case("percave")
            .map_or(false, |v| v.parse().unwrap());
        let per_wreck = element
            .attribute_ignore_ascii_case("perwreck")
            .map_or(false, |v| v.parse().unwrap());
        let disable_in_hunting_grounds = element
            .attribute_ignore_ascii_case("disableinhuntinggrounds")
            .map_or(false, |v| v.parse().unwrap());
        let ignore_cooldown = element
            .attribute_ignore_ascii_case("ignorecooldown")
            .map_or(per_ruin || per_cave || per_wreck, |v| v.parse().unwrap());
        let ignore_intensity = element
            .attribute_ignore_ascii_case("ignoreintensity")
            .map_or(false, |v| v.parse().unwrap());
        let delay_when_crew_away = element
            .attribute_ignore_ascii_case("delaywhencrewaway")
            .map_or(!per_ruin && !per_cave && !per_wreck, |v| v.parse().unwrap());
        let once_per_level = element
            .attribute_ignore_ascii_case("onceperlevel")
            .or(element.attribute_ignore_ascii_case("onceperoutpost"))
            .map_or(false, |v| v.parse().unwrap());
        let trigger_event_cooldown = element
            .attribute_ignore_ascii_case("triggereventcooldown")
            .map_or(true, |v| v.parse().unwrap());
        let is_campaign_set = element.attribute_ignore_ascii_case("campaign").map_or(
            matches!(
                level_type,
                level_generation_parameters_file::LevelType::Outpost
            ),
            |v| v.parse().unwrap(),
        );
        let reset_time = element
            .attribute_ignore_ascii_case("resettime")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let campaign_tutorial_only = element
            .attribute_ignore_ascii_case("campaigntutorialonly")
            .map_or(false, |v| v.parse().unwrap());
        let force_at_discovered_number = element
            .attribute_ignore_ascii_case("forceatdiscoverednr")
            .map(|v| v.parse::<u32>().unwrap());
        let force_at_visited_number = element
            .attribute_ignore_ascii_case("forceatvisitednr")
            .map(|v| v.parse::<u32>().unwrap());
        let mut default_commonness = element
            .attribute_ignore_ascii_case("commonness")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());

        let mut override_commonness = HashMap::new();
        let mut child_sets = Vec::new();
        let mut override_event_count = HashMap::new();
        let mut event_prefabs = Vec::new();
        for (i, child) in element.children().filter(Node::is_element).enumerate() {
            match child.tag_name().name().to_lowercase().as_str() {
                "commonness" => {
                    default_commonness = child
                        .attribute_ignore_ascii_case("commonness")
                        .map_or(default_commonness, |v| v.parse::<f32>().unwrap());
                    for child in element
                        .children()
                        .filter(Node::is_element)
                        .filter(|child| child.tag_name().name().eq_ignore_ascii_case("override"))
                    {
                        let level_type =
                            child.attribute("leveltype").map(|v| v.to_owned()).unwrap();
                        if !override_commonness.contains_key(&level_type) {
                            override_commonness.insert(
                                level_type,
                                child
                                    .attribute_ignore_ascii_case("commonness")
                                    .map_or(0.0, |v| v.parse::<f32>().unwrap()),
                            );
                        }
                    }
                }
                "eventset" => {
                    child_sets.push(EventSet::new(child));
                }
                "overrideeventcount" => {
                    let location_type = child
                        .attribute("locationtype")
                        .map(|v| v.to_owned())
                        .unwrap();
                    if !override_event_count.contains_key(&location_type) {
                        override_event_count.insert(
                            location_type,
                            child
                                .attribute_ignore_ascii_case("eventcount")
                                .map_or(event_count, |v| v.parse::<u32>().unwrap()),
                        );
                    }
                }
                _ => {
                    if !child.has_children() && child.has_attribute_ignore_ascii_case("identifier")
                    {
                        let identifiers = child
                            .attribute_ignore_ascii_case("identifier")
                            .map(|v| v.split(',').map(|v| v.to_string()).collect::<Vec<_>>())
                            .unwrap();
                        let commonness = child
                            .attribute_ignore_ascii_case("commonness")
                            .map(|v| v.parse::<f32>().unwrap());
                        let probability = child
                            .attribute_ignore_ascii_case("probability")
                            .map(|v| v.parse::<f32>().unwrap());
                        let faction_id = child.attribute("faction").map(|v| v.to_owned());
                        event_prefabs.push(SubEventPrefab {
                            prefab_or_identifiers: PrefabOrIdentifiers::Identifiers(identifiers),
                            commonness,
                            probability,
                            faction_id,
                        });
                    } else {
                        let prefab = EventPrefab::new(child, Some(format!("{}-{}", identifier, i)));
                        event_prefabs.push(SubEventPrefab {
                            faction_id: prefab.faction.clone(),
                            commonness: Some(prefab.commonness),
                            probability: Some(prefab.probability),
                            prefab_or_identifiers: PrefabOrIdentifiers::Prefab(prefab),
                        })
                    }
                }
            }
        }

        Self {
            identifier,
            biome_identifier,
            min_level_difficulty,
            max_level_difficulty,
            additive,
            level_type,
            faction,
            location_type_identifiers,
            min_intensity,
            max_intensity,
            choose_random,
            event_count,
            subset_count,
            exhaustible,
            min_distance_traveled,
            min_mission_time,
            allow_at_start,
            per_ruin,
            per_cave,
            per_wreck,
            disable_in_hunting_grounds,
            ignore_cooldown,
            ignore_intensity,
            delay_when_crew_away,
            once_per_level,
            trigger_event_cooldown,
            is_campaign_set,
            reset_time,
            campaign_tutorial_only,
            force_at_discovered_number,
            force_at_visited_number,
            default_commonness,
            override_commonness,
            child_sets,
            override_event_count,
            event_prefabs,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct SubEventPrefab {
    pub prefab_or_identifiers: PrefabOrIdentifiers,
    pub commonness: Option<f32>,
    pub probability: Option<f32>,
    pub faction_id: Option<String>,
}

#[derive(Debug)]
pub enum PrefabOrIdentifiers {
    Prefab(EventPrefab),
    Identifiers(Vec<String>),
}
