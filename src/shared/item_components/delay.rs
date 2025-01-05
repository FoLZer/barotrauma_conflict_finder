use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct DelayComponent {
    pub item: ItemComponent,

    pub delay: f32,
    pub reset_when_signal_received: bool,
    pub reset_when_different_signal_received: bool,
}

impl DelayComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            delay: element
                .attribute_ignore_ascii_case("delay")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            reset_when_signal_received: element
                .attribute_ignore_ascii_case("resetwhensignalreceived")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            reset_when_different_signal_received: element
                .attribute_ignore_ascii_case("resetwhendifferentsignalreceived")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
