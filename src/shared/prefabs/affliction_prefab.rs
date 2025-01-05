use std::{collections::HashMap, str::FromStr};

use bitfield_struct::bitfield;
use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::{
    human_prefab::StatType,
    item_prefab::{BarotraumaSprite, Color, DoesNotExistError},
    level_object_prefab::{LimbType, StatusEffect},
};

#[derive(Debug)]
pub struct AfflictionPrefab {
    pub prefab_type: AfflictionPrefabType,
    pub identifier: String,
    pub affliction_type: Option<String>,
    pub translation_identifier: Option<String>,
    pub fallback_name: Option<String>,
    pub fallback_description: Option<String>,
    pub is_buff: bool,
    pub affect_machines: bool,
    pub show_bar_in_health_menu: bool,
    pub healable_in_medical_clinic: bool,
    pub heal_cost_multiplier: f32,
    pub base_heal_cost: u32,
    pub ignore_treatment_if_afflicted_by: Option<Vec<String>>,
    pub duration: f32,
    pub name_identifier: Option<String>,
    pub limb_specific: bool,
    pub indicator_limb: Option<LimbType>,
    pub hide_icon_after_delay: bool,
    pub activation_threshold: f32,
    pub show_icon_threshold: f32,
    pub show_icon_to_others_threshold: f32,
    pub max_strength: f32,
    pub grain_burst: f32,
    pub show_in_health_scanner_threshold: Option<f32>,
    pub treatment_threshold: f32,
    pub damage_overlay_alpha: f32,
    pub burn_overlay_alpha: f32,
    pub karma_change_on_applied: f32,
    pub fallback_cause_of_death_description: Option<String>,
    pub fallback_self_cause_of_death_description: Option<String>,
    pub icon_colors: Option<Vec<Color>>,
    pub affliction_overlay_alpha_is_linear: bool,
    pub achievement_on_received: Option<String>,
    pub achievement_on_removed: Option<String>,
    pub target_species: Option<Vec<String>>,
    pub reset_between_rounds: bool,
    pub damage_particles: bool,
    pub weapons_skill_gain: f32,
    pub medical_skill_gain: f32,
    pub descriptions: Vec<Description>,
    pub icon: Option<BarotraumaSprite>,
    pub affliction_overlay: Option<BarotraumaSprite>,
    pub effects: Vec<Effect>,
    pub periodic_effects: Vec<PeriodicEffect>,
}

impl AfflictionPrefab {
    pub fn new(element: Node, prefab_type: AfflictionPrefabType) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let affliction_type = element
            .attribute_ignore_ascii_case("type")
            .map(|v| v.to_owned());
        let translation_identifier = element
            .attribute_ignore_ascii_case("translationoverride")
            .map(|v| v.to_owned());
        let fallback_name = element
            .attribute_ignore_ascii_case("name")
            .map(|v| v.to_owned());
        let fallback_description = element
            .attribute_ignore_ascii_case("description")
            .map(|v| v.to_owned());
        let is_buff = element
            .attribute_ignore_ascii_case("isbuff")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
        let affect_machines = element
            .attribute_ignore_ascii_case("affectmachines")
            .map_or(true, |v| v.parse::<bool>().unwrap());
        let show_bar_in_health_menu = element
            .attribute_ignore_ascii_case("showbarinhealthmenu")
            .map_or(true, |v| v.parse::<bool>().unwrap());
        let healable_in_medical_clinic = element
            .attribute_ignore_ascii_case("healableinmedicalclinic")
            .map_or(
                !is_buff
                    && !affliction_type
                        .as_ref()
                        .is_some_and(|v| v == "geneticmaterialbuff")
                    && !affliction_type
                        .as_ref()
                        .is_some_and(|v| v == "geneticmaterialdebuff"),
                |v| v.parse::<bool>().unwrap(),
            );
        let heal_cost_multiplier = element
            .attribute_ignore_ascii_case("healcostmultiplier")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let base_heal_cost = element
            .attribute_ignore_ascii_case("basehealcost")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let ignore_treatment_if_afflicted_by = element
            .attribute_ignore_ascii_case("ignoretreatmentifafflictedby")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let duration = element
            .attribute_ignore_ascii_case("duration")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let name_identifier = element
            .attribute_ignore_ascii_case("nameidentifier")
            .map(|v| v.to_owned());
        let limb_specific = element
            .attribute_ignore_ascii_case("limbspecific")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
        let indicator_limb = if !limb_specific {
            element
                .attribute_ignore_ascii_case("indicatorlimb")
                .map(|v| v.parse::<LimbType>().unwrap())
        } else {
            None
        };
        let hide_icon_after_delay = element
            .attribute_ignore_ascii_case("hideiconafterdelay")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let activation_threshold = element
            .attribute_ignore_ascii_case("activationthreshold")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let show_icon_threshold = element
            .attribute_ignore_ascii_case("showiconthreshold")
            .map_or(activation_threshold.max(0.05), |v| {
                v.parse::<f32>().unwrap()
            });
        let show_icon_to_others_threshold = element
            .attribute_ignore_ascii_case("showicontoothersthreshold")
            .map_or(show_icon_threshold, |v| v.parse::<f32>().unwrap());
        let max_strength = element
            .attribute_ignore_ascii_case("maxstrength")
            .map_or(100.0, |v| v.parse::<f32>().unwrap());
        let grain_burst = element
            .attribute_ignore_ascii_case("grainburst")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let show_in_health_scanner_threshold = element
            .attribute_ignore_ascii_case("showinhealthscannerthreshold")
            .map(|v| v.parse::<f32>().unwrap())
            .or_else(|| Some(activation_threshold.max(show_icon_threshold)))
            .filter(|_| !affliction_type.as_ref().is_some_and(|v| v == "talentbuff"));
        let treatment_threshold = element
            .attribute_ignore_ascii_case("treatmentthreshold")
            .map_or(activation_threshold.max(10.0), |v| {
                v.parse::<f32>().unwrap()
            });
        let damage_overlay_alpha = element
            .attribute_ignore_ascii_case("damageoverlayalpha")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let burn_overlay_alpha = element
            .attribute_ignore_ascii_case("burnoverlayalpha")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let karma_change_on_applied = element
            .attribute_ignore_ascii_case("karmachangeonapplied")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let fallback_cause_of_death_description = element
            .attribute_ignore_ascii_case("causeofdeathdescription")
            .map(|v| v.to_owned());
        let fallback_self_cause_of_death_description = element
            .attribute_ignore_ascii_case("selfcauseofdeathdescription")
            .map(|v| v.to_owned());
        let icon_colors = element.attribute_ignore_ascii_case("iconcolors").map(|v| {
            v.split(';')
                .map(|v| v.parse::<Color>().unwrap())
                .collect::<Vec<_>>()
        });
        let affliction_overlay_alpha_is_linear = element
            .attribute_ignore_ascii_case("afflictionoverlayalphaislinear")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let achievement_on_received = element
            .attribute_ignore_ascii_case("achievementonreceived")
            .map(|v| v.to_owned());
        let achievement_on_removed = element
            .attribute_ignore_ascii_case("achievementonremoved")
            .map(|v| v.to_owned());
        let target_species = element
            .attribute_ignore_ascii_case("targets")
            .map(|v| v.split(';').map(|v| v.to_owned()).collect::<Vec<_>>());
        let reset_between_rounds = element
            .attribute_ignore_ascii_case("resetbetweenrounds")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let damage_particles = element
            .attribute_ignore_ascii_case("damageparticles")
            .map_or(true, |v| v.parse::<bool>().unwrap());
        let weapons_skill_gain = element
            .attribute_ignore_ascii_case("weaponsskillgain")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let medical_skill_gain = element
            .attribute_ignore_ascii_case("medicalskillgain")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());

        let mut descriptions = Vec::new();
        let mut icon = None;
        let mut affliction_overlay = None;
        let mut effects = Vec::new();
        let mut periodic_effects = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "icon" => {
                    icon = Some(BarotraumaSprite::new(child));
                }
                "afflictionoverlay" => {
                    affliction_overlay = Some(BarotraumaSprite::new(child));
                }
                "effect" => {
                    effects.push(Effect::new(child));
                }
                "periodiceffect" => {
                    periodic_effects.push(PeriodicEffect::new(child));
                }
                "description" => {
                    descriptions.push(Description::new(child));
                }
                "statvalue" => {
                    println!(
                        "Error in affliction \"{}\" - stat values should be configured inside the affliction's effects.",
                        identifier
                    );
                }
                _ => {
                    println!(
                        "Unrecognized element in affliction \"{}\" ({})",
                        identifier,
                        child.tag_name().name()
                    )
                }
            }
        }

        Self {
            prefab_type,
            identifier,
            affliction_type,
            translation_identifier,
            fallback_name,
            fallback_description,
            is_buff,
            affect_machines,
            show_bar_in_health_menu,
            healable_in_medical_clinic,
            heal_cost_multiplier,
            base_heal_cost,
            ignore_treatment_if_afflicted_by,
            duration,
            name_identifier,
            limb_specific,
            indicator_limb,
            hide_icon_after_delay,
            activation_threshold,
            show_icon_threshold,
            show_icon_to_others_threshold,
            max_strength,
            grain_burst,
            show_in_health_scanner_threshold,
            treatment_threshold,
            damage_overlay_alpha,
            burn_overlay_alpha,
            karma_change_on_applied,
            fallback_cause_of_death_description,
            fallback_self_cause_of_death_description,
            icon_colors,
            affliction_overlay_alpha_is_linear,
            achievement_on_received,
            achievement_on_removed,
            target_species,
            reset_between_rounds,
            damage_particles,
            weapons_skill_gain,
            medical_skill_gain,
            descriptions,
            icon,
            affliction_overlay,
            effects,
            periodic_effects,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub enum AfflictionPrefabType {
    Psychosis,
    Bleeding,
    Husk,
    SpaceHerpes,
    BuffDurationIncrease,
    Normal,
}

#[derive(Debug)]
pub struct Description {
    pub text_identifier: Option<String>,
    pub text_fallback: Option<String>,
    pub min_strength: f32,
    pub max_strength: f32,
    pub target: TargetType,
}

impl Description {
    pub fn new(element: Node) -> Self {
        let text_identifier = element
            .attribute_ignore_ascii_case("textidentifier")
            .map(|v| v.to_owned());
        let text_fallback = element
            .attribute_ignore_ascii_case("text")
            .map(|v| v.to_owned());
        let min_strength = element
            .attribute_ignore_ascii_case("minstrength")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let max_strength = element
            .attribute_ignore_ascii_case("maxstrength")
            .map_or(100.0, |v| v.parse::<f32>().unwrap());
        let target = element
            .attribute_ignore_ascii_case("target")
            .map_or(TargetType::Any, |v| v.parse::<TargetType>().unwrap());

        Self {
            text_identifier,
            text_fallback,
            min_strength,
            max_strength,
            target,
        }
    }
}

#[derive(Debug)]
pub enum TargetType {
    Any,
    AffectedCharacter,
    OtherCharacter,
}

impl FromStr for TargetType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //dbg!(s);
        match s.trim() {
            "Any" => Ok(Self::Any),
            "Self" => Ok(Self::AffectedCharacter),
            "OtherCharacter" => Ok(Self::OtherCharacter),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct Effect {
    pub properties: EffectProperties,
    pub resistance_for: Option<Vec<String>>,
    pub block_transformation: Option<Vec<String>>,
    pub affliction_ability_flags: AbilityFlags,
    pub affliction_stat_values: HashMap<StatType, AppliedStatValue>,
    pub status_effects: Vec<StatusEffect>,
}

impl Effect {
    pub fn new(element: Node) -> Self {
        let properties = EffectProperties::new(element);
        let resistance_for = element
            .attribute_ignore_ascii_case("resistancefor")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let block_transformation = element
            .attribute_ignore_ascii_case("blocktransformation")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());

        let mut affliction_ability_flags = AbilityFlags::new();
        let mut affliction_stat_values = HashMap::new();
        let mut status_effects = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "statuseffect" => {
                    status_effects.push(StatusEffect::new(child));
                }
                "statvalue" => {
                    let new_stat_value = AppliedStatValue::new(child);
                    affliction_stat_values.insert(new_stat_value.stat_type.clone(), new_stat_value);
                }
                "abilityflag" => {
                    let flag_type = child
                        .attribute_ignore_ascii_case("flagtype")
                        .map(|v| v.parse::<AbilityFlags>().unwrap());
                    affliction_ability_flags = AbilityFlags::from_bits(
                        affliction_ability_flags.into_bits() | flag_type.unwrap().into_bits(),
                    );
                }
                _ => (),
            }
        }

        Self {
            properties,
            resistance_for,
            block_transformation,
            affliction_ability_flags,
            affliction_stat_values,
            status_effects,
        }
    }
}

#[derive(Debug)]
pub struct EffectProperties {
    pub min_strength: f32,
    pub max_strength: f32,
    pub min_vitality_decrease: f32,
    pub max_vitality_decrease: f32,
    pub strength_change: f32,
    pub multiply_by_max_vitality: bool,
    pub min_screen_blur: f32,
    pub max_screen_blur: f32,
    pub min_screen_distort: f32,
    pub max_screen_distort: f32,
    pub min_radial_distort: f32,
    pub max_radial_distort: f32,
    pub min_chromatic_aberration: f32,
    pub max_chromatic_aberration: f32,
    pub grain_color: Color,
    pub min_grain_strength: f32,
    pub max_grain_strength: f32,
    pub screen_effect_fluctuation_frequency: f32,
    pub min_affliction_overlay_alpha_multiplier: f32,
    pub max_affliction_overlay_alpha_multiplier: f32,
    pub min_buff_multiplier: f32,
    pub max_buff_multiplier: f32,
    pub min_speed_multiplier: f32,
    pub max_speed_multiplier: f32,
    pub min_skill_multiplier: f32,
    pub max_skill_multiplier: f32,
    pub min_resistance: f32,
    pub max_resistance: f32,
    pub dialogue_flag: Option<String>,
    pub tag: Option<String>,
    pub min_face_tint: Color,
    pub max_face_tint: Color,
    pub min_body_tint: Color,
    pub max_body_tint: Color,
}

impl EffectProperties {
    pub fn new(element: Node) -> Self {
        Self {
            min_strength: element
                .attribute_ignore_ascii_case("minstrength")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_strength: element
                .attribute_ignore_ascii_case("maxstrength")
                .map_or(0.0, |v| v.parse().unwrap()),
            min_vitality_decrease: element
                .attribute_ignore_ascii_case("minvitalitydecrease")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_vitality_decrease: element
                .attribute_ignore_ascii_case("maxvitalitydecrease")
                .map_or(0.0, |v| v.parse().unwrap()),
            strength_change: element
                .attribute_ignore_ascii_case("strengthchange")
                .map_or(0.0, |v| v.parse().unwrap()),
            multiply_by_max_vitality: element
                .attribute_ignore_ascii_case("multiplybymaxvitality")
                .map_or(false, |v| v.parse().unwrap()),
            min_screen_blur: element
                .attribute_ignore_ascii_case("minscreenblur")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_screen_blur: element
                .attribute_ignore_ascii_case("maxscreenblur")
                .map_or(0.0, |v| v.parse().unwrap()),
            min_screen_distort: element
                .attribute_ignore_ascii_case("minscreendistort")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_screen_distort: element
                .attribute_ignore_ascii_case("maxscreendistort")
                .map_or(0.0, |v| v.parse().unwrap()),
            min_radial_distort: element
                .attribute_ignore_ascii_case("minradialdistort")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_radial_distort: element
                .attribute_ignore_ascii_case("maxradialdistort")
                .map_or(0.0, |v| v.parse().unwrap()),
            min_chromatic_aberration: element
                .attribute_ignore_ascii_case("minchromaticaberration")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_chromatic_aberration: element
                .attribute_ignore_ascii_case("maxchromaticaberration")
                .map_or(0.0, |v| v.parse().unwrap()),
            grain_color: element.attribute_ignore_ascii_case("graincolor").map_or(
                Color::Simple {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                |v| v.parse().unwrap(),
            ),
            min_grain_strength: element
                .attribute_ignore_ascii_case("mingrainstrength")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_grain_strength: element
                .attribute_ignore_ascii_case("maxgrainstrength")
                .map_or(0.0, |v| v.parse().unwrap()),
            screen_effect_fluctuation_frequency: element
                .attribute_ignore_ascii_case("screeneffectfluctuationfrequency")
                .map_or(0.0, |v| v.parse().unwrap()),
            min_affliction_overlay_alpha_multiplier: element
                .attribute_ignore_ascii_case("minafflictionoverlayalphamultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            max_affliction_overlay_alpha_multiplier: element
                .attribute_ignore_ascii_case("maxafflictionoverlayalphamultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            min_buff_multiplier: element
                .attribute_ignore_ascii_case("minbuffmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            max_buff_multiplier: element
                .attribute_ignore_ascii_case("maxbuffmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            min_speed_multiplier: element
                .attribute_ignore_ascii_case("minspeedmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            max_speed_multiplier: element
                .attribute_ignore_ascii_case("maxspeedmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            min_skill_multiplier: element
                .attribute_ignore_ascii_case("minskillmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            max_skill_multiplier: element
                .attribute_ignore_ascii_case("maxskillmultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            min_resistance: element
                .attribute_ignore_ascii_case("minresistance")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_resistance: element
                .attribute_ignore_ascii_case("maxresistance")
                .map_or(0.0, |v| v.parse().unwrap()),
            dialogue_flag: element
                .attribute_ignore_ascii_case("dialogflag")
                .map(|v| v.parse().unwrap()),
            tag: element
                .attribute_ignore_ascii_case("tag")
                .map(|v| v.parse().unwrap()),
            min_face_tint: element.attribute_ignore_ascii_case("minfacetint").map_or(
                Color::Simple {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
                |v| v.parse().unwrap(),
            ),
            max_face_tint: element.attribute_ignore_ascii_case("maxfacetint").map_or(
                Color::Simple {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
                |v| v.parse().unwrap(),
            ),
            min_body_tint: element.attribute_ignore_ascii_case("minbodytint").map_or(
                Color::Simple {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
                |v| v.parse().unwrap(),
            ),
            max_body_tint: element.attribute_ignore_ascii_case("maxbodytint").map_or(
                Color::Simple {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
                |v| v.parse().unwrap(),
            ),
        }
    }
}

#[derive(Debug)]
pub struct AppliedStatValue {
    pub value: Option<f32>,
    pub stat_type: StatType,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
}

impl AppliedStatValue {
    pub fn new(element: Node) -> Self {
        let value = element
            .attribute_ignore_ascii_case("value")
            .map(|v| v.parse::<f32>().unwrap());
        let stat_type = element
            .attribute_ignore_ascii_case("stattype")
            .map_or(StatType::None, |v| v.parse::<StatType>().unwrap());
        let min_value = element
            .attribute_ignore_ascii_case("value")
            .map(|v| v.parse::<f32>().unwrap());
        let max_value = element
            .attribute_ignore_ascii_case("value")
            .map(|v| v.parse::<f32>().unwrap());

        Self {
            value,
            stat_type,
            min_value,
            max_value,
        }
    }
}

#[bitfield(u16)]

pub struct AbilityFlags {
    pub must_walk: bool,
    pub immune_to_pressure: bool,
    pub ignored_by_enemy_ai: bool,
    pub move_normally_while_dragging: bool,
    pub can_tinker: bool,
    pub can_tinker_fabricators_and_deconstructors: bool,
    pub tinkering_powers_devices: bool,
    pub gain_skill_past_maximum: bool,
    pub retain_experience_for_new_character: bool,
    pub allow_second_ordered_target: bool,
    pub always_stay_conscious: bool,
    pub can_not_die_to_afflictions: bool,
    #[bits(4)]
    _unused: u8,
}

impl FromStr for AbilityFlags {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "none" => Ok(Self::new()),
            "mustwalk" => Ok(Self::new().with_must_walk(true)),
            "immunetopressure" => Ok(Self::new().with_immune_to_pressure(true)),
            "ignoredbyenemyai" => Ok(Self::new().with_ignored_by_enemy_ai(true)),
            "movenormallywhiledragging" => Ok(Self::new().with_move_normally_while_dragging(true)),
            "cantinker" => Ok(Self::new().with_can_tinker(true)),
            "cantinkerfabricatorsanddeconstructors" => {
                Ok(Self::new().with_can_tinker_fabricators_and_deconstructors(true))
            }
            "tinkeringpowersdevices" => Ok(Self::new().with_tinkering_powers_devices(true)),
            "gainskillpastmaximum" => Ok(Self::new().with_gain_skill_past_maximum(true)),
            "retainexperiencefornewcharacter" => {
                Ok(Self::new().with_retain_experience_for_new_character(true))
            }
            "allowsecondorderedtarget" => Ok(Self::new().with_allow_second_ordered_target(true)),
            "alwaysstayconscious" => Ok(Self::new().with_always_stay_conscious(true)),
            "cannotdietoafflictions" => Ok(Self::new().with_can_not_die_to_afflictions(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct PeriodicEffect {
    pub status_effects: Vec<StatusEffect>,
    pub interval: Option<f32>,
    pub min_interval: Option<f32>,
    pub max_interval: Option<f32>,
    pub min_strength: Option<f32>,
    pub max_strength: Option<f32>,
}

impl PeriodicEffect {
    pub fn new(element: Node) -> Self {
        let mut status_effects = Vec::new();
        for child in element.children().filter(Node::is_element) {
            status_effects.push(StatusEffect::new(child));
        }
        let interval = element
            .attribute_ignore_ascii_case("interval")
            .map(|v| v.parse::<f32>().unwrap().max(1.0));
        let min_interval = element
            .attribute_ignore_ascii_case("mininterval")
            .map(|v| v.parse::<f32>().unwrap().max(1.0));
        let max_interval = element
            .attribute_ignore_ascii_case("maxinterval")
            .map(|v| v.parse::<f32>().unwrap().max(min_interval.unwrap_or(1.0)));
        let min_strength = element
            .attribute_ignore_ascii_case("minstrength")
            .map(|v| v.parse::<f32>().unwrap().max(0.0));
        let max_strength = element
            .attribute_ignore_ascii_case("maxstrength")
            .map(|v| v.parse::<f32>().unwrap().max(min_strength.unwrap_or(0.0)));

        Self {
            status_effects,
            interval,
            min_interval,
            max_interval,
            min_strength,
            max_strength,
        }
    }
}
