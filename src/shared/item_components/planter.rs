use std::collections::HashMap;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::level_object_prefab::RelatedItem, submarine_info::Vector2, util::NodeExp,
};

use super::pickable::PickableComponent;

#[derive(Debug)]
pub struct PlanterComponent {
    pub pickable: PickableComponent,

    pub fertilizer: f32,
    pub fertilizer_capacity: f32,

    pub plant_slots: HashMap<u32, PlantSlot>,
    pub suitable_fertilizers: Vec<RelatedItem>,
    pub suitable_seeds: Vec<RelatedItem>,
}

impl PlanterComponent {
    pub fn from_xml(element: &Node) -> Self {
        let mut plant_slots = HashMap::new();
        let mut suitable_fertilizers = Vec::new();
        let mut suitable_seeds = Vec::new();

        for child in element.children().filter(Node::is_element) {
            let tag_name = child.tag_name().name();
            match tag_name {
                "plantslot" => {
                    plant_slots.insert(
                        child
                            .attribute_ignore_ascii_case("slot")
                            .map(|v| v.parse().unwrap())
                            .unwrap(),
                        PlantSlot::from_xml(&child),
                    );
                }
                "suitablefertilizer" => {
                    suitable_fertilizers.push(RelatedItem::new(child));
                }
                "suitableseed" => {
                    suitable_seeds.push(RelatedItem::new(child));
                }
                _ => {
                    panic!("Unexpected tag name in PlanterComponent: {}", tag_name);
                }
            }
        }

        Self {
            pickable: PickableComponent::from_xml(element),

            fertilizer: element
                .attribute_ignore_ascii_case("fertilizer")
                .map_or(0.0, |v| v.parse().unwrap()),
            fertilizer_capacity: element
                .attribute_ignore_ascii_case("fertilizercapacity")
                .map_or(100.0, |v| v.parse().unwrap()),

            plant_slots,
            suitable_fertilizers,
            suitable_seeds,
        }
    }
}

#[derive(Debug)]
pub struct PlantSlot {
    pub offset: Vec2,
    pub size: f32,
}

impl PlantSlot {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            offset: element
                .attribute_ignore_ascii_case("offset")
                .map(|v| v.parse::<Vector2>().unwrap().0)
                .unwrap(),
            size: element
                .attribute_ignore_ascii_case("size")
                .map(|v| v.parse().unwrap())
                .unwrap(),
        }
    }
}
