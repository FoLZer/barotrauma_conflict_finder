use glam::Vec4;
use roxmltree::Node;

use crate::shared::{
    prefabs::item_prefab::{Color, Vector4},
    util::NodeExp,
};

use super::ItemComponent;

#[derive(Debug)]
pub struct GrowableComponent {
    pub item: ItemComponent,

    pub growth_speed: f32,
    pub max_health: f32,
    pub flood_tolerance: f32,
    pub hardiness: f32,
    pub seed_rate: f32,
    pub product_rate: f32,
    pub mutation_probability: f32,
    pub flower_tint: Color,
    pub flower_quantity: u32,
    pub base_flower_scale: f32,
    pub base_leaf_scale: f32,
    pub leaf_tint: Color,
    pub leaf_probability: f32,
    pub vine_tint: Color,
    pub maximum_vines: u32,
    pub vine_scale: f32,
    pub dead_tint: Color,
    pub growth_weights: Vec4,
    pub fire_vulnerability: Option<f32>,
    pub health: f32,
    //pub produced_items: Vec<ProducedItem>,
}

impl GrowableComponent {
    pub fn from_xml(element: &Node) -> Self {
        //let mut produced_items = Vec::new();

        //TODO: vine sprites, produced items
        Self {
            item: ItemComponent::from_xml(element),

            growth_speed: element
                .attribute_ignore_ascii_case("growthspeed")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            max_health: element
                .attribute_ignore_ascii_case("maxhealth")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            flood_tolerance: element
                .attribute_ignore_ascii_case("floodtolerance")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            hardiness: element
                .attribute_ignore_ascii_case("hardiness")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            seed_rate: element
                .attribute_ignore_ascii_case("seedrate")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            product_rate: element
                .attribute_ignore_ascii_case("productrate")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            mutation_probability: element
                .attribute_ignore_ascii_case("mutationprobability")
                .map_or(0.5, |v| v.parse().unwrap()),
            flower_tint: element
                .attribute_ignore_ascii_case("flowertint")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            flower_quantity: element
                .attribute_ignore_ascii_case("flowerquantity")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            base_flower_scale: element
                .attribute_ignore_ascii_case("baseflowerscale")
                .map_or(0.25, |v| v.parse().unwrap()),
            base_leaf_scale: element
                .attribute_ignore_ascii_case("baseleafscale")
                .map_or(0.5, |v| v.parse().unwrap()),
            leaf_tint: element
                .attribute_ignore_ascii_case("leaftint")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            leaf_probability: element
                .attribute_ignore_ascii_case("leafprobability")
                .map_or(0.33, |v| v.parse().unwrap()),
            vine_tint: element
                .attribute_ignore_ascii_case("vinetint")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            maximum_vines: element
                .attribute_ignore_ascii_case("maximumvines")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            vine_scale: element
                .attribute_ignore_ascii_case("vinescale")
                .map_or(0.25, |v| v.parse().unwrap()),
            dead_tint: element
                .attribute_ignore_ascii_case("deadtint")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            growth_weights: element
                .attribute_ignore_ascii_case("growthweights")
                .map(|v| v.parse::<Vector4>().unwrap().0)
                .unwrap(),
            fire_vulnerability: element
                .attribute_ignore_ascii_case("firevulnerability")
                .map(|v| v.parse().unwrap()),
            health: element
                .attribute_ignore_ascii_case("health")
                .map_or(100.0, |v| v.parse().unwrap()),
            //produced_items,
        }
    }
}
