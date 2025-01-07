use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{prefabs::map_generation_params::MapGenerationParams, util::Overridable},
};

#[derive(Debug)]
pub struct MapGenerationParametersFile {
    pub map_generation_params: Overridable<MapGenerationParams>,
}

impl MapGenerationParametersFile {
    pub fn new(element: Node) -> Self {
        if element.tag_name().name().eq_ignore_ascii_case("override") {
            Self {
                map_generation_params: Overridable {
                    value: MapGenerationParams::new(
                        element.children().find(Node::is_element).unwrap(),
                    ),
                    is_override: true,
                },
            }
        } else {
            Self {
                map_generation_params: Overridable {
                    value: MapGenerationParams::new(element),
                    is_override: false,
                },
            }
        }
    }
}

impl XmlContentFile for MapGenerationParametersFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
