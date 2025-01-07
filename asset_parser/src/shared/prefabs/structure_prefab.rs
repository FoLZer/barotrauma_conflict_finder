use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use bitfield_struct::bitfield;
use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::{
    item_prefab::{
        BarotraumaSprite, Color, DoesNotExistError, MapEntityCategory, MapEntityProperties,
    },
    level_object_prefab::PropertyConditional,
};

#[derive(Debug)]
pub struct StructurePrefab {
    pub identifier: String,
    pub original_name: Option<String>,
    pub name_identifier: Option<String>,
    pub fallback_name_identifier: Option<String>,
    pub tags: HashSet<String>,
    pub is_horizontal: bool,
    pub sprite: BarotraumaSprite,
    pub sprite_effects: SpriteEffects,
    pub can_sprite_flip_x: bool,
    pub can_sprite_flip_y: bool,
    pub background_sprite: Option<BarotraumaSprite>,
    pub background_sprite_effects: SpriteEffects,
    pub background_sprite_color: Color,
    pub decorative_sprite_groups: HashMap<u32, Option<DecorativeSprite>>,
    pub properties: StructurePrefabProperties,
    pub category: MapEntityCategory,
    pub aliases: Vec<String>,
    pub description_identifier: Option<String>,
}

impl StructurePrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let original_name = element
            .attribute_ignore_ascii_case("name")
            .map(|v| v.to_owned());
        //TODO: ConfigElement
        let name_identifier = element
            .attribute_ignore_ascii_case("nameidentifier")
            .map(|v| v.to_owned());
        let fallback_name_identifier = element
            .attribute_ignore_ascii_case("fallbacknameidentifier")
            .map(|v| v.to_owned());
        let mut tags = element
            .attribute_ignore_ascii_case("tags")
            .map_or(Default::default(), |v| {
                v.split(',').map(|v| v.to_owned()).collect::<HashSet<_>>()
            });
        let is_horizontal = element
            .attribute_ignore_ascii_case("fallbacknameidentifier")
            .map_or(false, |v| v.parse().unwrap());

        let mut sprite = None;
        let mut sprite_effects = SpriteEffects::new();
        let mut can_sprite_flip_x = true;
        let mut can_sprite_flip_y = true;
        let mut background_sprite = None;
        let mut background_sprite_effects = SpriteEffects::new();
        let mut background_sprite_color = Color::Simple {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };
        let mut decorative_sprite_groups = HashMap::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "sprite" => {
                    sprite = Some(BarotraumaSprite::new(child));

                    if child
                        .attribute_ignore_ascii_case("flipvertical")
                        .map_or(false, |v| v.parse().unwrap())
                    {
                        sprite_effects = SpriteEffects::new().with_flip_vertically(true)
                    } else if child
                        .attribute_ignore_ascii_case("fliphorizontal")
                        .map_or(false, |v| v.parse().unwrap())
                    {
                        sprite_effects = SpriteEffects::new().with_flip_horizontally(true)
                    } else {
                        sprite_effects = SpriteEffects::new();
                    }

                    can_sprite_flip_x = child
                        .attribute_ignore_ascii_case("canflipx")
                        .map_or(true, |v| v.parse().unwrap());
                    can_sprite_flip_y = child
                        .attribute_ignore_ascii_case("canflipy")
                        .map_or(true, |v| v.parse().unwrap())
                }
                "backgroundsprite" => {
                    background_sprite = Some(BarotraumaSprite::new(child));
                    if child
                        .attribute_ignore_ascii_case("flipvertical")
                        .map_or(false, |v| v.parse().unwrap())
                    {
                        background_sprite_effects = SpriteEffects::new().with_flip_vertically(true)
                    } else if child
                        .attribute_ignore_ascii_case("fliphorizontal")
                        .map_or(false, |v| v.parse().unwrap())
                    {
                        background_sprite_effects =
                            SpriteEffects::new().with_flip_horizontally(true)
                    } else {
                        background_sprite_effects = SpriteEffects::new();
                    }
                    background_sprite_color = child.attribute_ignore_ascii_case("color").map_or(
                        Color::Simple {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        },
                        |v| v.parse().unwrap(),
                    )
                }
                "decorativesprite" => {
                    let (group_id, decorative_sprite) = if child.has_attribute("texture") {
                        let sprite = DecorativeSprite::new(child);
                        (sprite.properties.random_group_id, Some(sprite))
                    } else {
                        (
                            child
                                .attribute_ignore_ascii_case("randomgroupid")
                                .map_or(0, |v| v.parse::<u32>().unwrap()),
                            None,
                        )
                    };
                    decorative_sprite_groups.insert(group_id, decorative_sprite);
                }
                _ => (),
            }
        }
        let sprite = sprite.unwrap();

        let mut properties = StructurePrefabProperties::new(element);
        let category = element.attribute_ignore_ascii_case("category").map_or(
            MapEntityCategory::new().with_structure(true),
            |v| {
                if v.eq_ignore_ascii_case("Thalamus") {
                    //backwards compatibility
                    properties.map_entity_properties.subcategory = "Thalamus".to_owned();
                    MapEntityCategory::new().with_wrecked(true)
                } else {
                    v.split(',')
                        .map(|v| v.parse::<MapEntityCategory>().unwrap())
                        .fold(MapEntityCategory::new(), |acc, e| {
                            MapEntityCategory::from_bits(acc.into_bits() | e.into_bits())
                        })
                }
            },
        );
        let aliases = {
            let mut v = element
                .attribute_ignore_ascii_case("aliases")
                .map_or(Vec::new(), |v| {
                    v.split(',').map(|v| v.to_owned()).collect::<Vec<_>>()
                });

            let non_translated_name = element
                .attribute_ignore_ascii_case("name")
                .map_or(element.tag_name().name().to_owned(), |v| v.to_owned());
            v.push(non_translated_name);
            v
        };
        if properties.body {
            tags.insert("wall".to_owned());
        }
        let description_identifier = element
            .attribute_ignore_ascii_case("descriptionidentifier")
            .map(|v| v.to_owned());
        if !element.has_attribute_ignore_ascii_case("size") {
            //backwards compatibility
            if !element.has_attribute_ignore_ascii_case("width")
                && !element.has_attribute_ignore_ascii_case("height")
            {
                let source_rect = sprite.source_rect.as_ref().unwrap();
                properties.size = Vec2::new(source_rect.width as f32, source_rect.height as f32);
            } else {
                properties.size = Vec2::new(
                    element
                        .attribute_ignore_ascii_case("width")
                        .map_or(0.0, |v| v.parse().unwrap()),
                    element
                        .attribute_ignore_ascii_case("height")
                        .map_or(0.0, |v| v.trim().parse().unwrap()),
                )
            }
        }

        Self {
            identifier,
            original_name,
            name_identifier,
            fallback_name_identifier,
            tags,
            is_horizontal,
            sprite,
            sprite_effects,
            can_sprite_flip_x,
            can_sprite_flip_y,
            background_sprite,
            background_sprite_effects,
            background_sprite_color,
            decorative_sprite_groups,
            properties,
            category,
            aliases,
            description_identifier,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct StructurePrefabProperties {
    pub map_entity_properties: MapEntityProperties,
    pub body: bool,
    pub body_rotation: f32,
    pub body_width: f32,
    pub body_height: f32,
    pub body_offset: Vec2,
    pub platform: bool,
    pub allow_attach_items: bool,
    pub min_health: f32,
    pub health: f32,
    pub indestructible_in_outposts: bool,
    pub cast_shadow: bool,
    pub stair_direction: Direction,
    pub stair_angle: f32,
    pub no_ai_target: bool,
    pub size: Vec2,
    pub damage_sound: Option<String>,
    pub damage_particle: String,
    pub texture_scale: Vec2,
}

impl StructurePrefabProperties {
    pub fn new(element: Node) -> Self {
        Self {
            map_entity_properties: MapEntityProperties::new(element),
            body: element
                .attribute_ignore_ascii_case("body")
                .map_or(false, |v| v.parse().unwrap()),
            body_rotation: element
                .attribute_ignore_ascii_case("bodyrotation")
                .map_or(0.0, |v| v.parse().unwrap()),
            body_width: element
                .attribute_ignore_ascii_case("bodywidth")
                .map_or(0.0, |v| v.parse().unwrap()),
            body_height: element
                .attribute_ignore_ascii_case("bodyheight")
                .map_or(0.0, |v| v.parse().unwrap()),
            body_offset: element
                .attribute_ignore_ascii_case("bodyoffset")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            platform: element
                .attribute_ignore_ascii_case("platform")
                .map_or(false, |v| v.parse().unwrap()),
            allow_attach_items: element
                .attribute_ignore_ascii_case("allowattachitems")
                .map_or(false, |v| v.parse().unwrap()),
            min_health: element
                .attribute_ignore_ascii_case("minhealth")
                .map_or(0.0, |v| v.parse().unwrap()),
            health: element
                .attribute_ignore_ascii_case("health")
                .map_or(100.0, |v| v.parse().unwrap()),
            indestructible_in_outposts: element
                .attribute_ignore_ascii_case("indestructibleinoutposts")
                .map_or(true, |v| v.parse().unwrap()),
            cast_shadow: element
                .attribute_ignore_ascii_case("castshadow")
                .map_or(false, |v| v.parse().unwrap()),
            stair_direction: element
                .attribute_ignore_ascii_case("stairdirection")
                .map_or(Direction::None, |v| v.parse().unwrap()),
            stair_angle: element
                .attribute_ignore_ascii_case("stairangle")
                .map_or(45.0, |v| v.parse().unwrap()),
            no_ai_target: element
                .attribute_ignore_ascii_case("noaitarget")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            size: element
                .attribute_ignore_ascii_case("size")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            damage_sound: element
                .attribute_ignore_ascii_case("damagesound")
                .map(|v| v.to_owned()),
            damage_particle: element
                .attribute_ignore_ascii_case("damageparticle")
                .map_or("shrapnel".to_owned(), |v| v.to_owned()),
            texture_scale: element
                .attribute_ignore_ascii_case("texturescale")
                .map_or(Vec2::ONE, |v| v.parse::<Vector2>().unwrap().0),
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    None,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Left" => Ok(Self::Left),
            "Right" => Ok(Self::Right),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[bitfield(u8)]

pub struct SpriteEffects {
    pub flip_horizontally: bool,
    pub flip_vertically: bool,
    #[bits(6)]
    _unused: u8,
}

impl FromStr for SpriteEffects {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::new()),
            "FlipHorizontally" => Ok(Self::new().with_flip_horizontally(true)),
            "FlipVertically" => Ok(Self::new().with_flip_vertically(true)),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct DecorativeSprite {
    pub sprite: BarotraumaSprite,
    pub properties: DecorativeSpriteProperties,
    pub is_active_conditionals: Vec<PropertyConditional>,
    pub animation_conditionals: Vec<PropertyConditional>,
}

impl DecorativeSprite {
    pub fn new(element: Node) -> Self {
        let sprite = BarotraumaSprite::new(element);
        let properties = DecorativeSpriteProperties::new(element);

        let mut is_active_conditionals = Vec::new();
        let mut animation_conditionals = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "conditional" | "isactiveconditional" => {
                    is_active_conditionals.push(PropertyConditional::from_xml(child));
                }
                "animationconditional" => {
                    animation_conditionals.push(PropertyConditional::from_xml(child));
                }
                _ => (),
            }
        }

        Self {
            sprite,
            properties,
            is_active_conditionals,
            animation_conditionals,
        }
    }
}

#[derive(Debug)]
pub struct DecorativeSpriteProperties {
    pub offset: Vec2,
    pub random_offset: Vec2,
    pub offset_anim: AnimationType,
    pub offset_anim_speed: f32,
    pub rotation_speed: f32,
    pub rotation: f32,
    pub random_rotation: Vec2,
    pub scale: f32,
    pub random_scale: Vec2,
    pub rotation_anim: AnimationType,
    pub random_group_id: u32,
    pub color: Color,
}

impl DecorativeSpriteProperties {
    pub fn new(element: Node) -> Self {
        Self {
            offset: element
                .attribute_ignore_ascii_case("offset")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            random_offset: element
                .attribute_ignore_ascii_case("randomoffset")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            offset_anim: element
                .attribute_ignore_ascii_case("offsetanim")
                .map_or(AnimationType::None, |v| v.parse().unwrap()),
            offset_anim_speed: element
                .attribute_ignore_ascii_case("offsetanimspeed")
                .map_or(0.0, |v| v.parse().unwrap()),
            rotation_speed: element
                .attribute_ignore_ascii_case("rotationspeed")
                .map_or(0.0, |v| v.parse().unwrap()),
            rotation: element
                .attribute_ignore_ascii_case("rotation")
                .map_or(0.0, |v| v.parse().unwrap()),
            random_rotation: element
                .attribute_ignore_ascii_case("randomrotation")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            scale: element
                .attribute_ignore_ascii_case("scale")
                .map_or(1.0, |v| v.parse().unwrap()),
            random_scale: element
                .attribute_ignore_ascii_case("randomscale")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            rotation_anim: element
                .attribute_ignore_ascii_case("rotationanim")
                .map_or(AnimationType::None, |v| v.parse().unwrap()),
            random_group_id: element
                .attribute_ignore_ascii_case("randomgroupid")
                .map_or(0, |v| v.parse().unwrap()),
            color: element.attribute_ignore_ascii_case("color").map_or(
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

#[derive(Debug)]
pub enum AnimationType {
    None,
    Sine,
    Noise,
}

impl FromStr for AnimationType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Sine" => Ok(Self::Sine),
            "Noise" => Ok(Self::Noise),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
