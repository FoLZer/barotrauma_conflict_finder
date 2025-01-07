use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct ConnectionSelectorComponent {
    pub item_component: ItemComponent,

    pub selected_connection: u32,
    pub wrap_around: bool,
    pub skip_empty_connections: bool,
}

impl ConnectionSelectorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item_component: ItemComponent::from_xml(element),

            selected_connection: element
                .attribute_ignore_ascii_case("selectedconnection")
                .map_or(0, |v| v.parse().unwrap()),
            wrap_around: element
                .attribute_ignore_ascii_case("wraparound")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            skip_empty_connections: element
                .attribute_ignore_ascii_case("skipemptyconnections")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
        }
    }
}
