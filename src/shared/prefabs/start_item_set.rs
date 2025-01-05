use roxmltree::Node;

use crate::shared::util::NodeExp;

#[derive(Debug)]
pub struct StartItemSet {
    pub identifier: String,
    pub items: Vec<StartItem>,
    pub order: i32,
}

impl StartItemSet {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let items = element
            .children()
            .filter(Node::is_element)
            .map(|v| StartItem::new(v))
            .collect::<Vec<_>>();
        let order = element
            .attribute_ignore_ascii_case("order")
            .map(|v| v.parse::<i32>().unwrap())
            .unwrap();

        Self {
            identifier,
            items,
            order,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct StartItem {
    pub identifier: String,
    pub amount: u32,
    pub multi_player_only: bool,
}

impl StartItem {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let amount = element
            .attribute_ignore_ascii_case("amount")
            .map_or(1, |v| v.parse::<u32>().unwrap());
        let multi_player_only = element
            .attribute_ignore_ascii_case("multiplayeronly")
            .map_or(false, |v| v.parse::<bool>().unwrap());

        Self {
            identifier,
            amount,
            multi_player_only,
        }
    }
}
