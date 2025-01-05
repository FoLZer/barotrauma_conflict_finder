use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::item_prefab::{BarotraumaSprite, Color};

#[derive(Debug)]
pub struct DecalPrefab {
    pub identifier: String,
    pub color: Color,
    pub lifetime: f32,
    pub fade_out_time: f32,
    pub fade_in_time: f32,
    pub sprites: Vec<BarotraumaSprite>,
}

impl DecalPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element.tag_name().name().to_owned();
        let color = element.attribute_ignore_ascii_case("color").map_or(
            Color::Simple {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            |v| v.parse().unwrap(),
        );
        let lifetime = element
            .attribute_ignore_ascii_case("lifetime")
            .map_or(10.0, |v| v.parse::<f32>().unwrap());
        let fade_out_time = element
            .attribute_ignore_ascii_case("fadeouttime")
            .map_or(1.0, |v| v.parse::<f32>().unwrap())
            .min(lifetime);
        let fade_in_time = element
            .attribute_ignore_ascii_case("fadeintime")
            .map_or(0.0, |v| v.parse::<f32>().unwrap())
            .min(lifetime - fade_out_time);

        let sprites = element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("sprite"))
            .map(|child| BarotraumaSprite::new(child))
            .collect::<Vec<_>>();

        Self {
            identifier,
            color,
            lifetime,
            fade_out_time,
            fade_in_time,
            sprites,
        }
    }
}
