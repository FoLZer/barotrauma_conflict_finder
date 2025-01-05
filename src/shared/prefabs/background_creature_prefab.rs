use std::collections::HashMap;

use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::{
    item_prefab::BarotraumaSprite,
    level_object_prefab::{DeformableSprite, SpriteDeformation},
};

#[derive(Debug)]
pub struct BackgroundCreaturePrefab {
    pub speed: f32,
    pub wander_amount: f32,
    pub wander_z_amount: f32,
    pub swarm_min: u32,
    pub swarm_max: u32,
    pub swarm_radius: f32,
    pub swarm_cohesion: f32,
    pub min_depth: f32,
    pub max_depth: f32,
    pub disable_rotation: bool,
    pub disable_flipping: bool,
    pub scale: f32,
    pub commonness: f32,
    pub max_count: u32,
    pub flash_interval: f32,
    pub flash_duration: f32,
    pub sprite: Option<BarotraumaSprite>,
    pub deformable_sprite: Option<DeformableSprite>,
    pub sprite_deformations: Vec<SpriteDeformation>,
    pub light_sprite: Option<BarotraumaSprite>,
    pub deformable_light_sprite: Option<DeformableSprite>,
    pub light_sprite_deformations: Vec<SpriteDeformation>,
    pub override_commonness: HashMap<String, Option<f32>>,
    pub unique_sprite_deformations: Vec<SpriteDeformation>,
}

impl BackgroundCreaturePrefab {
    pub fn new(element: Node) -> Self {
        let speed = element
            .attribute_ignore_ascii_case("speed")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let wander_amount = element
            .attribute_ignore_ascii_case("wanderamount")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let wander_z_amount = element
            .attribute_ignore_ascii_case("wanderzamount")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let swarm_min = element
            .attribute_ignore_ascii_case("swarmmin")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let swarm_max = element
            .attribute_ignore_ascii_case("swarmmax")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let swarm_radius = element
            .attribute_ignore_ascii_case("swarmradius")
            .map_or(200.0, |v| v.parse::<f32>().unwrap());
        let swarm_cohesion = element
            .attribute_ignore_ascii_case("swarmcohesion")
            .map_or(0.2, |v| v.parse::<f32>().unwrap());
        let min_depth = element
            .attribute_ignore_ascii_case("mindepth")
            .map_or(10.0, |v| v.parse::<f32>().unwrap());
        let max_depth = element
            .attribute_ignore_ascii_case("maxdepth")
            .map_or(1000.0, |v| v.parse::<f32>().unwrap());
        let disable_rotation = element
            .attribute_ignore_ascii_case("disablerotation")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let disable_flipping = element
            .attribute_ignore_ascii_case("disableflipping")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let scale = element
            .attribute_ignore_ascii_case("scale")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let commonness = element
            .attribute_ignore_ascii_case("commonness")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let max_count = element
            .attribute_ignore_ascii_case("maxcount")
            .map_or(1000, |v| v.parse::<u32>().unwrap());
        let flash_interval = element
            .attribute_ignore_ascii_case("flashinterval")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let flash_duration = element
            .attribute_ignore_ascii_case("flashduration")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());

        let mut sprite = None;
        let mut deformable_sprite = None;
        let mut sprite_deformations = Vec::new();
        let mut light_sprite = None;
        let mut deformable_light_sprite = None;
        let mut light_sprite_deformations = Vec::new();
        let mut override_commonness = HashMap::new();
        let mut unique_sprite_deformations: Vec<SpriteDeformation> = Vec::new();
        for child in element.children().filter(Node::is_element) {
            let elem_name = child.tag_name().name().to_lowercase();
            match elem_name.as_str() {
                "sprite" => {
                    sprite = Some(BarotraumaSprite::new(child));
                }
                "deformablesprite" => {
                    deformable_sprite = Some(DeformableSprite::new(child));
                    for child in child.children().filter(Node::is_element) {
                        let sync = child
                            .attribute_ignore_ascii_case("sync")
                            .map(|v| v.parse::<u32>().unwrap());
                        match sync {
                            Some(sync) => {
                                let type_name = child
                                    .attribute_ignore_ascii_case("type")
                                    .unwrap()
                                    .to_lowercase();
                                match unique_sprite_deformations.iter().find(|v| {
                                    let v = v.sprite_deformation_params();
                                    v.ty.as_ref().is_some_and(|v| v == &type_name)
                                        && v.sync.is_some_and(|v| v == sync)
                                }) {
                                    Some(deformation) => {
                                        sprite_deformations.push(deformation.clone());
                                    }
                                    None => {
                                        let deformation = SpriteDeformation::new(child);
                                        unique_sprite_deformations.push(deformation.clone());
                                        sprite_deformations.push(deformation);
                                    }
                                }
                            }
                            None => {
                                let deformation = SpriteDeformation::new(child);
                                unique_sprite_deformations.push(deformation.clone());
                                sprite_deformations.push(deformation);
                            }
                        }
                    }
                }
                "lightsprite" => {
                    light_sprite = Some(BarotraumaSprite::new(child));
                }
                "deformablelightsprite" => {
                    deformable_light_sprite = Some(DeformableSprite::new(child));
                    for child in child.children().filter(Node::is_element) {
                        let sync = child
                            .attribute_ignore_ascii_case("sync")
                            .map(|v| v.parse::<u32>().unwrap());
                        match sync {
                            Some(sync) => {
                                let type_name = child
                                    .attribute_ignore_ascii_case("type")
                                    .unwrap()
                                    .to_lowercase();
                                match unique_sprite_deformations.iter().find(|v| {
                                    let v = v.sprite_deformation_params();
                                    v.ty.as_ref().is_some_and(|v| v == &type_name)
                                        && v.sync.is_some_and(|v| v == sync)
                                }) {
                                    Some(deformation) => {
                                        light_sprite_deformations.push(deformation.clone());
                                    }
                                    None => {
                                        let deformation = SpriteDeformation::new(child);
                                        unique_sprite_deformations.push(deformation.clone());
                                        light_sprite_deformations.push(deformation);
                                    }
                                }
                            }
                            None => {
                                let deformation = SpriteDeformation::new(child);
                                unique_sprite_deformations.push(deformation.clone());
                                light_sprite_deformations.push(deformation);
                            }
                        }
                    }
                }
                "overridecommonness" => {
                    let level_type = child
                        .attribute_ignore_ascii_case("leveltype")
                        .map(|v| v.to_owned())
                        .unwrap();
                    if !override_commonness.contains_key(&level_type) {
                        override_commonness.insert(
                            level_type,
                            element
                                .attribute_ignore_ascii_case("commonness")
                                .map(|v| v.parse::<f32>().unwrap()),
                        );
                    }
                }
                _ => (),
            }
        }

        Self {
            speed,
            wander_amount,
            wander_z_amount,
            swarm_min,
            swarm_max,
            swarm_radius,
            swarm_cohesion,
            min_depth,
            max_depth,
            disable_rotation,
            disable_flipping,
            scale,
            commonness,
            max_count,
            flash_interval,
            flash_duration,
            sprite,
            deformable_sprite,
            sprite_deformations,
            light_sprite,
            deformable_light_sprite,
            light_sprite_deformations,
            override_commonness,
            unique_sprite_deformations,
        }
    }
}
