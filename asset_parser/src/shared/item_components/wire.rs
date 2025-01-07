use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct WireComponent {
    pub item: ItemComponent,

    pub max_length: f32,
    pub hidden_in_game: bool,
    pub no_auto_lock: bool,
    pub use_sprite_depth: bool,
    pub drop_on_connect: bool,
}

impl WireComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            max_length: element
                .attribute_ignore_ascii_case("maxlength")
                .map_or(5000.0, |v| v.parse().unwrap()),
            hidden_in_game: element
                .attribute_ignore_ascii_case("hiddeningame")
                .map_or(false, |v| v.parse().unwrap()),
            no_auto_lock: element
                .attribute_ignore_ascii_case("noautolock")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            use_sprite_depth: element
                .attribute_ignore_ascii_case("usespritedepth")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            drop_on_connect: element
                .attribute_ignore_ascii_case("droponconnect")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
        }
    }
}
