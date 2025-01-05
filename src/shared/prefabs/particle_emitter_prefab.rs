use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::item_prefab::Color;

#[derive(Debug)]
pub struct ParticleEmitterPrefab {
    pub properties: ParticleEmitterProperties,
    pub particle_prefab_name: Option<String>,
}

impl ParticleEmitterPrefab {
    pub fn new(element: Node) -> Self {
        let properties = ParticleEmitterProperties::new(element);
        let particle_prefab_name = element
            .attribute_ignore_ascii_case("particle")
            .map(|v| v.to_owned());

        Self {
            properties,
            particle_prefab_name,
        }
    }
}

#[derive(Debug)]
pub struct ParticleEmitterProperties {
    pub angle_min: f32,
    pub angle_max: f32,
    pub distance_min: f32,
    pub distance_max: f32,
    pub velocity_min: f32,
    pub velocity_max: f32,
    pub scale_min: f32,
    pub scale_max: f32,
    pub scale_multiplier: Vec2,
    pub emit_interval: f32,
    pub particle_amount: u32,
    pub particles_per_second: f32,
    pub emit_across_ray_interval: f32,
    pub initial_delay: f32,
    pub high_quality_collision_detection: bool,
    pub copy_entity_angle: bool,
    pub copy_entity_dir: bool,
    pub copy_target_angle: bool,
    pub copy_parent_particle_scale: bool,
    pub color_multiplier: Color,
    pub lifetime_multiplier: f32,
    pub draw_on_top: bool,
    pub angle: f32,
    pub distance: f32,
    pub velocity: f32,
}

impl ParticleEmitterProperties {
    pub fn new(element: Node) -> Self {
        Self {
            angle_min: element
                .attribute_ignore_ascii_case("anglemin")
                .map_or(0.0, |v| v.parse().unwrap()),
            angle_max: element
                .attribute_ignore_ascii_case("anglemax")
                .map_or(0.0, |v| v.parse().unwrap()),
            distance_min: element
                .attribute_ignore_ascii_case("distancemin")
                .map_or(0.0, |v| v.parse().unwrap()),
            distance_max: element
                .attribute_ignore_ascii_case("distancemax")
                .map_or(0.0, |v| v.parse().unwrap()),
            velocity_min: element
                .attribute_ignore_ascii_case("velocitymin")
                .map_or(0.0, |v| v.parse().unwrap()),
            velocity_max: element
                .attribute_ignore_ascii_case("velocitymax")
                .map_or(0.0, |v| v.parse().unwrap()),
            scale_min: element
                .attribute_ignore_ascii_case("scalemin")
                .map_or(1.0, |v| v.parse().unwrap()),
            scale_max: element
                .attribute_ignore_ascii_case("scalemax")
                .map_or(1.0, |v| v.parse().unwrap()),
            scale_multiplier: element
                .attribute_ignore_ascii_case("scalemultiplier")
                .map_or(Vec2::new(1.0, 1.0), |v| v.parse::<Vector2>().unwrap().0),
            emit_interval: element
                .attribute_ignore_ascii_case("emitinterval")
                .map_or(0.0, |v| v.parse().unwrap()),
            particle_amount: element
                .attribute_ignore_ascii_case("particleamount")
                .map_or(0, |v| v.parse().unwrap()),
            particles_per_second: element
                .attribute_ignore_ascii_case("particlespersecond")
                .map_or(0.0, |v| v.parse().unwrap()),
            emit_across_ray_interval: element
                .attribute_ignore_ascii_case("emitacrossrayinterval")
                .map_or(0.0, |v| v.parse().unwrap()),
            initial_delay: element
                .attribute_ignore_ascii_case("initialdelay")
                .map_or(0.0, |v| v.parse().unwrap()),
            high_quality_collision_detection: element
                .attribute_ignore_ascii_case("highqualitycollisiondetection")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            copy_entity_angle: element
                .attribute_ignore_ascii_case("copyentityangle")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            copy_entity_dir: element
                .attribute_ignore_ascii_case("copyentitydir")
                .map_or(true, |v| v.parse().unwrap()),
            copy_target_angle: element
                .attribute_ignore_ascii_case("copytargetangle")
                .map_or(false, |v| v.parse().unwrap()),
            copy_parent_particle_scale: element
                .attribute_ignore_ascii_case("copyparentparticlescale")
                .map_or(false, |v| v.parse().unwrap()),
            color_multiplier: element
                .attribute_ignore_ascii_case("colormultiplier")
                .map_or(
                    Color::Simple {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    |v| v.parse().unwrap(),
                ),
            lifetime_multiplier: element
                .attribute_ignore_ascii_case("lifetimemultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            draw_on_top: element
                .attribute_ignore_ascii_case("drawontop")
                .map_or(false, |v| v.parse().unwrap()),
            angle: element
                .attribute_ignore_ascii_case("angle")
                .map_or(0.0, |v| v.parse().unwrap()),
            distance: element
                .attribute_ignore_ascii_case("distance")
                .map_or(0.0, |v| v.parse().unwrap()),
            velocity: element
                .attribute_ignore_ascii_case("velocity")
                .map_or(0.0, |v| v.parse().unwrap()),
        }
    }
}
