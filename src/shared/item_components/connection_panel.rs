use roxmltree::Node;

use crate::shared::{prefabs::level_object_prefab::StatusEffect, util::NodeExp};

use super::ItemComponent;

pub const MAX_CONNECTION_COUNT: usize = 256;
pub const DEFAULT_MAX_WIRES: u32 = 5;

#[derive(Debug)]
pub struct ConnectionPannelComponent {
    pub item: ItemComponent,

    pub locked: bool,

    pub connections: Vec<Connection>,
}

impl ConnectionPannelComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            locked: element
                .attribute_ignore_ascii_case("locked")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),

            connections: {
                let mut connections = Vec::new();

                for child in element.children().filter(Node::is_element) {
                    let tag_name = child.tag_name().name();
                    match tag_name {
                        "input" | "output" => {
                            connections.push(Connection::from_xml(&child));
                        }
                        "requireditem" => (), //handled in ItemComponent
                        _ => {
                            panic!(
                                "Unexpected tag name in ConnectionPanelComponent: {}",
                                tag_name
                            );
                        }
                    }
                }

                assert!(
                    connections.len() < MAX_CONNECTION_COUNT,
                    "Tried to load too many connections: {}, max: {}",
                    connections.len(),
                    MAX_CONNECTION_COUNT
                );

                connections
            },
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub max_wires: u32,
    pub max_player_connectable_wires: u32,
    pub name: String,
    pub display_name: Option<String>,
    pub fallback_display_name: Option<String>,

    pub loaded_wires: Vec<(u16, Option<u32>)>,
    pub effects: Vec<StatusEffect>,
}

impl Connection {
    pub fn from_xml(element: &Node) -> Self {
        let max_wires: u32 = element
            .attribute_ignore_ascii_case("maxwires")
            .map_or(DEFAULT_MAX_WIRES, |v| v.parse().unwrap());

        let mut loaded_wires = Vec::new();
        let mut effects = Vec::new();

        for child in element.children().filter(Node::is_element) {
            let tag_name = child.tag_name().name();
            match tag_name {
                "link" => {
                    let id = child
                        .attribute_ignore_ascii_case("w")
                        .map(|v| v.parse().unwrap())
                        .unwrap();
                    let i = child
                        .attribute_ignore_ascii_case("i")
                        .map(|v| v.parse().unwrap());
                    assert!(loaded_wires.len() < max_wires as usize);
                    loaded_wires.push((id, i));
                }
                "statuseffect" => effects.push(StatusEffect::new(child)),
                _ => {
                    panic!("Unexpected tag name in Connection: {}", tag_name);
                }
            }
        }

        Self {
            max_wires,
            max_player_connectable_wires: element
                .attribute_ignore_ascii_case("maxplayerconnectablewires")
                .map_or(max_wires, |v| v.parse().unwrap()),
            name: element
                .attribute_ignore_ascii_case("name")
                .map(|v| v.to_owned())
                .unwrap(),
            display_name: element
                .attribute_ignore_ascii_case("displayname")
                .map(|v| v.to_owned()),
            fallback_display_name: element
                .attribute_ignore_ascii_case("fallbackdisplayname")
                .map(|v| v.to_owned()),

            loaded_wires,
            effects,
        }
    }
}
