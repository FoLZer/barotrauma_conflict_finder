use std::collections::HashMap;

use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::item_prefab::BarotraumaSprite;

//TODO: remove clone here
#[derive(Debug, Clone)]
pub struct CaveGenerationParams {
    pub identifier: String,
    pub commonness: f32,
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
    pub min_branch_count: u32,
    pub max_branch_count: u32,
    pub level_object_amount: u32,
    pub destructible_wall_ratio: f32,
    pub wall_sprite: Option<BarotraumaSprite>,
    pub wall_edge_sprite: Option<BarotraumaSprite>,
    pub override_commonness: HashMap<String, f32>,
}

impl CaveGenerationParams {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .unwrap()
            .to_owned();
        let commonness = element
            .attribute_ignore_ascii_case("commonness")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let min_width = element
            .attribute_ignore_ascii_case("minwidth")
            .map_or(8000, |v| v.parse::<u32>().unwrap());
        let max_width = element
            .attribute_ignore_ascii_case("maxwidth")
            .map_or(10000, |v| v.parse::<u32>().unwrap());
        let min_height = element
            .attribute_ignore_ascii_case("minheight")
            .map_or(8000, |v| v.parse::<u32>().unwrap());
        let max_height = element
            .attribute_ignore_ascii_case("maxheight")
            .map_or(10000, |v| v.parse::<u32>().unwrap());
        let min_branch_count = element
            .attribute_ignore_ascii_case("minbranchcount")
            .map_or(2, |v| v.parse::<u32>().unwrap());
        let max_branch_count = element
            .attribute_ignore_ascii_case("maxbranchcount")
            .map_or(4, |v| v.parse::<u32>().unwrap());
        let level_object_amount = element
            .attribute_ignore_ascii_case("levelobjectamount")
            .map_or(50, |v| v.parse::<u32>().unwrap());
        let destructible_wall_ratio = element
            .attribute_ignore_ascii_case("destructiblewallratio")
            .map_or(0.1, |v| v.parse::<f32>().unwrap());

        let mut wall_sprite = None;
        let mut wall_edge_sprite = None;
        let mut override_commonness = HashMap::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "wall" => {
                    wall_sprite = Some(BarotraumaSprite::new(child));
                }
                "walledge" => {
                    wall_edge_sprite = Some(BarotraumaSprite::new(child));
                }
                "overridecommonness" => {
                    let level_type = child
                        .attribute_ignore_ascii_case("leveltype")
                        .map(|v| v.to_owned())
                        .unwrap();
                    if !override_commonness.contains_key(&level_type) {
                        override_commonness.insert(
                            level_type,
                            child
                                .attribute_ignore_ascii_case("commonness")
                                .map(|v| v.parse::<f32>().unwrap())
                                .unwrap(),
                        );
                    }
                }
                _ => (),
            }
        }

        Self {
            identifier,
            commonness,
            min_width,
            max_width,
            min_height,
            max_height,
            min_branch_count,
            max_branch_count,
            level_object_amount,
            destructible_wall_ratio,
            wall_sprite,
            wall_edge_sprite,
            override_commonness,
        }
    }

    pub fn get_commonness(
        &self,
        generation_params_identifier: Option<&str>,
        biome_identifier: Option<&str>,
        abyss: bool,
    ) -> f32 {
        if let Some(identifier) = generation_params_identifier {
            if let Some(commonness) =
                self.override_commonness
                    .get(if abyss { "abyss" } else { identifier })
            {
                return *commonness;
            }
        }
        if let Some(identifier) = biome_identifier {
            if let Some(commonness) = self.override_commonness.get(identifier) {
                return *commonness;
            }
        }

        self.commonness
    }
}
