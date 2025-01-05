use roxmltree::Node;

use crate::shared::{prefabs::item_prefab::Color, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct StatusHUDComponent {
    pub item: ItemComponent,

    pub range: f32,
    pub fade_out_range: f32,
    pub thermal_goggles: bool,
    pub debug_wiring: bool,
    pub show_dead_characters: bool,
    pub show_texts: bool,
    pub overlay_color: Color,
}

impl StatusHUDComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            range: element
                .attribute_ignore_ascii_case("range")
                .map_or(500.0, |v| v.parse().unwrap()),
            fade_out_range: element
                .attribute_ignore_ascii_case("fadeoutrange")
                .map_or(50.0, |v| v.parse().unwrap()),
            thermal_goggles: element
                .attribute_ignore_ascii_case("thermalgoggles")
                .map_or(false, |v| v.parse().unwrap()),
            debug_wiring: element
                .attribute_ignore_ascii_case("debugwiring")
                .map_or(false, |v| v.parse().unwrap()),
            show_dead_characters: element
                .attribute_ignore_ascii_case("showdeadcharacters")
                .map_or(true, |v| v.parse().unwrap()),
            show_texts: element
                .attribute_ignore_ascii_case("showtexts")
                .map_or(true, |v| v.parse().unwrap()),
            overlay_color: element.attribute_ignore_ascii_case("overlaycolor").map_or(
                "72,119,72,120".parse().unwrap(),
                |v| v.parse().unwrap(),
            ),
        }
    }
}
