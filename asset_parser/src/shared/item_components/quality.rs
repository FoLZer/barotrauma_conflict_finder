use std::{collections::HashMap, str::FromStr};

use roxmltree::Node;

use crate::shared::{prefabs::item_prefab::DoesNotExistError, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct QualityComponent {
    pub item: ItemComponent,

    pub quality_level: u32,

    pub stat_values: HashMap<StatType, f32>,
}

impl QualityComponent {
    pub fn from_xml(element: &Node) -> Self {
        let mut stat_values = HashMap::new();
        for child in element.children().filter(Node::is_element) {
            let tag_name = child.tag_name().name();
            match tag_name {
                "stattype" | "statvalue" | "qualitystat" => {
                    let stat_type = child
                        .attribute_ignore_ascii_case("stattype")
                        .map(|v| v.parse::<StatType>().unwrap())
                        .unwrap();
                    let value = element
                        .attribute_ignore_ascii_case("value")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap();
                    stat_values.insert(stat_type, value);
                }
                _ => {
                    panic!("Unexpected tag name in QualityComponent: {}", tag_name);
                }
            }
        }

        Self {
            item: ItemComponent::from_xml(element),

            quality_level: element
                .attribute_ignore_ascii_case("qualitylevel")
                .map(|v| v.parse().unwrap())
                .unwrap(),

            stat_values,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum StatType {
    Condition,
    ExplosionRadius,
    ExplosionDamage,
    RepairSpeed,
    RepairToolStructureRepairMultiplier,
    RepairToolStructureDamageMultiplier,
    RepairToolDeattachTimeMultiplier,
    FirepowerMultiplier,
    StrikingPowerMultiplier,
    StrikingSpeedMultiplier,
    FiringRateMultiplier,
}

impl FromStr for StatType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "condition" => Ok(Self::Condition),
            "explosionradius" => Ok(Self::ExplosionRadius),
            "explosiondamage" => Ok(Self::ExplosionDamage),
            "repairspeed" => Ok(Self::RepairSpeed),
            "repairtoolstructurerepairmultiplier" => Ok(Self::RepairToolStructureRepairMultiplier),
            "repairtoolstructuredamagemultiplier" => Ok(Self::RepairToolStructureDamageMultiplier),
            "repairtooldeattachtimemultiplier" => Ok(Self::RepairToolDeattachTimeMultiplier),
            "firepowermultiplier" => Ok(Self::FirepowerMultiplier),
            "strikingpowermultiplier" => Ok(Self::StrikingPowerMultiplier),
            "strikingspeedmultiplier" => Ok(Self::StrikingSpeedMultiplier),
            "firingratemultiplier" => Ok(Self::FiringRateMultiplier),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
