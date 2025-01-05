use std::str::FromStr;

use roxmltree::Node;

use crate::shared::{prefabs::item_prefab::DoesNotExistError, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct OscillatorComponent {
    pub item: ItemComponent,

    pub output_type: WaveType,
    pub frequency: f32,
}

impl OscillatorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            output_type: element
                .attribute_ignore_ascii_case("outputtype")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            frequency: element
                .attribute_ignore_ascii_case("frequency")
                .map(|v| v.parse().unwrap())
                .unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum WaveType {
    Pulse,
    Sawtooth,
    Sine,
    Square,
    Triangle,
}

impl FromStr for WaveType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pulse" => Ok(Self::Pulse),
            "sawtooth" => Ok(Self::Sawtooth),
            "sine" => Ok(Self::Sine),
            "square" => Ok(Self::Square),
            "triangle" => Ok(Self::Triangle),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
