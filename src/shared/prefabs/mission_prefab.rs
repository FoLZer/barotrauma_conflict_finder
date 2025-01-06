use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use bitfield_struct::bitfield;
use roxmltree::Node;

use crate::shared::{
    prefabs::{
        event_prefab::SingleMissionType,
        item_prefab::{BarotraumaSprite, Color},
    },
    submarine_info::SubmarineType,
    util::NodeExp,
};

use super::{
    human_prefab::{Item, PositionType},
    item_prefab::DoesNotExistError,
    level_object_prefab::StatusEffect,
    location_type::{CharacterTeamType, LocationTypeChange},
};

#[derive(Debug)]
pub struct MissionPrefab {
    pub identifier: String,
    pub text_identifier: String,
    pub tags: Option<Vec<String>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub reward: u32,
    pub allow_retry: bool,
    pub show_in_menus: bool,
    pub show_start_message: bool,
    pub is_side_objective: bool,
    pub require_wreck: bool,
    pub require_ruin: bool,
    pub block_location_type_changes: bool,
    pub required_location_faction: Option<String>,
    pub commonness: u32,
    pub allow_other_missions_in_level: bool,
    pub difficulty: Option<u32>,
    pub show_progress_bar: bool,
    pub show_progress_in_numbers: bool,
    pub max_progress_state: u32,
    pub progress_bar_label: Option<String>,
    pub success_message_tag: Option<String>,
    pub failure_message_tag: Option<String>,
    pub sonar_label_tag: Option<String>,
    pub sonar_icon_identifier: Option<String>,
    pub multiplayer_only: bool,
    pub singleplayer_only: bool,
    pub achievement_identifier: Option<String>,
    pub unhide_entity_sub_categories: Option<Vec<String>>,
    pub display_target_hud_icons: bool,
    pub hud_icon_max_distance: f32,
    pub mission_type_specific_properties: MissionTypeSpecific,

    pub messages: Vec<Message>,
    pub allowed_location_types: Vec<String>,
    pub allowed_connection_types: Vec<(String, String)>,
    pub location_type_change_on_completed: Option<LocationTypeChange>,
    pub reputation_rewards: Vec<ReputationReward>,
    pub data_rewards: Vec<(String, SetDataActionConvert, OperationType)>,
    pub trigger_events: Vec<TriggerEvent>,
    pub icon: Option<BarotraumaSprite>,
    pub icon_color: Option<Color>,
    pub hud_icon: Option<BarotraumaSprite>,
    pub hud_icon_color: Option<Color>,
    pub override_music: HashMap<u32, String>,
    pub portraits: Vec<BarotraumaSprite>,
}

const MIN_DIFFICULTY: u32 = 1;
const MAX_DIFFICULTY: u32 = 4;

impl MissionPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let text_identifier = element
            .attribute_ignore_ascii_case("textidentifier")
            .map_or(identifier.clone(), |v| v.to_owned());
        let tags = element
            .attribute_ignore_ascii_case("tags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(|v| v.to_owned());
        let description = element
            .attribute_ignore_ascii_case("description")
            .map(|v| v.to_owned());
        let reward = element
            .attribute_ignore_ascii_case("reward")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let allow_retry = element
            .attribute_ignore_ascii_case("allowretry")
            .map_or(false, |v| v.parse().unwrap());
        let show_in_menus = element
            .attribute_ignore_ascii_case("showinmenus")
            .map_or(true, |v| v.parse().unwrap());
        let show_start_message = element
            .attribute_ignore_ascii_case("showstartmessage")
            .map_or(true, |v| v.parse().unwrap());
        let is_side_objective = element
            .attribute_ignore_ascii_case("sideobjective")
            .map_or(false, |v| v.parse().unwrap());
        let require_wreck = element
            .attribute_ignore_ascii_case("requirewreck")
            .map_or(false, |v| v.parse().unwrap());
        let require_ruin = element
            .attribute_ignore_ascii_case("requireruin")
            .map_or(false, |v| v.parse().unwrap());
        let block_location_type_changes = element
            .attribute_ignore_ascii_case("blocklocationtypechanges")
            .map_or(false, |v| v.parse().unwrap());
        let required_location_faction = element
            .attribute_ignore_ascii_case("requiredlocationfaction")
            .map(|v| v.to_owned());
        let commonness = element
            .attribute_ignore_ascii_case("commonness")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let allow_other_missions_in_level = element
            .attribute_ignore_ascii_case("allowothermissionsinlevel")
            .map_or(true, |v| v.parse().unwrap());
        let difficulty = element.attribute_ignore_ascii_case("difficulty").map(|v| {
            v.parse::<u32>()
                .unwrap()
                .clamp(MIN_DIFFICULTY, MAX_DIFFICULTY)
        });
        let show_progress_bar = element
            .attribute_ignore_ascii_case("showprogressbar")
            .map_or(false, |v| v.parse().unwrap());
        let show_progress_in_numbers = element
            .attribute_ignore_ascii_case("showprogressinnumbers")
            .map_or(false, |v| v.parse().unwrap());
        let max_progress_state = element
            .attribute_ignore_ascii_case("maxprogressstate")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let progress_bar_label = element
            .attribute_ignore_ascii_case("progressbarlabel")
            .map(|v| v.to_owned());
        let success_message_tag = element
            .attribute_ignore_ascii_case("successmessage")
            .map(|v| v.to_owned());
        let failure_message_tag = element
            .attribute_ignore_ascii_case("failuremessage")
            .map(|v| v.to_owned());
        let sonar_label_tag = element
            .attribute_ignore_ascii_case("sonarlabel")
            .map(|v| v.to_owned());
        let sonar_icon_identifier = element
            .attribute_ignore_ascii_case("sonaricon")
            .map(|v| v.to_owned());
        let multiplayer_only = element
            .attribute_ignore_ascii_case("multiplayeronly")
            .map_or(false, |v| v.parse().unwrap());
        let singleplayer_only = element
            .attribute_ignore_ascii_case("singleplayeronly")
            .map_or(false, |v| v.parse().unwrap());
        let achievement_identifier = element
            .attribute_ignore_ascii_case("achievementidentifier")
            .map(|v| v.to_owned());
        let unhide_entity_sub_categories = element
            .attribute_ignore_ascii_case("unhideentitysubcategories")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let display_target_hud_icons = element
            .attribute_ignore_ascii_case("displaytargethudicons")
            .map_or(false, |v| v.parse().unwrap());
        let hud_icon_max_distance = element
            .attribute_ignore_ascii_case("hudiconmaxdistance")
            .map_or(1000.0, |v| v.parse::<f32>().unwrap());

        let ty = element
            .attribute_ignore_ascii_case("type")
            .map(|v| v.to_owned())
            .unwrap();

        let mission_class: SingleMissionType = if let Ok(v) = element.tag_name().name().parse() {
            v
        } else {
            if ty.eq_ignore_ascii_case("outpostdestroy") || ty.eq_ignore_ascii_case("outpostrescue")
            {
                SingleMissionType::AbandonedOutpost //backwards compatibility
            } else {
                ty.parse().unwrap()
            }
        };

        let mut messages = Vec::new();
        let mut allowed_location_types = Vec::new();
        let mut allowed_connection_types = Vec::new();
        let mut location_type_change_on_completed = None;
        let mut reputation_rewards = Vec::new();
        let mut data_rewards = Vec::new();
        let mut trigger_events = Vec::new();
        let mut icon = None;
        let mut icon_color = None;
        let mut hud_icon = None;
        let mut hud_icon_color = None;
        let mut override_music = HashMap::new();
        let mut portraits = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "message" => {
                    messages.push(Message {
                        header: child
                            .attribute_ignore_ascii_case("header")
                            .map(|v| v.to_owned()),
                        text: child
                            .attribute_ignore_ascii_case("text")
                            .map(|v| v.to_owned()),
                    });
                }
                "locationtype" | "connectiontype" => {
                    if let Some(identifier) = child
                        .attribute_ignore_ascii_case("identifier")
                        .map(|v| v.to_owned())
                    {
                        allowed_location_types.push(identifier);
                    } else {
                        allowed_connection_types.push((
                            child
                                .attribute_ignore_ascii_case("from")
                                .map(|v| v.to_owned())
                                .unwrap(),
                            child
                                .attribute_ignore_ascii_case("to")
                                .map(|v| v.to_owned())
                                .unwrap(),
                        ));
                    }
                }
                "locationtypechange" => {
                    location_type_change_on_completed = Some(LocationTypeChange::new(
                        child,
                        &child
                            .attribute_ignore_ascii_case("from")
                            .map(|v| v.to_owned())
                            .unwrap(),
                    ));
                }
                "reputation" | "reputationreward" => {
                    reputation_rewards.push(ReputationReward::new(child));
                }
                "metadata" => {
                    let identifier = child
                        .attribute_ignore_ascii_case("identifier")
                        .map(|v| v.to_owned())
                        .unwrap();
                    let value = child
                        .attribute_ignore_ascii_case("value")
                        .map(|v| v.parse::<SetDataActionConvert>().unwrap())
                        .unwrap();
                    let operation = child
                        .attribute_ignore_ascii_case("operation")
                        .map_or(OperationType::Set, |v| v.parse::<OperationType>().unwrap());
                    data_rewards.push((identifier, value, operation));
                }
                "triggerevent" => {
                    trigger_events.push(TriggerEvent::new(child));
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
                "hudicon" => {
                    hud_icon = Some(BarotraumaSprite::new(child));
                    hud_icon_color = Some(child.attribute_ignore_ascii_case("color").map_or(
                        Color::Simple {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        },
                        |v| v.parse().unwrap(),
                    ));
                }
                "overridemusic" => {
                    override_music.insert(
                        child
                            .attribute_ignore_ascii_case("state")
                            .map_or(0, |v| v.parse::<u32>().unwrap()),
                        child
                            .attribute_ignore_ascii_case("type")
                            .map(|v| v.to_owned())
                            .unwrap(),
                    );
                }
                "portrait" => {
                    portraits.push(BarotraumaSprite::new(child));
                }
                _ => (),
            }
        }

        let mission_type_specific_properties = match mission_class {
            SingleMissionType::Salvage => {
                MissionTypeSpecific::Salvage(SalvageMission::new(element))
            }
            SingleMissionType::Monster => {
                MissionTypeSpecific::Monster(MonsterMission::new(element))
            }
            SingleMissionType::Cargo => MissionTypeSpecific::Cargo(CargoMission::new(element)),
            SingleMissionType::Beacon => MissionTypeSpecific::Beacon(BeaconMission::new(element)),
            SingleMissionType::Nest => MissionTypeSpecific::Nest(NestMission::new(element)),
            SingleMissionType::Mineral => {
                MissionTypeSpecific::Mineral(MineralMission::new(element))
            }
            SingleMissionType::AbandonedOutpost => {
                MissionTypeSpecific::AbandonedOutpost(AbandonedOutpostMission::new(element))
            }
            SingleMissionType::Escort => MissionTypeSpecific::Escort(EscortMission::new(element)),
            SingleMissionType::Pirate => MissionTypeSpecific::Pirate(PirateMission::new(element)),
            SingleMissionType::GoTo => MissionTypeSpecific::GoTo(GoToMission::new(element)),
            SingleMissionType::ScanAlienRuins => {
                MissionTypeSpecific::ScanAlienRuins(ScanMission::new(element))
            }
            SingleMissionType::EliminateTargets => {
                MissionTypeSpecific::EliminateTargets(EliminateTargetsMission::new(element))
            }
            SingleMissionType::End => MissionTypeSpecific::End(EndMission::new(element)),
            SingleMissionType::Combat => MissionTypeSpecific::Combat(CombatMission::new(element)),
        };

        Self {
            identifier,
            text_identifier,
            tags,
            name,
            description,
            reward,
            allow_retry,
            show_in_menus,
            show_start_message,
            is_side_objective,
            require_wreck,
            require_ruin,
            block_location_type_changes,
            required_location_faction,
            commonness,
            allow_other_missions_in_level,
            difficulty,
            show_progress_bar,
            show_progress_in_numbers,
            max_progress_state,
            progress_bar_label,
            success_message_tag,
            failure_message_tag,
            sonar_label_tag,
            sonar_icon_identifier,
            multiplayer_only,
            singleplayer_only,
            achievement_identifier,
            unhide_entity_sub_categories,
            display_target_hud_icons,
            hud_icon_max_distance,
            mission_type_specific_properties,
            messages,
            allowed_location_types,
            allowed_connection_types,
            location_type_change_on_completed,
            reputation_rewards,
            data_rewards,
            trigger_events,
            icon,
            icon_color,
            hud_icon,
            hud_icon_color,
            override_music,
            portraits,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub enum MissionTypeSpecific {
    Salvage(SalvageMission),
    Monster(MonsterMission),
    Cargo(CargoMission),
    Beacon(BeaconMission),
    Nest(NestMission),
    Mineral(MineralMission),
    AbandonedOutpost(AbandonedOutpostMission),
    Escort(EscortMission),
    Pirate(PirateMission),
    GoTo(GoToMission),
    ScanAlienRuins(ScanMission),
    EliminateTargets(EliminateTargetsMission),
    End(EndMission),
    Combat(CombatMission),
}

#[derive(Debug)]
pub struct SalvageMission {
    pub required_delivery_amount: f32,
    pub partially_retrieved_message: Option<String>,
    pub all_retrieved_message: Option<String>,
    pub targets: Vec<Vec<SalvageTargetWithAmount>>,
    pub mission: Mission,
}
impl SalvageMission {
    pub fn new(element: Node) -> Self {
        let required_delivery_amount = element
            .attribute_ignore_ascii_case("requireddeliveryamount")
            .map_or(0.98, |v| v.parse().unwrap());
        let partially_retrieved_message = element
            .attribute_ignore_ascii_case("partiallyretrievedmessage")
            .map(|v| v.to_owned());
        let all_retrieved_message = element
            .attribute_ignore_ascii_case("allretrievedmessage")
            .map(|v| v.to_owned());

        let mut targets = element
            .children()
            .filter(Node::is_element)
            .filter(|child| {
                child.tag_name().name().eq_ignore_ascii_case("target")
                    || child.tag_name().name().eq_ignore_ascii_case("chooserandom")
            })
            .filter_map(|child| Self::load_targets(child))
            .collect::<Vec<_>>();
        if targets.is_empty() {
            targets.push(vec![SalvageTargetWithAmount {
                amount: Some(1),
                min_amount: None,
                max_amount: None,
                target: SalvageTarget::new(element),
                internal_targets: vec![],
            }]);
        }
        Self {
            required_delivery_amount,
            partially_retrieved_message,
            all_retrieved_message,
            targets,
            mission: Mission::new(element),
        }
    }

    fn load_targets(element: Node) -> Option<Vec<SalvageTargetWithAmount>> {
        if element
            .tag_name()
            .name()
            .eq_ignore_ascii_case("chooserandom")
        {
            if element
                .children()
                .filter(Node::is_element)
                .any(|child| child.tag_name().name().eq_ignore_ascii_case("statuseffect"))
            {
                return None;
            }
            let mut random_targets = Vec::new();
            for child in element.children().filter(Node::is_element) {
                let amount = child
                    .attribute_ignore_ascii_case("amount")
                    .map(|v| v.parse().unwrap());
                let min_amount = child
                    .attribute_ignore_ascii_case("minamount")
                    .map(|v| v.parse().unwrap());
                let max_amount = child
                    .attribute_ignore_ascii_case("maxamount")
                    .map(|v| v.parse().unwrap());
                let target = SalvageTarget::new(child);
                let mut internal_targets = Vec::new();
                for child in child.children().filter(Node::is_element) {
                    if let Some(t) = Self::load_targets(child) {
                        internal_targets.push(t);
                    }
                }
                random_targets.push(SalvageTargetWithAmount {
                    target,
                    amount,
                    min_amount,
                    max_amount,
                    internal_targets,
                });
            }
            Some(random_targets)
        } else {
            let amount = element
                .attribute_ignore_ascii_case("amount")
                .map(|v| v.parse().unwrap());
            let min_amount = element
                .attribute_ignore_ascii_case("minamount")
                .map(|v| v.parse().unwrap());
            let max_amount = element
                .attribute_ignore_ascii_case("maxamount")
                .map(|v| v.parse().unwrap());
            let target = SalvageTarget::new(element);
            let mut internal_targets = Vec::new();
            for child in element.children().filter(Node::is_element) {
                if let Some(t) = Self::load_targets(child) {
                    internal_targets.push(t);
                }
            }
            Some(vec![SalvageTargetWithAmount {
                amount,
                min_amount,
                max_amount,
                target,
                internal_targets,
            }])
        }
    }
}
#[derive(Debug)]
pub struct SalvageTarget {
    pub container_tag: Option<String>,
    pub required_retreival_state: RetreivalState,
    pub allow_continue_before_retrieved: bool,
    pub hide_label_after_retrieved: bool,
    pub require_inside_original_container: bool,
    pub sonar_label_tag: Option<String>,
    pub existing_item_tag: Option<String>,
    pub remove_item: bool,
    pub item_prefab_ident: Option<ItemIdentifierOrTag>,
    pub status_effects: Vec<StatusEffect>,
    pub random_status_effects: Vec<Vec<StatusEffect>>,
    pub spawn_position_type: PositionType,
}
impl SalvageTarget {
    pub fn new(element: Node) -> Self {
        let container_tag = element
            .attribute_ignore_ascii_case("containertag")
            .map(|v| v.to_owned());
        let required_retreival_state = element
            .attribute_ignore_ascii_case("requireretrieval")
            .map_or(RetreivalState::RetrievedToSub, |v| v.parse().unwrap());
        let allow_continue_before_retrieved = element
            .attribute_ignore_ascii_case("allowcontinuebeforeretrieved")
            .map_or(false, |v| v.parse().unwrap());
        let hide_label_after_retrieved = element
            .attribute_ignore_ascii_case("hidelabelafterretrieved")
            .map_or(false, |v| v.parse().unwrap());
        let require_inside_original_container = element
            .attribute_ignore_ascii_case("requireinsideoriginalcontainer")
            .map_or(false, |v| v.parse().unwrap());
        let sonar_label_tag = element
            .attribute_ignore_ascii_case("sonarlabel")
            .map(|v| v.to_owned());
        let existing_item_tag = element
            .attribute_ignore_ascii_case("existingitemtag")
            .map(|v| v.to_owned());
        let remove_item = element
            .attribute_ignore_ascii_case("removeitem")
            .map_or(true, |v| v.parse().unwrap());

        let item_prefab_ident = if element.has_attribute_ignore_ascii_case("itemname") {
            panic!()
        } else if let Some(item_identifier) = element
            .attribute_ignore_ascii_case("itemidentifier")
            .map(|v| v.to_owned())
        {
            Some(ItemIdentifierOrTag::Identifier(item_identifier))
        } else if let Some(item_tag) = element
            .attribute_ignore_ascii_case("itemtag")
            .map(|v| v.to_owned())
        {
            Some(ItemIdentifierOrTag::Tag(item_tag))
        } else if existing_item_tag.is_none() {
            panic!()
        } else {
            None
        };

        let mut status_effects = Vec::new();
        let mut random_status_effects = Vec::new();
        let spawn_position_type = element
            .attribute_ignore_ascii_case("spawntype")
            .map_or(PositionType::new().with_cave(true).with_ruin(true), |v| {
                v.parse().unwrap()
            });

        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "statuseffect" => {
                    status_effects.push(StatusEffect::new(child));
                }
                "chooserandom" => {
                    random_status_effects.push(
                        child
                            .children()
                            .filter(Node::is_element)
                            .map(|child| StatusEffect::new(child))
                            .collect::<Vec<_>>(),
                    );
                }
                _ => (),
            }
        }

        Self {
            container_tag,
            required_retreival_state,
            allow_continue_before_retrieved,
            hide_label_after_retrieved,
            require_inside_original_container,
            sonar_label_tag,
            existing_item_tag,
            remove_item,
            item_prefab_ident,
            status_effects,
            random_status_effects,
            spawn_position_type,
        }
    }
}
#[derive(Debug)]

pub struct SalvageTargetWithAmount {
    pub amount: Option<i32>,
    pub min_amount: Option<i32>,
    pub max_amount: Option<i32>,
    pub target: SalvageTarget,
    pub internal_targets: Vec<Vec<SalvageTargetWithAmount>>,
}
#[derive(Debug)]
pub enum RetreivalState {
    None,
    Interact,
    PickedUp,
    RetrievedToSub,
}
impl FromStr for RetreivalState {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Interact" => Ok(Self::Interact),
            "PickedUp" => Ok(Self::PickedUp),
            "RetrievedToSub" => Ok(Self::RetrievedToSub),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
#[derive(Debug)]
pub enum ItemIdentifierOrTag {
    Identifier(String),
    Tag(String),
}
#[derive(Debug)]
pub struct MonsterMission {
    pub monster_prefabs: Vec<(String, RangeInclusive<u32>)>,
    pub max_sonar_marker_distance: f32,
    pub spawn_pos_type: PositionType,
    pub mission: Mission,
}
impl MonsterMission {
    pub fn new(element: Node) -> Self {
        let mut monster_prefabs = Vec::new();
        if let Some(species_name) = element
            .attribute_ignore_ascii_case("monsterfile")
            .map(|v| v.to_owned())
        {
            let monster_count = element
                .attribute_ignore_ascii_case("monstercount")
                .map_or(1, |v| v.parse::<u32>().unwrap().min(255));
            monster_prefabs.push((species_name, monster_count..=monster_count));
        }
        let max_sonar_marker_distance = element
            .attribute_ignore_ascii_case("maxsonarmarkerdistance")
            .map_or(10000.0, |v| v.parse::<f32>().unwrap());
        let spawn_pos_type = element.attribute_ignore_ascii_case("spawntype").map_or(
            PositionType::new()
                .with_main_path(true)
                .with_side_path(true),
            |v| v.parse().unwrap(),
        );
        for child in element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("monster"))
        {
            let species_name = child
                .attribute_ignore_ascii_case("character")
                .map(|v| v.to_owned())
                .unwrap();
            let default_count = child
                .attribute_ignore_ascii_case("count")
                .or(child.attribute_ignore_ascii_case("amount"))
                .map_or(1, |v| v.parse::<u32>().unwrap());
            let min = child
                .attribute_ignore_ascii_case("min")
                .map_or(default_count, |v| v.parse::<u32>().unwrap().min(255));
            let max = child
                .attribute_ignore_ascii_case("max")
                .map_or(default_count, |v| v.parse::<u32>().unwrap().clamp(min, 255));
            monster_prefabs.push((species_name, min..=max));
        }

        Self {
            monster_prefabs,
            max_sonar_marker_distance,
            spawn_pos_type,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct CargoMission {
    pub required_delivery_amount: f32,
    pub items: Vec<CargoItem>,
    pub mission: Mission,
}
impl CargoMission {
    pub fn new(element: Node) -> Self {
        let required_delivery_amount = element
            .attribute_ignore_ascii_case("requireddeliveryamount")
            .map_or(0.98, |v| v.parse::<f32>().unwrap().min(1.0));
        let items = element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("items"))
            .unwrap()
            .children()
            .filter(Node::is_element)
            .map(|child| CargoItem::new(child))
            .collect::<Vec<_>>();
        Self {
            required_delivery_amount,
            items,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]

pub struct CargoItem {
    pub max_count: u32,
    pub reward: u32,
    pub item_indentifier: String,
    pub child_items: Vec<(CargoItem, u32)>,
    pub mission: Mission,
}
impl CargoItem {
    pub fn new(element: Node) -> Self {
        Self {
            max_count: element
                .attribute_ignore_ascii_case("maxcount")
                .map_or(10, |v| v.parse::<u32>().unwrap()),
            reward: element
                .attribute_ignore_ascii_case("reward")
                .map_or(10, |v: &str| v.parse::<u32>().unwrap()),
            item_indentifier: element
                .attribute_ignore_ascii_case("identifier")
                .map(|v| v.to_owned())
                .unwrap(),
            child_items: element
                .children()
                .filter(Node::is_element)
                .map(|child| {
                    (
                        CargoItem::new(child),
                        element
                            .attribute_ignore_ascii_case("amount")
                            .map_or(1, |v: &str| v.parse::<u32>().unwrap()),
                    )
                })
                .collect(),
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct BeaconMission {
    pub monster_sets: Vec<(Vec<(String, RangeInclusive<u32>)>, f32)>,
    pub mission: Mission,
}
impl BeaconMission {
    pub fn new(element: Node) -> Self {
        let mut monster_sets = Vec::new();

        for child in element
            .children()
            .filter(Node::is_element)
            .filter(|v| v.tag_name().name().eq_ignore_ascii_case("monster"))
        {
            let set = if monster_sets.is_empty() {
                let commonness = child
                    .attribute_ignore_ascii_case("commonness")
                    .map_or(100.0, |v| v.parse::<f32>().unwrap());

                monster_sets.push((Vec::new(), commonness));
                &mut monster_sets[0].0
            } else {
                &mut monster_sets[0].0
            };

            let species_name = child
                .attribute_ignore_ascii_case("character")
                .map(|v| v.to_owned())
                .unwrap();
            let default_count = child
                .attribute_ignore_ascii_case("count")
                .or(child.attribute_ignore_ascii_case("amount"))
                .map_or(1, |v| v.parse::<u32>().unwrap());
            let min = child
                .attribute_ignore_ascii_case("min")
                .map_or(default_count, |v| v.parse::<u32>().unwrap().min(255));
            let max = child
                .attribute_ignore_ascii_case("max")
                .map_or(default_count, |v| v.parse::<u32>().unwrap().clamp(min, 255));
            set.push((species_name, min..=max));
        }
        for child in element
            .children()
            .filter(Node::is_element)
            .filter(|v| v.tag_name().name().eq_ignore_ascii_case("monsters"))
        {
            let commonness = child
                .attribute_ignore_ascii_case("commonness")
                .map_or(100.0, |v| v.parse::<f32>().unwrap());
            let mut monster_prefabs = Vec::new();
            for child in child
                .children()
                .filter(Node::is_element)
                .filter(|child| child.tag_name().name().eq_ignore_ascii_case("monster"))
            {
                let species_name = child
                    .attribute_ignore_ascii_case("character")
                    .map(|v| v.to_owned())
                    .unwrap();
                let default_count = child
                    .attribute_ignore_ascii_case("count")
                    .or(child.attribute_ignore_ascii_case("amount"))
                    .map_or(1, |v| v.parse::<u32>().unwrap());
                let min = child
                    .attribute_ignore_ascii_case("min")
                    .map_or(default_count, |v| v.parse::<u32>().unwrap().min(255));
                let max = child
                    .attribute_ignore_ascii_case("max")
                    .map_or(default_count, |v| v.parse::<u32>().unwrap().clamp(min, 255));
                monster_prefabs.push((species_name, min..=max));
            }
            monster_sets.push((monster_prefabs, commonness));
        }
        Self {
            monster_sets,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct AbandonedOutpostMission {
    pub allow_ordering_rescuees: bool,
    pub hostages_killed_message: Option<String>,
    pub item_tag: Option<String>,
    pub humans: Vec<AbandonedOutpostHuman>,
    pub monsters: Vec<AbandonedOutpostMonster>,
    pub items: Option<Vec<AbandonedOutpostItem>>,
    pub mission: Mission,
}
impl AbandonedOutpostMission {
    pub fn new(element: Node) -> Self {
        let allow_ordering_rescuees = element
            .attribute_ignore_ascii_case("alloworderingrescuees")
            .map_or(true, |v| v.parse().unwrap());
        let hostages_killed_message = element
            .attribute_ignore_ascii_case("hostageskilledmessage")
            .map(|v| v.to_owned());
        let item_tag = element
            .attribute_ignore_ascii_case("targetitem")
            .map(|v| v.to_owned());

        let mut humans = Vec::new();
        let mut monsters = Vec::new();
        for child in element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("characters"))
            .unwrap()
            .children()
            .filter(Node::is_element)
        {
            if child.has_attribute_ignore_ascii_case("identifier")
                && child.has_attribute_ignore_ascii_case("from")
            {
                humans.push(AbandonedOutpostHuman::new(child));
            } else {
                monsters.push(AbandonedOutpostMonster::new(child));
            }
        }

        let items = element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("items"))
            .map(|child| {
                child
                    .children()
                    .filter(Node::is_element)
                    .map(|child| AbandonedOutpostItem::new(child))
                    .collect::<Vec<_>>()
            });

        Self {
            allow_ordering_rescuees,
            hostages_killed_message,
            item_tag,
            humans,
            monsters,
            items,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct NestMission {
    pub item_spawn_radius: f32,
    pub approach_items_radius: f32,
    pub monster_spawn_radius: f32,
    pub nest_object_radius: f32,
    pub nest_object_amount: u32,
    pub require_delivery: bool,
    pub spawn_position_type: PositionType,
    pub items: Vec<NestItem>,
    pub monster_prefabs: Vec<NestMonsterPrefab>,
    pub mission: Mission,
}
impl NestMission {
    pub fn new(element: Node) -> Self {
        let item_spawn_radius = element
            .attribute_ignore_ascii_case("itemspawnradius")
            .map_or(800.0, |v| v.parse::<f32>().unwrap());
        let approach_items_radius = element
            .attribute_ignore_ascii_case("approachitemsradius")
            .map_or(item_spawn_radius * 2.0, |v| v.parse::<f32>().unwrap());
        let monster_spawn_radius = element
            .attribute_ignore_ascii_case("monsterspawnradius")
            .map_or(approach_items_radius * 2.0, |v| v.parse::<f32>().unwrap());

        let nest_object_radius = element
            .attribute_ignore_ascii_case("nestobjectradius")
            .map_or(item_spawn_radius * 2.0, |v| v.parse::<f32>().unwrap());
        let nest_object_amount = element
            .attribute_ignore_ascii_case("nestobjectamount")
            .map_or(10, |v| v.parse::<u32>().unwrap());

        let require_delivery = element
            .attribute_ignore_ascii_case("requiredelivery")
            .map_or(false, |v| v.parse().unwrap());

        let spawn_position_type = element
            .attribute_ignore_ascii_case("spawntype")
            .map_or(PositionType::new().with_cave(true).with_ruin(true), |v| {
                v.parse().unwrap()
            });

        let items = element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("items"))
            .unwrap()
            .children()
            .filter(Node::is_element)
            .map(|child| NestItem::new(child))
            .collect::<Vec<_>>();

        let monster_prefabs = element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("monster"))
            .map(|child| NestMonsterPrefab::new(child))
            .collect::<Vec<_>>();

        Self {
            item_spawn_radius,
            approach_items_radius,
            monster_spawn_radius,
            nest_object_radius,
            nest_object_amount,
            require_delivery,
            spawn_position_type,
            items,
            monster_prefabs,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct MineralMission {
    pub position_type: MineralSpawnPositionType,
    pub handover_amount: f32,
    pub resource_amounts: HashMap<String, u32>,
    pub mission: Mission,
}
impl MineralMission {
    pub fn new(element: Node) -> Self {
        let position_type = element
            .attribute_ignore_ascii_case("PositionType")
            .map_or(MineralSpawnPositionType::Cave, |v| v.parse().unwrap());
        let handover_amount = element
            .attribute_ignore_ascii_case("ResourceHandoverAmount")
            .map_or(0.0, |v| v.parse::<f32>().unwrap().clamp(0.0, 1.0));

        let mut resource_amounts: HashMap<String, u32> = HashMap::new();
        for child in element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("Items"))
            .unwrap()
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("Item"))
        {
            let identifier = child.attribute_ignore_ascii_case("identifier").unwrap();
            if let Some(v) = resource_amounts.get_mut(identifier) {
                *v += 1;
            } else {
                resource_amounts.insert(identifier.to_owned(), 1);
            }
        }

        Self {
            position_type,
            handover_amount,
            resource_amounts,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct CombatMission {
    pub description_neutral: Option<String>,
    pub description_team1: Option<String>,
    pub description_team2: Option<String>,
    pub team1_name: Option<String>,
    pub team2_name: Option<String>,
    pub mission: Mission,
}
impl CombatMission {
    pub fn new(element: Node) -> Self {
        let description_neutral = element
            .attribute_ignore_ascii_case("descriptionneutral")
            .map(|v| v.to_owned());
        let description_team1 = element
            .attribute_ignore_ascii_case("description1")
            .map(|v| v.to_owned());
        let description_team2 = element
            .attribute_ignore_ascii_case("description2")
            .map(|v| v.to_owned());
        let team1_name = element
            .attribute_ignore_ascii_case("teamname1")
            .map(|v| v.to_owned());
        let team2_name = element
            .attribute_ignore_ascii_case("teamname2")
            .map(|v| v.to_owned());

        Self {
            description_neutral,
            description_team1,
            description_team2,
            team1_name,
            team2_name,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct EscortMission {
    pub base_escorted_characters: u32,
    pub scaling_escorted_characters: f32,
    pub terrorist_chance: f32,
    pub terrorist_announce_dialog_tag: Option<String>,
    pub characters: Vec<EscortMissionCharacter>,
    pub possible_terrorist_items: Option<Vec<(f32, Item)>>,
    pub mission: Mission,
}
impl EscortMission {
    pub fn new(element: Node) -> Self {
        let base_escorted_characters = element
            .attribute_ignore_ascii_case("baseescortedcharacters")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let scaling_escorted_characters = element
            .attribute_ignore_ascii_case("scalingescortedcharacters")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let terrorist_chance = element
            .attribute_ignore_ascii_case("terroristchance")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let terrorist_announce_dialog_tag = element
            .attribute_ignore_ascii_case("terroristannouncedialogtag")
            .map(|v| v.to_owned());
        let characters = element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("Characters"))
            .unwrap()
            .children()
            .filter(Node::is_element)
            .map(|child| EscortMissionCharacter::new(child))
            .collect::<Vec<_>>();
        let possible_terrorist_items = element
            .children()
            .filter(Node::is_element)
            .find(|child| {
                child
                    .tag_name()
                    .name()
                    .eq_ignore_ascii_case("TerroristItems")
            })
            .map(|child| {
                child
                    .children()
                    .filter(Node::is_element)
                    .map(|child| {
                        let min_difficulty = child
                            .attribute_ignore_ascii_case("mindifficulty")
                            .map_or(0.0, |v| v.parse::<f32>().unwrap());
                        let item = Item::new(child);
                        (min_difficulty, item)
                    })
                    .collect::<Vec<_>>()
            });

        Self {
            base_escorted_characters,
            scaling_escorted_characters,
            terrorist_chance,
            terrorist_announce_dialog_tag,
            characters,
            possible_terrorist_items,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct PirateMission {
    pub added_mission_difficulty_per_player: f32,
    pub submarine_type_configs: Option<Vec<PirateSubmarineConfig>>,
    pub character_configs: Vec<(String, RangeInclusive<u32>)>,
    pub character_type_configs: Vec<(String, Vec<PirateCharacterTypeVariant>)>,
    pub mission: Mission,
}
impl PirateMission {
    pub fn new(element: Node) -> Self {
        let added_mission_difficulty_per_player = element
            .attribute_ignore_ascii_case("addedmissiondifficultyperplayer")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let submarine_type_configs = element
            .children()
            .filter(Node::is_element)
            .find(|child| {
                child
                    .tag_name()
                    .name()
                    .eq_ignore_ascii_case("SubmarineTypes")
            })
            .map(|v| {
                v.children()
                    .filter(Node::is_element)
                    .map(|child| PirateSubmarineConfig::new(child))
                    .collect::<Vec<_>>()
            });
        let character_configs = element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("Characters"))
            .unwrap()
            .children()
            .filter(Node::is_element)
            .map(|child| {
                let character_type_identifier = child
                    .attribute_ignore_ascii_case("typeidentifier")
                    .map(|v| v.to_owned())
                    .unwrap();
                let min_amount = child
                    .attribute_ignore_ascii_case("minamount")
                    .map_or(0, |v| v.parse::<u32>().unwrap());
                let max_amount = child
                    .attribute_ignore_ascii_case("maxamount")
                    .map_or(0, |v| v.parse::<u32>().unwrap());
                (character_type_identifier, min_amount..=max_amount)
            })
            .collect::<Vec<_>>();
        let character_type_configs = element
            .children()
            .filter(Node::is_element)
            .find(|child| {
                child
                    .tag_name()
                    .name()
                    .eq_ignore_ascii_case("CharacterTypes")
            })
            .unwrap()
            .children()
            .filter(Node::is_element)
            .map(|child| {
                let type_identifier = child
                    .attribute_ignore_ascii_case("typeidentifier")
                    .map(|v| v.to_owned())
                    .unwrap();
                let variants = child
                    .children()
                    .filter(Node::is_element)
                    .map(|child| PirateCharacterTypeVariant::new(child))
                    .collect::<Vec<_>>();
                (type_identifier, variants)
            })
            .collect::<Vec<_>>();

        Self {
            added_mission_difficulty_per_player,
            submarine_type_configs,
            character_configs,
            character_type_configs,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct GoToMission {
    pub mission: Mission,
}
impl GoToMission {
    pub fn new(element: Node) -> Self {
        Self {
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct EndMission {
    pub boss_species_name: String,
    pub minion_species_name: String,
    pub minion_count: u32,
    pub minion_scatter: f32,
    pub projectile_identifier: String,
    pub spawn_point_tag: String,
    pub destructible_item_tag: String,
    pub end_cinematic_sound: String,
    pub start_cinematic_distance: u32,
    pub mission: Mission,
}
impl EndMission {
    pub fn new(element: Node) -> Self {
        let boss_species_name = element
            .attribute_ignore_ascii_case("bossfile")
            .map(|v| v.to_owned())
            .unwrap();
        let minion_species_name = element
            .attribute_ignore_ascii_case("minionfile")
            .map(|v| v.to_owned())
            .unwrap();
        let minion_count = element
            .attribute_ignore_ascii_case("minioncount")
            .map_or(0, |v| v.parse::<u32>().unwrap().min(255));
        let minion_scatter = element
            .attribute_ignore_ascii_case("minionscatter")
            .map_or(0.0, |v| v.parse::<f32>().unwrap().min(10000.0));
        let projectile_identifier = element
            .attribute_ignore_ascii_case("projectile")
            .map(|v| v.to_owned())
            .unwrap();
        let spawn_point_tag = element
            .attribute_ignore_ascii_case("spawnPointTag")
            .map(|v| v.to_owned())
            .unwrap();
        let destructible_item_tag = element
            .attribute_ignore_ascii_case("destructibleItemTag")
            .map(|v| v.to_owned())
            .unwrap();
        let end_cinematic_sound = element
            .attribute_ignore_ascii_case("endCinematicSound")
            .map(|v| v.to_owned())
            .unwrap();
        let start_cinematic_distance = element
            .attribute_ignore_ascii_case("startCinematicDistance")
            .map_or(0, |v| v.parse::<u32>().unwrap());

        Self {
            boss_species_name,
            minion_species_name,
            minion_count,
            minion_scatter,
            projectile_identifier,
            spawn_point_tag,
            destructible_item_tag,
            end_cinematic_sound,
            start_cinematic_distance,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct ScanMission {
    pub targets_to_scan: u32,
    pub min_target_distance: f32,
    pub items: Vec<Item>,
    pub mission: Mission,
}
impl ScanMission {
    pub fn new(element: Node) -> Self {
        let targets_to_scan = element
            .attribute_ignore_ascii_case("targets")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let min_target_distance = element
            .attribute_ignore_ascii_case("mintargetdistance")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let items = element
            .children()
            .filter(Node::is_element)
            .find(|child| child.tag_name().name().eq_ignore_ascii_case("Items"))
            .unwrap()
            .children()
            .filter(Node::is_element)
            .map(|child| Item::new(child))
            .collect::<Vec<_>>();

        Self {
            targets_to_scan,
            min_target_distance,
            items,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct EliminateTargetsMission {
    pub target_item_identifiers: Option<Vec<String>>,
    pub target_enemy_identifiers: Option<Vec<String>>,
    pub min_enemy_count: u32,
    pub target_sub_type: SubmarineType,
    pub mission: Mission,
}
impl EliminateTargetsMission {
    pub fn new(element: Node) -> Self {
        let target_item_identifiers = element
            .attribute_ignore_ascii_case("targetitems")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let target_enemy_identifiers = element
            .attribute_ignore_ascii_case("targetenemies")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let min_enemy_count = element
            .attribute_ignore_ascii_case("minenemycount")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let target_sub_type = element
            .attribute_ignore_ascii_case("targetsub")
            .map_or(SubmarineType::Ruin, |v| v.parse::<SubmarineType>().unwrap());

        Self {
            target_item_identifiers,
            target_enemy_identifiers,
            min_enemy_count,
            target_sub_type,
            mission: Mission::new(element),
        }
    }
}
#[derive(Debug)]
pub struct Mission {
    pub complete_check_data_action: Option<CheckDataAction>,
}
impl Mission {
    pub fn new(element: Node) -> Self {
        Self {
            complete_check_data_action: element
                .children()
                .filter(Node::is_element)
                .find(|child| {
                    child
                        .tag_name()
                        .name()
                        .eq_ignore_ascii_case("completecheckdataaction")
                })
                .map(|child| CheckDataAction::new(child)),
        }
    }
}
#[derive(Debug)]
pub struct CheckDataAction {
    pub identifier: String,
    pub condition: String,
    pub force_string: bool,
    pub check_against_metadata: bool,
    pub binary_option_action: BinaryOptionAction,
}
impl CheckDataAction {
    pub fn new(element: Node) -> Self {
        let binary_option_action = BinaryOptionAction::new(element);
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let condition = element
            .attribute_ignore_ascii_case("condition")
            .or(element.attribute_ignore_ascii_case("value"))
            .map(|v| v.to_owned())
            .unwrap();
        let force_string = element
            .attribute_ignore_ascii_case("forcestring")
            .map_or(false, |v| v.parse().unwrap());
        let check_against_metadata = element
            .attribute_ignore_ascii_case("checkagainstmetadata")
            .map_or(false, |v| v.parse().unwrap());

        Self {
            identifier,
            condition,
            force_string,
            check_against_metadata,
            binary_option_action,
        }
    }
}
#[derive(Debug)]
pub struct BinaryOptionAction {
    pub success: Option<SubactionGroup>,
    pub failure: Option<SubactionGroup>,
}
impl BinaryOptionAction {
    pub fn new(element: Node) -> Self {
        let mut success = None;
        let mut failure = None;
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "success" => {
                    if success.is_none() {
                        success = Some(SubactionGroup::new(child));
                    }
                }
                "failure" => {
                    if failure.is_none() {
                        failure = Some(SubactionGroup::new(child));
                    }
                }
                _ => (),
            }
        }
        Self { success, failure }
    }
}
#[derive(Debug)]
pub struct SubactionGroup {
    pub text: String,
    pub end_conversation: bool,
    pub actions: Vec<()>,
}
impl SubactionGroup {
    pub fn new(element: Node) -> Self {
        let text = element
            .attribute_ignore_ascii_case("text")
            .map(|v| v.to_owned())
            .unwrap();
        let end_conversation = element
            .attribute_ignore_ascii_case("endconversation")
            .map_or(false, |v| v.parse().unwrap());
        let actions = element
            .children()
            .filter(Node::is_element)
            .map(|child| {
                let type_name = child.tag_name().name();
                match type_name {
                    _ => todo!("Action not yet implemented: {}", type_name),
                }
            })
            .collect::<Vec<_>>();

        Self {
            text,
            end_conversation,
            actions,
        }
    }
}

#[derive(Debug)]
pub struct PirateCharacterTypeVariant {
    pub preferred_difficulty: f32,
    pub is_commander: bool,
    pub character_identifier: String,
    pub character_from: String,
}
impl PirateCharacterTypeVariant {
    pub fn new(element: Node) -> Self {
        let preferred_difficulty = element
            .attribute_ignore_ascii_case("preferreddifficulty")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let is_commander = element
            .attribute_ignore_ascii_case("iscommander")
            .map_or(false, |v| v.parse().unwrap());
        let character_identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let character_from = element
            .attribute_ignore_ascii_case("from")
            .map(|v| v.to_owned())
            .unwrap();

        Self {
            preferred_difficulty,
            is_commander,
            character_identifier,
            character_from,
        }
    }
}

#[derive(Debug)]
pub struct PirateSubmarineConfig {
    pub preferred_difficulty: f32,
    pub alternate_reward: Option<f32>,
    pub faction_identifier: Option<String>,
    pub path: String,
}
impl PirateSubmarineConfig {
    pub fn new(element: Node) -> Self {
        let preferred_difficulty = element
            .attribute_ignore_ascii_case("preferreddifficulty")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let alternate_reward = element
            .attribute_ignore_ascii_case("alternatereward")
            .map(|v| v.parse::<f32>().unwrap());
        let faction_identifier = element
            .attribute_ignore_ascii_case("faction")
            .map(|v| v.to_owned());
        let path = element
            .attribute_ignore_ascii_case("path")
            .map(|v| v.to_owned())
            .unwrap();
        Self {
            preferred_difficulty,
            alternate_reward,
            faction_identifier,
            path,
        }
    }
}

#[derive(Debug)]
pub struct EscortMissionCharacter {
    pub escort_identifier: Option<String>,
    pub color: Color,
    pub character_identifier: String,
    pub character_from: String,
    pub status_effects: Vec<StatusEffect>,
}
impl EscortMissionCharacter {
    pub fn new(element: Node) -> Self {
        let escort_identifier = element
            .attribute_ignore_ascii_case("escortidentifier")
            .map(|v| v.to_owned());
        let color = element.attribute_ignore_ascii_case("color").map_or(
            Color::Simple {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            |v| v.parse().unwrap(),
        );
        let character_identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let character_from = element
            .attribute_ignore_ascii_case("from")
            .map(|v| v.to_owned())
            .unwrap();
        let status_effects = element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("statuseffect"))
            .map(|child| StatusEffect::new(child))
            .collect::<Vec<_>>();

        Self {
            escort_identifier,
            color,
            character_identifier,
            character_from,
            status_effects,
        }
    }
}

#[derive(Debug)]
pub enum MineralSpawnPositionType {
    MainPath,
    SidePath,
    Cave,
    AbyssCave,
}
impl FromStr for MineralSpawnPositionType {
    type Err = InvalidMineralSpawnPosition;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Err(InvalidMineralSpawnPosition::Invalid),
            "mainpath" => Ok(Self::MainPath),
            "sidepath" => Ok(Self::SidePath),
            "cave" => Ok(Self::Cave),
            "ruin" => Err(InvalidMineralSpawnPosition::Invalid),
            "wreck" => Err(InvalidMineralSpawnPosition::Invalid),
            "beaconstation" => Err(InvalidMineralSpawnPosition::Invalid),
            "abyss" => Err(InvalidMineralSpawnPosition::Invalid),
            "abysscave" => Ok(Self::AbyssCave),
            "outpost" => Err(InvalidMineralSpawnPosition::Invalid),
            _ => Err(InvalidMineralSpawnPosition::DoesNotExistError(
                DoesNotExistError(s.to_owned()),
            )),
        }
    }
}
#[derive(Debug)]
pub enum InvalidMineralSpawnPosition {
    Invalid,
    DoesNotExistError(DoesNotExistError),
}

#[derive(Debug)]
pub struct AbandonedOutpostHuman {
    pub character_identifier: String,
    pub character_from: String,
    pub module_flags: Option<Vec<String>>,
    pub spawn_point_tags: Option<Vec<String>>,
    pub spawn_point_type: SpawnType,
    pub as_far_as_possible: bool,
    pub requires_rescue: bool,
    pub team_id: CharacterTeamType,
    pub require_kill: bool,

    pub multiplayer_only: bool,
    pub count_range: RangeInclusive<u32>,
}
impl AbandonedOutpostHuman {
    pub fn new(element: Node) -> Self {
        let character_identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let character_from = element
            .attribute_ignore_ascii_case("from")
            .map(|v| v.to_owned())
            .unwrap();
        let module_flags = element
            .attribute_ignore_ascii_case("moduleflags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let spawn_point_tags = element
            .attribute_ignore_ascii_case("spawnpointtags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let spawn_point_type = element
            .attribute_ignore_ascii_case("spawnpointtype")
            .map_or(SpawnType::new().with_human(true), |v| v.parse().unwrap());
        let as_far_as_possible = element
            .attribute_ignore_ascii_case("asfaraspossible")
            .map_or(false, |v| v.parse().unwrap());
        let requires_rescue = element
            .attribute_ignore_ascii_case("requirerescue")
            .map_or(false, |v| v.parse().unwrap());
        let team_id = element.attribute_ignore_ascii_case("teamid").map_or(
            if requires_rescue {
                CharacterTeamType::FriendlyNPC
            } else {
                CharacterTeamType::None
            },
            |v| v.parse().unwrap(),
        );
        let require_kill = element
            .attribute_ignore_ascii_case("requirekill")
            .map_or(false, |v| v.parse().unwrap());
        let multiplayer_only = element
            .attribute_ignore_ascii_case("multiplayeronly")
            .map_or(false, |v| v.parse().unwrap());
        let default_count = element
            .attribute_ignore_ascii_case("count")
            .or(element.attribute_ignore_ascii_case("amount"))
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let min = element
            .attribute_ignore_ascii_case("min")
            .map_or(default_count, |v| {
                v.parse::<i32>().unwrap().clamp(0, 255) as u32
            });
        let max = element
            .attribute_ignore_ascii_case("max")
            .map_or(default_count, |v| v.parse::<u32>().unwrap().clamp(min, 255));
        Self {
            character_identifier,
            character_from,
            module_flags,
            spawn_point_tags,
            spawn_point_type,
            as_far_as_possible,
            requires_rescue,
            team_id,
            require_kill,
            multiplayer_only,
            count_range: min..=max,
        }
    }
}
#[derive(Debug)]
pub struct AbandonedOutpostMonster {
    pub species_name: Option<String>,
    pub module_flags: Option<Vec<String>>,
    pub spawn_point_tags: Option<Vec<String>>,
    pub as_far_as_possible: bool,
    pub require_kill: bool,

    pub multiplayer_only: bool,
    pub count_range: RangeInclusive<u32>,
}
impl AbandonedOutpostMonster {
    pub fn new(element: Node) -> Self {
        let species_name = element
            .attribute_ignore_ascii_case("character")
            .map(|v| v.to_owned());
        let module_flags = element
            .attribute_ignore_ascii_case("moduleflags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let spawn_point_tags = element
            .attribute_ignore_ascii_case("spawnpointtags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let as_far_as_possible = element
            .attribute_ignore_ascii_case("asfaraspossible")
            .map_or(false, |v| v.parse().unwrap());
        let require_kill = element
            .attribute_ignore_ascii_case("requirekill")
            .map_or(false, |v| v.parse().unwrap());
        let multiplayer_only = element
            .attribute_ignore_ascii_case("multiplayeronly")
            .map_or(false, |v| v.parse().unwrap());
        let default_count = element
            .attribute_ignore_ascii_case("count")
            .or(element.attribute_ignore_ascii_case("amount"))
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let min = element
            .attribute_ignore_ascii_case("min")
            .map_or(default_count, |v| {
                v.parse::<i32>().unwrap().clamp(0, 255) as u32
            });
        let max = element
            .attribute_ignore_ascii_case("max")
            .map_or(default_count, |v| v.parse::<u32>().unwrap().clamp(min, 255));

        Self {
            species_name,
            module_flags,
            spawn_point_tags,
            as_far_as_possible,
            require_kill,
            multiplayer_only,
            count_range: min..=max,
        }
    }
}
#[derive(Debug)]
pub struct AbandonedOutpostItem {
    pub item_identifier: String,
    pub module_flags: Option<Vec<String>>,
    pub spawn_point_tags: Option<Vec<String>>,
    pub as_far_as_possible: bool,
}
impl AbandonedOutpostItem {
    pub fn new(element: Node) -> Self {
        let item_identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let module_flags = element
            .attribute_ignore_ascii_case("moduleflags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let spawn_point_tags = element
            .attribute_ignore_ascii_case("spawnpointtags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let as_far_as_possible = element
            .attribute_ignore_ascii_case("asfaraspossible")
            .map_or(false, |v| v.parse().unwrap());
        Self {
            item_identifier,
            module_flags,
            spawn_point_tags,
            as_far_as_possible,
        }
    }
}
#[derive(Debug)]
pub struct NestMonsterPrefab {
    pub species_name: String,
    pub count_range: RangeInclusive<u32>,
}
impl NestMonsterPrefab {
    pub fn new(element: Node) -> Self {
        let species_name = element
            .attribute_ignore_ascii_case("character")
            .map(|v| v.to_owned())
            .unwrap();
        let default_count = element
            .attribute_ignore_ascii_case("count")
            .or(element.attribute_ignore_ascii_case("amount"))
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let min = element
            .attribute_ignore_ascii_case("min")
            .map_or(default_count, |v| {
                v.parse::<i32>().unwrap().clamp(0, 255) as u32
            });
        let max = element
            .attribute_ignore_ascii_case("max")
            .map_or(default_count, |v| v.parse::<u32>().unwrap().clamp(min, 255));

        Self {
            species_name,
            count_range: min..=max,
        }
    }
}
#[derive(Debug)]
pub struct NestItem {
    pub item_identifier: String,
    pub status_effect_on_approach: Option<StatusEffect>,
}
impl NestItem {
    pub fn new(element: Node) -> Self {
        let item_identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let status_effect_on_approach = element
            .children()
            .filter(Node::is_element)
            .find(|child| {
                child
                    .tag_name()
                    .name()
                    .eq_ignore_ascii_case("StatusEffectOnApproach")
            })
            .map(|child| StatusEffect::new(child));

        Self {
            item_identifier,
            status_effect_on_approach,
        }
    }
}

#[bitfield(u8)]

pub struct SpawnType {
    pub path: bool,
    pub human: bool,
    pub enemy: bool,
    pub cargo: bool,
    pub corpse: bool,
    pub submarine: bool,
    pub exit_point: bool,
    #[bits(1)]
    _unused: u8,
}

impl FromStr for SpawnType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "path" => Ok(Self::new().with_path(true)),
            "human" => Ok(Self::new().with_human(true)),
            "enemy" => Ok(Self::new().with_enemy(true)),
            "cargo" => Ok(Self::new().with_cargo(true)),
            "corpse" => Ok(Self::new().with_corpse(true)),
            "submarine" => Ok(Self::new().with_submarine(true)),
            "exitpoint" => Ok(Self::new().with_exit_point(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub header: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug)]
pub struct ReputationReward {
    pub faction_identifier: String,
    pub amount: f32,
    pub amount_for_opposing_faction: f32,
}

impl ReputationReward {
    pub fn new(element: Node) -> Self {
        let faction_identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let amount = element
            .attribute_ignore_ascii_case("amount")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let amount_for_opposing_faction = element
            .attribute_ignore_ascii_case("amountforopposingfaction")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());

        Self {
            faction_identifier,
            amount,
            amount_for_opposing_faction,
        }
    }
}

#[derive(Debug)]
pub enum SetDataActionConvert {
    Bool(bool),
    Float(f32),
    String(String),
}
impl FromStr for SetDataActionConvert {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse() {
            return Ok(Self::Bool(v));
        }
        if let Ok(v) = s.parse() {
            return Ok(Self::Float(v));
        }
        Ok(Self::String(s.to_owned()))
    }
}

#[derive(Debug)]
pub enum OperationType {
    Set,
    Multiply,
    Add,
}
impl FromStr for OperationType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Set" => Ok(Self::Set),
            "Multiply" => Ok(Self::Multiply),
            "Add" => Ok(Self::Add),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct TriggerEvent {
    pub event_identifier: Option<String>,
    pub event_tag: Option<String>,
    pub state: u32,
    pub delay: f32,
    pub campaign_only: bool,
}
impl TriggerEvent {
    pub fn new(element: Node) -> Self {
        Self {
            event_identifier: element
                .attribute_ignore_ascii_case("eventidentifier")
                .map(|v| v.to_owned()),
            event_tag: element
                .attribute_ignore_ascii_case("eventtag")
                .map(|v| v.to_owned()),
            state: element
                .attribute_ignore_ascii_case("state")
                .map_or(0, |v| v.parse().unwrap()),
            delay: element
                .attribute_ignore_ascii_case("delay")
                .map_or(0.0, |v| v.parse().unwrap()),
            campaign_only: element
                .attribute_ignore_ascii_case("campaignonly")
                .map_or(false, |v| v.parse().unwrap()),
        }
    }
}
