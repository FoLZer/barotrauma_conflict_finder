use std::{collections::HashMap, str::FromStr};

use bitfield_struct::bitfield;
use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::{
    event_prefab::EventPrefab,
    item_prefab::{BarotraumaSprite, Color, DoesNotExistError},
    particle_emitter_prefab::ParticleEmitterPrefab,
};

#[derive(Debug)]
pub struct LevelObjectPrefab {
    pub identifier: String,
    pub properties: LevelObjectPrefabProperties,
    pub sprites: Vec<(BarotraumaSprite, Option<PhysicsBody>)>,
    pub deformable_sprite: Option<DeformableSprite>,
    pub override_commonness: HashMap<String, f32>,
    pub child_objects: Vec<ChildObject>,
    pub level_triggers: Vec<LevelTriggerWithPosition>,
    pub physics_body: Option<PhysicsBody>,
}

impl LevelObjectPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap_or_default();
        let properties = LevelObjectPrefabProperties::new(element);

        let el = Self::load_elements(element);

        Self {
            identifier,
            properties,
            sprites: el.sprites,
            deformable_sprite: el.deformable_sprite,
            override_commonness: el.override_commonness,
            child_objects: el.child_objects,
            level_triggers: el.level_triggers,
            physics_body: el.physics_body,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn load_elements(element: Node) -> LoadedElements {
        let mut sprites = Vec::new();
        let mut deformable_sprite = None;
        let mut override_commonness = HashMap::new();
        let mut child_objects = Vec::new();
        let mut level_triggers = Vec::new();
        let mut physics_body = None;
        let mut properties_overriden = None;
        let mut sprite_deformations = Vec::new();
        let mut sounds = Vec::new();
        let mut particle_emitter_prefabs = Vec::new();
        let mut light_source_params = Vec::new();

        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "sprite" => {
                    let sprite = BarotraumaSprite::new(child);
                    let mut sprite_specific_physics_body_element = None;
                    for child in child.children().filter(Node::is_element) {
                        match child.tag_name().name().to_lowercase().as_str() {
                            "physicsbody" | "body" => {
                                sprite_specific_physics_body_element = Some(child);
                                break;
                            }
                            _ => (),
                        }
                    }
                    let physics_body = sprite_specific_physics_body_element.map(PhysicsBody::new);
                    sprites.push((sprite, physics_body));
                }
                "deformablesprite" => {
                    deformable_sprite = Some(DeformableSprite::new(child));
                    for child in child.children().filter(Node::is_element) {
                        sprite_deformations.push(SpriteDeformation::new(child));
                    }
                }
                "overridecommonness" => {
                    let level_type = child
                        .attribute_ignore_ascii_case("leveltype")
                        .unwrap()
                        .to_owned();
                    let commonness = child
                        .attribute_ignore_ascii_case("commonness")
                        .map_or(1.0, |v| v.parse::<f32>().unwrap());
                    override_commonness.insert(level_type, commonness);
                }
                "leveltrigger" | "trigger" => {
                    let position = child
                        .attribute_ignore_ascii_case("position")
                        .map(|v| v.parse::<Vector2>().unwrap().0);
                    level_triggers.push(LevelTriggerWithPosition {
                        position,
                        level_trigger: LevelTrigger::new(child),
                    });
                }
                "childobject" => {
                    child_objects.push(ChildObject::new(child));
                }
                "overrideproperties" => {
                    properties_overriden = Some(LevelObjectPrefab::new(child));
                }
                "body" | "physicsbody" => {
                    physics_body = Some(PhysicsBody::new(child));
                }
                "lightsource" => {
                    light_source_params.push(LightSourceParams::new(child));
                }
                "particleemitter" => {
                    particle_emitter_prefabs.push(ParticleEmitterPrefabWithPosition::new(child));
                }
                "sound" => {
                    sounds.push(SoundConfig::new(child));
                }
                _ => (),
            }
        }

        LoadedElements {
            sprites,
            deformable_sprite,
            override_commonness,
            child_objects,
            level_triggers,
            properties_overriden,
            physics_body,
        }
    }
}

#[derive(Debug)]
pub struct LevelTriggerWithPosition {
    pub position: Option<Vec2>,
    pub level_trigger: LevelTrigger,
}

#[derive(Debug)]
struct LoadedElements {
    pub sprites: Vec<(BarotraumaSprite, Option<PhysicsBody>)>,
    pub deformable_sprite: Option<DeformableSprite>,
    pub override_commonness: HashMap<String, f32>,
    pub child_objects: Vec<ChildObject>,
    pub level_triggers: Vec<LevelTriggerWithPosition>,
    pub properties_overriden: Option<LevelObjectPrefab>,
    pub physics_body: Option<PhysicsBody>,
}

#[derive(Debug)]

pub struct TriggerPropertiesOverrides {
    pub sprites: Vec<(BarotraumaSprite, Option<PhysicsBody>)>,
    pub deformable_sprite: Option<DeformableSprite>,
    pub override_commonness: HashMap<String, f32>,
    pub child_objects: Vec<ChildObject>,
    pub level_triggers: Vec<LevelTriggerWithPosition>,
    pub properties_overriden: Option<LevelObjectPrefab>,
    pub physics_body: Option<PhysicsBody>,
}

#[derive(Debug)]
pub struct LevelTrigger {
    pub physics_body: Option<PhysicsBodyWithIsSensor>,
    pub camera_shake: Option<f32>,
    pub infect_identifier: Option<String>,
    pub infection_chance: Option<f32>,
    pub trigger_once: bool,
    pub stay_triggered_delay: Option<f32>,
    pub random_trigger_interval: Option<f32>,
    pub random_trigger_probability: Option<f32>,
    pub use_network_syncing: bool,
    pub unrotated_force: Vec2,
    pub force_fluctuation_interval: Option<f32>,
    pub force_fluctuation_strength: Option<f32>,
    pub force_falloff: bool,
    pub global_force_decrease_interval: Option<f32>,
    pub force_velocity_limit: Option<f32>,
    pub force_mode: Option<TriggerForceMode>,
    pub triggered_by: Option<TriggererType>,
    pub trigger_others_distance: Option<f32>,
    pub tags: Option<Vec<String>>,
    pub allowed_other_trigger_tags: Option<Vec<String>>,
    pub status_effects: Vec<StatusEffect>,
    pub attacks: Vec<Attack>,
    pub overrides: TriggerPropertiesOverrides,
}

impl LevelTrigger {
    pub fn new(element: Node) -> Self {
        let physics_body = if element.has_attribute_ignore_ascii_case("radius")
            || element.has_attribute_ignore_ascii_case("width")
            || element.has_attribute_ignore_ascii_case("height")
        {
            Some(PhysicsBodyWithIsSensor::new(element))
        } else {
            None
        };
        let camera_shake = element
            .attribute_ignore_ascii_case("camerashake")
            .map(|v| v.parse::<f32>().unwrap());
        let infect_identifier = element
            .attribute_ignore_ascii_case("infectidentifier")
            .map(|v| v.to_owned());
        let infection_chance = element
            .attribute_ignore_ascii_case("infectionchance")
            .map(|v| v.parse::<f32>().unwrap());
        let trigger_once = element
            .attribute_ignore_ascii_case("triggeronce")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let stay_triggered_delay = element
            .attribute_ignore_ascii_case("staytriggereddelay")
            .map(|v| v.parse::<f32>().unwrap());
        let random_trigger_interval = element
            .attribute_ignore_ascii_case("randomtriggerinterval")
            .map(|v| v.parse::<f32>().unwrap());
        let random_trigger_probability = element
            .attribute_ignore_ascii_case("randomtriggerprobability")
            .map(|v| v.parse::<f32>().unwrap());
        let use_network_syncing = element
            .attribute_ignore_ascii_case("networksyncing")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let unrotated_force = element
            .attribute_ignore_ascii_case("force")
            .map(|v| {
                if v.contains(',') {
                    v.parse::<Vector2>().unwrap().0
                } else {
                    Vec2::new(v.parse::<f32>().unwrap(), 0.0)
                }
            })
            .unwrap_or_default();
        let force_fluctuation_interval = element
            .attribute_ignore_ascii_case("forcefluctuationinterval")
            .map(|v| v.parse::<f32>().unwrap());
        let force_fluctuation_strength = element
            .attribute_ignore_ascii_case("forcefluctuationstrength")
            .map(|v| v.parse::<f32>().unwrap());
        let force_falloff = element
            .attribute_ignore_ascii_case("forcefalloff")
            .map_or(true, |v| v.parse::<bool>().unwrap());
        let global_force_decrease_interval = element
            .attribute_ignore_ascii_case("globalforcedecreaseinterval")
            .map(|v| v.parse::<f32>().unwrap());
        let force_velocity_limit = element
            .attribute_ignore_ascii_case("forcevelocitylimit")
            .map(|v| v.parse::<f32>().unwrap()); //This is in display units
        let force_mode = element
            .attribute_ignore_ascii_case("forcemode")
            .map(|v| v.parse::<TriggerForceMode>().unwrap());
        let triggered_by = element.attribute_ignore_ascii_case("triggeredby").map(|v| {
            v.split(',')
                .map(|v| v.parse::<TriggererType>().unwrap())
                .fold(TriggererType::new(), |acc, e| {
                    TriggererType::from_bits(acc.into_bits() | e.into_bits())
                })
        });
        let trigger_others_distance = element
            .attribute_ignore_ascii_case("triggerothersdistance")
            .map(|v| v.parse::<f32>().unwrap());
        let tags = element
            .attribute_ignore_ascii_case("tags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let allowed_other_trigger_tags = if triggered_by.is_some_and(|v| v.other_trigger()) {
            element
                .attribute_ignore_ascii_case("allowedothertriggertags")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
        } else {
            None
        };

        let mut status_effects = Vec::new();
        let mut attacks = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "statuseffect" => {
                    status_effects.push(StatusEffect::new(child));
                }
                "attack" | "damage" => {
                    attacks.push(Attack::new(child));
                }
                _ => (),
            }
        }
        let overrides = LevelObjectPrefab::load_elements(element);
        let overrides = TriggerPropertiesOverrides {
            sprites: overrides.sprites,
            deformable_sprite: overrides.deformable_sprite,
            override_commonness: overrides.override_commonness,
            child_objects: overrides.child_objects,
            level_triggers: overrides.level_triggers,
            properties_overriden: overrides.properties_overriden,
            physics_body: overrides.physics_body,
        };

        Self {
            physics_body,
            camera_shake,
            infect_identifier,
            infection_chance,
            trigger_once,
            stay_triggered_delay,
            random_trigger_interval,
            random_trigger_probability,
            use_network_syncing,
            unrotated_force,
            force_fluctuation_interval,
            force_fluctuation_strength,
            force_falloff,
            global_force_decrease_interval,
            force_velocity_limit,
            force_mode,
            triggered_by,
            trigger_others_distance,
            tags,
            allowed_other_trigger_tags,
            status_effects,
            attacks,
            overrides,
        }
    }
}

#[derive(Debug)]
pub enum StatusEffect {
    Delayed(DelayedStatusEffect),
    Normal(NormalStatusEffect),
}

impl StatusEffect {
    pub fn new(element: Node) -> Self {
        if element.has_attribute_ignore_ascii_case("delay")
            || element.has_attribute_ignore_ascii_case("delaytype")
        {
            Self::Delayed(DelayedStatusEffect::new(element))
        } else {
            Self::Normal(NormalStatusEffect::new(element))
        }
    }
}

#[derive(Debug)]
pub struct DelayedStatusEffect {
    pub normal_status_effect: NormalStatusEffect,
    pub delay_type: DelayType,
    pub delay: Option<f32>,
}

impl DelayedStatusEffect {
    pub fn new(element: Node) -> Self {
        let delay_type = element
            .attribute_ignore_ascii_case("delaytype")
            .map_or(DelayType::Timer, |v| v.parse::<DelayType>().unwrap());
        let delay = if let DelayType::Timer = delay_type {
            element
                .attribute_ignore_ascii_case("delay")
                .map(|v| v.parse::<f32>().unwrap())
        } else {
            None
        };
        let normal_status_effect = NormalStatusEffect::new(element);

        Self {
            normal_status_effect,
            delay_type,
            delay,
        }
    }
}

#[derive(Debug)]
pub struct NormalStatusEffect {
    pub tags: Option<Vec<String>>,
    pub only_inside: bool,
    pub only_outside: bool,
    pub only_when_damaged_by_player: bool,
    pub allow_when_broken: bool,
    pub interval: Option<f32>,
    pub duration: Option<f32>,
    pub disable_delta_time: bool,
    pub set_value: bool,
    pub stackable: bool,
    pub lifetime: Option<f32>,
    pub check_conditional_always: bool,
    pub target_item_component: Option<String>,
    pub target_slot: Option<u32>,
    pub range: Option<f32>,
    pub offset: Option<Vec2>,
    pub target_limbs: Option<Vec<LimbType>>,
    pub sever_limbs_probability: Option<f32>,
    pub target_types: Option<TargetType>,
    pub target_identifiers: Option<Vec<String>>,
    pub triggered_event_target_tag: Option<String>,
    pub triggered_event_entity_tag: Option<String>,
    pub triggered_event_user_tag: Option<String>,
    pub spawn_item_randomly: bool,
    pub multiply_afflictions_by_max_vitality: bool,
    pub play_sound_on_required_item_failure: bool,
    pub explosions: Vec<Explosion>,
    pub fire_size: Option<f32>,
    pub use_item_count: i32,
    pub remove_item: bool,
    pub drop_contained_items: bool,
    pub drop_item: bool,
    pub remove_character: bool,
    pub break_limb: bool,
    pub hide_limb: bool,
    pub hide_limb_timer: Option<f32>,
    pub required_afflictions: HashMap<String, f32>, //Affliction identifier -> min strength required
    pub property_conditionals: Vec<PropertyConditional>,
    pub required_items: Vec<RelatedItem>,
    pub spawn_items: Vec<ItemSpawnInfo>,
    pub triggered_events_identifiers: Vec<String>,
    pub scripted_triggered_events: Vec<EventPrefab>,
    pub spawn_characters: Vec<CharacterSpawnInfo>,
    pub give_talent_infos: Vec<GiveTalentInfo>,
    pub ai_triggers: Vec<AITrigger>,
    pub talent_triggers: Vec<String>,
    pub event_target_tags: Vec<(String, String)>,
    pub give_experiences: Vec<i32>,
    pub give_skills: Vec<GiveSkill>,
    pub give_afflictions: Vec<GiveAffliction>,
    pub reduce_afflictions: Vec<ReduceAffliction>,
    pub particle_emitters: Vec<ParticleEmitterPrefab>,
    pub sounds: Vec<RoundSound>,
    pub loop_sound: bool,
    pub sound_selection_mode: SoundSelectionMode,
    pub force_play_sounds: bool,
}

impl NormalStatusEffect {
    pub fn new(element: Node) -> Self {
        let tags = element
            .attribute_ignore_ascii_case("tags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let only_inside = element
            .attribute_ignore_ascii_case("onlyinside")
            .map_or(false, |v| v.to_lowercase().parse().unwrap());
        let only_outside = element
            .attribute_ignore_ascii_case("onlyoutside")
            .map_or(false, |v| v.to_lowercase().parse().unwrap());
        let only_when_damaged_by_player = element
            .attribute_ignore_ascii_case("onlyplayertriggered")
            .or(element.attribute_ignore_ascii_case("onlywhendamagedbyplayer"))
            .map_or(false, |v| v.parse().unwrap());
        let allow_when_broken = element
            .attribute_ignore_ascii_case("allowwhenbroken")
            .map_or(false, |v| v.parse().unwrap());
        let interval = element
            .attribute_ignore_ascii_case("interval")
            .map(|v| v.parse::<f32>().unwrap());
        let duration = element
            .attribute_ignore_ascii_case("duration")
            .map(|v| v.parse::<f32>().unwrap());
        let disable_delta_time = element
            .attribute_ignore_ascii_case("disabledeltatime")
            .map_or(false, |v| v.to_lowercase().parse().unwrap());
        let set_value = element
            .attribute_ignore_ascii_case("setvalue")
            .map_or(false, |v| v.to_lowercase().parse().unwrap());
        let stackable = element
            .attribute_ignore_ascii_case("stackable")
            .map_or(true, |v| v.to_lowercase().parse().unwrap());
        let lifetime = element
            .attribute_ignore_ascii_case("lifetime")
            .map(|v| v.parse::<f32>().unwrap());
        let check_conditional_always = element
            .attribute_ignore_ascii_case("checkconditionalalways")
            .map_or(false, |v| v.to_lowercase().parse().unwrap());
        let target_item_component = element
            .attribute_ignore_ascii_case("targetitemcomponent")
            .map(|v| v.to_owned());
        let target_slot = element
            .attribute_ignore_ascii_case("targetslot")
            .map(|v| v.parse::<u32>().unwrap());
        let range = element
            .attribute_ignore_ascii_case("range")
            .map(|v| v.parse::<f32>().unwrap());
        let offset = element
            .attribute_ignore_ascii_case("offset")
            .map(|v| v.parse::<Vector2>().unwrap().0);
        let target_limbs = element
            .attribute_ignore_ascii_case("targetlimb")
            .or(element.attribute_ignore_ascii_case("targetslimb"))
            .map(|v| {
                v.split(',')
                    .map(|v| v.parse::<LimbType>().unwrap())
                    .collect::<Vec<_>>()
            });
        let sever_limbs_probability = element
            .attribute_ignore_ascii_case("severlimbs")
            .or(element.attribute_ignore_ascii_case("severlimbsprobability"))
            .map(|v| v.parse::<f32>().unwrap());
        let target_types = element
            .attribute_ignore_ascii_case("target")
            .or(element.attribute_ignore_ascii_case("targettype"))
            .map(|v| {
                v.split(',')
                    .map(|v| v.parse::<TargetType>().unwrap())
                    .fold(TargetType::new(), |acc, e| {
                        TargetType::from_bits(acc.into_bits() | e.into_bits())
                    })
            });
        let target_identifiers = element
            .attribute_ignore_ascii_case("targetnames")
            .or(element.attribute_ignore_ascii_case("targets"))
            .or(element.attribute_ignore_ascii_case("targetidentifiers"))
            .or(element.attribute_ignore_ascii_case("targettags"))
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let triggered_event_target_tag = element
            .attribute_ignore_ascii_case("eventtargettag")
            .map(|v| v.to_owned());
        let triggered_event_entity_tag = element
            .attribute_ignore_ascii_case("evententitytag")
            .map(|v| v.to_owned());
        let triggered_event_user_tag = element
            .attribute_ignore_ascii_case("eventusertag")
            .map(|v| v.to_owned());
        let spawn_item_randomly = element
            .attribute_ignore_ascii_case("spawnitemrandomly")
            .map_or(false, |v| v.parse().unwrap());
        let multiply_afflictions_by_max_vitality = element
            .attribute_ignore_ascii_case("multiplyafflictionsbymaxvitality")
            .map_or(false, |v| v.parse().unwrap());
        let play_sound_on_required_item_failure = element
            .attribute_ignore_ascii_case("playsoundonrequireditemfailure")
            .map_or(false, |v| v.parse().unwrap());

        {
            //TODO: propertyAttributes
            let ty = element
                .attribute_ignore_ascii_case("type")
                .map(|v| v.parse::<ActionType>().unwrap());
            let required_afflictions = element
                .attribute_ignore_ascii_case("allowedafflictions")
                .or(element.attribute_ignore_ascii_case("requiredafflictions"))
                .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
            let conditional_logical_operator = element
                .attribute_ignore_ascii_case("conditionalcomparison")
                .or(element.attribute_ignore_ascii_case("comparison"))
                .map(|v| v.parse::<LogicalOperatorType>().unwrap());
        }

        let mut explosions = Vec::new();
        let mut fire_size = None;
        let mut use_item_count = 0;
        let mut remove_item = false;
        let mut drop_contained_items = false;
        let mut drop_item = false;
        let mut remove_character = false;
        let mut break_limb = false;
        let mut hide_limb = false;
        let mut hide_limb_timer = None;
        let mut required_afflictions = HashMap::new();
        let mut property_conditionals = Vec::new();
        let mut required_items = Vec::new();
        let mut spawn_items = Vec::new();
        let mut triggered_events_identifiers = Vec::new();
        let mut scripted_triggered_events = Vec::new();
        let mut spawn_characters = Vec::new();
        let mut give_talent_infos = Vec::new();
        let mut ai_triggers = Vec::new();
        let mut talent_triggers = Vec::new();
        let mut event_target_tags = Vec::new();
        let mut give_experiences = Vec::new();
        let mut give_skills = Vec::new();
        let mut give_afflictions = Vec::new();
        let mut reduce_afflictions = Vec::new();
        let mut particle_emitters = Vec::new();
        let mut sounds = Vec::new();
        let mut loop_sound = false;
        let mut sound_selection_mode = SoundSelectionMode::Random;
        let force_play_sounds = element
            .attribute_ignore_ascii_case("forceplaysounds")
            .map_or(false, |v| v.parse().unwrap());

        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "explosion" => {
                    explosions.push(Explosion::new(child));
                }
                "fire" => {
                    fire_size = child
                        .attribute_ignore_ascii_case("size")
                        .map(|v| v.parse::<f32>().unwrap());
                }
                "use" | "useitem" => {
                    use_item_count += 1;
                }
                "remove" | "removeitem" => {
                    remove_item = true;
                }
                "dropcontaineditems" => {
                    drop_contained_items = true;
                }
                "dropitem" => {
                    drop_item = true;
                }
                "removecharacter" => {
                    remove_character = true;
                }
                "breaklimb" => {
                    break_limb = true;
                }
                "hidelimb" => {
                    hide_limb = true;
                    hide_limb_timer = child
                        .attribute_ignore_ascii_case("duration")
                        .map(|v| v.parse::<f32>().unwrap());
                }
                "requireditem" | "requireditems" => {
                    required_items.push(RelatedItem::new(child));
                }
                "requiredaffliction" => {
                    let min_strength = child
                        .attribute_ignore_ascii_case("minstrength")
                        .map_or(0.0, |v| v.parse::<f32>().unwrap());
                    required_afflictions.extend(
                        child
                            .attribute_ignore_ascii_case("identifier")
                            .or(child.attribute_ignore_ascii_case("type"))
                            .map(|v| v.split(',').map(|v| (v.to_owned(), min_strength)))
                            .unwrap(),
                    );
                }
                "conditional" => {
                    property_conditionals.push(PropertyConditional::from_xml(child));
                }
                "affliction" => {
                    if child.has_attribute_ignore_ascii_case("name") {
                        panic!();
                    } else {
                        let affliction_identifier = child
                            .attribute_ignore_ascii_case("identifier")
                            .map(|v| v.to_owned())
                            .unwrap();
                        let strength = child
                            .attribute_ignore_ascii_case("amount")
                            .or(child.attribute_ignore_ascii_case("strength"))
                            .map_or(1.0, |v| v.parse::<f32>().unwrap());
                        let probability = child
                            .attribute_ignore_ascii_case("probability")
                            .map_or(1.0, |v| v.parse::<f32>().unwrap());
                        give_afflictions.push(GiveAffliction {
                            affliction_identifier,
                            strength,
                            probability,
                        });
                    }
                }
                "reduceaffliction" => {
                    if child.has_attribute_ignore_ascii_case("name") {
                        panic!();
                    } else {
                        let affliction_identifier = child
                            .attribute_ignore_ascii_case("identifier")
                            .map(|v| v.to_owned());
                        let affliction_type = child
                            .attribute_ignore_ascii_case("type")
                            .map(|v| v.to_owned());
                        let affliction = if let Some(identifier) = affliction_identifier {
                            AfflictionIdentifierOrType::Identifier(identifier)
                        } else if let Some(ty) = affliction_type {
                            AfflictionIdentifierOrType::Type(ty)
                        } else {
                            panic!()
                        };
                        let strength = child
                            .attribute_ignore_ascii_case("amount")
                            .or(child.attribute_ignore_ascii_case("strength"))
                            .or(child.attribute_ignore_ascii_case("reduceamount"))
                            .map_or(1.0, |v| v.parse::<f32>().unwrap());
                        reduce_afflictions.push(ReduceAffliction {
                            affliction,
                            strength,
                        })
                    }
                }
                "spawnitem" => {
                    spawn_items.push(ItemSpawnInfo::new(child));
                }
                "triggerevent" => {
                    let identifier = child
                        .attribute_ignore_ascii_case("identifier")
                        .map(|v| v.to_owned());
                    if let Some(identifier) = identifier {
                        triggered_events_identifiers.push(identifier);
                    }

                    scripted_triggered_events.extend(
                        child
                            .children()
                            .filter(Node::is_element)
                            .filter(|child| {
                                child
                                    .tag_name()
                                    .name()
                                    .eq_ignore_ascii_case("ScriptedEvent")
                            })
                            .map(|child| EventPrefab::new(child, None)),
                    );
                }
                "spawncharacter" => {
                    spawn_characters.push(CharacterSpawnInfo::new(child));
                }
                "givetalentinfo" => {
                    give_talent_infos.push(GiveTalentInfo::new(child));
                }
                "aitrigger" => {
                    ai_triggers.push(AITrigger::new(child));
                }
                "talenttrigger" => {
                    talent_triggers.push(
                        child
                            .attribute_ignore_ascii_case("effectidentifier")
                            .map(|v| v.to_owned())
                            .unwrap(),
                    );
                }
                "eventtrigger" => {
                    event_target_tags.push((
                        child
                            .attribute_ignore_ascii_case("eventidentifier")
                            .map(|v| v.to_owned())
                            .unwrap(),
                        child
                            .attribute_ignore_ascii_case("tag")
                            .map(|v| v.to_owned())
                            .unwrap(),
                    ));
                }
                "giveexperience" => {
                    give_experiences.push(
                        child
                            .attribute_ignore_ascii_case("amount")
                            .map_or(0, |v| v.parse::<i32>().unwrap()),
                    );
                }
                "giveskill" => {
                    give_skills.push(GiveSkill::new(child));
                }
                "particleemitter" => {
                    particle_emitters.push(ParticleEmitterPrefab::new(child));
                }
                "sound" => {
                    let sound = RoundSound::new(child);
                    loop_sound = child
                        .attribute_ignore_ascii_case("loop")
                        .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
                    if let Some(v) = child.attribute_ignore_ascii_case("selectionmode") {
                        sound_selection_mode = v.parse().unwrap();
                    }
                    sounds.push(sound);
                }
                _ => (),
            }
        }

        Self {
            tags,
            only_inside,
            only_outside,
            only_when_damaged_by_player,
            allow_when_broken,
            interval,
            duration,
            disable_delta_time,
            set_value,
            stackable,
            lifetime,
            check_conditional_always,
            target_item_component,
            target_slot,
            range,
            offset,
            target_limbs,
            sever_limbs_probability,
            target_types,
            target_identifiers,
            triggered_event_target_tag,
            triggered_event_entity_tag,
            triggered_event_user_tag,
            spawn_item_randomly,
            multiply_afflictions_by_max_vitality,
            play_sound_on_required_item_failure,
            explosions,
            fire_size,
            use_item_count,
            remove_item,
            drop_contained_items,
            drop_item,
            remove_character,
            break_limb,
            hide_limb,
            hide_limb_timer,
            required_afflictions,
            property_conditionals,
            required_items,
            spawn_items,
            triggered_events_identifiers,
            scripted_triggered_events,
            spawn_characters,
            give_talent_infos,
            ai_triggers,
            talent_triggers,
            event_target_tags,
            give_experiences,
            give_skills,
            give_afflictions,
            reduce_afflictions,
            particle_emitters,
            sounds,
            loop_sound,
            sound_selection_mode,
            force_play_sounds,
        }
    }
}

#[derive(Debug)]
pub struct Explosion {
    pub attack: Attack,
    pub force: Option<f32>,
    pub sparks: bool,
    pub shockwave: bool,
    pub flames: bool,
    pub underwater_bubble: bool,
    pub smoke: bool,
    pub debris: bool,
    pub play_tinnitus: bool,
    pub apply_fire_effects: bool,
    pub ignore_fire_effects_for_tags: Option<Vec<String>>,
    pub ignore_cover: bool,
    pub only_inside: bool,
    pub only_outside: bool,
    pub flash: bool,
    pub flash_duration: Option<f32>,
    pub flash_range: Option<f32>,
    pub flash_color: Option<Color>,
    pub emp_strength: Option<f32>,
    pub ballast_flora_damage: Option<f32>,
    pub item_repair_strength: Option<f32>,
    pub decal: Option<String>,
    pub decal_size: Option<f32>,
    pub camera_shake: Option<f32>,
    pub camera_shake_range: Option<f32>,
    pub screen_color_range: Option<f32>,
    pub screen_color: Option<Color>,
    pub screen_color_duration: Option<f32>,
}

impl Explosion {
    pub fn new(element: Node) -> Self {
        let attack = Attack::new(element);
        let force = element
            .attribute_ignore_ascii_case("force")
            .map(|v| v.parse::<f32>().unwrap());
        let show_effects = !element
            .attribute_ignore_ascii_case("abilityexplosion")
            .map_or(false, |v| v.parse().unwrap())
            && element
                .attribute_ignore_ascii_case("showeffects")
                .map_or(true, |v| v.parse().unwrap());
        let sparks = element
            .attribute_ignore_ascii_case("sparks")
            .map_or(show_effects, |v| v.parse().unwrap());
        let shockwave = element
            .attribute_ignore_ascii_case("shockwave")
            .map_or(show_effects, |v| v.parse().unwrap());
        let flames = element
            .attribute_ignore_ascii_case("flames")
            .map_or(show_effects, |v| v.parse().unwrap());
        let underwater_bubble = element
            .attribute_ignore_ascii_case("underwaterbubble")
            .map_or(show_effects, |v| v.parse().unwrap());
        let smoke = element
            .attribute_ignore_ascii_case("smoke")
            .map_or(show_effects, |v| v.parse().unwrap());
        let debris = element
            .attribute_ignore_ascii_case("debris")
            .map_or(false, |v| v.parse().unwrap());
        let play_tinnitus = element
            .attribute_ignore_ascii_case("playtinnitus")
            .map_or(show_effects, |v| v.parse().unwrap());
        let apply_fire_effects = element
            .attribute_ignore_ascii_case("applyfireeffects")
            .map_or(flames && show_effects, |v| v.parse().unwrap());
        let ignore_fire_effects_for_tags = element
            .attribute_ignore_ascii_case("ignorefireeffectsfortags")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let ignore_cover = element
            .attribute_ignore_ascii_case("ignorecover")
            .map_or(false, |v| v.parse().unwrap());
        let only_inside = element
            .attribute_ignore_ascii_case("onlyinside")
            .map_or(false, |v| v.parse().unwrap());
        let only_outside = element
            .attribute_ignore_ascii_case("onlyoutside")
            .map_or(false, |v| v.parse().unwrap());
        let flash = element
            .attribute_ignore_ascii_case("flash")
            .map_or(show_effects, |v| v.parse().unwrap());
        let flash_duration = element
            .attribute_ignore_ascii_case("flashduration")
            .map(|v| v.parse::<f32>().unwrap());
        let flash_range = element
            .attribute_ignore_ascii_case("flashrange")
            .map(|v| v.parse::<f32>().unwrap());
        let flash_color = element
            .attribute_ignore_ascii_case("flashcolor")
            .map(|v| v.parse::<Color>().unwrap());
        let emp_strength = element
            .attribute_ignore_ascii_case("empstrength")
            .map(|v| v.parse::<f32>().unwrap());
        let ballast_flora_damage = element
            .attribute_ignore_ascii_case("ballastfloradamage")
            .map(|v| v.parse::<f32>().unwrap());
        let item_repair_strength = element
            .attribute_ignore_ascii_case("itemrepairstrength")
            .map(|v| v.parse::<f32>().unwrap());
        let decal = element
            .attribute_ignore_ascii_case("decal")
            .map(|v| v.to_owned());
        let decal_size = element
            .attribute_ignore_ascii_case("decalSize")
            .or(element.attribute_ignore_ascii_case("decalsize"))
            .map(|v| v.parse::<f32>().unwrap());
        let camera_shake = element
            .attribute_ignore_ascii_case("camerashake")
            .map(|v| v.parse::<f32>().unwrap());
        let camera_shake_range = element
            .attribute_ignore_ascii_case("camerashakerange")
            .map(|v| v.parse::<f32>().unwrap());
        let screen_color_range = element
            .attribute_ignore_ascii_case("screencolorrange")
            .map(|v| v.parse::<f32>().unwrap());
        let screen_color = element
            .attribute_ignore_ascii_case("screencolor")
            .map(|v| v.parse::<Color>().unwrap());
        let screen_color_duration = element
            .attribute_ignore_ascii_case("screencolorduration")
            .map(|v| v.parse::<f32>().unwrap());

        Self {
            attack,
            force,
            sparks,
            shockwave,
            flames,
            underwater_bubble,
            smoke,
            debris,
            play_tinnitus,
            apply_fire_effects,
            ignore_fire_effects_for_tags,
            ignore_cover,
            only_inside,
            only_outside,
            flash,
            flash_duration,
            flash_range,
            flash_color,
            emp_strength,
            ballast_flora_damage,
            item_repair_strength,
            decal,
            decal_size,
            camera_shake,
            camera_shake_range,
            screen_color_range,
            screen_color,
            screen_color_duration,
        }
    }
}

#[derive(Debug)]

pub struct Attack {
    pub properties: AttackProperties,
    pub status_effects: Vec<StatusEffect>,
    pub conditionals: Vec<PropertyConditional>,
    pub particle_emitter: Option<ParticleEmitterPrefab>,
    pub sound: Option<RoundSound>,
    pub afflictions: Vec<AfflictionProperties>,
}

impl Attack {
    pub fn new(element: Node) -> Self {
        let properties = AttackProperties::new(element);

        let mut status_effects = Vec::new();
        let mut conditionals = Vec::new();
        let mut particle_emitter = None;
        let mut sound = None;
        let mut afflictions = Vec::new();

        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "statuseffect" => {
                    status_effects.push(StatusEffect::new(child));
                }
                "affliction" => {
                    if child.has_attribute_ignore_ascii_case("name") {
                        panic!()
                        //backwards compatibility
                    } else {
                        let mut affliction_properties = AfflictionProperties::new(child);
                        if child.has_attribute_ignore_ascii_case("amount")
                            && !child.has_attribute_ignore_ascii_case("strength")
                        {
                            affliction_properties.strength = child
                                .attribute_ignore_ascii_case("amount")
                                .map(|v| v.parse().unwrap())
                                .unwrap();
                        }
                        afflictions.push(affliction_properties);
                    }
                }
                "conditional" => {
                    conditionals.push(PropertyConditional::from_xml(child));
                }
                "particleemitter" => {
                    particle_emitter = Some(ParticleEmitterPrefab::new(child));
                }
                "sound" => {
                    sound = Some(RoundSound::new(child));
                }
                _ => (),
            }
        }

        Self {
            properties,
            status_effects,
            conditionals,
            particle_emitter,
            sound,
            afflictions,
        }
    }
}

#[derive(Debug)]
pub struct PropertyConditional {
    pub target_item_component: Option<String>,
    pub target_container: bool,
    pub target_self: bool,
    pub target_grand_parent: bool,
    pub target_contained_item: bool,
    pub conditions: HashMap<String, Condition>,
}

impl PropertyConditional {
    pub fn from_xml(element: Node) -> Self {
        let target_item_component = element
            .attribute_ignore_ascii_case("targetitemcomponent")
            .map(|v| v.to_owned());
        let target_container = element
            .attribute_ignore_ascii_case("targetcontainer")
            .map_or(false, |v| v.parse().unwrap());
        let target_self = element
            .attribute_ignore_ascii_case("targetself")
            .map_or(false, |v| v.parse().unwrap());
        let target_grand_parent = element
            .attribute_ignore_ascii_case("targetgrandparent")
            .map_or(false, |v| v.parse().unwrap());
        let target_contained_item = element
            .attribute_ignore_ascii_case("targetcontaineditem")
            .map_or(false, |v| v.parse().unwrap());

        let is_skill_requirement = element
            .attribute_ignore_ascii_case("skillrequirement")
            .map_or(false, |v| v.parse().unwrap());
        let mut conditions = HashMap::new();
        for attribute in element.attributes().filter(|a| {
            !matches!(
                a.name(),
                "targetitemcomponent"
                    | "targetcontainer"
                    | "targetself"
                    | "targetgrandparent"
                    | "targetcontaineditem"
                    | "skillrequirement"
                    | "targetslot"
            )
        }) {
            let mut spl = attribute.name().split(' ');
            let op = spl
                .next()
                .and_then(|v| ComparisonOperatorType::from_str(v).ok());
            let condition = spl.next().map(|v| v.to_owned());

            let condition_type = if is_skill_requirement {
                ConditionType::SkillRequirement
            } else {
                attribute
                    .name()
                    .parse()
                    .unwrap_or(ConditionType::PropertyValueOrAffliction)
            };

            conditions.insert(attribute.name().to_owned(), Condition {
                operator: op,
                condition_value: condition,
                condition_type,
            });
        }

        Self {
            target_item_component,
            target_container,
            target_self,
            target_grand_parent,
            target_contained_item,
            conditions,
        }
    }
}

#[derive(Debug)]
pub enum ConditionType {
    PropertyValueOrAffliction,
    SkillRequirement,
    Name,
    SpeciesName,
    SpeciesGroup,
    HasTag,
    HasStatusTag,
    HasSpecifierTag,
    EntityType,
    LimbType,
}

impl FromStr for ConditionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "propertyvalueoraffliction" => Ok(Self::PropertyValueOrAffliction),
            "skillrequirement" => Ok(Self::SkillRequirement),
            "name" => Ok(Self::Name),
            "speciesname" => Ok(Self::SpeciesName),
            "speciesgroup" => Ok(Self::SpeciesGroup),
            "hastag" => Ok(Self::HasTag),
            "hasstatustag" => Ok(Self::HasStatusTag),
            "hasspecifiertag" => Ok(Self::HasSpecifierTag),
            "entitytype" => Ok(Self::EntityType),
            "limbtype" => Ok(Self::LimbType),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct Condition {
    pub operator: Option<ComparisonOperatorType>,
    pub condition_value: Option<String>,
    pub condition_type: ConditionType,
}

#[derive(Debug)]
pub enum ComparisonOperatorType {
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
}

impl FromStr for ComparisonOperatorType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "e" | "eq" | "equals" => Ok(Self::Equals),
            "ne" | "neq" | "notequals" | "!" | "!e" | "!eq" | "!equals" => Ok(Self::NotEquals),
            "gt" | "greaterthan" => Ok(Self::GreaterThan),
            "lt" | "lessthan" => Ok(Self::LessThan),
            "gte" | "gteq" | "greaterthanequals" => Ok(Self::GreaterThanEquals),
            "lte" | "lteq" | "lessthanequals" => Ok(Self::LessThanEquals),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct AttackProperties {
    pub context: AttackContext,
    pub target_type: AttackTarget,
    pub target_limb_type: LimbType,
    pub hit_detection_type: HitDetection,
    pub after_attack: AIBehaviorAfterAttack,
    pub after_attack_delay: f32,
    pub reverse: bool,
    pub snap_rope_on_new_attack: bool,
    pub retreat: bool,
    pub range: f32,
    pub damage_range: f32,
    pub min_range: f32,
    pub duration: f32,
    pub cooldown: f32,
    pub secondary_cooldown: f32,
    pub cooldown_random_factor: f32,
    pub full_speed_after_attack: bool,
    pub structure_damage: f32,
    pub emit_structure_damage_particles: bool,
    pub item_damage: f32,
    pub penetration: f32,
    pub level_wall_damage: f32,
    pub ranged: bool,
    pub avoid_friendly_fire: bool,
    pub required_angle: f32,
    pub required_angle_to_shoot: f32,
    pub aim_rotation_torque: f32,
    pub rotation_limb_index: Option<u32>,
    pub sway_amount: f32,
    pub sway_frequency: f32,
    pub stun: f32,
    pub only_humans: bool,
    pub force_on_limbs_indices: Vec<u32>,
    pub force: f32,
    pub root_force_world_start: Vec2,
    pub root_force_world_middle: Vec2,
    pub root_force_world_end: Vec2,
    pub root_transition_easing: TransitionMode,
    pub torque: f32,
    pub apply_forces_only_once: bool,
    pub target_impulse: f32,
    pub target_impulse_world: Vec2,
    pub target_force: f32,
    pub target_force_world: Vec2,
    pub submarine_impact_multiplier: f32,
    pub sever_limbs_probability: f32,
    pub priority: f32,
    pub blink: bool,
}

impl AttackProperties {
    pub fn new(element: Node) -> Self {
        Self {
            context: element
                .attribute_ignore_ascii_case("context")
                .map_or(AttackContext::Any, |v| v.parse().unwrap()),
            target_type: element
                .attribute_ignore_ascii_case("context")
                .map_or(AttackTarget::Any, |v| v.parse().unwrap()),
            target_limb_type: element
                .attribute_ignore_ascii_case("context")
                .map_or(LimbType::None, |v| v.parse().unwrap()),
            hit_detection_type: element
                .attribute_ignore_ascii_case("context")
                .map_or(HitDetection::Distance, |v| v.parse().unwrap()),
            after_attack: element
                .attribute_ignore_ascii_case("context")
                .map_or(AIBehaviorAfterAttack::FallBack, |v| v.parse().unwrap()),
            after_attack_delay: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            reverse: element
                .attribute_ignore_ascii_case("context")
                .map_or(false, |v| v.parse().unwrap()),
            snap_rope_on_new_attack: element
                .attribute_ignore_ascii_case("context")
                .map_or(true, |v| v.parse().unwrap()),
            retreat: element
                .attribute_ignore_ascii_case("context")
                .map_or(false, |v| v.parse().unwrap()),
            range: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            damage_range: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            min_range: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            duration: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.25, |v| v.parse().unwrap()),
            cooldown: element
                .attribute_ignore_ascii_case("context")
                .map_or(5.0, |v| v.parse().unwrap()),
            secondary_cooldown: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            cooldown_random_factor: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            full_speed_after_attack: element
                .attribute_ignore_ascii_case("context")
                .map_or(false, |v| v.parse().unwrap()),
            structure_damage: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            emit_structure_damage_particles: element
                .attribute_ignore_ascii_case("context")
                .map_or(true, |v| v.parse().unwrap()),
            item_damage: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            penetration: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            level_wall_damage: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            ranged: element
                .attribute_ignore_ascii_case("context")
                .map_or(false, |v| v.parse().unwrap()),
            avoid_friendly_fire: element
                .attribute_ignore_ascii_case("context")
                .map_or(false, |v| v.parse().unwrap()),
            required_angle: element
                .attribute_ignore_ascii_case("context")
                .map_or(20.0, |v| v.parse().unwrap()),
            required_angle_to_shoot: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            aim_rotation_torque: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            rotation_limb_index: element
                .attribute_ignore_ascii_case("context")
                .map(|v| v.parse().unwrap()),
            sway_amount: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            sway_frequency: element
                .attribute_ignore_ascii_case("context")
                .map_or(5.0, |v| v.parse().unwrap()),
            stun: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            only_humans: element
                .attribute_ignore_ascii_case("context")
                .map_or(false, |v| v.parse().unwrap()),
            force_on_limbs_indices: element
                .attribute_ignore_ascii_case("context")
                .map_or(Vec::default(), |v| {
                    v.split(',').map(|v| v.parse().unwrap()).collect()
                }),
            force: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            root_force_world_start: element
                .attribute_ignore_ascii_case("context")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            root_force_world_middle: element
                .attribute_ignore_ascii_case("context")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            root_force_world_end: element
                .attribute_ignore_ascii_case("context")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            root_transition_easing: element
                .attribute_ignore_ascii_case("context")
                .map_or(TransitionMode::Linear, |v| v.parse().unwrap()),
            torque: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            apply_forces_only_once: element
                .attribute_ignore_ascii_case("context")
                .map_or(false, |v| v.parse().unwrap()),
            target_impulse: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            target_impulse_world: element
                .attribute_ignore_ascii_case("context")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            target_force: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            target_force_world: element
                .attribute_ignore_ascii_case("context")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            submarine_impact_multiplier: element
                .attribute_ignore_ascii_case("context")
                .map_or(1.0, |v| v.parse().unwrap()),
            sever_limbs_probability: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            priority: element
                .attribute_ignore_ascii_case("context")
                .map_or(0.0, |v| v.parse().unwrap()),
            blink: element
                .attribute_ignore_ascii_case("context")
                .map_or(false, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransitionMode {
    Linear,
    Smooth,
    Smoother,
    EaseIn,
    EaseOut,
    Exponential,
}

impl FromStr for TransitionMode {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "linear" => Ok(Self::Linear),
            "smooth" => Ok(Self::Smooth),
            "smoother" => Ok(Self::Smoother),
            "easein" => Ok(Self::EaseIn),
            "easeout" => Ok(Self::EaseOut),
            "exponential" => Ok(Self::Exponential),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum AIBehaviorAfterAttack {
    FallBack,
    FallBackUntilCanAttack,
    PursueIfCanAttack,
    Pursue,
    FollowThrough,
    FollowThroughUntilCanAttack,
    IdleUntilCanAttack,
    Reverse,
    ReverseUntilCanAttack,
}

impl FromStr for AIBehaviorAfterAttack {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FallBack" => Ok(Self::FallBack),
            "FallBackUntilCanAttack" => Ok(Self::FallBackUntilCanAttack),
            "PursueIfCanAttack" => Ok(Self::PursueIfCanAttack),
            "Pursue" => Ok(Self::Pursue),
            "FollowThrough" => Ok(Self::FollowThrough),
            "FollowThroughUntilCanAttack" => Ok(Self::FollowThroughUntilCanAttack),
            "IdleUntilCanAttack" => Ok(Self::IdleUntilCanAttack),
            "Reverse" => Ok(Self::Reverse),
            "ReverseUntilCanAttack" => Ok(Self::ReverseUntilCanAttack),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum HitDetection {
    Distance,
    Contact,
    None,
}

impl FromStr for HitDetection {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Distance" => Ok(Self::Distance),
            "Contact" => Ok(Self::Contact),
            "None" => Ok(Self::None),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum AttackTarget {
    Any,
    Character,
    Structure,
}

impl FromStr for AttackTarget {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Any" => Ok(Self::Any),
            "Character" => Ok(Self::Character),
            "Structure" => Ok(Self::Structure),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum AttackContext {
    Any,
    Water,
    Ground,
    Inside,
    Outside,
    NotDefined,
}

impl FromStr for AttackContext {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Any" => Ok(Self::Any),
            "Water" => Ok(Self::Water),
            "Ground" => Ok(Self::Ground),
            "Inside" => Ok(Self::Inside),
            "Outside" => Ok(Self::Outside),
            "NotDefined" => Ok(Self::NotDefined),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum LogicalOperatorType {
    And,
    Or,
}

impl FromStr for LogicalOperatorType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum ActionType {
    Always,
    OnPicked,
    OnUse,
    OnSecondaryUse,
    OnWearing,
    OnContaining,
    OnContained,
    OnNotContained,
    OnActive,
    OnFailure,
    OnBroken,
    OnFire,
    InWater,
    NotInWater,
    OnImpact,
    OnEating,
    OnDamaged,
    OnSevered,
    OnProduceSpawned,
    OnOpen,
    OnClose,
    OnSpawn,
    OnSuccess,
    OnAbility,
    OnInserted,
    OnRemoved,
    OnDeath,
}

impl FromStr for ActionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "always" => Ok(Self::Always),
            "onpicked" => Ok(Self::OnPicked),
            "onuse" => Ok(Self::OnUse),
            "onsecondaryuse" => Ok(Self::OnSecondaryUse),
            "onwearing" => Ok(Self::OnWearing),
            "oncontaining" => Ok(Self::OnContaining),
            "oncontained" => Ok(Self::OnContained),
            "onnotcontained" => Ok(Self::OnNotContained),
            "onactive" => Ok(Self::OnActive),
            "onfailure" => Ok(Self::OnFailure),
            "onbroken" => Ok(Self::OnBroken),
            "onfire" => Ok(Self::OnFire),
            "inwater" => Ok(Self::InWater),
            "notinwater" => Ok(Self::NotInWater),
            "onimpact" => Ok(Self::OnImpact),
            "oneating" => Ok(Self::OnEating),
            "ondamaged" => Ok(Self::OnDamaged),
            "onsevered" => Ok(Self::OnSevered),
            "onproducespawned" => Ok(Self::OnProduceSpawned),
            "onopen" => Ok(Self::OnOpen),
            "onclose" => Ok(Self::OnClose),
            "onspawn" => Ok(Self::OnSpawn),
            "onsuccess" => Ok(Self::OnSuccess),
            "onability" => Ok(Self::OnAbility),
            "oninserted" => Ok(Self::OnInserted),
            "onremoved" => Ok(Self::OnRemoved),
            "ondeath" => Ok(Self::OnDeath),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[bitfield(u16)]

pub struct TargetType {
    pub this: bool,
    pub parent: bool,
    pub character: bool,
    pub contained: bool,
    pub nearby_characters: bool,
    pub nearby_items: bool,
    pub use_target: bool,
    pub hull: bool,
    pub limb: bool,
    pub all_limbs: bool,
    pub last_limb: bool,
    #[bits(5)]
    _unused: u8,
}

impl FromStr for TargetType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "this" => Ok(Self::new().with_this(true)),
            "parent" => Ok(Self::new().with_parent(true)),
            "character" => Ok(Self::new().with_character(true)),
            "contained" => Ok(Self::new().with_contained(true)),
            "nearbycharacters" => Ok(Self::new().with_nearby_characters(true)),
            "nearbyitems" => Ok(Self::new().with_nearby_items(true)),
            "usetarget" => Ok(Self::new().with_use_target(true)),
            "hull" => Ok(Self::new().with_hull(true)),
            "limb" => Ok(Self::new().with_limb(true)),
            "alllimbs" => Ok(Self::new().with_all_limbs(true)),
            "lastlimb" => Ok(Self::new().with_last_limb(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum LimbType {
    None,
    LeftHand,
    RightHand,
    LeftArm,
    RightArm,
    LeftForearm,
    RightForearm,
    LeftLeg,
    RightLeg,
    LeftFoot,
    RightFoot,
    Head,
    Torso,
    Tail,
    Legs,
    RightThigh,
    LeftThigh,
    Waist,
    Jaw,
}

impl FromStr for LimbType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "lefthand" => Ok(Self::LeftHand),
            "righthand" => Ok(Self::RightHand),
            "leftarm" => Ok(Self::LeftArm),
            "rightarm" => Ok(Self::RightArm),
            "leftforearm" => Ok(Self::LeftForearm),
            "rightforearm" => Ok(Self::RightForearm),
            "leftleg" => Ok(Self::LeftLeg),
            "rightleg" => Ok(Self::RightLeg),
            "leftfoot" => Ok(Self::LeftFoot),
            "rightfoot" => Ok(Self::RightFoot),
            "head" => Ok(Self::Head),
            "torso" => Ok(Self::Torso),
            "tail" => Ok(Self::Tail),
            "legs" => Ok(Self::Legs),
            "rightthigh" => Ok(Self::RightThigh),
            "leftthigh" => Ok(Self::LeftThigh),
            "waist" => Ok(Self::Waist),
            "jaw" => Ok(Self::Jaw),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum DelayType {
    Timer,
    ReachCursor,
}

impl FromStr for DelayType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Timer" => Ok(Self::Timer),
            "ReachCursor" => Ok(Self::ReachCursor),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[bitfield(u8)]

pub struct TriggererType {
    pub human: bool,
    pub creature: bool,
    pub submarine: bool,
    pub item: bool,
    pub other_trigger: bool,
    #[bits(3)]
    _unused: u8,
}

impl FromStr for TriggererType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "None" => Ok(Self::new()),
            "Human" => Ok(Self::new().with_human(true)),
            "Creature" => Ok(Self::new().with_creature(true)),
            "Character" => Ok(Self::new().with_human(true).with_creature(true)),
            "Submarine" => Ok(Self::new().with_submarine(true)),
            "Item" => Ok(Self::new().with_item(true)),
            "OtherTrigger" => Ok(Self::new().with_other_trigger(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum TriggerForceMode {
    Force,
    Acceleration,
    Impulse,
    LimitVelocity,
}

impl FromStr for TriggerForceMode {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Force" => Ok(Self::Force),
            "Acceleration" => Ok(Self::Acceleration),
            "Impulse" => Ok(Self::Impulse),
            "LimitVelocity" => Ok(Self::LimitVelocity),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct ChildObject {
    pub allowed_names: Vec<String>,
    pub min_count: u32,
    pub max_count: u32,
}

impl ChildObject {
    pub fn new(element: Node) -> Self {
        let allowed_names = element
            .attribute_ignore_ascii_case("names")
            .map_or(Vec::new(), |v| {
                v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>()
            });
        let min_count = element
            .attribute_ignore_ascii_case("mincount")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let max_count = element
            .attribute_ignore_ascii_case("maxcount")
            .map_or(1, |v| v.parse::<u32>().unwrap().max(min_count));

        Self {
            allowed_names,
            min_count,
            max_count,
        }
    }
}

#[derive(Debug)]
pub struct DeformableSprite {
    pub sprite: BarotraumaSprite,
    pub subdivisions: Vec2,
}

impl DeformableSprite {
    pub fn new(element: Node) -> Self {
        let sprite = BarotraumaSprite::new(element);
        let subdivisions = element
            .attribute_ignore_ascii_case("subdivisions")
            .map_or(Vec2::ONE, |v| v.parse::<Vector2>().unwrap().0);

        Self {
            sprite,
            subdivisions,
        }
    }
}

#[derive(Debug)]
pub struct LevelObjectPrefabProperties {
    pub min_size: f32,
    pub max_size: f32,
    pub alignment: Alignment,
    pub spawn_pos: SpawnPosType,
    pub max_count: u32,
    pub depth_range: Vec2,
    pub clustering_amount: f32,
    pub clustering_group: f32,
    pub random_offset: Vec2,
    pub align_with_surface: bool,
    pub allow_at_start: bool,
    pub allow_at_end: bool,
    pub min_surface_width: f32,
    pub random_rotation: Vec2,
    pub swing_amount: f32,
    pub swing_frequency: f32,
    pub scale_oscillation: Vec2,
    pub scale_oscillation_frequency: f32,
    pub commonness: f32,
    pub sonar_disruption: f32,
    pub take_level_wall_damage: bool,
    pub hide_when_broken: bool,
    pub health: f32,
    pub sprite_color: Color,
}

impl LevelObjectPrefabProperties {
    pub fn new(element: Node) -> Self {
        Self {
            min_size: element
                .attribute_ignore_ascii_case("minsize")
                .map_or(1.0, |v| v.parse().unwrap()),
            max_size: element
                .attribute_ignore_ascii_case("maxsize")
                .map_or(1.0, |v| v.parse().unwrap()),
            alignment: element.attribute_ignore_ascii_case("alignment").map_or(
                Alignment::new()
                    .with_top(true)
                    .with_bottom(true)
                    .with_left(true)
                    .with_right(true),
                |v| {
                    v.split(',')
                        .map(|v| v.parse().unwrap())
                        .fold(Alignment::new(), |acc, e: Alignment| {
                            Alignment::from_bits(acc.into_bits() | e.into_bits())
                        })
                },
            ),
            spawn_pos: element.attribute_ignore_ascii_case("spawnpos").map_or(
                SpawnPosType::new()
                    .with_main_path(true)
                    .with_side_path_wall(true)
                    .with_cave_wall(true),
                |v| {
                    v.split(',').map(|v| v.parse().unwrap()).fold(
                        SpawnPosType::new(),
                        |acc, e: SpawnPosType| {
                            SpawnPosType::from_bits(acc.into_bits() | e.into_bits())
                        },
                    )
                },
            ),
            max_count: element
                .attribute_ignore_ascii_case("maxcount")
                .map_or(10000, |v| v.parse().unwrap()),
            depth_range: element
                .attribute_ignore_ascii_case("depthrange")
                .map_or(Vec2::new(0.0, 1.0), |v| v.parse::<Vector2>().unwrap().0),
            clustering_amount: element
                .attribute_ignore_ascii_case("clusteringamount")
                .map_or(0.0, |v| v.parse().unwrap()),
            clustering_group: element
                .attribute_ignore_ascii_case("clusteringgroup")
                .map_or(0.0, |v| v.parse().unwrap()),
            random_offset: element
                .attribute_ignore_ascii_case("randomoffset")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            align_with_surface: element
                .attribute_ignore_ascii_case("alignwithsurface")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            allow_at_start: element
                .attribute_ignore_ascii_case("allowatstart")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            allow_at_end: element
                .attribute_ignore_ascii_case("allowatend")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            min_surface_width: element
                .attribute_ignore_ascii_case("minsurfacewidth")
                .map_or(0.0, |v| v.parse().unwrap()),
            random_rotation: element
                .attribute_ignore_ascii_case("randomrotation")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            swing_amount: element
                .attribute_ignore_ascii_case("swingamount")
                .map_or(0.0, |v| v.parse().unwrap()),
            swing_frequency: element
                .attribute_ignore_ascii_case("swingfrequency")
                .map_or(0.0, |v| v.parse().unwrap()),
            scale_oscillation: element
                .attribute_ignore_ascii_case("scaleoscillation")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            scale_oscillation_frequency: element
                .attribute_ignore_ascii_case("scaleoscillationfrequency")
                .map_or(0.0, |v| v.parse().unwrap()),
            commonness: element
                .attribute_ignore_ascii_case("commonness")
                .map_or(1.0, |v| v.parse().unwrap()),
            sonar_disruption: element
                .attribute_ignore_ascii_case("sonardisruption")
                .map_or(0.0, |v| v.parse().unwrap()),
            take_level_wall_damage: element
                .attribute_ignore_ascii_case("takelevelwalldamage")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            hide_when_broken: element
                .attribute_ignore_ascii_case("hidewhenbroken")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            health: element
                .attribute_ignore_ascii_case("health")
                .map_or(100.0, |v| v.parse().unwrap()),
            sprite_color: element.attribute_ignore_ascii_case("spritecolor").map_or(
                Color::Simple {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                |v| v.parse().unwrap(),
            ),
        }
    }
}

#[bitfield(u8)]

pub struct Alignment {
    pub center_x: bool,
    pub left: bool,
    pub right: bool,
    pub center_y: bool,
    pub top: bool,
    pub bottom: bool,
    #[bits(2)]
    _unused: u8,
}

impl FromStr for Alignment {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "none" => Ok(Self::new()),
            "centerx" => Ok(Self::new().with_center_x(true)),
            "left" => Ok(Self::new().with_left(true)),
            "right" => Ok(Self::new().with_right(true)),
            "centery" => Ok(Self::new().with_center_y(true)),
            "top" => Ok(Self::new().with_top(true)),
            "bottom" => Ok(Self::new().with_bottom(true)),
            "topleft" => Ok(Self::new().with_top(true).with_left(true)),
            "topcenter" => Ok(Self::new().with_top(true).with_center_x(true)),
            "topright" => Ok(Self::new().with_top(true).with_right(true)),
            "centerleft" => Ok(Self::new().with_left(true).with_center_y(true)),
            "center" => Ok(Self::new().with_center_x(true).with_center_y(true)),
            "centerright" => Ok(Self::new().with_right(true).with_center_y(true)),
            "bottomleft" => Ok(Self::new().with_bottom(true).with_left(true)),
            "bottomcenter" => Ok(Self::new().with_center_x(true).with_bottom(true)),
            "bottomright" => Ok(Self::new().with_bottom(true).with_right(true)),
            "any" => Ok(Self::from_bits(u8::MAX)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[bitfield(u16)]

pub struct SpawnPosType {
    pub main_path_wall: bool,
    pub side_path_wall: bool,
    pub cave_wall: bool,
    pub nest_wall: bool,
    pub ruin_wall: bool,
    pub sea_floor: bool,
    pub main_path: bool,
    pub level_start: bool,
    pub level_end: bool,
    pub outpost_wall: bool,
    #[bits(6)]
    _unused: u8,
}

impl FromStr for SpawnPosType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "none" => Ok(Self::new()),
            "mainpathwall" => Ok(Self::new().with_main_path_wall(true)),
            "sidepathwall" => Ok(Self::new().with_side_path_wall(true)),
            "cavewall" => Ok(Self::new().with_cave_wall(true)),
            "nestwall" => Ok(Self::new().with_nest_wall(true)),
            "ruinwall" => Ok(Self::new().with_ruin_wall(true)),
            "seafloor" => Ok(Self::new().with_sea_floor(true)),
            "mainpath" => Ok(Self::new().with_main_path(true)),
            "levelstart" => Ok(Self::new().with_level_start(true)),
            "levelend" => Ok(Self::new().with_level_end(true)),
            "outpostwall" => Ok(Self::new().with_outpost_wall(true)),
            "wall" => Ok(Self::new()
                .with_main_path_wall(true)
                .with_side_path_wall(true)
                .with_cave_wall(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct PhysicsBody {
    pub radius: f32,
    pub height: f32,
    pub weight: f32,
    pub density: f32,
    pub body_type: BodyType,
    pub ignore_collisions: bool,
    pub friction: f32,
    pub restitution: f32,
}

impl PhysicsBody {
    pub fn new(element: Node) -> Self {
        const NEUTRAL_DENSITY: f32 = 10.0;
        const MIN_DENSITY: f32 = 0.01;

        let radius = element
            .attribute_ignore_ascii_case("radius")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let height = element
            .attribute_ignore_ascii_case("height")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let weight = element
            .attribute_ignore_ascii_case("weight")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let density = element
            .attribute_ignore_ascii_case("density")
            .map_or(NEUTRAL_DENSITY, |v| {
                v.parse::<f32>().unwrap().max(MIN_DENSITY)
            });
        let body_type = element
            .attribute_ignore_ascii_case("bodytype")
            .map_or(BodyType::Dynamic, |v| v.parse::<BodyType>().unwrap());
        let ignore_collisions = element
            .attribute_ignore_ascii_case("ignorecollision")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let friction = element
            .attribute_ignore_ascii_case("friction")
            .map_or(0.5, |v| v.parse::<f32>().unwrap());
        let restitution = element
            .attribute_ignore_ascii_case("restitution")
            .map_or(0.05, |v| v.parse::<f32>().unwrap());

        Self {
            radius,
            height,
            weight,
            density,
            body_type,
            ignore_collisions,
            friction,
            restitution,
        }
    }
}

#[derive(Debug)]
pub enum BodyType {
    Static,
    Kinematic,
    Dynamic,
}

impl FromStr for BodyType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "static" => Ok(Self::Static),
            "kinematic" => Ok(Self::Kinematic),
            "dynamic" => Ok(Self::Dynamic),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SpriteDeformation {
    Inflate(InflateDeformation),
    Custom(CustomDeformation),
    Noise(NoiseDeformation),
    JointBend(JointBendDeformation),
    Positional(PositionalDeformation),
}

impl SpriteDeformation {
    pub fn new(element: Node) -> Self {
        let type_name = element
            .attribute_ignore_ascii_case("typename")
            .or(element.attribute_ignore_ascii_case("type"))
            .map(|v| v.to_owned())
            .unwrap();
        match type_name.to_lowercase().as_str() {
            "inflate" => Self::Inflate(InflateDeformation::new(element)),
            "custom" => Self::Custom(CustomDeformation::new(element)),
            "noise" => Self::Noise(NoiseDeformation::new(element)),
            "jointbend" | "bendjoint" => Self::JointBend(JointBendDeformation::new(element)),
            "reacttotriggerers" => Self::Positional(PositionalDeformation::new(element, None)),
            _ => {
                let reaction_type = type_name.parse::<ReactionType>().unwrap();
                Self::Positional(PositionalDeformation::new(element, Some(reaction_type)))
            }
        }
    }

    pub fn sprite_deformation_params(&self) -> &SpriteDeformationParams {
        match self {
            SpriteDeformation::Inflate(v) => &v.params.sprite_deformation_params,
            SpriteDeformation::Custom(v) => &v.params.sprite_deformation_params,
            SpriteDeformation::Noise(v) => &v.params.sprite_deformation_params,
            SpriteDeformation::JointBend(v) => &v.params,
            SpriteDeformation::Positional(v) => &v.params.sprite_deformation_params,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ReactionType {
    ReactToTriggerers,
}

impl FromStr for ReactionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "reacttotriggerers" => Ok(ReactionType::ReactToTriggerers),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PositionalDeformation {
    pub reaction_type: Option<ReactionType>,
    pub params: PositionalDeformationParams,
}

impl PositionalDeformation {
    pub fn new(element: Node, reaction_type: Option<ReactionType>) -> Self {
        Self {
            reaction_type,
            params: PositionalDeformationParams::new(element),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PositionalDeformationParams {
    pub falloff: f32,
    pub max_deformation: f32,
    pub reaction_speed: f32,
    pub recover_speed: f32,
    pub sprite_deformation_params: SpriteDeformationParams,
}

impl PositionalDeformationParams {
    pub fn new(element: Node) -> Self {
        Self {
            falloff: element
                .attribute_ignore_ascii_case("falloff")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_deformation: element
                .attribute_ignore_ascii_case("maxdeformation")
                .map_or(1.0, |v| v.parse().unwrap()),
            reaction_speed: element
                .attribute_ignore_ascii_case("reactionspeed")
                .map_or(10.0, |v| v.parse().unwrap()),
            recover_speed: element
                .attribute_ignore_ascii_case("recoverspeed")
                .map_or(0.05, |v| v.parse().unwrap()),
            sprite_deformation_params: SpriteDeformationParams::new(element),
        }
    }
}

#[derive(Clone, Debug)]
pub struct JointBendDeformation {
    pub params: SpriteDeformationParams,
}

impl JointBendDeformation {
    pub fn new(element: Node) -> Self {
        Self {
            params: SpriteDeformationParams::new(element),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoiseDeformation {
    pub params: NoiseDeformationParams,
}

impl NoiseDeformation {
    pub fn new(element: Node) -> Self {
        Self {
            params: NoiseDeformationParams::new(element),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoiseDeformationParams {
    pub frequency: f32,
    pub amplitude: f32,
    pub change_speed: f32,
    pub sprite_deformation_params: SpriteDeformationParams,
}

impl NoiseDeformationParams {
    pub fn new(element: Node) -> Self {
        Self {
            frequency: element
                .attribute_ignore_ascii_case("frequency")
                .map_or(0.0, |v| v.parse().unwrap()),
            amplitude: element
                .attribute_ignore_ascii_case("amplitude")
                .map_or(1.0, |v| v.parse().unwrap()),
            change_speed: element
                .attribute_ignore_ascii_case("changespeed")
                .map_or(0.0, |v| v.parse().unwrap()),
            sprite_deformation_params: SpriteDeformationParams::new(element),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InflateDeformation {
    pub params: InflateParams,
}

impl InflateDeformation {
    pub fn new(element: Node) -> Self {
        Self {
            params: InflateParams::new(element),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomDeformation {
    pub deform_rows: Vec<Vec<Vec2>>,
    pub params: CustomDeformationParams,
}

impl CustomDeformation {
    pub fn new(element: Node) -> Self {
        let deform_rows = {
            let mut v = Vec::new();
            let mut i = 0;
            loop {
                let Some(row) = element.attribute_ignore_ascii_case(format!("row{}", i).as_str())
                else {
                    break;
                };
                let deform_row = row
                    .split(' ')
                    .map(|v| v.parse::<Vector2>().unwrap().0)
                    .collect::<Vec<_>>();
                v.push(deform_row);
                i += 1;
            }
            v
        };
        Self {
            deform_rows,
            params: CustomDeformationParams::new(element),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomDeformationParams {
    pub frequency: f32,
    pub amplitude: f32,
    pub sprite_deformation_params: SpriteDeformationParams,
}

impl CustomDeformationParams {
    pub fn new(element: Node) -> Self {
        Self {
            frequency: element
                .attribute_ignore_ascii_case("frequency")
                .map_or(0.0, |v| v.parse().unwrap()),
            amplitude: element
                .attribute_ignore_ascii_case("amplitude")
                .map_or(1.0, |v| v.parse().unwrap()),
            sprite_deformation_params: SpriteDeformationParams::new(element),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InflateParams {
    pub frequency: f32,
    pub scale: f32,
    pub sprite_deformation_params: SpriteDeformationParams,
}

impl InflateParams {
    pub fn new(element: Node) -> Self {
        Self {
            frequency: element
                .attribute_ignore_ascii_case("frequency")
                .map_or(0.0, |v| v.parse().unwrap()),
            scale: element
                .attribute_ignore_ascii_case("scale")
                .map_or(1.0, |v| v.parse().unwrap()),
            sprite_deformation_params: SpriteDeformationParams::new(element),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpriteDeformationParams {
    pub sync: Option<u32>,
    pub ty: Option<String>,
    pub blend_mode: DeformationBlendMode,
    pub strength: f32,
    pub max_rotation: f32,
    pub use_movement_sine: bool,
    pub stop_when_host_is_dead: bool,
    pub only_in_water: bool,
    pub sine_offset: f32,
    pub resolution: Vec2,
}

impl SpriteDeformationParams {
    pub fn new(element: Node) -> Self {
        Self {
            sync: element.attribute_ignore_ascii_case("sync").and_then(|v| {
                let v = v.parse::<i32>().unwrap();
                if v == -1 { None } else { Some(v as u32) }
            }),
            ty: element
                .attribute_ignore_ascii_case("type")
                .map(|v| v.to_owned()),
            blend_mode: element
                .attribute_ignore_ascii_case("blendmode")
                .map_or(DeformationBlendMode::Add, |v| v.parse().unwrap()),
            strength: element
                .attribute_ignore_ascii_case("strength")
                .map_or(1.0, |v| v.parse().unwrap()),
            max_rotation: element
                .attribute_ignore_ascii_case("maxrotation")
                .map_or(90.0, |v| v.parse().unwrap()),
            use_movement_sine: element
                .attribute_ignore_ascii_case("usemovementsine")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            stop_when_host_is_dead: element
                .attribute_ignore_ascii_case("stopwhenhostisdead")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            only_in_water: element
                .attribute_ignore_ascii_case("onlyinwater")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            sine_offset: element
                .attribute_ignore_ascii_case("sineoffset")
                .map_or(0.0, |v| v.parse().unwrap()),
            resolution: element
                .attribute_ignore_ascii_case("resolution")
                .map_or(Vec2::splat(2.0), |v| v.parse::<Vector2>().unwrap().0),
        }
    }
}

#[derive(Clone, Debug)]
pub enum DeformationBlendMode {
    Add,
    Multiply,
    Override,
}

impl FromStr for DeformationBlendMode {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "add" => Ok(Self::Add),
            "multiply" => Ok(Self::Multiply),
            "override" => Ok(Self::Override),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

pub struct LightSourceParams {
    pub light_sprite: Option<BarotraumaSprite>,
    pub deformable_light_sprite: Option<DeformableSprite>,
    pub override_light_sprite_alpha: Option<f32>,
    pub override_light_texture: Option<BarotraumaSprite>,
    pub properties: LightSourceParamsProperties,
}

impl LightSourceParams {
    pub fn new(element: Node) -> Self {
        let properties = LightSourceParamsProperties::new(element);

        let mut light_sprite = None;
        let mut override_light_sprite_alpha = None;
        let mut deformable_light_sprite = None;
        let mut override_light_texture = None;
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "sprite" | "lightsprite" => {
                    light_sprite = Some(BarotraumaSprite::new(child));
                    override_light_sprite_alpha = child
                        .attribute_ignore_ascii_case("alpha")
                        .map(|v| v.parse::<f32>().unwrap());
                }
                "deformablesprite" => {
                    deformable_light_sprite = Some(DeformableSprite::new(child));
                    override_light_sprite_alpha = child
                        .attribute_ignore_ascii_case("alpha")
                        .map(|v| v.parse::<f32>().unwrap());
                }
                "lighttexture" => {
                    override_light_texture = Some(BarotraumaSprite::new(child));
                }
                _ => (),
            }
        }

        Self {
            light_sprite,
            deformable_light_sprite,
            override_light_sprite_alpha,
            override_light_texture,
            properties,
        }
    }
}

pub struct LightSourceParamsProperties {
    pub color: Color,
    pub range: f32,
    pub scale: f32,
    pub offset: Vec2,
    pub rotation: f32,
    pub directional: bool,
    pub flicker: f32,
    pub flicker_speed: f32,
    pub pulse_frequency: f32,
    pub pulse_amount: f32,
    pub blink_frequency: f32,
}

impl LightSourceParamsProperties {
    pub fn new(element: Node) -> Self {
        Self {
            color: element.attribute_ignore_ascii_case("color").map_or(
                Color::Simple {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                |v| v.parse().unwrap(),
            ),
            range: element
                .attribute_ignore_ascii_case("range")
                .map_or(100.0, |v| v.parse().unwrap()),
            scale: element
                .attribute_ignore_ascii_case("scale")
                .map_or(1.0, |v| v.parse().unwrap()),
            offset: element
                .attribute_ignore_ascii_case("offset")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            rotation: element
                .attribute_ignore_ascii_case("rotation")
                .map_or(0.0, |v| v.parse().unwrap()),
            directional: element
                .attribute_ignore_ascii_case("directional")
                .map_or(false, |v| v.parse().unwrap()),
            flicker: element
                .attribute_ignore_ascii_case("flicker")
                .map_or(0.0, |v| v.parse().unwrap()),
            flicker_speed: element
                .attribute_ignore_ascii_case("flickerspeed")
                .map_or(1.0, |v| v.parse().unwrap()),
            pulse_frequency: element
                .attribute_ignore_ascii_case("pulsefrequency")
                .map_or(0.0, |v| v.parse().unwrap()),
            pulse_amount: element
                .attribute_ignore_ascii_case("pulseamount")
                .map_or(0.0, |v| v.parse().unwrap()),
            blink_frequency: element
                .attribute_ignore_ascii_case("blinkfrequency")
                .map_or(0.0, |v| v.parse().unwrap()),
        }
    }
}

pub struct SoundConfig {
    pub position: Vec2,
    pub sound: RoundSound,
}

impl SoundConfig {
    pub fn new(element: Node) -> Self {
        let position = element
            .attribute_ignore_ascii_case("position")
            .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0);
        let sound = RoundSound::new(element);

        Self { position, sound }
    }
}

#[derive(Debug)]
pub struct RoundSound {
    pub filename: String,
    pub range: f32,
    pub volume: f32,
    pub freq_mult_range: Vec2,
    pub ignore_muffling: bool,
}

impl RoundSound {
    pub fn new(element: Node) -> Self {
        let filename = element
            .attribute_ignore_ascii_case("file")
            .or(element.attribute_ignore_ascii_case("sound"))
            .map(|v| v.to_string());
        //.unwrap();
        if filename.is_none() {
            dbg!(element);
        }
        let filename = filename.unwrap();
        let range = element
            .attribute_ignore_ascii_case("range")
            .map_or(1000.0, |v| v.parse::<f32>().unwrap());
        let volume = element
            .attribute_ignore_ascii_case("volume")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let freq_mult_range = element
            .attribute_ignore_ascii_case("frequencymultiplier")
            .or(element.attribute_ignore_ascii_case("frequency"))
            .map_or(Vec2::splat(1.0), |v| {
                if v.contains(',') {
                    v.parse::<Vector2>().unwrap().0
                } else {
                    Vec2::splat(v.parse::<f32>().unwrap())
                }
            });
        let ignore_muffling = element
            .attribute_ignore_ascii_case("dontmuffle")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());

        Self {
            filename,
            range,
            volume,
            freq_mult_range,
            ignore_muffling,
        }
    }
}

pub struct ParticleEmitterPrefabWithPosition {
    pub particle_emitter_prefab: ParticleEmitterPrefab,
    pub position: Vec2,
}

impl ParticleEmitterPrefabWithPosition {
    pub fn new(element: Node) -> Self {
        Self {
            particle_emitter_prefab: ParticleEmitterPrefab::new(element),
            position: element
                .attribute_ignore_ascii_case("position")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
        }
    }
}

#[derive(Debug)]
pub struct PhysicsBodyWithIsSensor {
    pub physics_body: PhysicsBody,
    pub is_sensor: bool,
}

impl PhysicsBodyWithIsSensor {
    pub fn new(element: Node) -> Self {
        Self {
            physics_body: PhysicsBody::new(element),
            is_sensor: element
                .attribute_ignore_ascii_case("sensor")
                .map_or(true, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]

pub struct RelatedItem {
    pub identifiers: Option<Vec<String>>,
    pub excluded_identifiers: Option<Vec<String>>,
    pub exclude_broken: bool,
    pub require_empty: bool,
    pub exclude_full_condition: bool,
    pub allow_variants: bool,
    pub rotation: f32,
    pub set_active: bool,
    pub character_inventory_slot_type: InvSlotType,
    pub hide: bool,
    pub item_pos: Vec2,
    pub relation_type: RelationType,
    pub msg_tag: String,
    pub is_optional: bool,
    pub ignore_in_editor: bool,
    pub match_on_empty: bool,
    pub target_slot: Option<u32>,
    pub status_effects: Vec<StatusEffect>,
}

impl RelatedItem {
    pub fn new(element: Node) -> Self {
        let identifiers = if element.has_attribute_ignore_ascii_case("name") {
            panic!()
            //this is backwards compatibility
        } else {
            element
                .attribute_ignore_ascii_case("items")
                .or(element.attribute_ignore_ascii_case("item"))
                .or(element.attribute_ignore_ascii_case("identifiers"))
                .or(element.attribute_ignore_ascii_case("tags"))
                .or(element.attribute_ignore_ascii_case("identifier"))
                .or(element.attribute_ignore_ascii_case("tag"))
                .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
        };
        let excluded_identifiers = element
            .attribute_ignore_ascii_case("excludeditems")
            .or(element.attribute_ignore_ascii_case("excludeditem"))
            .or(element.attribute_ignore_ascii_case("excludedidentifiers"))
            .or(element.attribute_ignore_ascii_case("excludedtags"))
            .or(element.attribute_ignore_ascii_case("excludedidentifier"))
            .or(element.attribute_ignore_ascii_case("excludedtag"))
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>());
        let exclude_broken = element
            .attribute_ignore_ascii_case("excludebroken")
            .map_or(true, |v| v.parse().unwrap());
        let require_empty = element
            .attribute_ignore_ascii_case("requireempty")
            .map_or(false, |v| v.parse().unwrap());
        let exclude_full_condition = element
            .attribute_ignore_ascii_case("excludefullcondition")
            .map_or(false, |v| v.parse().unwrap());
        let allow_variants = element
            .attribute_ignore_ascii_case("allowvariants")
            .map_or(true, |v| v.parse().unwrap());
        let rotation = element
            .attribute_ignore_ascii_case("rotation")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let set_active = element
            .attribute_ignore_ascii_case("setactive")
            .map_or(false, |v| v.parse().unwrap());
        let character_inventory_slot_type = element
            .attribute_ignore_ascii_case("characterinventoryslottype")
            .map_or(InvSlotType::new(), |v| v.parse().unwrap());
        let hide = element
            .attribute_ignore_ascii_case("hide")
            .map_or(false, |v| v.parse().unwrap());
        let item_pos = element
            .attribute_ignore_ascii_case("itempos")
            .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0);
        let relation_type = match element.attribute_ignore_ascii_case("type") {
            Some(v) => v.parse::<RelationType>().unwrap(),
            None => match element.tag_name().name().to_lowercase().as_str() {
                "containable" => RelationType::Contained,
                "suitablefertilizer" | "suitableseed" => RelationType::None,
                _ => panic!(),
            },
        };
        let msg_tag = element
            .attribute_ignore_ascii_case("msg")
            .map(|v| v.to_owned())
            .unwrap();
        let is_optional = element
            .attribute_ignore_ascii_case("optional")
            .map_or(false, |v| v.parse().unwrap());
        let ignore_in_editor = element
            .attribute_ignore_ascii_case("ignoreineditor")
            .map_or(false, |v| v.parse().unwrap());
        let match_on_empty = element
            .attribute_ignore_ascii_case("matchonempty")
            .map_or(false, |v| v.parse().unwrap());
        let target_slot = element
            .attribute_ignore_ascii_case("optional")
            .and_then(|v| {
                let v = v.parse::<i32>().unwrap();
                if v <= -1 { None } else { Some(v as u32) }
            });
        let status_effects = element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("statuseffect"))
            .map(|child| StatusEffect::new(child))
            .collect::<Vec<_>>();

        Self {
            identifiers,
            excluded_identifiers,
            exclude_broken,
            require_empty,
            exclude_full_condition,
            allow_variants,
            rotation,
            set_active,
            character_inventory_slot_type,
            hide,
            item_pos,
            relation_type,
            msg_tag,
            is_optional,
            ignore_in_editor,
            match_on_empty,
            target_slot,
            status_effects,
        }
    }
}

#[bitfield(u16)]

pub struct InvSlotType {
    pub any: bool,
    pub right_hand: bool,
    pub left_hand: bool,
    pub head: bool,
    pub inner_clothes: bool,
    pub outer_clothes: bool,
    pub headset: bool,
    pub card: bool,
    pub bag: bool,
    pub health_interface: bool,
    #[bits(6)]
    _unused: u8,
}

impl FromStr for InvSlotType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::new()),
            "Any" => Ok(Self::new().with_any(true)),
            "RightHand" => Ok(Self::new().with_right_hand(true)),
            "LeftHand" => Ok(Self::new().with_left_hand(true)),
            "Head" => Ok(Self::new().with_head(true)),
            "InnerClothes" => Ok(Self::new().with_inner_clothes(true)),
            "OuterClothes" => Ok(Self::new().with_outer_clothes(true)),
            "Headset" => Ok(Self::new().with_headset(true)),
            "Card" => Ok(Self::new().with_card(true)),
            "Bag" => Ok(Self::new().with_bag(true)),
            "HealthInterface" => Ok(Self::new().with_health_interface(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum RelationType {
    None,
    Contained,
    Equipped,
    Picked,
    Container,
}

impl FromStr for RelationType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Contained" => Ok(Self::Contained),
            "Equipped" => Ok(Self::Equipped),
            "Picked" => Ok(Self::Picked),
            "Container" => Ok(Self::Container),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct ItemSpawnInfo {
    pub item_identifier: String,
    pub spawn_if_inventory_full: bool,
    pub spawn_if_not_in_inventory: bool,
    pub spawn_if_cant_be_contained: bool,
    pub impulse: f32,
    pub condition: f32,
    pub rotation_rad: f32,
    pub count: u32,
    pub spread: f32,
    pub aim_spread_rad: f32,
    pub equip: bool,
    pub spawn_position: SpawnPositionType,
    pub rotation_type: SpawnRotationType,
    pub inherit_event_tags: bool,
}

impl ItemSpawnInfo {
    pub fn new(element: Node) -> Self {
        let item_identifier = if element.has_attribute_ignore_ascii_case("name") {
            panic!()
            //backwards compatibility
        } else {
            element
                .attribute_ignore_ascii_case("identifier")
                .or(element.attribute_ignore_ascii_case("identifiers"))
                .map(|v| v.to_owned())
                .unwrap()
        };
        let spawn_if_inventory_full = element
            .attribute_ignore_ascii_case("spawnifinventoryfull")
            .map_or(false, |v| v.parse().unwrap());
        let spawn_if_not_in_inventory = element
            .attribute_ignore_ascii_case("spawnifnotininventory")
            .map_or(false, |v| v.parse().unwrap());
        let spawn_if_cant_be_contained = element
            .attribute_ignore_ascii_case("spawnifcantbecontained")
            .map_or(true, |v| v.parse().unwrap());
        let impulse = element
            .attribute_ignore_ascii_case("impulse")
            .or(element.attribute_ignore_ascii_case("launchimpulse"))
            .or(element.attribute_ignore_ascii_case("speed"))
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let condition = element
            .attribute_ignore_ascii_case("condition")
            .map_or(1.0, |v| v.parse::<f32>().unwrap().clamp(0.0, 1.0));
        let rotation_rad = element
            .attribute_ignore_ascii_case("rotation")
            .map_or(0.0, |v| v.parse::<f32>().unwrap().to_radians());
        let count = element
            .attribute_ignore_ascii_case("count")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let spread = element
            .attribute_ignore_ascii_case("spread")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let aim_spread_rad = element
            .attribute_ignore_ascii_case("aimspread")
            .map_or(0.0, |v| v.parse::<f32>().unwrap().to_radians());
        let equip = element
            .attribute_ignore_ascii_case("equip")
            .map_or(false, |v| v.parse().unwrap());
        let spawn_position = element
            .attribute_ignore_ascii_case("spawnposition")
            .map_or(SpawnPositionType::This, |v| v.parse().unwrap());
        let rotation_type = element.attribute_ignore_ascii_case("rotationtype").map_or(
            if rotation_rad != 0.0 {
                SpawnRotationType::Fixed
            } else {
                SpawnRotationType::Target
            },
            |v| v.parse().unwrap(),
        );
        let inherit_event_tags = element
            .attribute_ignore_ascii_case("inheriteventtags")
            .map_or(false, |v| v.parse().unwrap());

        Self {
            item_identifier,
            spawn_if_inventory_full,
            spawn_if_not_in_inventory,
            spawn_if_cant_be_contained,
            impulse,
            condition,
            rotation_rad,
            count,
            spread,
            aim_spread_rad,
            equip,
            spawn_position,
            rotation_type,
            inherit_event_tags,
        }
    }
}

#[derive(Debug)]
pub enum SpawnPositionType {
    This,
    ThisInventory,
    SameInventory,
    ContainedInventory,
}

impl FromStr for SpawnPositionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "This" => Ok(Self::This),
            "ThisInventory" => Ok(Self::ThisInventory),
            "SameInventory" => Ok(Self::SameInventory),
            "ContainedInventory" => Ok(Self::ContainedInventory),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum SpawnRotationType {
    Fixed,
    Target,
    Limb,
    MainLimb,
    Collider,
    Random,
}

impl FromStr for SpawnRotationType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Fixed" => Ok(Self::Fixed),
            "Target" => Ok(Self::Target),
            "Limb" => Ok(Self::Limb),
            "MainLimb" => Ok(Self::MainLimb),
            "Collider" => Ok(Self::Collider),
            "Random" => Ok(Self::Random),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct CharacterSpawnInfo {
    pub species_name: String,
    pub count: u32,
    pub transfer_buffs: bool,
    pub transfer_afflictions: bool,
    pub transfer_inventory: bool,
    pub total_max_count: u32,
    pub stun: u32,
    pub affliction_on_spawn: Option<String>,
    pub affliction_strength: u32,
    pub transfer_control: bool,
    pub remove_previous_character: bool,
    pub spread: f32,
    pub offset: Vec2,
    pub inherit_event_tags: bool,
}

impl CharacterSpawnInfo {
    pub fn new(element: Node) -> Self {
        Self {
            species_name: element
                .attribute_ignore_ascii_case("speciesname")
                .map(|v| v.to_owned())
                .unwrap(),
            count: element
                .attribute_ignore_ascii_case("count")
                .map_or(1, |v| v.parse().unwrap()),
            transfer_buffs: element
                .attribute_ignore_ascii_case("transferbuffs")
                .map_or(false, |v| v.parse().unwrap()),
            transfer_afflictions: element
                .attribute_ignore_ascii_case("transferafflictions")
                .map_or(false, |v| v.parse().unwrap()),
            transfer_inventory: element
                .attribute_ignore_ascii_case("transferinventory")
                .map_or(false, |v| v.parse().unwrap()),
            total_max_count: element
                .attribute_ignore_ascii_case("totalmaxcount")
                .map_or(0, |v| v.parse().unwrap()),
            stun: element
                .attribute_ignore_ascii_case("stun")
                .map_or(0, |v| v.parse().unwrap()),
            affliction_on_spawn: element
                .attribute_ignore_ascii_case("afflictiononspawn")
                .map(|v| v.parse().unwrap()),
            affliction_strength: element
                .attribute_ignore_ascii_case("afflictionstrength")
                .map_or(1, |v| v.parse().unwrap()),
            transfer_control: element
                .attribute_ignore_ascii_case("transfercontrol")
                .map_or(false, |v| v.parse().unwrap()),
            remove_previous_character: element
                .attribute_ignore_ascii_case("removepreviouscharacter")
                .map_or(false, |v| v.parse().unwrap()),
            spread: element
                .attribute_ignore_ascii_case("spread")
                .map_or(0.0, |v| v.parse().unwrap()),
            offset: element
                .attribute_ignore_ascii_case("offset")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            inherit_event_tags: element
                .attribute_ignore_ascii_case("inheriteventtags")
                .map_or(false, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct GiveTalentInfo {
    pub talent_identifiers: Vec<String>,
    pub give_random: bool,
}

impl GiveTalentInfo {
    pub fn new(element: Node) -> Self {
        Self {
            talent_identifiers: element
                .attribute_ignore_ascii_case("talentidentifiers")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap(),
            give_random: element
                .attribute_ignore_ascii_case("giverandom")
                .map_or(false, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct GiveSkill {
    pub skill_identifier: String,
    pub amount: f32,
    pub trigger_talents: bool,
}

impl GiveSkill {
    pub fn new(element: Node) -> Self {
        Self {
            skill_identifier: element
                .attribute_ignore_ascii_case("skillidentifier")
                .map(|v| v.to_owned())
                .unwrap(),
            amount: element
                .attribute_ignore_ascii_case("amount")
                .map_or(0.0, |v| v.parse().unwrap()),
            trigger_talents: element
                .attribute_ignore_ascii_case("triggertalents")
                .map_or(false, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct AITrigger {
    pub state: AIState,
    pub duration: f32,
    pub probability: f32,
    pub min_damage: f32,
    pub allow_to_override: bool,
    pub allow_to_be_overriden: bool,
}

impl AITrigger {
    pub fn new(element: Node) -> Self {
        Self {
            state: element
                .attribute_ignore_ascii_case("state")
                .map_or(AIState::Idle, |v| v.parse().unwrap()),
            duration: element
                .attribute_ignore_ascii_case("duration")
                .map_or(0.0, |v| v.parse().unwrap()),
            probability: element
                .attribute_ignore_ascii_case("probability")
                .map_or(1.0, |v| v.parse().unwrap()),
            min_damage: element
                .attribute_ignore_ascii_case("mindamage")
                .map_or(0.0, |v| v.parse().unwrap()),
            allow_to_override: element
                .attribute_ignore_ascii_case("allowtooverride")
                .map_or(true, |v| v.parse().unwrap()),
            allow_to_be_overriden: element
                .attribute_ignore_ascii_case("allowtobeoverriden")
                .map_or(true, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub enum AIState {
    Idle,
    Attack,
    Escape,
    Eat,
    Flee,
    Avoid,
    Aggressive,
    PassiveAggressive,
    Protect,
    Observe,
    Freeze,
    Follow,
    FleeTo,
    Patrol,
}

impl FromStr for AIState {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Idle" => Ok(Self::Idle),
            "Attack" => Ok(Self::Attack),
            "Escape" => Ok(Self::Escape),
            "Eat" => Ok(Self::Eat),
            "Flee" => Ok(Self::Flee),
            "Avoid" => Ok(Self::Avoid),
            "Aggressive" => Ok(Self::Aggressive),
            "PassiveAggressive" => Ok(Self::PassiveAggressive),
            "Protect" => Ok(Self::Protect),
            "Observe" => Ok(Self::Observe),
            "Freeze" => Ok(Self::Freeze),
            "Follow" => Ok(Self::Follow),
            "FleeTo" => Ok(Self::FleeTo),
            "Patrol" => Ok(Self::Patrol),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct GiveAffliction {
    pub affliction_identifier: String,
    pub strength: f32,
    pub probability: f32,
}

#[derive(Debug)]
pub struct ReduceAffliction {
    pub affliction: AfflictionIdentifierOrType,
    pub strength: f32,
}

#[derive(Debug)]
pub enum AfflictionIdentifierOrType {
    Identifier(String),
    Type(String),
}

#[derive(Debug)]
pub enum SoundSelectionMode {
    Random,
    CharacterSpecific,
    ItemSpecific,
    All,
    Manual,
}

impl FromStr for SoundSelectionMode {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "random" => Ok(Self::Random),
            "characterspecific" => Ok(Self::CharacterSpecific),
            "itemspecific" => Ok(Self::ItemSpecific),
            "all" => Ok(Self::All),
            "manual" => Ok(Self::Manual),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct AfflictionProperties {
    pub strength: f32,
    pub identifier: String,
    pub probability: f32,
    pub divide_by_limb_count: bool,
    pub multiply_by_max_vitality: bool,
}

impl AfflictionProperties {
    pub fn new(element: Node) -> Self {
        Self {
            strength: element
                .attribute_ignore_ascii_case("strength")
                .map_or(0.0, |v| v.parse().unwrap()),
            identifier: element
                .attribute_ignore_ascii_case("identifier")
                .map(|v| v.to_owned())
                .unwrap(),
            probability: element
                .attribute_ignore_ascii_case("probability")
                .map_or(1.0, |v| v.parse().unwrap()),
            divide_by_limb_count: element
                .attribute_ignore_ascii_case("dividebylimbcount")
                .map_or(true, |v| v.parse().unwrap()),
            multiply_by_max_vitality: element
                .attribute_ignore_ascii_case("multiplybymaxvitality")
                .map_or(false, |v| v.parse().unwrap()),
        }
    }
}
