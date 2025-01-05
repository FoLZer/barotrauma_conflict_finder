use std::str::FromStr;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::{
    gui_style_prefabs::SpriteSheet,
    item_prefab::{BarotraumaSprite, Color, DoesNotExistError},
    particle_emitter_prefab::ParticleEmitterPrefab,
};

#[derive(Debug)]
pub struct ParticlePrefab {
    pub identifier: String,
    pub life_time: f32,
    pub life_time_min: f32,
    pub start_delay_min: f32,
    pub start_delay_max: f32,
    pub angular_velocity_min: f32,
    pub angular_velocity_max: f32,
    pub start_rotation_min: f32,
    pub start_rotation_max: f32,
    pub rotate_to_direction: bool,
    pub drag: f32,
    pub water_drag: f32,
    pub velocity_change: Vec2,
    pub velocity_change_water: Vec2,
    pub collision_radius: f32,
    pub invariant_collision_size: bool,
    pub use_collision: bool,
    pub delete_on_collision: bool,
    pub friction: f32,
    pub restitution: f32,
    pub start_size_min: Vec2,
    pub start_size_max: Vec2,
    pub size_change_min: Vec2,
    pub size_change_max: Vec2,
    pub grow_time: f32,
    pub start_color: Color,
    pub middle_color: Color,
    pub end_color: Color,
    pub use_middle_color: bool,
    pub draw_target: DrawTargetType,
    pub draw_on_top: bool,
    pub draw_always: bool,
    pub blend_state: ParticleBlendState,
    pub priority: i32,
    pub anim_duration: f32,
    pub loop_anim: bool,
    pub sprites: Vec<BarotraumaSprite>,
    pub sprite_sheets: Vec<SpriteSheet>,
    pub sub_emitters: Vec<ParticleEmitterPrefab>,
}

impl ParticlePrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element.tag_name().name().to_lowercase();
        let life_time = element
            .attribute_ignore_ascii_case("lifetime")
            .map_or(5.0, |v| v.parse::<f32>().unwrap());
        let life_time_min = element
            .attribute_ignore_ascii_case("lifetimemin")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let start_delay_min = element
            .attribute_ignore_ascii_case("startdelaymin")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let start_delay_max = element
            .attribute_ignore_ascii_case("startdelaymax")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let angular_velocity = element
            .attribute_ignore_ascii_case("angularvelocity")
            .map(|v| v.parse::<f32>().unwrap()); //
        let angular_velocity_min = element
            .attribute_ignore_ascii_case("angularvelocitymin")
            .map_or(angular_velocity.unwrap_or(0.0), |v| {
                v.parse::<f32>().unwrap()
            });
        let angular_velocity_max = element
            .attribute_ignore_ascii_case("angularvelocitymax")
            .map_or(angular_velocity.unwrap_or(0.0), |v| {
                v.parse::<f32>().unwrap()
            });
        let start_rotation = element
            .attribute_ignore_ascii_case("startrotation")
            .map(|v| v.parse::<f32>().unwrap()); //
        let start_rotation_min = element
            .attribute_ignore_ascii_case("startrotationmin")
            .map_or(start_rotation.unwrap_or(0.0), |v| v.parse::<f32>().unwrap());
        let start_rotation_max = element
            .attribute_ignore_ascii_case("startrotationmax")
            .map_or(start_rotation.unwrap_or(0.0), |v| v.parse::<f32>().unwrap());
        let rotate_to_direction = element
            .attribute_ignore_ascii_case("rotatetodirection")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
        let drag = element
            .attribute_ignore_ascii_case("drag")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let water_drag = element
            .attribute_ignore_ascii_case("waterdrag")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let velocity_change = element
            .attribute_ignore_ascii_case("velocitychange")
            .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0);
        let velocity_change_water = element
            .attribute_ignore_ascii_case("velocitychangewater")
            .map_or(velocity_change, |v| v.parse::<Vector2>().unwrap().0);
        let collision_radius = element
            .attribute_ignore_ascii_case("collisionradius")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let invariant_collision_size = element
            .attribute_ignore_ascii_case("invariantcollisionsize")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
        let use_collision = element
            .attribute_ignore_ascii_case("usecollision")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
        let delete_on_collision = element
            .attribute_ignore_ascii_case("deleteoncollision")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
        let friction = element
            .attribute_ignore_ascii_case("friction")
            .map_or(0.5, |v| v.parse::<f32>().unwrap());
        let restitution = element
            .attribute_ignore_ascii_case("restitution")
            .map_or(0.5, |v| v.parse::<f32>().unwrap());
        let start_size = element
            .attribute_ignore_ascii_case("startsize")
            .map(|v| v.parse::<Vector2>().unwrap().0); //
        let start_size_min = element
            .attribute_ignore_ascii_case("startsizemin")
            .map_or(start_size.unwrap_or(Vec2::ONE), |v| {
                v.parse::<Vector2>().unwrap().0
            });
        let start_size_max = element
            .attribute_ignore_ascii_case("startsizemax")
            .map_or(start_size.unwrap_or(Vec2::ONE), |v| {
                v.parse::<Vector2>().unwrap().0
            });
        let size_change = element
            .attribute_ignore_ascii_case("sizechange")
            .map(|v| v.parse::<Vector2>().unwrap().0); //
        let size_change_min = element
            .attribute_ignore_ascii_case("sizechangemin")
            .map_or(size_change.unwrap_or(Vec2::ZERO), |v| {
                v.parse::<Vector2>().unwrap().0
            });
        let size_change_max = element
            .attribute_ignore_ascii_case("sizechangemax")
            .map_or(size_change.unwrap_or(Vec2::ZERO), |v| {
                v.parse::<Vector2>().unwrap().0
            });
        let grow_time = element
            .attribute_ignore_ascii_case("growtime")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let start_color = element.attribute_ignore_ascii_case("startcolor").map_or(
            Color::Simple {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            |v| v.parse::<Color>().unwrap(),
        );
        let middle_color = element.attribute_ignore_ascii_case("middlecolor").map_or(
            Color::Simple {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            |v| v.parse::<Color>().unwrap(),
        );
        let end_color = element.attribute_ignore_ascii_case("endcolor").map_or(
            Color::Simple {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            |v| v.parse::<Color>().unwrap(),
        );
        let use_middle_color = element
            .attribute_ignore_ascii_case("usemiddlecolor")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
        let draw_target = element
            .attribute_ignore_ascii_case("drawtarget")
            .map_or(DrawTargetType::Air, |v| {
                v.parse::<DrawTargetType>().unwrap()
            });
        let draw_on_top = element
            .attribute_ignore_ascii_case("drawontop")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
        let draw_always = element
            .attribute_ignore_ascii_case("drawalways")
            .map_or(false, |v| v.to_lowercase().parse::<bool>().unwrap());
        let blend_state = element
            .attribute_ignore_ascii_case("blendstate")
            .map_or(ParticleBlendState::AlphaBlend, |v| {
                v.parse::<ParticleBlendState>().unwrap()
            });
        let priority = element
            .attribute_ignore_ascii_case("priority")
            .map_or(0, |v| v.parse::<i32>().unwrap());
        let anim_duration = element
            .attribute_ignore_ascii_case("animduration")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let loop_anim = element
            .attribute_ignore_ascii_case("loopanim")
            .map_or(true, |v| v.to_lowercase().parse::<bool>().unwrap());

        let mut sprites = Vec::new();
        let mut sprite_sheets = Vec::new();
        let mut sub_emitters = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "sprite" => {
                    sprites.push(BarotraumaSprite::new(child));
                }
                "spritesheet" | "animatedsprite" => {
                    sprite_sheets.push(SpriteSheet::new(child));
                }
                "particleemitter" | "emitter" | "subemitter" => {
                    sub_emitters.push(ParticleEmitterPrefab::new(child));
                }
                _ => (),
            }
        }

        Self {
            identifier,
            life_time,
            life_time_min,
            start_delay_min,
            start_delay_max,
            angular_velocity_min,
            angular_velocity_max,
            start_rotation_min,
            start_rotation_max,
            rotate_to_direction,
            drag,
            water_drag,
            velocity_change,
            velocity_change_water,
            collision_radius,
            invariant_collision_size,
            use_collision,
            delete_on_collision,
            friction,
            restitution,
            start_size_min,
            start_size_max,
            size_change_min,
            size_change_max,
            grow_time,
            start_color,
            middle_color,
            end_color,
            use_middle_color,
            draw_target,
            draw_on_top,
            draw_always,
            blend_state,
            priority,
            anim_duration,
            loop_anim,
            sprites,
            sprite_sheets,
            sub_emitters,
        }
    }
}

#[derive(Debug)]
pub enum DrawTargetType {
    Air,
    Water,
    Both,
}
impl FromStr for DrawTargetType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Air" => Ok(Self::Air),
            "Water" => Ok(Self::Water),
            "Both" => Ok(Self::Both),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum ParticleBlendState {
    AlphaBlend,
    Additive,
}
impl FromStr for ParticleBlendState {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AlphaBlend" => Ok(Self::AlphaBlend),
            "Additive" => Ok(Self::Additive),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
