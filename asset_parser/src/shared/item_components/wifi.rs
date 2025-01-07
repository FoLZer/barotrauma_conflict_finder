use roxmltree::Node;

use crate::shared::{prefabs::location_type::CharacterTeamType, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct WifiComponent {
    pub item: ItemComponent,

    pub team_id: Option<CharacterTeamType>,
    pub range: f32,
    pub channel: u32,
    pub allow_cross_team_communication: bool,
    pub link_to_chat: bool,
    pub min_chat_message_interval: f32,
    pub discard_duplicate_chat_messages: bool,
}

impl WifiComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            team_id: element
                .attribute_ignore_ascii_case("teamid")
                .map(|v| v.parse().unwrap()),
            range: element
                .attribute_ignore_ascii_case("range")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            channel: element
                .attribute_ignore_ascii_case("channel")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            allow_cross_team_communication: element
                .attribute_ignore_ascii_case("allowcrossteamcommunication")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            link_to_chat: element
                .attribute_ignore_ascii_case("linktochat")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            min_chat_message_interval: element
                .attribute_ignore_ascii_case("minchatmessageinterval")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            discard_duplicate_chat_messages: element
                .attribute_ignore_ascii_case("discardduplicatechatmessages")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
