use std::{
    collections::{HashMap, HashSet},
    num::ParseFloatError,
    str::FromStr,
};

use bitfield_struct::bitfield;
use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::{item_assembly_prefab::Rect, item_prefab::Color},
    util::NodeExp,
};

use super::{
    item_components::ItemComponents,
    math::quad2d::Quad2D,
    prefabs::{
        item_prefab::{DoesNotExistError, ItemPrefab},
        map_generation_params::Point,
        mission_prefab::SpawnType,
        structure_prefab::StructurePrefab,
    },
    version::Version,
};

const DEFAULT_REAL_WORLD_CRUSH_DEPTH: f32 = 3500.0;

#[derive(Debug)]
pub struct SubmarineInfo {
    pub name: Option<String>,
    pub description: Option<String>,
    pub equality_check_val: Option<u32>,
    pub price: Option<u32>,
    pub initial_supplies_spawned: bool,
    pub no_items: bool,
    pub low_fuel: bool,
    pub is_manually_outfitted: bool,
    pub game_version: Option<Version>,

    pub tags: Option<SubmarineTag>,
    pub dimensions: Option<Vec2>,
    pub cargo_capacity: Option<u32>,
    pub recommended_crew_size_min: Option<u32>,
    pub recommended_crew_size_max: Option<u32>,
    pub recommended_crew_experience: Option<CrewExperienceLevel>,
    pub tier: Option<u32>,
    pub submarine_type: Option<SubmarineType>,
    pub additional_info: Option<AdditionalSubmarineInfo>,
    pub submarine_class: Option<SubmarineClass>,
    pub required_content_packages: Option<Vec<String>>,

    pub structures: Vec<Structure>,
    pub items: Vec<Item>,
    pub waypoints: Vec<WayPoint>,
    pub hulls: Vec<Hull>,
    pub gaps: Vec<Gap>,
    pub linked_submarines: Vec<LinkedSubmarine>,
}

impl SubmarineInfo {
    pub fn new(element: Node) -> Self {
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(std::borrow::ToOwned::to_owned);
        let description = element
            .attribute_ignore_ascii_case("description")
            .map(std::borrow::ToOwned::to_owned);
        let equality_check_val = element
            .attribute_ignore_ascii_case("checkval")
            .map(|v| v.parse::<u32>().unwrap());
        let price = element
            .attribute_ignore_ascii_case("price")
            .map(|v| v.parse::<u32>().unwrap());
        let initial_supplies_spawned = element
            .attribute_ignore_ascii_case("initialsuppliesspawned")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let no_items = element
            .attribute_ignore_ascii_case("noitems")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let low_fuel = element
            .attribute_ignore_ascii_case("lowfuel")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let is_manually_outfitted = element
            .attribute_ignore_ascii_case("ismanuallyoutfitted")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let game_version = element
            .attribute_ignore_ascii_case("gameversion")
            .map(|v| v.parse::<Version>().unwrap());
        let tags = element.attribute_ignore_ascii_case("tags").map(|v| {
            v.split(',')
                .map(|v| v.parse::<SubmarineTag>().unwrap())
                .fold(SubmarineTag::new(), |acc, x| {
                    SubmarineTag::from_bits(acc.into_bits() | x.into_bits())
                })
        });
        let dimensions = element
            .attribute_ignore_ascii_case("dimensions")
            .map(|v| v.parse::<Vector2>().unwrap());
        let cargo_capacity = element
            .attribute_ignore_ascii_case("cargocapacity")
            .map(|v| v.parse::<u32>().unwrap());
        let recommended_crew_size_min = element
            .attribute_ignore_ascii_case("recommendedcrewsizemin")
            .map(|v| v.parse::<u32>().unwrap());
        let recommended_crew_size_max = element
            .attribute_ignore_ascii_case("recommendedcrewsizemax")
            .map(|v| v.parse::<u32>().unwrap());
        let recommended_crew_experience = element
            .attribute_ignore_ascii_case("recommendedcrewexperience")
            .map(|v| v.parse::<CrewExperienceLevel>().unwrap());
        let tier = element
            .attribute_ignore_ascii_case("tier")
            .map(|v| v.parse::<u32>().unwrap());
        let submarine_type = element
            .attribute_ignore_ascii_case("type")
            .map(|v| v.parse::<SubmarineType>().unwrap());
        let additional_info = if let Some(submarine_type) = &submarine_type {
            match submarine_type {
                SubmarineType::OutpostModule => Some(AdditionalSubmarineInfo::OutpostModule {
                    max_count: element
                        .attribute_ignore_ascii_case("maxcount")
                        .map_or(100, |v| v.parse::<u32>().unwrap()),
                    commonness: element
                        .attribute_ignore_ascii_case("commonness")
                        .map_or(10.0, |v| v.parse::<f32>().unwrap()),
                    gap_positions: element.attribute_ignore_ascii_case("gappositions").map_or(
                        GapPositions::new(),
                        |v| {
                            v.split(',')
                                .map(|v| v.parse::<GapPositions>().unwrap())
                                .fold(GapPositions::new(), |acc, x| {
                                    GapPositions::from_bits(acc.into_bits() | x.into_bits())
                                })
                        },
                    ),
                    can_attach_to_previous: element
                        .attribute_ignore_ascii_case("canattachtoprevious")
                        .map_or(GapPositions::all(), |v| {
                            v.split(',')
                                .map(|v| v.parse::<GapPositions>().unwrap())
                                .fold(GapPositions::new(), |acc, x| {
                                    GapPositions::from_bits(acc.into_bits() | x.into_bits())
                                })
                        }),
                    flags: element
                        .attribute_ignore_ascii_case("flags")
                        .or(element.attribute_ignore_ascii_case("moduletypes"))
                        .map(|v| {
                            v.split(',')
                                .map(std::borrow::ToOwned::to_owned)
                                .collect::<Vec<_>>()
                        })
                        .map(|mut v| {
                            if v.is_empty() {
                                v.push("none".to_owned());
                            }
                            v
                        })
                        .unwrap_or(vec!["none".to_owned()]),
                    allow_attach_to_modules: element
                        .attribute_ignore_ascii_case("allowattachto")
                        .map(|v| {
                            v.split(',')
                                .map(std::borrow::ToOwned::to_owned)
                                .collect::<Vec<_>>()
                        })
                        .map(|mut v| {
                            if v.is_empty() {
                                v.push("any".to_owned());
                            }
                            v
                        })
                        .unwrap_or(vec!["any".to_owned()]),
                    allowed_location_types: element
                        .attribute_ignore_ascii_case("allowedlocationtypes")
                        .map(|v| {
                            v.split(',')
                                .map(std::borrow::ToOwned::to_owned)
                                .collect::<HashSet<_>>()
                        })
                        .unwrap_or_default(),
                }),
                SubmarineType::BeaconStation => Some(AdditionalSubmarineInfo::BeaconStation {
                    allow_damaged_walls: element
                        .attribute_ignore_ascii_case("allowdamagedwalls")
                        .map_or(true, |v| v.to_lowercase().parse::<bool>().unwrap()),
                    allow_disconnected_wires: element
                        .attribute_ignore_ascii_case("allowdisconnectedwires")
                        .map_or(true, |v| v.to_lowercase().parse::<bool>().unwrap()),
                    min_level_difficulty: element
                        .attribute_ignore_ascii_case("minleveldifficulty")
                        .map(|v| v.parse::<f32>().unwrap()),
                    max_level_difficulty: element
                        .attribute_ignore_ascii_case("maxleveldifficulty")
                        .map(|v| v.parse::<f32>().unwrap()),
                    placement: element
                        .attribute_ignore_ascii_case("placement")
                        .map_or(PlacementType::Bottom, |v| {
                            v.parse::<PlacementType>().unwrap()
                        }),
                }),
                _ => None,
            }
        } else {
            None
        };
        let submarine_class = if let Some(SubmarineType::Player) = submarine_type {
            element
                .attribute_ignore_ascii_case("class")
                .map(|v| v.parse::<SubmarineClass>().unwrap())
        } else {
            Some(SubmarineClass::Undefined)
        };
        let required_content_packages = element
            .attribute_ignore_ascii_case("requiredcontentpackages")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            });

        //client specific
        let preview_image_data = element.attribute_ignore_ascii_case("previewimage");

        let mut structures = Vec::new();
        let mut items = Vec::new();
        let mut waypoints = Vec::new();
        let mut hulls = Vec::new();
        let mut gaps = Vec::new();
        let mut linked_submarines = Vec::new();
        for child in element.children().filter(Node::is_element) {
            let tag_name = child.tag_name().name();
            match tag_name {
                "Structure" => {
                    let name = child
                        .attribute_ignore_ascii_case("name")
                        .unwrap()
                        .to_owned();
                    let identifier = child
                        .attribute_ignore_ascii_case("identifier")
                        .unwrap()
                        .to_owned();
                    let rect = child
                        .attribute_ignore_ascii_case("rect")
                        .map(|v| Rect::from_str(v, false).unwrap())
                        .unwrap();
                    let id = child
                        .attribute_ignore_ascii_case("id")
                        .map(|v| v.parse::<u32>().unwrap())
                        .unwrap();
                    let flipped_x = child
                        .attribute_ignore_ascii_case("flippedx")
                        .map_or(false, |v| v.parse::<bool>().unwrap());
                    let flipped_y = child
                        .attribute_ignore_ascii_case("flippedy")
                        .map_or(false, |v| v.parse::<bool>().unwrap());

                    //map entity
                    let disallowed_upgrades = child
                        .attribute_ignore_ascii_case("disallowedupgrades")
                        .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
                        .unwrap();
                    let rect_width = child
                        .attribute_ignore_ascii_case("rectwidth")
                        .map_or(0, |v| v.parse::<u32>().unwrap());
                    let rect_height = child
                        .attribute_ignore_ascii_case("rectheight")
                        .map_or(0, |v| v.parse::<u32>().unwrap());
                    let sprite_depth = child
                        .attribute_ignore_ascii_case("spritedepth")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap();
                    let scale = child
                        .attribute_ignore_ascii_case("scale")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap();
                    let hidden_in_game = child
                        .attribute_ignore_ascii_case("hiddeningame")
                        .map(|v| v.to_lowercase().parse::<bool>().unwrap())
                        .unwrap();
                    let remove_if_linked_outpost_door_in_use = child
                        .attribute_ignore_ascii_case("removeiflinkedoutpostdoorinuse")
                        .map_or(true, |v| v.to_lowercase().parse::<bool>().unwrap());
                    let layer = child
                        .attribute_ignore_ascii_case("layer")
                        .map(|v| v.to_owned());

                    //structure
                    let indestructible = child
                        .attribute_ignore_ascii_case("indestructible")
                        .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
                    let cast_shadow = child
                        .attribute_ignore_ascii_case("castshadow")
                        .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
                    let max_health = child
                        .attribute_ignore_ascii_case("maxhealth")
                        .map_or(100.0, |v| v.parse::<f32>().unwrap());
                    let crush_depth = child
                        .attribute_ignore_ascii_case("crushdepth")
                        .map_or(DEFAULT_REAL_WORLD_CRUSH_DEPTH, |v| {
                            v.parse::<f32>().unwrap()
                        });
                    let special_tag = child
                        .attribute_ignore_ascii_case("specialtag")
                        .map(|v| v.to_owned());
                    let sprite_color = child
                        .attribute_ignore_ascii_case("spritecolor")
                        .map(|v| v.parse::<Color>().unwrap())
                        .unwrap();
                    let use_drop_shadow = child
                        .attribute_ignore_ascii_case("usedropshadow")
                        .map(|v| v.to_lowercase().parse::<bool>().unwrap())
                        .unwrap();
                    let drop_shadow_offset = child
                        .attribute_ignore_ascii_case("dropshadowoffset")
                        .map(|v| v.parse::<Vector2>().unwrap().0)
                        .unwrap();
                    let rotation = child
                        .attribute_ignore_ascii_case("rotation")
                        .map_or(0.0, |v| v.parse::<f32>().unwrap());
                    let texture_scale = child
                        .attribute_ignore_ascii_case("texturescale")
                        .map(|v| v.parse::<Vector2>().unwrap().0)
                        .unwrap();
                    let texture_offset = child
                        .attribute_ignore_ascii_case("textureoffset")
                        .map(|v| v.parse::<Vector2>().unwrap().0)
                        .unwrap();
                    let no_ai_target = child
                        .attribute_ignore_ascii_case("noaitarget")
                        .map(|v| v.to_lowercase().parse::<bool>().unwrap())
                        .unwrap();

                    let mut section_damage = HashMap::new();
                    for child in child.children().filter(Node::is_element) {
                        let tag_name = child.tag_name().name();
                        match tag_name {
                            "section" => {
                                let index = child
                                    .attribute_ignore_ascii_case("i")
                                    .map(|v| v.parse::<u32>().unwrap())
                                    .unwrap();
                                let damage = child
                                    .attribute_ignore_ascii_case("damage")
                                    .map(|v| v.parse::<f32>().unwrap())
                                    .unwrap();
                                section_damage.insert(index as usize, damage);
                            }
                            "upgrade" => {
                                let upgrade_identifier = child
                                    .attribute_ignore_ascii_case("identifier")
                                    .unwrap()
                                    .to_owned();
                                let level = child
                                    .attribute_ignore_ascii_case("level")
                                    .map(|v| v.parse::<u32>().unwrap())
                                    .unwrap();
                                todo!()
                            }
                            _ => {
                                panic!("Unknown tag name found in structure data: {}", tag_name);
                            }
                        }
                    }
                    structures.push(Structure {
                        map_entity: MapEntity {
                            disallowed_upgrades,
                            rect_width,
                            rect_height,
                            sprite_depth,
                            scale,
                            hidden_in_game,
                            remove_if_linked_outpost_door_in_use,
                            layer,
                        },
                        name,
                        identifier,
                        rect,
                        id,
                        flipped_x,
                        flipped_y,
                        indestructible,
                        cast_shadow,
                        max_health,
                        crush_depth,
                        special_tag,
                        sprite_color,
                        use_drop_shadow,
                        drop_shadow_offset,
                        rotation,
                        texture_scale,
                        texture_offset,
                        no_ai_target,
                        section_damage,
                    })
                }
                "Item" => {
                    let name = child
                        .attribute_ignore_ascii_case("name")
                        .unwrap()
                        .to_owned();
                    let identifier = child
                        .attribute_ignore_ascii_case("identifier")
                        .unwrap()
                        .to_owned();
                    let pending_swap = child
                        .attribute_ignore_ascii_case("pendingswap")
                        .map(|v| v.to_owned());
                    let rect = child
                        .attribute_ignore_ascii_case("rect")
                        .map(|v| Rect::from_str(v, false).unwrap())
                        .unwrap();
                    let id = child
                        .attribute_ignore_ascii_case("id")
                        .map(|v| v.parse::<u32>().unwrap())
                        .unwrap();
                    let linked_to_ids = child
                        .attribute_ignore_ascii_case("linked")
                        .map(|v| {
                            v.split(',')
                                .map(|v| v.parse::<u32>().unwrap())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let is_override = child
                        .attribute_ignore_ascii_case("isoverride")
                        .map_or(false, |v| v.parse::<bool>().unwrap());
                    let available_swap_ids = child
                        .attribute_ignore_ascii_case("availableswaps")
                        .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
                        .unwrap_or_default();
                    let marked_for_deconstruction = child
                        .attribute_ignore_ascii_case("markedfordeconstruction")
                        .map_or(false, |v| v.parse::<bool>().unwrap());
                    let flipped_x = child
                        .attribute_ignore_ascii_case("flippedx")
                        .map_or(false, |v| v.parse::<bool>().unwrap());
                    let flipped_y = child
                        .attribute_ignore_ascii_case("flippedy")
                        .map_or(false, |v| v.parse::<bool>().unwrap());

                    let sprite_depth = child
                        .attribute_ignore_ascii_case("spritedepth")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap();
                    let sprite_color = child
                        .attribute_ignore_ascii_case("spritecolor")
                        .map(|v| v.parse::<Color>().unwrap())
                        .unwrap();
                    let rotation = child
                        .attribute_ignore_ascii_case("rotation")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap();
                    let purchased_new_swap = child
                        .attribute_ignore_ascii_case("purchasednewswap")
                        .map_or(false, |v| v.parse::<bool>().unwrap());
                    let scale = child
                        .attribute_ignore_ascii_case("scale")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap();
                    let condition = if let Some(v) = child
                        .attribute_ignore_ascii_case("conditionpercentage")
                        .map(|v| v.parse::<f32>().unwrap())
                    {
                        Condition::Percentage(v)
                    } else {
                        //backwards compatibility
                        Condition::Value(
                            child
                                .attribute_ignore_ascii_case("condition")
                                .map_or(0.0, |v| v.parse::<f32>().unwrap()),
                        )
                    };
                    let properties = ItemSaveableProperties::from_xml(&child);

                    let mut item_stats = HashMap::new();
                    let mut components = ItemComponents::default();
                    for child in child.children().filter(Node::is_element) {
                        let tag_name = child.tag_name().name();
                        match tag_name {
                            "upgrade" => {
                                let upgrade_identifier = child
                                    .attribute_ignore_ascii_case("identifier")
                                    .unwrap()
                                    .to_owned();
                                let level = child
                                    .attribute_ignore_ascii_case("level")
                                    .map(|v| v.parse::<u32>().unwrap())
                                    .unwrap();
                                todo!()
                            }
                            "itemstats" => {
                                for child in child.children().filter(Node::is_element) {
                                    let identifier = TalentStatIdentifier {
                                        ty: child
                                            .attribute_ignore_ascii_case("type")
                                            .map(|v| v.parse().unwrap())
                                            .unwrap(),
                                        talent_identifier: child
                                            .attribute_ignore_ascii_case("identifier")
                                            .map(|v| v.to_owned())
                                            .unwrap(),
                                    };
                                    let value = child
                                        .attribute_ignore_ascii_case("value")
                                        .map(|v| v.parse::<f32>().unwrap())
                                        .unwrap();
                                    assert!(!item_stats.contains_key(&identifier));
                                    item_stats.insert(identifier, value);
                                }
                            }
                            _ => {
                                components.add_from_xml(&child);
                            }
                        }
                    }
                    items.push(Item {
                        name,
                        identifier,
                        pending_swap,
                        rect,
                        id,
                        linked_to_ids,
                        is_override,
                        available_swap_ids,
                        marked_for_deconstruction,
                        flipped_x,
                        flipped_y,
                        sprite_depth,
                        sprite_color,
                        rotation,
                        purchased_new_swap,
                        scale,
                        condition,
                        properties,
                        components,
                    })
                }
                "WayPoint" => {
                    let x = child
                        .attribute_ignore_ascii_case("x")
                        .map(|v| v.parse::<i32>().unwrap())
                        .unwrap();
                    let y = child
                        .attribute_ignore_ascii_case("y")
                        .map(|v| v.parse::<i32>().unwrap())
                        .unwrap();
                    let spawn_type = child
                        .attribute_ignore_ascii_case("spawn")
                        .map(|v| v.parse::<SpawnType>().unwrap())
                        .unwrap();
                    let id = child
                        .attribute_ignore_ascii_case("id")
                        .map(|v| v.parse::<u32>().unwrap())
                        .unwrap();
                    let layer = child
                        .attribute_ignore_ascii_case("layer")
                        .map(|v| v.to_owned());
                    let id_card_description = child
                        .attribute_ignore_ascii_case("idcarddesc")
                        .map(|v| v.to_owned());
                    let id_card_tags = child
                        .attribute_ignore_ascii_case("idcardtags")
                        .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
                        .unwrap_or_default();
                    let exit_point_size = child
                        .attribute_ignore_ascii_case("exitpointsize")
                        .map(|v| v.parse::<Point>().unwrap());
                    let tags = child
                        .attribute_ignore_ascii_case("tags")
                        .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
                        .unwrap_or_default();
                    let job = child
                        .attribute_ignore_ascii_case("job")
                        .map(|v| v.to_owned());
                    let ladder_id = child
                        .attribute_ignore_ascii_case("ladders")
                        .map(|v| v.parse::<u32>().unwrap());
                    let gap_id = child
                        .attribute_ignore_ascii_case("gap")
                        .map(|v| v.parse::<u32>().unwrap());
                    let linked_to_ids = {
                        let mut vec = Vec::new();
                        let mut i = 0;
                        while let Some(v) =
                            child.attribute_ignore_ascii_case(&format!("linkedto{}", i))
                        {
                            vec.push(v.parse::<u32>().unwrap());
                            i += 1;
                        }
                        vec
                    };
                    waypoints.push(WayPoint {
                        x,
                        y,
                        spawn_type,
                        id,
                        layer,
                        id_card_description,
                        id_card_tags,
                        exit_point_size,
                        tags,
                        job,
                        ladder_id,
                        gap_id,
                        linked_to_ids,
                    });
                }
                "Hull" => {
                    let rect = child
                        .attribute_ignore_ascii_case("rect")
                        .map(|v| Rect::from_str(v, false).unwrap())
                        .unwrap();
                    let id = child
                        .attribute_ignore_ascii_case("id")
                        .map(|v| v.parse::<u32>().unwrap())
                        .unwrap();
                    let water_volume = child
                        .attribute_ignore_ascii_case("water")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap();
                    let original_ambient_light = child
                        .attribute_ignore_ascii_case("originalambientlight")
                        .map(|v| v.parse::<Color>().unwrap());
                    let linked_to_ids = child
                        .attribute_ignore_ascii_case("linked")
                        .map(|v| {
                            v.split(',')
                                .map(|v| v.parse::<u32>().unwrap())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let background_sections = child
                        .attribute_ignore_ascii_case("backgroundsections")
                        .filter(|v| !v.is_empty())
                        .map(|v| {
                            v.split(';')
                                .map(|v| {
                                    let mut spl = v.split(':');
                                    let index = spl.next().unwrap().parse::<u32>().unwrap();
                                    let color = spl.next().unwrap().parse::<Color>().unwrap();
                                    let strength = spl.next().unwrap().parse::<f32>().unwrap();
                                    (index, color, strength)
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    let room_name = child
                        .attribute_ignore_ascii_case("roomname")
                        .map(|v| v.to_owned())
                        .unwrap();
                    let ambient_light = child
                        .attribute_ignore_ascii_case("ambientlight")
                        .map(|v| v.parse::<Color>().unwrap())
                        .unwrap();
                    let oxygen = child
                        .attribute_ignore_ascii_case("oxygen")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap();
                    let is_wet_room = child
                        .attribute_ignore_ascii_case("iswetroom")
                        .map(|v| v.to_lowercase().parse::<bool>().unwrap())
                        .unwrap();
                    let avoid_staying = child
                        .attribute_ignore_ascii_case("avoidstaying")
                        .map(|v| v.to_lowercase().parse::<bool>().unwrap())
                        .unwrap();

                    for child in child.children().filter(Node::is_element) {
                        let tag_name = child.tag_name().name();
                        match tag_name {
                            "decal" => {
                                todo!()
                            }
                            "ballastflorabehavior" => {
                                todo!()
                            }
                            _ => {
                                panic!("Unexpected tag name in Hull: {}", tag_name);
                            }
                        }
                    }

                    hulls.push(Hull {
                        rect,
                        id,
                        water_volume,
                        original_ambient_light,
                        linked_to_ids,
                        background_sections,
                        room_name,
                        ambient_light,
                        oxygen,
                        is_wet_room,
                        avoid_staying,
                    });
                }
                "Gap" => {
                    let rect = child
                        .attribute_ignore_ascii_case("rect")
                        .map(|v| Rect::from_str(v, false).unwrap())
                        .unwrap();
                    let horizontal = child
                        .attribute_ignore_ascii_case("horizontal")
                        .map(|v| v.parse::<bool>().unwrap())
                        .unwrap();
                    let id = child
                        .attribute_ignore_ascii_case("id")
                        .map(|v| v.parse::<u32>().unwrap())
                        .unwrap();
                    let layer = child
                        .attribute_ignore_ascii_case("layer")
                        .map(|v| v.to_owned());
                    let hidden_in_game = child
                        .attribute_ignore_ascii_case("hiddeningame")
                        .map_or(false, |v| v.parse::<bool>().unwrap());
                    gaps.push(Gap {
                        rect,
                        horizontal,
                        id,
                        layer,
                        hidden_in_game,
                    });
                }
                "LinkedSubmarine" => {
                    let pos = child
                        .attribute_ignore_ascii_case("pos")
                        .map(|v| v.parse::<Vector2>().unwrap().0)
                        .unwrap();
                    let level_seed = child
                        .attribute_ignore_ascii_case("location")
                        .map(|v| v.to_owned());
                    let file_path = child
                        .attribute_ignore_ascii_case("filepath")
                        .map(|v| v.to_owned())
                        .unwrap();
                    let linked_to_ids = child
                        .attribute_ignore_ascii_case("linkedto")
                        .map(|v| {
                            v.split(',')
                                .map(|v| v.parse::<u32>().unwrap())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let original_linked_to_id = child
                        .attribute_ignore_ascii_case("originallinkedto")
                        .map(|v| v.parse::<u32>().unwrap())
                        .unwrap();
                    let original_my_port_id = child
                        .attribute_ignore_ascii_case("originalmyport")
                        .map(|v| v.parse::<u16>().unwrap())
                        .unwrap();
                    let cargo_capacity = child
                        .attribute_ignore_ascii_case("cargocapacity")
                        .map(|v| v.parse::<u32>().unwrap())
                        .unwrap();
                    //TODO: Seems to include submarine data but seems to not be used in vanilla barotrauma code
                    linked_submarines.push(LinkedSubmarine {
                        pos,
                        level_seed,
                        file_path,
                        linked_to_ids,
                        original_linked_to_id,
                        original_my_port_id,
                        cargo_capacity,
                    })
                }
                _ => {
                    panic!(
                        "Found unimplemented item type in submarine info: {}",
                        tag_name
                    );
                }
            }
        }

        assert!(!hulls.is_empty());

        Self {
            name,
            description,
            equality_check_val,
            price,
            initial_supplies_spawned,
            no_items,
            low_fuel,
            is_manually_outfitted,
            game_version,
            tags,
            dimensions: dimensions.map(|v| v.0),
            cargo_capacity,
            recommended_crew_size_min,
            recommended_crew_size_max,
            recommended_crew_experience,
            tier,
            submarine_type,
            additional_info,
            submarine_class,
            required_content_packages,

            structures,
            items,
            waypoints,
            hulls,
            gaps,
            linked_submarines,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AdditionalSubmarineInfo {
    OutpostModule {
        max_count: u32,
        commonness: f32,

        gap_positions: GapPositions,

        can_attach_to_previous: GapPositions,
        flags: Vec<String>,
        allow_attach_to_modules: Vec<String>,
        allowed_location_types: HashSet<String>,
    },
    BeaconStation {
        allow_damaged_walls: bool,
        allow_disconnected_wires: bool,
        min_level_difficulty: Option<f32>,
        max_level_difficulty: Option<f32>,
        placement: PlacementType,
    },
}

#[derive(Debug, Clone)]
pub enum PlacementType {
    Top,
    Bottom,
}

impl FromStr for PlacementType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Top" | "0" => Ok(Self::Top),
            "Bottom" | "1" => Ok(Self::Bottom),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[bitfield(u8)]
pub struct GapPositions {
    pub right: bool,
    pub left: bool,
    pub top: bool,
    pub bottom: bool,
    #[bits(4)]
    _unused: u8,
}

impl GapPositions {
    pub fn all() -> Self {
        Self::new()
            .with_right(true)
            .with_left(true)
            .with_top(true)
            .with_bottom(true)
    }
}

impl FromStr for GapPositions {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "none" => Ok(Self::new()),
            "right" => Ok(Self::new().with_right(true)),
            "left" => Ok(Self::new().with_left(true)),
            "top" => Ok(Self::new().with_top(true)),
            "bottom" => Ok(Self::new().with_bottom(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SubmarineClass {
    Undefined,
    Scout,
    Attack,
    Transport,
}

impl FromStr for SubmarineClass {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "undefined" => Ok(Self::Undefined),
            "scout" => Ok(Self::Scout),
            "attack" => Ok(Self::Attack),
            "transport" => Ok(Self::Transport),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubmarineType {
    Player,
    Outpost,
    OutpostModule,
    Wreck,
    BeaconStation,
    EnemySubmarine,
    Ruin,
}

impl FromStr for SubmarineType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Player" => Ok(Self::Player),
            "Outpost" => Ok(Self::Outpost),
            "OutpostModule" => Ok(Self::OutpostModule),
            "Wreck" => Ok(Self::Wreck),
            "BeaconStation" => Ok(Self::BeaconStation),
            "EnemySubmarine" => Ok(Self::EnemySubmarine),
            "Ruin" => Ok(Self::Ruin),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CrewExperienceLevel {
    Unknown,
    Low,
    Mid,
    High,
}

impl FromStr for CrewExperienceLevel {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Unknown" => Ok(CrewExperienceLevel::Unknown),
            "CrewExperienceLow" | "Beginner" => Ok(CrewExperienceLevel::Low),
            "CrewExperienceMid" | "Intermediate" => Ok(CrewExperienceLevel::Mid),
            "CrewExperienceHigh" | "Experienced" => Ok(CrewExperienceLevel::High),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

pub struct Vector2(pub Vec2);

impl FromStr for Vector2 {
    type Err = ParseVectorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',').map(|v| v.trim());
        let x = split
            .next()
            .ok_or(ParseVectorError::NotEnoughComponents)?
            .parse::<f32>()?;
        let y = split
            .next()
            .ok_or(ParseVectorError::NotEnoughComponents)?
            .parse::<f32>()?;
        Ok(Self(Vec2 { x, y }))
    }
}

#[derive(Debug)]
pub enum ParseVectorError {
    NotEnoughComponents,
    ParseFloatError(ParseFloatError),
}

impl From<ParseFloatError> for ParseVectorError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

#[bitfield(u8)]
pub struct SubmarineTag {
    pub shuttle: bool,
    pub hide_in_menus: bool,
    #[bits(6)]
    _unused: u8,
}

impl FromStr for SubmarineTag {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "0" => Ok(Self::new()),
            "shuttle" | "1" => Ok(Self::new().with_shuttle(true)),
            "hideinmenus" | "2" => Ok(Self::new().with_hide_in_menus(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct Structure {
    pub map_entity: MapEntity,

    pub name: String,
    pub identifier: String,
    pub rect: Rect,
    pub id: u32,
    pub flipped_x: bool,
    pub flipped_y: bool,
    pub indestructible: bool,
    pub cast_shadow: bool,
    pub max_health: f32,
    pub crush_depth: f32,
    pub special_tag: Option<String>,
    pub sprite_color: Color,
    pub use_drop_shadow: bool,
    pub drop_shadow_offset: Vec2,
    pub rotation: f32,
    pub texture_scale: Vec2,
    pub texture_offset: Vec2,
    pub no_ai_target: bool,

    pub section_damage: HashMap<usize, f32>,
    //TODO: upgrade
}

impl Structure {
    pub fn get_transformed_quad(&self, prefab: &StructurePrefab) -> Quad2D {
        Quad2D::from_submarine_rectangle(self.get_scaled_rect(prefab)).rotated(
            if self.flipped_x != self.flipped_y {
                self.rotation_rad()
            } else {
                -self.rotation_rad()
            },
        )
    }

    pub fn rotation_rad(&self) -> f32 {
        wrap_angle(self.rotation.to_radians())
    }

    pub fn get_scaled_rect(&self, prefab: &StructurePrefab) -> Rect {
        let relative_scale = self.map_entity.scale / prefab.properties.map_entity_properties.scale;

        let new_width = ((self.rect.width as f32 * relative_scale) as i32).max(1);
        let new_height = ((self.rect.height as f32 * relative_scale) as i32).max(1);
        Rect {
            x: self.rect.x,
            y: self.rect.y,
            width: new_width.try_into().unwrap(),
            height: new_height.try_into().unwrap(),
        }
    }
}

/// Reduces a given angle to a value between pi and -pi.
fn wrap_angle(mut angle: f32) -> f32 {
    if angle > -std::f32::consts::PI && angle <= std::f32::consts::PI {
        return angle;
    }
    angle %= std::f32::consts::TAU;
    if angle <= -std::f32::consts::PI {
        return angle + std::f32::consts::TAU;
    }
    if angle > std::f32::consts::PI {
        return angle - std::f32::consts::TAU;
    }
    angle
}

#[derive(Debug)]
pub struct MapEntity {
    pub disallowed_upgrades: Vec<String>,
    pub rect_width: u32,
    pub rect_height: u32,
    pub sprite_depth: f32,
    pub scale: f32,
    pub hidden_in_game: bool,
    pub remove_if_linked_outpost_door_in_use: bool,
    pub layer: Option<String>,
}

#[derive(Debug)]
pub enum Condition {
    Percentage(f32),
    Value(f32),
}

#[derive(Debug)]
pub struct ItemSaveableProperties {
    pub description_tag: Option<String>,
    pub non_interactable: bool,
    pub non_player_team_interactable: bool,
    pub allow_swapping: bool,
    pub purchased_new_swap: bool,
    pub rotation: f32,
    pub sprite_color: Color,
    pub inventory_icon_color: Color,
    pub container_color: Color,
    pub sonar_label: Option<String>,
    pub health_multiplier: f32,
    pub max_repair_condition_multiplier: f32,
    pub has_been_instantiated_once: bool,
    pub invulnerable_to_damage: bool,
    pub allow_stealing: bool,
    pub original_outpost: Option<String>,
    pub tags: Vec<String>,
    pub display_side_by_side_when_linked: bool,
}

impl ItemSaveableProperties {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            description_tag: element
                .attribute_ignore_ascii_case("descriptiontag")
                .map(|v| v.to_owned()),
            non_interactable: element
                .attribute_ignore_ascii_case("noninteractable")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            non_player_team_interactable: element
                .attribute_ignore_ascii_case("nonplayerteaminteractable")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            allow_swapping: element
                .attribute_ignore_ascii_case("allowswapping")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            purchased_new_swap: element
                .attribute_ignore_ascii_case("purchasednewswap")
                .map_or(false, |v| v.parse().unwrap()),
            rotation: element
                .attribute_ignore_ascii_case("rotation")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            sprite_color: element
                .attribute_ignore_ascii_case("spritecolor")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            inventory_icon_color: element
                .attribute_ignore_ascii_case("inventoryiconcolor")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            container_color: element
                .attribute_ignore_ascii_case("containercolor")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            sonar_label: element
                .attribute_ignore_ascii_case("sonarlabel")
                .map(|v| v.to_owned()),
            health_multiplier: element
                .attribute_ignore_ascii_case("healthmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            max_repair_condition_multiplier: element
                .attribute_ignore_ascii_case("maxrepairconditionmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            has_been_instantiated_once: element
                .attribute_ignore_ascii_case("hasbeeninstantiatedonce")
                .map_or(false, |v| v.parse().unwrap()),
            invulnerable_to_damage: element
                .attribute_ignore_ascii_case("invulnerabletodamage")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            allow_stealing: element
                .attribute_ignore_ascii_case("allowstealing")
                .map_or(true, |v| v.parse().unwrap()),
            original_outpost: element
                .attribute_ignore_ascii_case("originaloutpost")
                .map(|v| v.to_owned()),
            tags: element
                .attribute_ignore_ascii_case("tags")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap(),
            display_side_by_side_when_linked: element
                .attribute_ignore_ascii_case("displaysidebysidewhenlinked")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub identifier: String,
    pub pending_swap: Option<String>,
    pub rect: Rect,
    pub id: u32,
    pub linked_to_ids: Vec<u32>,
    pub is_override: bool,
    pub available_swap_ids: Vec<String>,
    pub marked_for_deconstruction: bool,
    pub flipped_x: bool,
    pub flipped_y: bool,
    pub sprite_depth: f32,
    pub sprite_color: Color,
    pub rotation: f32,
    pub purchased_new_swap: bool,
    pub scale: f32,
    pub condition: Condition,
    pub properties: ItemSaveableProperties,

    pub components: ItemComponents,
}

impl Item {
    pub fn get_transformed_quad(&self, prefab: &ItemPrefab) -> Quad2D {
        Quad2D::from_submarine_rectangle(self.rect).rotated(-self.rotation_rad())
    }

    pub fn rotation_rad(&self) -> f32 {
        wrap_angle(self.rotation.to_radians())
    }

    pub fn position(&self, prefab: &ItemPrefab) -> Vec2 {
        if let Some(body) = &prefab.body {
            todo!();
            //body.physics_body.position
        } else {
            let rect_pos = Vec2 {
                x: self.rect.x as f32 + self.rect.width as f32 / 2.0,
                y: self.rect.y as f32 + self.rect.height as f32 / 2.0,
            };

            rect_pos
        }
    }
}

#[derive(Debug)]
pub struct Gap {
    pub rect: Rect,
    pub horizontal: bool,
    pub id: u32,
    pub layer: Option<String>,
    pub hidden_in_game: bool,
}

#[derive(Debug)]
pub struct Hull {
    pub rect: Rect,
    pub id: u32,
    pub water_volume: f32,
    pub original_ambient_light: Option<Color>,
    pub linked_to_ids: Vec<u32>,
    pub background_sections: Vec<(u32, Color, f32)>,

    pub room_name: String,
    pub ambient_light: Color,
    pub oxygen: f32,
    pub is_wet_room: bool,
    pub avoid_staying: bool,
}

#[derive(Hash, PartialEq, Eq)]
pub struct TalentStatIdentifier {
    ty: ItemTalentStat,
    talent_identifier: String,
}

#[derive(Hash, PartialEq, Eq)]
pub enum ItemTalentStat {
    DetoriationSpeed,
    BatteryCapacity,
    EngineSpeed,
    EngineMaxSpeed,
    PumpSpeed,
    ReactorMaxOutput,
    ReactorFuelConsumption,
    DeconstructorSpeed,
    FabricatorSpeed,
    ExtraStackSize,

    #[deprecated(note = "Use PumpSpeed instead.")]
    PumpMaxFlow,
}

impl FromStr for ItemTalentStat {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "detoriationspeed" => Ok(Self::DetoriationSpeed),
            "batterycapacity" => Ok(Self::BatteryCapacity),
            "enginespeed" => Ok(Self::EngineSpeed),
            "enginemaxspeed" => Ok(Self::EngineMaxSpeed),
            "pumpspeed" => Ok(Self::PumpSpeed),
            #[allow(deprecated)]
            "pumpmaxflow" => Ok(Self::PumpMaxFlow),
            "reactormaxoutput" => Ok(Self::ReactorMaxOutput),
            "reactorfuelconsumption" => Ok(Self::ReactorFuelConsumption),
            "deconstructorspeed" => Ok(Self::DeconstructorSpeed),
            "fabricatorspeed" => Ok(Self::FabricatorSpeed),
            "extrastacksize" => Ok(Self::ExtraStackSize),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct WayPoint {
    pub x: i32,
    pub y: i32,
    pub spawn_type: SpawnType,
    pub id: u32,
    pub layer: Option<String>,
    pub id_card_description: Option<String>,
    pub id_card_tags: Vec<String>,
    pub exit_point_size: Option<Point>,
    pub tags: Vec<String>,
    pub job: Option<String>,
    pub ladder_id: Option<u32>,
    pub gap_id: Option<u32>,
    pub linked_to_ids: Vec<u32>,
}

#[derive(Debug)]
pub struct LinkedSubmarine {
    pub pos: Vec2,
    pub level_seed: Option<String>,
    pub file_path: String,
    pub linked_to_ids: Vec<u32>,
    pub original_linked_to_id: u32,
    pub original_my_port_id: u16,
    pub cargo_capacity: u32,
}
