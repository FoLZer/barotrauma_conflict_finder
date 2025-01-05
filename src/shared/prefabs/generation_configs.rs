use std::collections::HashSet;

use roxmltree::Node;

use crate::shared::{content_files::level_generation_parameters_file::LevelType, util::NodeExp};

use super::human_prefab::HumanPrefab;

#[derive(Debug)]
pub struct OutpostGenerationParams {
    pub identifier: String,
    pub name: Option<String>,
    pub allowed_location_types: Option<HashSet<String>>,
    pub force_to_end_location_index: i32,
    pub preferred_difficulty: i32,
    pub total_module_count: u32,
    pub append_to_reach_total_module_count: bool,
    pub min_hallway_length: f32,
    pub always_destructible: bool,
    pub always_rewireable: bool,
    pub allow_stealing: bool,
    pub spawn_crew_inside_outpost: bool,
    pub lock_unused_doors: bool,
    pub remove_unused_gaps: bool,
    pub draw_behind_subs: bool,
    pub min_water_percentage: f32,
    pub max_water_percentage: f32,
    pub replace_in_radiation: Option<String>,
    pub level_type: Option<LevelType>,
    pub outpost_file_path: Option<String>,
    pub module_counts: Vec<ModuleCount>,
    pub human_prefab_collections: Vec<Vec<HumanPrefab>>,
}

impl OutpostGenerationParams {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .unwrap();
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(|v| v.to_owned());
        let allowed_location_types = element
            .attribute_ignore_ascii_case("allowedlocationtypes")
            .map(|v| v.split(',').map(|v| v.to_owned()).collect::<HashSet<_>>());
        let force_to_end_location_index = element
            .attribute_ignore_ascii_case("forcetoendlocationindex")
            .map_or(-1, |v| v.parse::<i32>().unwrap());
        let preferred_difficulty = element
            .attribute_ignore_ascii_case("preferreddifficulty")
            .map_or(-1, |v| v.parse::<i32>().unwrap());
        let total_module_count = element
            .attribute_ignore_ascii_case("totalmodulecount")
            .map_or(10, |v| v.parse::<u32>().unwrap());
        let append_to_reach_total_module_count = element
            .attribute_ignore_ascii_case("appendtoreachtotalmodulecount")
            .map_or(true, |v| v.parse().unwrap());
        let min_hallway_length = element
            .attribute_ignore_ascii_case("minhallwaylength")
            .map_or(200.0, |v| v.parse::<f32>().unwrap());
        let always_destructible = element
            .attribute_ignore_ascii_case("alwaysdestructible")
            .map_or(false, |v| v.parse().unwrap());
        let always_rewireable = element
            .attribute_ignore_ascii_case("alwaysrewireable")
            .map_or(false, |v| v.parse().unwrap());
        let allow_stealing = element
            .attribute_ignore_ascii_case("allowstealing")
            .map_or(false, |v| v.parse().unwrap());
        let spawn_crew_inside_outpost = element
            .attribute_ignore_ascii_case("spawncrewinsideoutpost")
            .map_or(true, |v| v.parse().unwrap());
        let lock_unused_doors = element
            .attribute_ignore_ascii_case("lockunuseddoors")
            .map_or(true, |v| v.parse().unwrap());
        let remove_unused_gaps = element
            .attribute_ignore_ascii_case("removeunusedgaps")
            .map_or(true, |v| v.parse().unwrap());
        let draw_behind_subs = element
            .attribute_ignore_ascii_case("drawbehindsubs")
            .map_or(true, |v| v.parse().unwrap());
        let min_water_percentage = element
            .attribute_ignore_ascii_case("minwaterpercentage")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let max_water_percentage = element
            .attribute_ignore_ascii_case("maxwaterpercentage")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let replace_in_radiation = element
            .attribute_ignore_ascii_case("replaceinradiation")
            .map(|v| v.to_owned());
        let level_type = element
            .attribute_ignore_ascii_case("leveltype")
            .map(|v| v.parse::<LevelType>().unwrap());
        let outpost_file_path = element
            .attribute_ignore_ascii_case("outpostfilepath")
            .map(|v| v.to_owned());

        let mut module_counts = Vec::new();
        let mut human_prefab_collections = Vec::new();
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "modulecount" => {
                    module_counts.push(ModuleCount::new(child));
                }
                "npcs" => {
                    let mut npcs = Vec::new();
                    for child in child.children().filter(Node::is_element) {
                        let from = child
                            .attribute_ignore_ascii_case("from")
                            .map(|v| v.to_owned())
                            .unwrap();

                        npcs.push(HumanPrefab::new(child, Some(from)))
                    }
                    human_prefab_collections.push(npcs);
                }
                _ => (),
            }
        }

        Self {
            identifier,
            name,
            allowed_location_types,
            force_to_end_location_index,
            preferred_difficulty,
            total_module_count,
            append_to_reach_total_module_count,
            min_hallway_length,
            always_destructible,
            always_rewireable,
            allow_stealing,
            spawn_crew_inside_outpost,
            lock_unused_doors,
            remove_unused_gaps,
            draw_behind_subs,
            min_water_percentage,
            max_water_percentage,
            replace_in_radiation,
            level_type,
            outpost_file_path,
            module_counts,
            human_prefab_collections,
        }
    }
}

#[derive(Debug)]
pub struct ModuleCount {
    pub identifier: String,
    pub count: u32,
    pub order: u32,
    pub required_faction: Option<String>,
}

impl ModuleCount {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("flag")
            .or(element.attribute_ignore_ascii_case("moduletype"))
            .map(|v| v.to_owned())
            .unwrap();
        let count = element
            .attribute_ignore_ascii_case("count")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let order = element
            .attribute_ignore_ascii_case("order")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        let required_faction = element
            .attribute_ignore_ascii_case("requiredfaction")
            .map(|v| v.to_owned());

        Self {
            identifier,
            count,
            order,
            required_faction,
        }
    }
}

#[derive(Debug)]
pub struct RuinGenerationParams {
    pub outpost_generation_params: OutpostGenerationParams,
    pub is_mission_ready: bool,
}

impl RuinGenerationParams {
    pub fn new(element: Node) -> Self {
        Self {
            outpost_generation_params: OutpostGenerationParams::new(element),
            is_mission_ready: element
                .attribute_ignore_ascii_case("ismissionready")
                .map_or(true, |v| v.parse().unwrap()),
        }
    }
}
