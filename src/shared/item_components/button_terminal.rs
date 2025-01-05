use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct ButtonTerminalComponent {
    pub item: ItemComponent,

    pub signals: Vec<String>,
    pub activating_items: Vec<String>,
}

impl ButtonTerminalComponent {
    pub fn from_xml(element: &Node) -> Self {
        //TODO: TerminalButton
        Self {
            item: ItemComponent::from_xml(element),

            signals: element
                .attribute_ignore_ascii_case("signals")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap(),
            activating_items: element
                .attribute_ignore_ascii_case("activatingitems")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap(),
        }
    }
}
