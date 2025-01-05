use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::item_prefab::BarotraumaSprite;

#[derive(Debug)]
pub struct SlideshowPrefab {
    pub identifier: String,
    pub slides: Vec<Slide>,
}

impl SlideshowPrefab {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let slides = element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("slide"))
            .map(|child| Slide::new(child))
            .collect::<Vec<_>>();

        Self { identifier, slides }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct Slide {
    pub text: String,
    pub fade_in_delay: f32,
    pub fade_in_duration: f32,
    pub fade_out_duration: f32,
    pub text_fade_in_delay: f32,
    pub text_fade_in_duration: f32,
    pub portrait: Option<BarotraumaSprite>,
}

impl Slide {
    pub fn new(element: Node) -> Self {
        let text = element
            .attribute_ignore_ascii_case("text")
            .map(std::borrow::ToOwned::to_owned)
            .unwrap();
        let fade_in_delay = element
            .attribute_ignore_ascii_case("fadeindelay")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let fade_in_duration = element
            .attribute_ignore_ascii_case("fadeinduration")
            .map_or(2.0, |v| v.parse::<f32>().unwrap());
        let fade_out_duration = element
            .attribute_ignore_ascii_case("fadeoutduration")
            .map_or(2.0, |v| v.parse::<f32>().unwrap());
        let text_fade_in_delay = element
            .attribute_ignore_ascii_case("textfadeindelay")
            .map_or(2.0, |v| v.parse::<f32>().unwrap());
        let text_fade_in_duration = element
            .attribute_ignore_ascii_case("textfadeinduration")
            .map_or(3.0, |v| v.parse::<f32>().unwrap());
        let mut portrait = None;
        for child in element
            .children()
            .filter(Node::is_element)
            .filter(|v| v.tag_name().name().eq_ignore_ascii_case("portrait"))
        {
            portrait = Some(BarotraumaSprite::new(child));
        }

        Self {
            text,
            fade_in_delay,
            fade_in_duration,
            fade_out_duration,
            text_fade_in_delay,
            text_fade_in_duration,
            portrait,
        }
    }
}
