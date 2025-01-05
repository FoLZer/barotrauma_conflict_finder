use roxmltree::Node;

use crate::shared::{prefabs::item_prefab::Color, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct TerminalComponent {
    pub item: ItemComponent,

    pub welcome_message: String,
    pub use_monospace_font: bool,
    pub auto_hide_scrollbar: bool,
    pub welcome_message_displayed: bool,
    pub text_color: Color,
    pub line_start_symbol: String,
    pub readonly: bool,
    pub auto_scroll_to_bottom: bool,
}

impl TerminalComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            welcome_message: element
                .attribute_ignore_ascii_case("welcomemessage")
                .map(|v| v.to_owned())
                .unwrap(),
            use_monospace_font: element
                .attribute_ignore_ascii_case("usemonospacefont")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            auto_hide_scrollbar: element
                .attribute_ignore_ascii_case("autohidescrollbar")
                .map_or(false, |v| v.parse().unwrap()),
            welcome_message_displayed: element
                .attribute_ignore_ascii_case("welcomemessagedisplayed")
                .map_or(false, |v| v.parse().unwrap()),
            text_color: element
                .attribute_ignore_ascii_case("textcolor")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            line_start_symbol: element
                .attribute_ignore_ascii_case("linestartsymbol")
                .map_or("> ", |v| v)
                .to_owned(),
            readonly: element
                .attribute_ignore_ascii_case("readonly")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            auto_scroll_to_bottom: element
                .attribute_ignore_ascii_case("autoscrolltobottom")
                .map_or(true, |v| v.parse().unwrap()),
        }
    }
}
