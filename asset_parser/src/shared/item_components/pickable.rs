use roxmltree::Node;

use crate::shared::{prefabs::level_object_prefab::InvSlotType, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct PickableComponent {
    pub item: ItemComponent,

    pub allowed_slots: Vec<InvSlotType>,
}

impl PickableComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            allowed_slots: element.attribute_ignore_ascii_case("slots").map_or(
                vec![InvSlotType::new().with_any(true)],
                |v| {
                    v.split(',')
                        .map(|v| {
                            let slots = v.split('+');
                            slots.fold(InvSlotType::new(), |acc, v| {
                                if v == "bothhands" {
                                    InvSlotType::new()
                                        .with_left_hand(true)
                                        .with_right_hand(true)
                                } else {
                                    InvSlotType::from_bits(
                                        acc.into_bits()
                                            | v.parse::<InvSlotType>().unwrap().into_bits(),
                                    )
                                }
                            })
                        })
                        .collect()
                },
            ),
        }
    }
}
