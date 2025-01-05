use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::{decal_prefab::DecalPrefab, item_prefab::BarotraumaSprite},
        util::Overridable,
    },
};

#[derive(Debug)]
pub struct DecalsFile {
    pub grime_sprites: Vec<Overridable<BarotraumaSprite>>,
    pub decal_prefabs: Vec<Overridable<DecalPrefab>>,
}

impl DecalsFile {
    pub fn new(mut element: Node) -> Self {
        let mut r = Self {
            grime_sprites: Vec::new(),
            decal_prefabs: Vec::new(),
        };

        let is_all_override = if element.tag_name().name().eq_ignore_ascii_case("override") {
            element = element.children().find(Node::is_element).unwrap();
            true
        } else {
            false
        };

        for mut child in element.children().filter(Node::is_element) {
            let elem_name = child.tag_name().name().to_lowercase();
            let is_child_override = elem_name.eq_ignore_ascii_case("override");
            if is_child_override {
                child = child.children().find(Node::is_element).unwrap();
            }
            let is_override = is_all_override || is_child_override;
            match elem_name.as_str() {
                "grime" => {
                    r.grime_sprites.push(Overridable {
                        value: BarotraumaSprite::new(child),
                        is_override,
                    });
                }
                _ => {
                    r.decal_prefabs.push(Overridable {
                        value: DecalPrefab::new(child),
                        is_override,
                    });
                }
            }
        }

        r.grime_sprites.shrink_to_fit();
        r.decal_prefabs.shrink_to_fit();

        r
    }
}

impl XmlContentFile for DecalsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
