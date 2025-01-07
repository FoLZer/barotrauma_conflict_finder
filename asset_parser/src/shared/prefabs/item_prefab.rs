use std::{collections::HashMap, num::ParseFloatError, str::FromStr};

use bitfield_struct::bitfield;
use glam::{Vec2, Vec4};
use lazy_static::lazy_static;
use log::warn;
use roxmltree::Node;

use crate::shared::{
    submarine_info::{ParseVectorError, Vector2},
    util::NodeExp,
};

use super::{item_assembly_prefab::Rect, level_object_prefab::PhysicsBody};

#[bitfield(u16)]

pub struct MapEntityCategory {
    pub structure: bool,
    pub decorative: bool,
    pub machine: bool,
    pub medical: bool,
    pub weapon: bool,
    pub diving: bool,
    pub equipment: bool,
    pub fuel: bool,
    pub electrical: bool,
    pub material: bool,
    pub alien: bool,
    pub wrecked: bool,
    pub item_assembly: bool,
    pub legacy: bool,
    pub misc: bool,
    _unused: bool,
}

impl FromStr for MapEntityCategory {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "none" => Ok(Self::new()),
            "structure" => Ok(Self::new().with_structure(true)),
            "decorative" => Ok(Self::new().with_decorative(true)),
            "machine" => Ok(Self::new().with_machine(true)),
            "medical" => Ok(Self::new().with_medical(true)),
            "weapon" => Ok(Self::new().with_weapon(true)),
            "diving" => Ok(Self::new().with_diving(true)),
            "equipment" => Ok(Self::new().with_equipment(true)),
            "fuel" => Ok(Self::new().with_fuel(true)),
            "electrical" => Ok(Self::new().with_electrical(true)),
            "material" => Ok(Self::new().with_material(true)),
            "alien" => Ok(Self::new().with_alien(true)),
            "wrecked" => Ok(Self::new().with_wrecked(true)),
            "itemassembly" => Ok(Self::new().with_item_assembly(true)),
            "legacy" => Ok(Self::new().with_legacy(true)),
            "misc" => Ok(Self::new().with_misc(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct DoesNotExistError(pub String);

#[derive(Debug)]
pub struct MapEntityProperties {
    pub resize_horizontal: bool,
    pub resize_vertical: bool,
    pub description: String,
    pub allowed_upgrades: String,
    pub hide_in_menus: bool,
    pub hide_in_editors: bool,
    pub subcategory: String,
    pub linkable: bool,
    pub sprite_color: Color,
    pub scale: f32,
}

impl MapEntityProperties {
    pub fn new(element: Node) -> Self {
        Self {
            resize_horizontal: element
                .attribute_ignore_ascii_case("resizehorizontal")
                .is_some_and(|v| v.parse().unwrap()),
            resize_vertical: element
                .attribute_ignore_ascii_case("resizevertical")
                .is_some_and(|v| v.parse().unwrap()),
            description: element
                .attribute_ignore_ascii_case("description")
                .map(|v| v.parse().unwrap())
                .unwrap_or_default(),
            allowed_upgrades: element
                .attribute_ignore_ascii_case("allowedupgrades")
                .map(|v| v.parse().unwrap())
                .unwrap_or_default(),
            hide_in_menus: element
                .attribute_ignore_ascii_case("hideinmenus")
                .is_some_and(|v| v.to_lowercase().parse().unwrap()),
            hide_in_editors: element
                .attribute_ignore_ascii_case("hideineditors")
                .is_some_and(|v| v.parse().unwrap()),
            subcategory: element
                .attribute_ignore_ascii_case("subcategory")
                .map(|v| v.parse().unwrap())
                .unwrap_or_default(),
            linkable: element
                .attribute_ignore_ascii_case("linkable")
                .is_some_and(|v| v.parse().unwrap()),
            sprite_color: element.attribute_ignore_ascii_case("spritecolor").map_or(
                Color::Simple {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                |v| {
                    v.trim().parse().unwrap()
                    /*
                    match v.trim().parse() {
                        Ok(v) => v,
                        Err(e) => {
                            dbg!(v);
                            panic!()
                        }
                    }*/
                },
            ),
            scale: element
                .attribute_ignore_ascii_case("scale")
                .map_or(1.0, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct ItemProperties {
    pub interact_distance: f32,
    pub interact_priority: f32,
    pub interact_through_walls: bool,
    pub hide_condition_bar: bool,
    pub hide_condition_in_tooltip: bool,
    pub require_body_inside_trigger: bool,
    pub require_cursor_inside_trigger: bool,
    pub require_campaign_interact: bool,
    pub focus_on_selected: bool,
    pub offset_on_selected: f32,
    pub grab_when_selected: bool,
    pub health: f32,
    pub allow_selling_when_broken: bool,
    pub allow_stealing_always: bool,
    pub indestructible: bool,
    pub damaged_by_explosions: bool,
    pub explosion_damage_multiplier: f32,
    pub item_damage_multiplier: f32,
    pub damaged_by_projectiles: bool,
    pub damaged_by_melee_weapons: bool,
    pub damaged_by_repair_tools: bool,
    pub damaged_by_monsters: bool,
    pub impact_tolerance: f32,
    pub on_damaged_threshold: f32,
    pub sonar_size: f32,
    pub use_in_health_interface: bool,
    pub disable_item_usage_when_selected: bool,
    pub cargo_container_identifier: String,
    pub use_contained_sprite_color: bool,
    pub use_contained_inventory_icon_color: bool,
    pub added_repair_speed_multiplier: f32,
    pub added_picking_speed_multiplier: f32,
    pub cannot_repair_fail: bool,
    pub equip_confirmation_text: Option<String>,
    pub allow_rotating_in_editor: bool,
    pub show_contents_in_tooltip: bool,
    pub can_flip_x: bool,
    pub can_flip_y: bool,
    pub is_dangerous: bool,
    pub max_stack_size: u32,
    pub max_stack_size_character_inventory: Option<u32>,
    pub max_stack_size_holdable_or_wearable_inventory: Option<u32>,
    pub allow_dropping_on_swap: bool,
    pub dont_transfer_between_subs: bool,
    pub show_health_bar: bool,
    pub bot_priority: f32,
    pub show_name_in_health_bar: bool,
    pub is_ai_turret_target: bool,
    pub ai_turret_priority: f32,
    pub ai_slow_turret_priority: f32,
    pub ai_turret_targeting_max_distance: Option<f32>,
    pub allow_stealing_contained_items: bool,
    pub signal_component_color: Color,
    pub disable_command_menu_when_selected: bool,
}

impl ItemProperties {
    pub fn new(element: Node) -> Self {
        Self {
            interact_distance: element
                .attribute_ignore_ascii_case("interactdistance")
                .map_or(120.0, |v| v.parse().unwrap()),
            interact_priority: element
                .attribute_ignore_ascii_case("interactpriority")
                .map_or(0.0, |v| v.parse().unwrap()),
            interact_through_walls: element
                .attribute_ignore_ascii_case("interactthroughwalls")
                .is_some_and(|v| v.parse().unwrap()),
            hide_condition_bar: element
                .attribute_ignore_ascii_case("hideconditionbar")
                .is_some_and(|v| v.parse().unwrap()),
            hide_condition_in_tooltip: element
                .attribute_ignore_ascii_case("hideconditionintooltip")
                .is_some_and(|v| v.parse().unwrap()),
            require_body_inside_trigger: element
                .attribute_ignore_ascii_case("requirebodyinsidetrigger")
                .map_or(true, |v| v.parse().unwrap()),
            require_cursor_inside_trigger: element
                .attribute_ignore_ascii_case("requirecursorinsidetrigger")
                .is_some_and(|v| v.parse().unwrap()),
            require_campaign_interact: element
                .attribute_ignore_ascii_case("requirecampaigninteract")
                .is_some_and(|v| v.parse().unwrap()),
            focus_on_selected: element
                .attribute_ignore_ascii_case("focusonselected")
                .is_some_and(|v| v.parse().unwrap()),
            offset_on_selected: element
                .attribute_ignore_ascii_case("offsetonselected")
                .map_or(0.0, |v| v.parse().unwrap()),
            grab_when_selected: element
                .attribute_ignore_ascii_case("grabwhenselected")
                .is_some_and(|v| v.parse().unwrap()),
            health: element
                .attribute_ignore_ascii_case("health")
                .map_or(100.0, |v| v.parse().unwrap()),
            allow_selling_when_broken: element
                .attribute_ignore_ascii_case("allowsellingwhenbroken")
                .is_some_and(|v| v.parse().unwrap()),
            allow_stealing_always: element
                .attribute_ignore_ascii_case("allowstealingalways")
                .is_some_and(|v| v.parse().unwrap()),
            indestructible: element
                .attribute_ignore_ascii_case("indestructible")
                .is_some_and(|v| v.parse().unwrap()),
            damaged_by_explosions: element
                .attribute_ignore_ascii_case("damagedbyexplosions")
                .is_some_and(|v| v.parse().unwrap()),
            explosion_damage_multiplier: element
                .attribute_ignore_ascii_case("explosiondamagemultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            item_damage_multiplier: element
                .attribute_ignore_ascii_case("itemdamagemultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            damaged_by_projectiles: element
                .attribute_ignore_ascii_case("damagedbyprojectiles")
                .is_some_and(|v| v.parse().unwrap()),
            damaged_by_melee_weapons: element
                .attribute_ignore_ascii_case("damagedbymeleeweapons")
                .is_some_and(|v| v.parse().unwrap()),
            damaged_by_repair_tools: element
                .attribute_ignore_ascii_case("damagedbyrepairtools")
                .is_some_and(|v| v.parse().unwrap()),
            damaged_by_monsters: element
                .attribute_ignore_ascii_case("damagedbymonsters")
                .is_some_and(|v| v.parse().unwrap()),
            impact_tolerance: element
                .attribute_ignore_ascii_case("impacttolerance")
                .map_or(0.0, |v| v.parse().unwrap()),
            on_damaged_threshold: element
                .attribute_ignore_ascii_case("ondamagedthreshold")
                .map_or(0.0, |v| v.parse().unwrap()),
            sonar_size: element
                .attribute_ignore_ascii_case("sonarsize")
                .map_or(0.0, |v| v.parse().unwrap()),
            use_in_health_interface: element
                .attribute_ignore_ascii_case("useinhealthinterface")
                .is_some_and(|v| v.to_lowercase().parse().unwrap()),
            disable_item_usage_when_selected: element
                .attribute_ignore_ascii_case("disableitemusagewhenselected")
                .is_some_and(|v| v.parse().unwrap()),
            cargo_container_identifier: element
                .attribute_ignore_ascii_case("cargocontaineridentifier")
                .unwrap_or("metalcrate")
                .to_owned(),
            use_contained_sprite_color: element
                .attribute_ignore_ascii_case("usecontainedspritecolor")
                .is_some_and(|v| v.parse().unwrap()),
            use_contained_inventory_icon_color: element
                .attribute_ignore_ascii_case("usecontainedinventoryiconcolor")
                .is_some_and(|v| v.parse().unwrap()),
            added_repair_speed_multiplier: element
                .attribute_ignore_ascii_case("addedrepairspeedmultiplier")
                .map_or(0.0, |v| v.parse().unwrap()),
            added_picking_speed_multiplier: element
                .attribute_ignore_ascii_case("addedpickingspeedmultiplier")
                .map_or(0.0, |v| v.parse().unwrap()),
            cannot_repair_fail: element
                .attribute_ignore_ascii_case("cannotrepairfail")
                .is_some_and(|v| v.parse().unwrap()),
            equip_confirmation_text: element
                .attribute_ignore_ascii_case("equipconfirmationtext")
                .map(|v| v.parse().unwrap()),
            allow_rotating_in_editor: element
                .attribute_ignore_ascii_case("allowrotatingineditor")
                .map_or(true, |v| v.parse().unwrap()),
            show_contents_in_tooltip: element
                .attribute_ignore_ascii_case("showcontentsintooltip")
                .is_some_and(|v| v.parse().unwrap()),
            can_flip_x: element
                .attribute_ignore_ascii_case("canflipx")
                .map_or(true, |v| v.parse().unwrap()),
            can_flip_y: element
                .attribute_ignore_ascii_case("canflipy")
                .map_or(true, |v| v.parse().unwrap()),
            is_dangerous: element
                .attribute_ignore_ascii_case("isdangerous")
                .is_some_and(|v| v.parse().unwrap()),
            max_stack_size: element
                .attribute_ignore_ascii_case("maxstacksize")
                .map_or(1, |v| v.parse().unwrap()),
            max_stack_size_character_inventory: element
                .attribute_ignore_ascii_case("maxstacksizecharacterinventory")
                .map(|v| v.parse().unwrap()),
            max_stack_size_holdable_or_wearable_inventory: element
                .attribute_ignore_ascii_case("maxstacksizeholdableorwearableinventory")
                .map(|v| v.parse().unwrap()),
            allow_dropping_on_swap: element
                .attribute_ignore_ascii_case("allowdroppingonswap")
                .is_some_and(|v| v.parse().unwrap()),
            dont_transfer_between_subs: element
                .attribute_ignore_ascii_case("donttransferbetweensubs")
                .is_some_and(|v| v.parse().unwrap()),
            show_health_bar: element
                .attribute_ignore_ascii_case("showhealthbar")
                .map_or(true, |v| v.parse().unwrap()),
            bot_priority: element
                .attribute_ignore_ascii_case("botpriority")
                .map_or(1.0, |v| v.parse().unwrap()),
            show_name_in_health_bar: element
                .attribute_ignore_ascii_case("shownameinhealthbar")
                .map_or(true, |v| v.parse().unwrap()),
            is_ai_turret_target: element
                .attribute_ignore_ascii_case("isaiturrettarget")
                .is_some_and(|v| v.parse().unwrap()),
            ai_turret_priority: element
                .attribute_ignore_ascii_case("aiturretpriority")
                .map_or(1.0, |v| v.parse().unwrap()),
            ai_slow_turret_priority: element
                .attribute_ignore_ascii_case("aislowturretpriority")
                .map_or(1.0, |v| v.parse().unwrap()),
            ai_turret_targeting_max_distance: element
                .attribute_ignore_ascii_case("aiturrettargetingmaxdistance")
                .map(|v| v.parse().unwrap()),
            allow_stealing_contained_items: element
                .attribute_ignore_ascii_case("allowstealingcontaineditems")
                .is_some_and(|v| v.parse().unwrap()),
            signal_component_color: element
                .attribute_ignore_ascii_case("signalcomponentcolor")
                .map(Color::from_str)
                .map_or(
                    Color::Simple {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    std::result::Result::unwrap,
                ),
            disable_command_menu_when_selected: element
                .attribute_ignore_ascii_case("disablecommandmenuwhenselected")
                .is_some_and(|v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    Gui(String),
    Faction(String),
    Simple { r: f32, g: f32, b: f32, a: f32 },
}

lazy_static! {
    static ref MONO_GAME_COLORS: HashMap<&'static str, Color> = {
        let mut map = HashMap::new();
        map.insert("Transparent", Color::Simple {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        });
        map
    };
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= 4 && s[..4].eq_ignore_ascii_case("gui.") {
            Ok(Self::Gui(s[4..].to_owned()))
        } else if s.len() >= 8 && s[..8].eq_ignore_ascii_case("faction.") {
            Ok(Self::Faction(s[8..].to_owned()))
        } else {
            let mut spl = s.split(',').map(|v| v.trim());
            let Some(a) = spl.next() else {
                return Err(ParseColorError::NotEnoughComponents);
            };
            if let Some(b) = spl.next() {
                let mut ar = [
                    a.parse::<f32>().unwrap_or(0.0),
                    b.parse::<f32>().unwrap_or(0.0),
                    1.0,
                    1.0,
                ];
                let c = spl.next();
                let d = spl.next();
                if let Some(c) = c {
                    ar[2] = c.parse::<f32>().unwrap_or(0.0);
                }
                if let Some(d) = d {
                    ar[3] = d.parse::<f32>().unwrap_or(0.0);
                }
                if ar.iter().any(|v| *v > 1.0) {
                    ar.iter_mut().for_each(|v| *v /= 255.0);
                }
                Ok(Color::Simple {
                    r: ar[0],
                    g: ar[1],
                    b: ar[2],
                    a: ar[3],
                })
            } else {
                if let Some(c) = MONO_GAME_COLORS.get(a) {
                    return Ok(c.clone());
                }
                let s = a.trim();
                if let Some(s) = s.strip_prefix('#') {
                    let mut hex =
                        u32::from_str_radix(s, 16).map_err(|_| ParseColorError::HexParseFailed)?;
                    if s.len() == 6 {
                        hex = (hex << 8) | 0xff;
                    }
                    let r = ((hex & 0xff000000) >> 24) as f32 / 255.0;
                    let g = ((hex & 0x00ff0000) >> 16) as f32 / 255.0;
                    let b = ((hex & 0x0000ff00) >> 8) as f32 / 255.0;
                    let a = (hex & 0x000000ff) as f32 / 255.0;
                    Ok(Self::Simple { r, g, b, a })
                } else if let Some(s) = s.strip_prefix('{') {
                    let s = &s[1..];
                    let spl = s.split(' ');
                    let (mut r, mut g, mut b, mut a) = (1.0, 1.0, 1.0, 1.0);
                    for comp in spl.take(4) {
                        match comp[..2].to_lowercase().as_str() {
                            "r:" => {
                                let val = comp[..2]
                                    .parse::<u32>()
                                    .map_err(|_| ParseColorError::NameComponentedParseFailed)?;
                                r = val as f32 / 255.0;
                            }
                            "g:" => {
                                let val = comp[..2]
                                    .parse::<u32>()
                                    .map_err(|_| ParseColorError::NameComponentedParseFailed)?;
                                g = val as f32 / 255.0;
                            }
                            "b:" => {
                                let val = comp[..2]
                                    .parse::<u32>()
                                    .map_err(|_| ParseColorError::NameComponentedParseFailed)?;
                                b = val as f32 / 255.0;
                            }
                            "a:" => {
                                let val = comp[..2]
                                    .parse::<u32>()
                                    .map_err(|_| ParseColorError::NameComponentedParseFailed)?;
                                a = val as f32 / 255.0;
                            }
                            _ => (),
                        }
                    }
                    Ok(Color::Simple { r, g, b, a })
                } else {
                    Err(ParseColorError::NotEnoughComponents)
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseColorError {
    NotEnoughComponents,
    HexParseFailed,
    ParseFloatError(ParseFloatError),
    NameComponentedParseFailed,
}

impl From<ParseFloatError> for ParseColorError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

const NEUTRAL_DENSITY: f32 = 10.0;

#[derive(Debug)]
pub struct ItemPrefab {
    pub identifier: String,
    pub name: Option<String>,
    pub variant_of: Option<String>,
    pub category: MapEntityCategory,
    pub name_identifier: Option<String>,
    pub fallback_name_identifier: Option<String>,
    pub description_identifier: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub allow_as_extra_cargo: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub item_properties: ItemProperties,
    pub map_entity_properties: MapEntityProperties,
    pub allow_dropping_on_swap_with: Option<Vec<String>>,
    pub allowed_links: Option<Vec<String>>,
    pub skill_requirement_hints: Vec<SkillRequirementHint>,
    pub sprite: Option<ItemSprite>,
    pub default_price: Option<DefaultPriceInfo>,
    pub store_prices: HashMap<String, PriceInfo>,
    pub deconstruct_time: Option<f32>,
    pub allow_deconstruct: bool,
    pub random_deconstruction_output: bool,
    pub random_deconstruction_output_amount: Option<u32>,
    pub deconstruct_items: Vec<DeconstructItem>,
    pub fabrication_recipes: Vec<FabricationRecipe>,
    pub preferred_containers: Vec<PreferredContainer>,
    pub swappable_item: Option<SwappableItem>,
    pub triggers: Vec<Trigger>,
    pub level_commonness: HashMap<String, CommonnessInfo>,
    pub default_level_commonness: Option<CommonnessInfo>,
    pub level_quantity: HashMap<String, FixedQuantityResourceInfo>,
    pub default_level_quantity: Option<FixedQuantityResourceInfo>,
    pub suitable_treatments: HashMap<String, f32>,
    pub static_body: Option<ItemStaticBody>,
    pub body: Option<ItemBody>,
}

impl ItemPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(std::borrow::ToOwned::to_owned);
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(std::borrow::ToOwned::to_owned);
        let variant_of = element
            .attribute_ignore_ascii_case("inherit")
            .or(element.attribute_ignore_ascii_case("variantof"))
            .map(std::borrow::ToOwned::to_owned);

        let category = element.attribute_ignore_ascii_case("category").map_or(
            MapEntityCategory::new().with_misc(true),
            |v| {
                v.split(',')
                    .map(|v| {
                        v.parse()
                            .unwrap_or(MapEntityCategory::new().with_misc(true))
                    })
                    .fold(MapEntityCategory::new(), |acc, e: MapEntityCategory| {
                        MapEntityCategory::from_bits(acc.into_bits() | e.into_bits())
                    })
            },
        );

        let fallback_name_identifier = element
            .attribute_ignore_ascii_case("fallbacknameidentifier")
            .map(std::borrow::ToOwned::to_owned);

        let aliases = element
            .attribute_ignore_ascii_case("aliases")
            .or(element.attribute_ignore_ascii_case("Aliases"))
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<String>>()
            });

        let allow_as_extra_cargo = element
            .attribute_ignore_ascii_case("allowasextracargo")
            .map(|v| v.to_lowercase().parse::<bool>().unwrap()); //TODO: Error handling

        let tags = element
            .attribute_ignore_ascii_case("tags")
            .or(element.attribute_ignore_ascii_case("Tags"))
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<String>>()
            });

        let mut item_properties = ItemProperties::new(element);
        let map_entity_properties = MapEntityProperties::new(element);

        let description_identifier = element
            .attribute_ignore_ascii_case("descriptionidentifier")
            .map(std::borrow::ToOwned::to_owned);
        let name_identifier = element
            .attribute_ignore_ascii_case("nameidentifier")
            .map(std::borrow::ToOwned::to_owned);

        let allow_dropping_on_swap_with = element
            .attribute_ignore_ascii_case("allowdroppingonswapwith")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            });
        item_properties.allow_dropping_on_swap = allow_dropping_on_swap_with.is_some();

        let allowed_links = element
            .attribute_ignore_ascii_case("allowedlinks")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            });

        let mut skill_requirement_hints = Vec::new();
        let mut sprite = None;
        let mut default_price = None;
        let mut store_prices = HashMap::new();
        let mut deconstruct_time = None;
        let mut allow_deconstruct = false;
        let mut random_deconstruction_output = false;
        let mut random_deconstruction_output_amount = None;
        let mut deconstruct_items = Vec::new();
        let mut fabrication_recipes = Vec::new();
        let mut preferred_containers = Vec::new();
        let mut swappable_item = None;
        let mut triggers = Vec::new();
        let mut level_commonness = HashMap::new();
        let mut default_level_commonness = None;
        let mut level_quantity = HashMap::new();
        let mut default_level_quantity = None;
        let mut suitable_treatments = HashMap::new();
        let mut static_body = None;
        let mut body = None;

        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "skillrequirementhint" => {
                    skill_requirement_hints.push(SkillRequirementHint::new(child));
                }
                "sprite" => {
                    sprite = Some(ItemSprite::new(child).unwrap());
                }
                "price" => {
                    let infos = PriceInfos::new(child);
                    default_price = Some(infos.default_price);
                    store_prices.extend(
                        infos
                            .other_prices
                            .into_iter()
                            .map(|v| (v.store_identifier.clone(), v)),
                    );
                }
                "deconstruct" => {
                    deconstruct_time = child
                        .attribute_ignore_ascii_case("time")
                        .map(|v| v.parse::<f32>().unwrap());
                    allow_deconstruct = true;
                    random_deconstruction_output = child
                        .attribute_ignore_ascii_case("chooserandom")
                        .is_some_and(|v| v.parse::<bool>().unwrap());
                    random_deconstruction_output_amount = child
                        .attribute_ignore_ascii_case("amount")
                        .map(|v| v.parse::<u32>().unwrap());
                    deconstruct_items = child
                        .children()
                        .filter(Node::is_element)
                        .filter(|child| child.attribute_ignore_ascii_case("identifier").is_some()) //thanks nucleardepthchargecheap and nuclearshellcheap
                        .map(|child| DeconstructItem::new(child))
                        .collect::<Vec<_>>();
                }
                "fabricate" | "fabricable" | "fabricableitem" => {
                    fabrication_recipes.push(FabricationRecipe::new(child, variant_of.is_some()));
                }
                "preferredcontainer" => {
                    preferred_containers.push(PreferredContainer::new(child, variant_of.is_some()));
                }
                "swappableitem" => {
                    swappable_item = Some(SwappableItem::new(child));
                }
                "trigger" => triggers.push(Trigger {
                    x: child
                        .attribute_ignore_ascii_case("x")
                        .map_or(0, |v| v.parse().unwrap()),
                    y: child
                        .attribute_ignore_ascii_case("y")
                        .map_or(0, |v| v.parse().unwrap()),
                    width: child
                        .attribute_ignore_ascii_case("width")
                        .map_or(0, |v| v.parse().unwrap()),
                    height: child
                        .attribute_ignore_ascii_case("height")
                        .map_or(0, |v| v.parse().unwrap()),
                }),
                "levelresource" => {
                    for child in child
                        .children()
                        .filter(Node::is_element)
                        .filter(|v| v.tag_name().name().eq_ignore_ascii_case("commonness"))
                    {
                        let level_name = child.attribute_ignore_ascii_case("leveltype");
                        if !child
                            .attribute_ignore_ascii_case("fixedquantity")
                            .is_some_and(|v| v.parse().unwrap())
                        {
                            if let Some(level_name) = level_name {
                                if !level_commonness.contains_key(level_name) {
                                    level_commonness
                                        .insert(level_name.to_owned(), CommonnessInfo::new(child));
                                }
                            } else if default_level_commonness.is_none() {
                                default_level_commonness = Some(CommonnessInfo::new(child));
                            }
                        } else {
                            let cluster_quantity = child
                                .attribute_ignore_ascii_case("clusterquantity")
                                .map(|v| v.parse::<u32>().unwrap())
                                .unwrap();
                            let cluster_size = child
                                .attribute_ignore_ascii_case("clustersize")
                                .map_or(0, |v| v.parse::<u32>().unwrap());
                            let is_island_specific = child
                                .attribute_ignore_ascii_case("isislandspecific")
                                .is_some_and(|v| v.parse::<bool>().unwrap());
                            let allow_at_start = child
                                .attribute_ignore_ascii_case("allowatstart")
                                .map_or(true, |v| v.parse::<bool>().unwrap());
                            if let Some(level_name) = level_name {
                                if !level_quantity.contains_key(level_name) {
                                    level_quantity.insert(
                                        level_name.to_owned(),
                                        FixedQuantityResourceInfo {
                                            cluster_quantity,
                                            cluster_size,
                                            is_island_specific,
                                            allow_at_start,
                                        },
                                    );
                                }
                            } else if default_level_quantity.is_none() {
                                default_level_quantity = Some(FixedQuantityResourceInfo {
                                    cluster_quantity,
                                    cluster_size,
                                    is_island_specific,
                                    allow_at_start,
                                });
                            }
                        }
                    }
                }
                "suitabletreatment" => {
                    let identifier = child
                        .attribute_ignore_ascii_case("identifier")
                        .or(child.attribute_ignore_ascii_case("type"))
                        .unwrap();
                    let suitability = child
                        .attribute_ignore_ascii_case("suitability")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap();
                    suitable_treatments.insert(identifier.to_owned(), suitability);
                }
                "staticbody" => {
                    assert!(static_body.is_none());
                    static_body = Some(ItemStaticBody {
                        radius: child
                            .attribute_ignore_ascii_case("radius")
                            .map_or(0.0, |v| v.parse::<f32>().unwrap()),
                        width: child
                            .attribute_ignore_ascii_case("width")
                            .map_or(0.0, |v| v.parse::<f32>().unwrap()),
                        height: child
                            .attribute_ignore_ascii_case("height")
                            .map_or(0.0, |v| v.parse::<f32>().unwrap()),
                    })
                }
                "body" => {
                    let physics_body = PhysicsBody::new(child);
                    let density = child
                        .attribute_ignore_ascii_case("density")
                        .map_or(NEUTRAL_DENSITY, |v| v.parse::<f32>().unwrap());
                    let min_density = child
                        .attribute_ignore_ascii_case("mindensity")
                        .map(|v| v.parse::<f32>().unwrap());
                    let max_density = child
                        .attribute_ignore_ascii_case("maxdensity")
                        .map(|v| v.parse::<f32>().unwrap());
                    let collision_category = child
                        .attribute_ignore_ascii_case("collisioncategory")
                        .map(|v| {
                            v.split(',')
                                .map(|v| v.parse::<CollisionCategory>().unwrap())
                                .reduce(|acc, e| {
                                    CollisionCategory::from_bits(acc.into_bits() | e.into_bits())
                                })
                                .unwrap()
                        })
                        .map(|mut category| {
                            if category.cat2() {
                                //CollisionCharacter
                                category.set_cat7(true); //CollisionProjectile
                            }
                            category
                        });
                    let angular_damping = child
                        .attribute_ignore_ascii_case("angulardamping")
                        .map_or(0.2, |v| v.parse::<f32>().unwrap());
                    let linear_damping = child
                        .attribute_ignore_ascii_case("lineardamping")
                        .map_or(0.1, |v| v.parse::<f32>().unwrap());
                    body = Some(ItemBody {
                        physics_body,
                        density,
                        min_density,
                        max_density,
                        collision_category,
                        angular_damping,
                        linear_damping,
                    })
                }
                _ => (),
            }
        }

        skill_requirement_hints.shrink_to_fit();
        store_prices.shrink_to_fit();
        deconstruct_items.shrink_to_fit();
        fabrication_recipes.shrink_to_fit();
        preferred_containers.shrink_to_fit();
        triggers.shrink_to_fit();
        level_commonness.shrink_to_fit();
        level_quantity.shrink_to_fit();
        suitable_treatments.shrink_to_fit();

        Self {
            identifier: identifier.unwrap(),
            name,
            variant_of,
            category,
            name_identifier,
            fallback_name_identifier,
            description_identifier,
            aliases,
            allow_as_extra_cargo,
            tags,
            item_properties,
            map_entity_properties,
            allow_dropping_on_swap_with,
            allowed_links,
            skill_requirement_hints,
            sprite,
            default_price,
            store_prices,
            deconstruct_time,
            allow_deconstruct,
            random_deconstruction_output,
            random_deconstruction_output_amount,
            deconstruct_items,
            fabrication_recipes,
            preferred_containers,
            swappable_item,
            triggers,
            level_commonness,
            default_level_commonness,
            level_quantity,
            default_level_quantity,
            suitable_treatments,
            static_body,
            body,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct SkillRequirementHint {
    pub skill: String,
    pub level: f32,
    //TODO: pub skill_name: String
}

impl SkillRequirementHint {
    pub fn new(element: Node) -> Self {
        Self {
            skill: element
                .attribute_ignore_ascii_case("identifier")
                .unwrap_or("")
                .to_owned(),
            level: element
                .attribute_ignore_ascii_case("level")
                .map_or(0.0, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct CommonnessInfo {
    pub commonness: Option<f32>,
    pub abyss_commonness: Option<f32>,
    pub cave_commonness: Option<f32>,
}

impl CommonnessInfo {
    pub fn new(element: Node) -> Self {
        let commonness = element
            .attribute_ignore_ascii_case("commonness")
            .map(|v| v.parse::<f32>().unwrap());
        let abyss_commonness = element
            .attribute_ignore_ascii_case("abysscommonness")
            .or(element.attribute_ignore_ascii_case("abyss"))
            .map(|v| v.parse::<f32>().unwrap());
        let cave_commonness = element
            .attribute_ignore_ascii_case("cavecommonness")
            .or(element.attribute_ignore_ascii_case("cave"))
            .map(|v| v.parse::<f32>().unwrap());

        Self {
            commonness,
            abyss_commonness,
            cave_commonness,
        }
    }
}

#[derive(Debug)]
pub struct FixedQuantityResourceInfo {
    pub cluster_quantity: u32,
    pub cluster_size: u32,
    pub is_island_specific: bool,
    pub allow_at_start: bool,
}

#[derive(Debug)]
pub struct Trigger {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct SwappableItem {
    pub base_price: Option<u32>,
    pub swap_identifier: Option<String>,
    pub can_be_bought: bool,
    pub replacement_on_uninstall: Option<String>,
    pub swap_origin: Option<Vec2>,
    pub spawn_with_id: Option<String>,
    pub schematic_sprite: Option<BarotraumaSprite>,
    pub connected_items_to_swap: Vec<SwapConnectedItem>,
}

impl SwappableItem {
    pub fn new(element: Node) -> Self {
        let base_price = element
            .attribute_ignore_ascii_case("price")
            .map(|v| v.parse::<u32>().unwrap());
        let swap_identifier = element
            .attribute_ignore_ascii_case("swapidentifier")
            .map(std::borrow::ToOwned::to_owned);
        let can_be_bought = element
            .attribute_ignore_ascii_case("canbebought")
            .map_or(base_price.is_some_and(|v| v != 0), |v| {
                v.parse::<bool>().unwrap()
            });
        let replacement_on_uninstall = element
            .attribute_ignore_ascii_case("replacementonuninstall")
            .map(std::borrow::ToOwned::to_owned);
        let swap_origin = element
            .attribute_ignore_ascii_case("origin")
            .map(|v| v.parse::<Vector2>().unwrap());
        let spawn_with_id = element
            .attribute_ignore_ascii_case("spawnwithid")
            .map(std::borrow::ToOwned::to_owned);

        let mut schematic_sprite = None;
        let mut connected_items_to_swap = Vec::new();

        for child in element.children() {
            if child.is_element() {
                match child.tag_name().name().to_lowercase().as_str() {
                    "schematicsprite" => {
                        schematic_sprite = Some(BarotraumaSprite::new(child));
                    }
                    "swapconnecteditem" => {
                        let tag = child
                            .attribute_ignore_ascii_case("tag")
                            .map(std::borrow::ToOwned::to_owned)
                            .unwrap();
                        let swap_to = child
                            .attribute_ignore_ascii_case("swapto")
                            .map(std::borrow::ToOwned::to_owned)
                            .unwrap();
                        connected_items_to_swap.push(SwapConnectedItem { tag, swap_to });
                    }
                    _ => (),
                }
            }
        }
        connected_items_to_swap.shrink_to_fit();

        Self {
            base_price,
            swap_identifier,
            can_be_bought,
            replacement_on_uninstall,
            swap_origin: swap_origin.map(|v| v.0),
            spawn_with_id,
            schematic_sprite,
            connected_items_to_swap,
        }
    }
}

#[derive(Debug)]
pub struct SwapConnectedItem {
    pub tag: String,
    pub swap_to: String,
}

#[derive(Debug)]
pub struct PreferredContainer {
    pub primary: Option<Vec<String>>,
    pub secondary: Option<Vec<String>>,
    pub spawn_probability: Option<f32>,
    pub min_amount: Option<u32>,
    pub max_amount: Option<u32>,
    pub amount: Option<u32>,
    pub min_condition: Option<f32>,
    pub max_condition: Option<f32>,
    pub campaign_only: bool,
    pub no_campaign: bool,
    pub transfer_only_one_per_container: bool,
    pub allow_transfers_here: bool,
    pub min_level_difficulty: Option<f32>,
    pub max_level_difficulty: Option<f32>,
}

impl PreferredContainer {
    pub fn new(element: Node, is_variant_of: bool) -> Self {
        let primary = element.attribute_ignore_ascii_case("primary").map(|v| {
            v.split(',')
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<_>>()
        });
        let secondary = element.attribute_ignore_ascii_case("secondary").map(|v| {
            v.split(',')
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<_>>()
        });
        let mut spawn_probability = element
            .attribute_ignore_ascii_case("spawnprobability")
            .map(|v| v.parse::<f32>().unwrap());
        let mut min_amount = element
            .attribute_ignore_ascii_case("minamount")
            .map(|v| v.parse::<u32>().unwrap());
        let mut max_amount = element
            .attribute_ignore_ascii_case("maxamount")
            .map(|v| v.parse::<u32>().unwrap());
        let mut amount = element
            .attribute_ignore_ascii_case("amount")
            .map(|v| v.parse::<u32>().unwrap());
        let min_condition = element
            .attribute_ignore_ascii_case("mincondition")
            .map(|v| v.parse::<f32>().unwrap());
        let max_condition = element
            .attribute_ignore_ascii_case("maxcondition")
            .map(|v| v.parse::<f32>().unwrap());
        let campaign_only = element
            .attribute_ignore_ascii_case("campaignonly")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let no_campaign = element
            .attribute_ignore_ascii_case("nocampaign")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let transfer_only_one_per_container = element
            .attribute_ignore_ascii_case("transferonlyonepercontainer")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let allow_transfers_here = element
            .attribute_ignore_ascii_case("allowtransfershere")
            .map_or(true, |v| v.parse::<bool>().unwrap());
        let min_level_difficulty = element
            .attribute_ignore_ascii_case("minleveldifficulty")
            .map(|v| v.parse::<f32>().unwrap());
        let max_level_difficulty = element
            .attribute_ignore_ascii_case("maxleveldifficulty")
            .map(|v| v.parse::<f32>().unwrap());
        if spawn_probability.is_none() {
            if max_amount.is_some_and(|v| v > 0) || amount.is_some_and(|v| v > 0) {
                spawn_probability = Some(1.0);
            }
        } else if min_amount.is_none() && max_amount.is_none() && amount.is_none() {
            min_amount = Some(1);
            max_amount = Some(1);
            amount = Some(1);
        }
        assert!(!(!is_variant_of && primary.is_none() && secondary.is_none()),);

        Self {
            primary,
            secondary,
            spawn_probability,
            min_amount,
            max_amount,
            amount,
            min_condition,
            max_condition,
            campaign_only,
            no_campaign,
            transfer_only_one_per_container,
            allow_transfers_here,
            min_level_difficulty,
            max_level_difficulty,
        }
    }
}

#[derive(Debug)]
pub struct FabricationRecipe {
    pub display_name: Option<String>,
    pub suitable_fabricators: Option<Vec<String>>,
    pub required_time: f32,
    pub required_money: u32,
    pub out_condition: f32,
    pub requires_recipe: bool,
    pub amount: u32,
    pub fabrication_limit: Option<u32>,
    pub fabrication_limit_min: Option<u32>,
    pub fabrication_limit_max: Option<u32>,
    pub hide_for_non_traitors: bool,
    pub quality: Option<u32>,
    pub required_skills: Vec<SkillRequirement>,
    pub required_items: Vec<RequiredItem>,
}

impl FabricationRecipe {
    pub fn new(element: Node, is_variant_of: bool) -> Self {
        let display_name = element
            .attribute_ignore_ascii_case("displayname")
            .map(std::borrow::ToOwned::to_owned);
        let suitable_fabricators = element
            .attribute_ignore_ascii_case("suitablefabricators")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            });
        let required_time = element
            .attribute_ignore_ascii_case("requiredtime")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let required_money = element
            .attribute_ignore_ascii_case("requiredmoney")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let out_condition = element
            .attribute_ignore_ascii_case("outcondition")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let requires_recipe = element
            .attribute_ignore_ascii_case("requiresrecipe")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let amount = element
            .attribute_ignore_ascii_case("amount")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let fabrication_limit = element
            .attribute_ignore_ascii_case("fabricationlimit")
            .map(|v| v.parse::<u32>().unwrap());
        let fabrication_limit_min = element
            .attribute_ignore_ascii_case("fabricationlimitmin")
            .map(|v| v.parse::<u32>().unwrap());
        let fabrication_limit_max = element
            .attribute_ignore_ascii_case("fabricationlimitmax")
            .map(|v| v.parse::<u32>().unwrap());
        let hide_for_non_traitors = element
            .attribute_ignore_ascii_case("hidefornontraitors")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let quality = element
            .attribute_ignore_ascii_case("quality")
            .map(|v| v.parse::<u32>().unwrap());
        let mut required_skills = Vec::new();
        let mut required_items = Vec::new();
        for child in element.children() {
            if child.is_element() {
                match child.tag_name().name().to_lowercase().as_str() {
                    "requiredskill" => required_skills.push(SkillRequirement {
                        identifier: child
                            .attribute_ignore_ascii_case("identifier")
                            .unwrap()
                            .to_owned(),
                        level: child
                            .attribute_ignore_ascii_case("level")
                            .unwrap()
                            .parse()
                            .unwrap(),
                    }),
                    "item" | "requireditem" => {
                        let identifier = child.attribute_ignore_ascii_case("identifier");
                        let tag = child.attribute_ignore_ascii_case("tag");
                        let min_condition = child
                            .attribute_ignore_ascii_case("mincondition")
                            .map(|v| v.parse::<f32>().unwrap());
                        let max_condition = child
                            .attribute_ignore_ascii_case("maxcondition")
                            .map(|v| v.parse::<f32>().unwrap());
                        let use_condition = child
                            .attribute_ignore_ascii_case("usecondition")
                            .map_or(true, |v| v.to_lowercase().parse::<bool>().unwrap());
                        let amount = child
                            .attribute_ignore_ascii_case("count")
                            .or(child.attribute_ignore_ascii_case("amount"))
                            .map_or(1, |v| v.trim().parse::<u32>().unwrap());
                        let description = child
                            .attribute_ignore_ascii_case("description")
                            .map(std::borrow::ToOwned::to_owned);
                        let header = child
                            .attribute_ignore_ascii_case("header")
                            .map(std::borrow::ToOwned::to_owned);
                        if let Some(identifier) = identifier {
                            required_items.push(RequiredItem::ByIdentifier {
                                identifier: identifier.to_owned(),
                                min_condition,
                                max_condition,
                                use_condition,
                                amount,
                                description,
                                header,
                            });
                        } else if let Some(tag) = tag {
                            required_items.push(RequiredItem::ByTag {
                                tag: tag.to_owned(),
                                min_condition,
                                max_condition,
                                use_condition,
                                amount,
                                description,
                                header,
                            });
                        } else if !is_variant_of {
                            panic!()
                        }
                    }
                    _ => (),
                }
            }
        }

        required_skills.shrink_to_fit();
        required_items.shrink_to_fit();

        Self {
            display_name,
            suitable_fabricators,
            required_time,
            required_money,
            out_condition,
            requires_recipe,
            amount,
            fabrication_limit,
            fabrication_limit_min,
            fabrication_limit_max,
            hide_for_non_traitors,
            quality,
            required_skills,
            required_items,
        }
    }
}

#[derive(Debug)]
pub enum RequiredItem {
    ByIdentifier {
        identifier: String,
        min_condition: Option<f32>,
        max_condition: Option<f32>,
        use_condition: bool,
        amount: u32,
        description: Option<String>,
        header: Option<String>,
    },
    ByTag {
        tag: String,
        min_condition: Option<f32>,
        max_condition: Option<f32>,
        use_condition: bool,
        amount: u32,
        description: Option<String>,
        header: Option<String>,
    },
}

#[derive(Debug)]
pub struct SkillRequirement {
    pub identifier: String,
    pub level: u32,
}

#[derive(Debug)]
pub struct DeconstructItem {
    pub item_identifier: String,
    pub amount: u32,
    pub min_condition: Option<f32>,
    pub max_condition: Option<f32>,
    pub out_condition_min: Option<f32>,
    pub out_condition_max: Option<f32>,
    pub copy_condition: bool,
    pub commonness: f32,
    pub required_deconstructor: Option<Vec<String>>,
    pub required_other_item: Option<Vec<String>>,
    pub activate_button_text: Option<String>,
    pub info_text: Option<String>,
    pub info_text_on_other_item_missing: Option<String>,
}

impl DeconstructItem {
    pub fn new(element: Node) -> Self {
        let item_identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let amount = element
            .attribute_ignore_ascii_case("amount")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let min_condition = element
            .attribute_ignore_ascii_case("mincondition")
            .map(|v| v.parse::<f32>().unwrap());
        let max_condition = element
            .attribute_ignore_ascii_case("maxcondition")
            .map(|v| v.parse::<f32>().unwrap());
        let out_condition_min = element
            .attribute_ignore_ascii_case("outconditionmin")
            .or(element.attribute_ignore_ascii_case("outcondition"))
            .map(|v| v.parse::<f32>().unwrap());
        let out_condition_max = element
            .attribute_ignore_ascii_case("outconditionmax")
            .or(element.attribute_ignore_ascii_case("outcondition"))
            .map(|v| v.parse::<f32>().unwrap());
        let copy_condition = element
            .attribute_ignore_ascii_case("copycondition")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let commonness = element
            .attribute_ignore_ascii_case("commonness")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let required_deconstructor = element
            .attribute_ignore_ascii_case("requireddeconstructor")
            .or(element
                .parent_element()
                .and_then(|parent| parent.attribute_ignore_ascii_case("requireddeconstructor")))
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            });
        let required_other_item = element
            .attribute_ignore_ascii_case("requiredotheritem")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            });
        let activate_button_text = element
            .attribute_ignore_ascii_case("activatebuttontext")
            .map(std::borrow::ToOwned::to_owned);
        let info_text = element
            .attribute_ignore_ascii_case("infotext")
            .map(std::borrow::ToOwned::to_owned);
        let info_text_on_other_item_missing = element
            .attribute_ignore_ascii_case("infotextonotheritemmissing")
            .map(std::borrow::ToOwned::to_owned);
        Self {
            item_identifier,
            amount,
            min_condition,
            max_condition,
            out_condition_min,
            out_condition_max,
            copy_condition,
            commonness,
            required_deconstructor,
            required_other_item,
            activate_button_text,
            info_text,
            info_text_on_other_item_missing,
        }
    }
}

#[derive(Debug)]
pub struct PriceInfos {
    pub default_price: DefaultPriceInfo,
    pub other_prices: Vec<PriceInfo>,
}

impl PriceInfos {
    pub fn new(element: Node) -> Self {
        let base_price = element
            .attribute_ignore_ascii_case("baseprice")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let min_amount = element
            .attribute_ignore_ascii_case("minamount")
            .or(element.attribute_ignore_ascii_case("minavailable"))
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let max_amount = element
            .attribute_ignore_ascii_case("maxamount")
            .or(element.attribute_ignore_ascii_case("maxavailable"))
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let min_level_difficulty = element
            .attribute_ignore_ascii_case("minleveldifficulty")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let can_be_special = element
            .attribute_ignore_ascii_case("canbespecial")
            .map_or(true, |v| v.parse::<bool>().unwrap());
        let buying_price_multiplier = element
            .attribute_ignore_ascii_case("buyingpricemultiplier")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let display_non_empty = element
            .attribute_ignore_ascii_case("displaynonempty")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let sold_by_default = element
            .attribute_ignore_ascii_case("sold")
            .or(element.attribute_ignore_ascii_case("soldbydefault"))
            .map_or(true, |v| v.parse::<bool>().unwrap());
        let requires_unlock = element
            .attribute_ignore_ascii_case("requiresunlock")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let min_reputations = element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("reputation"))
            .map(|child| {
                let faction_id = child
                    .attribute_ignore_ascii_case("faction")
                    .unwrap()
                    .to_owned();
                let rep = child
                    .attribute_ignore_ascii_case("min")
                    .map_or(0.0, |v| v.parse::<f32>().unwrap());
                assert!(rep > 0.0);
                (faction_id, rep)
            })
            .collect::<HashMap<_, _>>();
        let price_infos = element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("price"))
            .filter(|child| {
                child
                    .attribute_ignore_ascii_case("storeidentifier")
                    .is_some()
            }) //thanks to the dev, value without identifier doesn't make sense
            .map(|child| {
                let price_multiplier = child
                    .attribute_ignore_ascii_case("multiplier")
                    .map_or(1.0, |v| v.trim().parse::<f32>().unwrap());
                let sold = child
                    .attribute_ignore_ascii_case("sold")
                    .map_or(sold_by_default, |v| v.parse::<bool>().unwrap());
                let store_min_level_difficulty = child
                    .attribute_ignore_ascii_case("minleveldifficulty")
                    .map_or(min_level_difficulty, |v| v.parse::<u32>().unwrap());
                let store_buying_multiplier = child
                    .attribute_ignore_ascii_case("byingpricemultiplier")
                    .map_or(buying_price_multiplier, |v| v.parse::<f32>().unwrap());
                let store_identifier = child
                    .attribute_ignore_ascii_case("storeidentifier")
                    .unwrap()
                    .to_owned();
                let min_reputations = child
                    .children()
                    .filter(Node::is_element)
                    .filter(|child| child.tag_name().name().eq_ignore_ascii_case("reputation"))
                    .filter_map(|child| {
                        let faction_id = child
                            .attribute_ignore_ascii_case("faction")
                            .unwrap()
                            .to_owned();
                        let rep = child
                            .attribute_ignore_ascii_case("min")
                            .map_or(0.0, |v| v.parse::<f32>().unwrap());
                        if faction_id.is_empty() || rep <= 0.0 {
                            warn!("PriceInfo requires faction identifier to be non-empty and reputation be non-zero to work properly! (ignored by vanilla client)");
                            None
                        } else {
                            Some((faction_id, rep))
                        }
                    })
                    .collect::<HashMap<_, _>>();
                let min_amount = child
                    .attribute_ignore_ascii_case("minamount")
                    .or(element.attribute_ignore_ascii_case("minavailable"))
                    .map_or(0, |v| v.parse::<u32>().unwrap());
                let max_amount = child
                    .attribute_ignore_ascii_case("maxamount")
                    .or(element.attribute_ignore_ascii_case("maxavailable"))
                    .map_or(0, |v| v.parse::<u32>().unwrap());
                PriceInfo {
                    price: (price_multiplier * base_price as f32).round() as u32,
                    can_be_bought: sold,
                    buying_price_multiplier: store_buying_multiplier,
                    min_available_amount: min_amount,
                    max_available_amount: max_amount,
                    min_level_difficulty: store_min_level_difficulty,
                    can_be_special,
                    display_non_empty,
                    store_identifier,
                    requires_unlock,
                    min_reputations,
                }
            })
            .collect::<Vec<_>>();
        let sold_elsewhere = sold_by_default
            && element
                .attribute_ignore_ascii_case("soldelsewhere")
                .or(element.attribute_ignore_ascii_case("soldeverywhere"))
                .is_some_and(|v| v.parse().unwrap());
        Self {
            default_price: DefaultPriceInfo {
                price: base_price,
                can_be_bought: sold_elsewhere,
                buying_price_multiplier,
                min_available_amount: if sold_elsewhere { min_amount } else { 0 },
                max_available_amount: if sold_elsewhere { max_amount } else { 0 },
                min_level_difficulty,
                can_be_special,
                display_non_empty,
                requires_unlock,
                min_reputations,
            },
            other_prices: price_infos,
        }
    }
}

#[derive(Debug)]
pub struct PriceInfo {
    pub price: u32,
    pub can_be_bought: bool,
    pub buying_price_multiplier: f32,
    pub min_available_amount: u32,
    pub max_available_amount: u32,
    pub min_level_difficulty: u32,
    pub can_be_special: bool,
    pub display_non_empty: bool,
    pub store_identifier: String,
    pub requires_unlock: bool,
    pub min_reputations: HashMap<String, f32>,
}

#[derive(Debug)]
pub struct DefaultPriceInfo {
    pub price: u32,
    pub can_be_bought: bool,
    pub buying_price_multiplier: f32,
    pub min_available_amount: u32,
    pub max_available_amount: u32,
    pub min_level_difficulty: u32,
    pub can_be_special: bool,
    pub display_non_empty: bool,
    pub requires_unlock: bool,
    pub min_reputations: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub struct BarotraumaSprite {
    pub texture_path: String,
    pub name: Option<String>,
    pub source_rect: Option<Rect>,
    pub sheet_element_size: Option<Vec2>,
    pub sheet_index: Option<Vec2>,
    pub compress: bool,
    pub size: Vec2,
    pub relative_origin: Vec2,
    pub depth: f32,
}

impl BarotraumaSprite {
    pub fn new(element: Node) -> Self {
        let texture_path = element
            .attribute_ignore_ascii_case("texture")
            .map(|v| v.to_owned());
        if texture_path.is_none() {
            dbg!(element);
        }
        let texture_path = texture_path.unwrap();
        let name = element
            .attribute_ignore_ascii_case("name")
            .filter(|v| !v.is_empty())
            .map(|v| v.to_owned());
        let source_rect = element.attribute_ignore_ascii_case("sourcerect").map(|v| {
            let value = v.trim().parse::<Vector4>().unwrap().0;
            Rect {
                x: value.x as i32,
                y: value.y as i32,
                width: value.z as u32,
                height: value.w as u32,
            }
        });

        let sheet_element_size = element
            .attribute_ignore_ascii_case("sheetelementsize")
            .map(|v| v.parse::<Vector2>().unwrap().0);
        let sheet_index = element
            .attribute_ignore_ascii_case("sheetindex")
            .map(|v| v.parse::<Vector2>().unwrap().0);

        let compress = element
            .attribute_ignore_ascii_case("compress")
            .map_or(true, |v| v.parse::<bool>().unwrap());

        let size = element
            .attribute_ignore_ascii_case("size")
            .map_or(Vec2::ONE, |v| v.parse::<Vector2>().unwrap().0);
        let relative_origin = element
            .attribute_ignore_ascii_case("origin")
            .map_or(Vec2::splat(0.5), |v| v.parse::<Vector2>().unwrap().0);
        let depth = element
            .attribute_ignore_ascii_case("depth")
            .map_or(0.001, |v| v.parse::<f32>().unwrap());
        Self {
            texture_path,
            name,
            source_rect,
            sheet_element_size,
            sheet_index,
            compress,
            size,
            relative_origin,
            depth,
        }
    }
}

pub struct Vector4(pub Vec4);

impl FromStr for Vector4 {
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
        let z = split
            .next()
            .ok_or(ParseVectorError::NotEnoughComponents)?
            .parse::<f32>()?;
        let w = split
            .next()
            .ok_or(ParseVectorError::NotEnoughComponents)?
            .parse::<f32>()?;
        Ok(Self(Vec4::new(x, y, z, w)))
    }
}

#[derive(Debug)]
pub struct ItemSprite {
    pub can_flip_x: bool,
    pub can_flip_y: bool,
    pub sprite: BarotraumaSprite,
}

impl ItemSprite {
    pub fn new(element: Node) -> Result<Self, std::str::ParseBoolError> {
        Ok(Self {
            can_flip_x: element
                .attribute_ignore_ascii_case("canflipx")
                .map_or(Ok(true), str::parse)?,
            can_flip_y: element
                .attribute_ignore_ascii_case("canflipy")
                .map_or(Ok(true), str::parse)?,
            sprite: BarotraumaSprite::new(element),
        })
    }
}

#[derive(Debug)]
pub struct ItemStaticBody {
    pub radius: f32,
    pub width: f32,
    pub height: f32,
}

#[bitfield(u32)]
pub struct CollisionCategory {
    pub cat1: bool,
    pub cat2: bool,
    pub cat3: bool,
    pub cat4: bool,
    pub cat5: bool,
    pub cat6: bool,
    pub cat7: bool,
    pub cat8: bool,
    pub cat9: bool,
    pub cat10: bool,
    pub cat11: bool,
    pub cat12: bool,
    pub cat13: bool,
    pub cat14: bool,
    pub cat15: bool,
    pub cat16: bool,
    pub cat17: bool,
    pub cat18: bool,
    pub cat19: bool,
    pub cat20: bool,
    pub cat21: bool,
    pub cat22: bool,
    pub cat23: bool,
    pub cat24: bool,
    pub cat25: bool,
    pub cat26: bool,
    pub cat27: bool,
    pub cat28: bool,
    pub cat29: bool,
    pub cat30: bool,
    pub cat31: bool,
    _unused: bool,
}

impl FromStr for CollisionCategory {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "none" => Ok(Self::new()),
            "cat1" => Ok(Self::new().with_cat1(true)),
            "cat2" => Ok(Self::new().with_cat2(true)),
            "cat3" => Ok(Self::new().with_cat3(true)),
            "cat4" => Ok(Self::new().with_cat4(true)),
            "cat5" => Ok(Self::new().with_cat5(true)),
            "cat6" => Ok(Self::new().with_cat6(true)),
            "cat7" => Ok(Self::new().with_cat7(true)),
            "cat8" => Ok(Self::new().with_cat8(true)),
            "cat9" => Ok(Self::new().with_cat9(true)),
            "cat10" => Ok(Self::new().with_cat10(true)),
            "cat11" => Ok(Self::new().with_cat11(true)),
            "cat12" => Ok(Self::new().with_cat12(true)),
            "cat13" => Ok(Self::new().with_cat13(true)),
            "cat14" => Ok(Self::new().with_cat14(true)),
            "cat15" => Ok(Self::new().with_cat15(true)),
            "cat16" => Ok(Self::new().with_cat16(true)),
            "cat17" => Ok(Self::new().with_cat17(true)),
            "cat18" => Ok(Self::new().with_cat18(true)),
            "cat19" => Ok(Self::new().with_cat19(true)),
            "cat20" => Ok(Self::new().with_cat20(true)),
            "cat21" => Ok(Self::new().with_cat21(true)),
            "cat22" => Ok(Self::new().with_cat22(true)),
            "cat23" => Ok(Self::new().with_cat23(true)),
            "cat24" => Ok(Self::new().with_cat24(true)),
            "cat25" => Ok(Self::new().with_cat25(true)),
            "cat26" => Ok(Self::new().with_cat26(true)),
            "cat27" => Ok(Self::new().with_cat27(true)),
            "cat28" => Ok(Self::new().with_cat28(true)),
            "cat29" => Ok(Self::new().with_cat29(true)),
            "cat30" => Ok(Self::new().with_cat30(true)),
            "cat31" => Ok(Self::new().with_cat31(true)),
            "all" => Ok(Self::from_bits(u32::MAX)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct ItemBody {
    pub physics_body: PhysicsBody,
    pub density: f32,
    pub min_density: Option<f32>,
    pub max_density: Option<f32>,
    pub collision_category: Option<CollisionCategory>,
    pub angular_damping: f32,
    pub linear_damping: f32,
}
