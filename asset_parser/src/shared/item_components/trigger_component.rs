use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct TriggerComponent {
    pub item: ItemComponent,

    pub force: f32,
    pub distance_based_force: bool,
    pub force_fluctuation: bool,
    pub force_fluctuation_strength: f32,
    pub force_fluctuation_frequency: f32,
    pub force_fluctuation_interval: f32,
    pub apply_effects_to_characters_inside_sub: bool,
    pub move_outside_sub: bool,
}

impl TriggerComponent {
    pub fn from_xml(element: &Node) -> Self {
        //TODO: parse children
        Self {
            item: ItemComponent::from_xml(element),

            force: element
                .attribute_ignore_ascii_case("force")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            distance_based_force: element
                .attribute_ignore_ascii_case("distancebasedforce")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            force_fluctuation: element
                .attribute_ignore_ascii_case("forcefluctuation")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            force_fluctuation_strength: element
                .attribute_ignore_ascii_case("forcefluctuationstrength")
                .map_or(1.0, |v| v.parse().unwrap()),
            force_fluctuation_frequency: element
                .attribute_ignore_ascii_case("forcefluctuationfrequency")
                .map_or(1.0, |v| v.parse().unwrap()),
            force_fluctuation_interval: element
                .attribute_ignore_ascii_case("forcefluctuationinterval")
                .map_or(0.0, |v| v.parse().unwrap()),
            apply_effects_to_characters_inside_sub: element
                .attribute_ignore_ascii_case("applyeffectstocharactersinsidesub")
                .map_or(false, |v| v.parse().unwrap()),
            move_outside_sub: element
                .attribute_ignore_ascii_case("moveoutsidesub")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
        }
    }
}
