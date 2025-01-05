use std::{collections::HashMap, str::FromStr};

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::{
    item_assembly_prefab::Rect,
    item_prefab::{BarotraumaSprite, Color, DoesNotExistError},
    particle_emitter_prefab::ParticleEmitterPrefab,
};

#[derive(Debug)]
pub struct BallastFloraPrefab {
    pub identifier: String,
    pub name: String,
    pub properties: BallastFloraProperties,
    pub branch_atlas: Option<String>,
    pub decay_atlas: Option<String>,
    pub branch_sprites: HashMap<VineTileType, VineSprite>,
    pub flower_sprites: Vec<BarotraumaSprite>,
    pub damaged_flower_sprites: Vec<BarotraumaSprite>,
    pub hidden_flower_sprites: Vec<BarotraumaSprite>,
    pub leaf_sprites: Vec<BarotraumaSprite>,
    pub damaged_leaf_sprites: Vec<BarotraumaSprite>,
    pub damage_particles: Vec<ParticleEmitterPrefab>,
    pub death_particles: Vec<ParticleEmitterPrefab>,
    pub targets: Vec<AITarget>,
}

impl BallastFloraPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let name = element
            .attribute_ignore_ascii_case("name")
            .unwrap()
            .to_owned();
        let properties = BallastFloraProperties::new(element);
        let branch_atlas = element
            .attribute_ignore_ascii_case("branchatlas")
            .map(|v| v.to_owned());
        let decay_atlas = element
            .attribute_ignore_ascii_case("decayatlas")
            .map(|v| v.to_owned());

        let mut branch_sprites = HashMap::new();
        let mut flower_sprites = Vec::new();
        let mut damaged_flower_sprites = Vec::new();
        let mut hidden_flower_sprites = Vec::new();
        let mut leaf_sprites = Vec::new();
        let mut damaged_leaf_sprites = Vec::new();
        let mut damage_particles = Vec::new();
        let mut death_particles = Vec::new();
        let mut targets = Vec::new();

        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "branchsprite" => {
                    let ty = child
                        .attribute_ignore_ascii_case("type")
                        .map_or(VineTileType::Stem, |v| v.parse::<VineTileType>().unwrap());
                    branch_sprites.insert(ty, VineSprite::new(element));
                }
                "flowersprite" => {
                    flower_sprites.push(BarotraumaSprite::new(child));
                }
                "damagedflowersprite" => {
                    damaged_flower_sprites.push(BarotraumaSprite::new(child));
                }
                "hiddenflowersprite" => {
                    hidden_flower_sprites.push(BarotraumaSprite::new(child));
                }
                "leafsprite" => {
                    leaf_sprites.push(BarotraumaSprite::new(child));
                }
                "damagedleafsprite" => {
                    damaged_leaf_sprites.push(BarotraumaSprite::new(child));
                }
                "damageparticle" => {
                    damage_particles.push(ParticleEmitterPrefab::new(child));
                }
                "deathparticle" => {
                    death_particles.push(ParticleEmitterPrefab::new(child));
                }
                "targets" => {
                    for child in child.children().filter(Node::is_element) {
                        targets.push(AITarget::new(child));
                    }
                }
                _ => (),
            }
        }

        Self {
            identifier,
            name,
            properties,
            branch_atlas,
            decay_atlas,
            branch_sprites,
            flower_sprites,
            damaged_flower_sprites,
            hidden_flower_sprites,
            leaf_sprites,
            damaged_leaf_sprites,
            damage_particles,
            death_particles,
            targets,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct AITarget {
    pub tags: Vec<String>,
    pub priority: u32,
}

impl AITarget {
    pub fn new(element: Node) -> Self {
        Self {
            tags: element
                .attribute_ignore_ascii_case("tags")
                .map_or(Vec::new(), |v| v.split(',').map(|v| v.to_owned()).collect()),
            priority: element
                .attribute_ignore_ascii_case("priority")
                .map_or(0, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct VineSprite {
    pub source_rect: Rect,
    pub origin: Vec2,
}

impl VineSprite {
    pub fn new(element: Node) -> Self {
        Self {
            source_rect: element.attribute_ignore_ascii_case("sourcerect").map_or(
                Rect {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                },
                |v| Rect::from_str(v, true).unwrap(),
            ),
            origin: element
                .attribute_ignore_ascii_case("origin")
                .map_or(Vec2::new(0.5, 0.5), |v| v.parse::<Vector2>().unwrap().0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum VineTileType {
    Stem = 0b0000,
    CrossJunction = 0b1111,
    HorizontalLine = 0b1010,
    VerticalLine = 0b0101,
    TurnTopRight = 0b1001,
    TurnTopLeft = 0b0011,
    TurnBottomLeft = 0b0110,
    TurnBottomRight = 0b1100,
    TSectionTop = 0b1011,
    TSectionLeft = 0b0111,
    TSectionBottom = 0b1110,
    TSectionRight = 0b1101,
    StumpTop = 0b0001,
    StumpLeft = 0b0010,
    StumpBottom = 0b0100,
    StumpRight = 0b1000,
}

impl FromStr for VineTileType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Stem" => Ok(Self::Stem),
            "CrossJunction" => Ok(Self::CrossJunction),
            "HorizontalLine" => Ok(Self::HorizontalLine),
            "VerticalLine" => Ok(Self::VerticalLine),
            "VerticalLane" => Ok(Self::HorizontalLine),
            "HorizontalLane" => Ok(Self::VerticalLine),
            "TurnTopRight" => Ok(Self::TurnTopRight),
            "TurnTopLeft" => Ok(Self::TurnTopLeft),
            "TurnBottomLeft" => Ok(Self::TurnBottomLeft),
            "TurnBottomRight" => Ok(Self::TurnBottomRight),
            "TSectionTop" => Ok(Self::TSectionTop),
            "TSectionLeft" => Ok(Self::TSectionLeft),
            "TSectionBottom" => Ok(Self::TSectionBottom),
            "TSectionRight" => Ok(Self::TSectionRight),
            "StumpTop" => Ok(Self::StumpTop),
            "StumpLeft" => Ok(Self::StumpLeft),
            "StumpBottom" => Ok(Self::StumpBottom),
            "StumpRight" => Ok(Self::StumpRight),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct BallastFloraProperties {
    pub base_branch_scale: f32,
    pub base_flower_scale: f32,
    pub base_leaf_scale: f32,
    pub flower_probability: f32,
    pub leaf_probability: f32,
    pub pulse_delay: f32,
    pub pulse_inflate_speed: f32,
    pub pulse_deflate_speed: f32,
    pub breakthrough_point: u32,
    pub has_broken_through: bool,
    pub sight: u32,
    pub branch_health: u32,
    pub root_health: u32,
    pub health_regen_per_branch: f32,
    pub max_branch_health_regen_distance: u32,
    pub root_color: Color,
    pub power_consumption_min: f32,
    pub power_consumption_max: f32,
    pub power_consumption_duration: f32,
    pub power_requirement: f32,
    pub max_anger: f32,
    pub max_power_capacity: f32,
    pub attack_item_prefab: String,
    pub explosion_resistance: f32,
    pub fire_vulnerability: f32,
    pub submerged_water_resistance: f32,
    pub branch_depth: f32,
    pub burst_sound: String,
    pub available_power: f32,
    pub anger: f32,
}

impl BallastFloraProperties {
    pub fn new(element: Node) -> Self {
        Self {
            base_branch_scale: element
                .attribute_ignore_ascii_case("basebranchscale")
                .map_or(0.25, |v| v.parse().unwrap()),
            base_flower_scale: element
                .attribute_ignore_ascii_case("baseflowerscale")
                .map_or(0.25, |v| v.parse().unwrap()),
            base_leaf_scale: element
                .attribute_ignore_ascii_case("baseleafscale")
                .map_or(0.5, |v| v.parse().unwrap()),
            flower_probability: element
                .attribute_ignore_ascii_case("flowerprobability")
                .map_or(0.33, |v| v.parse().unwrap()),
            leaf_probability: element
                .attribute_ignore_ascii_case("leafprobability")
                .map_or(0.7, |v| v.parse().unwrap()),
            pulse_delay: element
                .attribute_ignore_ascii_case("pulsedelay")
                .map_or(3.0, |v| v.parse().unwrap()),
            pulse_inflate_speed: element
                .attribute_ignore_ascii_case("pulseinflatespeed")
                .map_or(3.0, |v| v.parse().unwrap()),
            pulse_deflate_speed: element
                .attribute_ignore_ascii_case("pulsedeflatespeed")
                .map_or(1.0, |v| v.parse().unwrap()),
            breakthrough_point: element
                .attribute_ignore_ascii_case("breakthroughpoint")
                .map_or(32, |v| v.parse().unwrap()),
            has_broken_through: element
                .attribute_ignore_ascii_case("hasbrokenthrough")
                .map_or(false, |v| v.parse().unwrap()),
            sight: element
                .attribute_ignore_ascii_case("sight")
                .map_or(300, |v| v.parse().unwrap()),
            branch_health: element
                .attribute_ignore_ascii_case("branchhealth")
                .map_or(100, |v| v.parse().unwrap()),
            root_health: element
                .attribute_ignore_ascii_case("roothealth")
                .map_or(400, |v| v.parse().unwrap()),
            health_regen_per_branch: element
                .attribute_ignore_ascii_case("healthregenperbranch")
                .map_or(0.00025, |v| v.parse().unwrap()),
            max_branch_health_regen_distance: element
                .attribute_ignore_ascii_case("maxbranchhealthregendistance")
                .map_or(30, |v| v.parse().unwrap()),
            root_color: element.attribute_ignore_ascii_case("rootcolor").map_or(
                Color::Simple {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                |v| v.parse().unwrap(),
            ),
            power_consumption_min: element
                .attribute_ignore_ascii_case("powerconsumptionmin")
                .map_or(300.0, |v| v.parse().unwrap()),
            power_consumption_max: element
                .attribute_ignore_ascii_case("powerconsumptionmax")
                .map_or(3000.0, |v| v.parse().unwrap()),
            power_consumption_duration: element
                .attribute_ignore_ascii_case("powerconsumptionduration")
                .map_or(10.0, |v| v.parse().unwrap()),
            power_requirement: element
                .attribute_ignore_ascii_case("powerrequirement")
                .map_or(250.0, |v| v.parse().unwrap()),
            max_anger: element
                .attribute_ignore_ascii_case("maxanger")
                .map_or(5.0, |v| v.parse().unwrap()),
            max_power_capacity: element
                .attribute_ignore_ascii_case("maxpowercapacity")
                .map_or(10000.0, |v| v.parse().unwrap()),
            attack_item_prefab: element
                .attribute_ignore_ascii_case("attackitemprefab")
                .map_or(String::new(), |v| v.to_owned()),
            explosion_resistance: element
                .attribute_ignore_ascii_case("explosionresistance")
                .map_or(0.8, |v| v.parse().unwrap()),
            fire_vulnerability: element
                .attribute_ignore_ascii_case("firevulnerability")
                .map_or(5.0, |v| v.parse().unwrap()),
            submerged_water_resistance: element
                .attribute_ignore_ascii_case("submergedwaterresistance")
                .map_or(0.5, |v| v.parse().unwrap()),
            branch_depth: element
                .attribute_ignore_ascii_case("branchdepth")
                .map_or(0.8, |v| v.parse().unwrap()),
            burst_sound: element
                .attribute_ignore_ascii_case("burstsound")
                .map_or(String::new(), |v| v.to_owned()),
            available_power: element
                .attribute_ignore_ascii_case("availablepower")
                .map_or(0.0, |v| v.parse().unwrap()),
            anger: element
                .attribute_ignore_ascii_case("anger")
                .map_or(1.0, |v| v.parse().unwrap()),
        }
    }
}
