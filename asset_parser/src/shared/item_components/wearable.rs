use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::pickable::PickableComponent;

#[derive(Debug)]
pub struct WearableComponent {
    pub pickable: PickableComponent,

    pub allow_use_when_worn: bool,
    pub variants: Option<u32>,
    pub auto_equip_when_full: bool,
    pub display_contained_status: bool,
}

impl WearableComponent {
    //TODO: children elements

    pub fn from_xml(element: &Node) -> Self {
        Self {
            pickable: PickableComponent::from_xml(element),

            allow_use_when_worn: element
                .attribute_ignore_ascii_case("allowusewhenworn")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            variants: element
                .attribute_ignore_ascii_case("variants")
                .map(|v| v.parse().unwrap()),
            auto_equip_when_full: element
                .attribute_ignore_ascii_case("autoequipwhenfull")
                .map_or(true, |v| v.parse().unwrap()),
            display_contained_status: element
                .attribute_ignore_ascii_case("displaycontainedstatus")
                .map_or(false, |v| v.parse().unwrap()),
        }
    }
}
