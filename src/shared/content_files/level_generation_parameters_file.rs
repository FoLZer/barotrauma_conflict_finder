use std::str::FromStr;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    prefabs::{
        item_prefab::{BarotraumaSprite, Color, DoesNotExistError},
        map_generation_params::Point,
    },
    submarine_info::{SubmarineClass, Vector2},
    util::{NodeExp, Overridable, PrefabWithKey, XmlContentFile},
};

#[derive(Debug)]
pub struct LevelGenerationParametersFile {
    pub biomes: Vec<Overridable<PrefabWithKey<Biome>>>,
    pub level_generation_params: Vec<Overridable<PrefabWithKey<LevelGenerationParameters>>>,
}

impl LevelGenerationParametersFile {
    pub fn new(element: Node) -> Self {
        let (mut biomes, mut level_generation_params) = Self::load_from_node(element, false);
        biomes.shrink_to_fit();
        level_generation_params.shrink_to_fit();
        Self {
            biomes,
            level_generation_params,
        }
    }

    fn load_from_node(
        element: Node,
        overriding: bool,
    ) -> (
        Vec<Overridable<PrefabWithKey<Biome>>>,
        Vec<Overridable<PrefabWithKey<LevelGenerationParameters>>>,
    ) {
        let mut params = Vec::new();
        let mut biomes = Vec::new();
        for element in element.children().filter(Node::is_element) {
            let elem_name = element.tag_name().name();
            if elem_name.eq_ignore_ascii_case("override") {
                let (mut b1, mut p1) = Self::load_from_node(element, true);
                params.append(&mut p1);
                biomes.append(&mut b1);
            } else if elem_name.eq_ignore_ascii_case("clear") {
                todo!();
                //self.prefabs.add_override_file(OverrideFile {
                //    hash: self.hash.clone(),
                //    content_package_index: todo!(),
                //});
            } else if elem_name.eq_ignore_ascii_case("biomes") {
                biomes.extend(element.children().filter(Node::is_element).map(|child| {
                    Overridable {
                        value: {
                            let b = Biome::new(child);
                            PrefabWithKey::new(&b.identifier.clone(), b)
                        },
                        is_override: overriding,
                    }
                }));
            } else {
                params.push(Overridable {
                    value: {
                        let v = LevelGenerationParameters::new(element);
                        PrefabWithKey::new(&v.identifier.clone(), v)
                    },
                    is_override: overriding,
                });
            }
        }
        (biomes, params)
    }
}

impl XmlContentFile for LevelGenerationParametersFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}

#[derive(Debug, Clone)]
pub struct Biome {
    pub identifier: String,
    pub old_identifier: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_end_biome: bool,
    pub end_biome_location_count: Option<u32>,
    pub allowed_zones: Option<Vec<u32>>,
    pub min_difficulty: Option<f32>,
    pub max_difficulty: Option<f32>,
    pub submarine_availability_overrides: Vec<SubmarineAvailability>,
    pub submarine_availability: Option<SubmarineAvailability>,
}

impl Biome {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(std::borrow::ToOwned::to_owned)
            .unwrap();
        let old_identifier = element
            .attribute_ignore_ascii_case("oldidentifier")
            .map(std::borrow::ToOwned::to_owned);
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(std::borrow::ToOwned::to_owned);
        let description = element
            .attribute_ignore_ascii_case("description")
            .map(std::borrow::ToOwned::to_owned);
        let is_end_biome = element
            .attribute_ignore_ascii_case("endbiome")
            .is_some_and(|v| v.parse::<bool>().unwrap());
        let end_biome_location_count = element
            .attribute_ignore_ascii_case("endbiomelocationcount")
            .map(|v| v.parse::<u32>().unwrap());
        let allowed_zones = element
            .attribute_ignore_ascii_case("AllowedZones")
            .map(|v| {
                v.split(',')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            });
        let min_difficulty = element
            .attribute_ignore_ascii_case("MinDifficulty")
            .map(|v| v.parse::<f32>().unwrap());
        let max_difficulty = element
            .attribute_ignore_ascii_case("MaxDifficulty")
            .map(|v| v.parse::<f32>().unwrap());

        let mut submarine_availability_overrides = Vec::new();
        let mut submarine_availability = None;
        if let Some(availability_element) = element
            .children()
            .filter(Node::is_element)
            .find(|v| v.tag_name().name().eq_ignore_ascii_case("submarines"))
        {
            submarine_availability = Some(SubmarineAvailability::new(availability_element));
            for child in availability_element
                .children()
                .filter(Node::is_element)
                .filter(|v| v.tag_name().name().eq_ignore_ascii_case("override"))
            {
                submarine_availability_overrides.push(SubmarineAvailability::new(child));
            }
        }

        Self {
            identifier,
            old_identifier,
            name,
            description,
            is_end_biome,
            end_biome_location_count,
            allowed_zones,
            min_difficulty,
            max_difficulty,
            submarine_availability_overrides,
            submarine_availability,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubmarineAvailability {
    pub location_type: Option<String>,
    pub class: SubmarineClass,
    pub max_tier: Option<u32>,
}

impl SubmarineAvailability {
    pub fn new(element: Node) -> Self {
        let location_type = element
            .attribute_ignore_ascii_case("locationtype")
            .map(std::borrow::ToOwned::to_owned);
        let class = element
            .attribute_ignore_ascii_case("class")
            .map_or(SubmarineClass::Undefined, |v| {
                v.parse::<SubmarineClass>().unwrap()
            });
        let max_tier = element
            .attribute_ignore_ascii_case("maxtier")
            .map(|v| v.parse::<u32>().unwrap());

        Self {
            location_type,
            class,
            max_tier,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LevelGenerationParameters {
    pub identifier: String,
    pub old_identifier: Option<String>,
    pub properties: LevelGenerationParametersProperties,
    pub allowed_biome_identifiers: Option<Vec<String>>,
    pub background_sprite: Option<BarotraumaSprite>,
    pub background_top_sprite: Option<BarotraumaSprite>,
    pub wall_sprite: Option<BarotraumaSprite>,
    pub wall_edge_sprite: Option<BarotraumaSprite>,
    pub destructible_wall_sprite: Option<BarotraumaSprite>,
    pub destructible_wall_edge_sprite: Option<BarotraumaSprite>,
    pub wall_sprite_destroyed: Option<BarotraumaSprite>,
    pub water_particles: Option<BarotraumaSprite>,
    pub flash_sound: Option<Sound>,
}

impl LevelGenerationParameters {
    pub fn new(element: Node) -> Self {
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(std::borrow::ToOwned::to_owned)
            .unwrap_or(element.tag_name().name().to_owned());
        let old_identifier = element
            .attribute_ignore_ascii_case("oldidentifier")
            .map(std::borrow::ToOwned::to_owned);
        let properties = LevelGenerationParametersProperties::new(element);
        let allowed_biome_identifiers = element.attribute_ignore_ascii_case("biomes").map(|v| {
            v.split(',')
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<_>>()
        });

        let mut background_sprite = None;
        let mut background_top_sprite = None;
        let mut wall_sprite = None;
        let mut wall_edge_sprite = None;
        let mut destructible_wall_sprite = None;
        let mut destructible_wall_edge_sprite = None;
        let mut wall_sprite_destroyed = None;
        let mut water_particles = None;
        let mut flash_sound = None;
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "background" => background_sprite = Some(BarotraumaSprite::new(child)),
                "backgroundtop" => background_top_sprite = Some(BarotraumaSprite::new(child)),
                "wall" => wall_sprite = Some(BarotraumaSprite::new(child)),
                "walledge" => wall_edge_sprite = Some(BarotraumaSprite::new(child)),
                "destructiblewall" => destructible_wall_sprite = Some(BarotraumaSprite::new(child)),
                "destructiblewalledge" => {
                    destructible_wall_edge_sprite = Some(BarotraumaSprite::new(child));
                }
                "walldestroyed" => wall_sprite_destroyed = Some(BarotraumaSprite::new(child)),
                "waterparticles" => water_particles = Some(BarotraumaSprite::new(child)),
                "flashsound" => flash_sound = Some(Sound::new(child)),
                _ => (),
            }
        }

        Self {
            identifier,
            old_identifier,
            properties,
            allowed_biome_identifiers,
            background_sprite,
            background_top_sprite,
            wall_sprite,
            wall_edge_sprite,
            destructible_wall_sprite,
            destructible_wall_edge_sprite,
            wall_sprite_destroyed,
            water_particles,
            flash_sound,
        }
    }

    pub fn use_random_ruin_count(&self) -> bool {
        self.properties.max_ruin_count > 0
    }
}

#[derive(Debug, Clone)]
pub struct Sound {
    pub file_path: String,
    pub range: f32,
    pub volume: f32,
    pub source_pool_index: SourcePoolIndex,
}

impl Sound {
    pub fn new(element: Node) -> Self {
        let file_path = element
            .attribute_ignore_ascii_case("file")
            .unwrap()
            .to_owned();
        let range = element
            .attribute_ignore_ascii_case("range")
            .map_or(1000.0, |v| v.parse::<f32>().unwrap());
        let volume = element
            .attribute_ignore_ascii_case("volume")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let source_pool_index = element
            .attribute_ignore_ascii_case("volume")
            .map_or(SourcePoolIndex::Default, |v| {
                v.parse::<SourcePoolIndex>().unwrap()
            });

        Self {
            file_path,
            range,
            volume,
            source_pool_index,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SourcePoolIndex {
    Default,
    Voice,
}

impl FromStr for SourcePoolIndex {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Default" => Ok(Self::Default),
            "Voice" => Ok(Self::Voice),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LevelGenerationParametersProperties {
    pub ty: LevelType,
    pub commonness: f32,
    pub min_level_difficulty: f32,
    pub max_level_difficulty: f32,
    pub ambient_light_color: Color,
    pub background_texture_color: Color,
    pub background_color: Color,
    pub wall_color: Color,
    pub water_particle_color: Color,
    pub start_position: Vec2,
    pub end_position: Vec2,
    pub force_outpost_position: Vec2,
    pub create_hole_next_to_end: bool,
    pub create_hole_to_abyss: bool,
    pub no_level_geometry: bool,
    pub level_object_amount: u32,
    pub background_creature_amount: u32,
    pub min_width: u32,
    pub max_width: u32,
    pub height: u32,
    pub initial_depth_min: u32,
    pub initial_depth_max: u32,
    pub min_tunnel_radius: u32,
    pub side_tunnel_count: Point,
    pub side_tunnel_variance: f32,
    pub min_side_tunnel_radius: Point,
    pub voronoi_site_interval: Point,
    pub voronoi_site_variance: Point,
    pub cell_subdivision_length: u32,
    pub cell_rounding_amount: f32,
    pub cell_irregularity: f32,
    pub main_path_node_interval_range: Vec2,
    pub main_path_variance: f32,
    pub cave_count: u32,
    pub item_count: u32,
    pub resource_interval_range: Vec2,
    pub cave_resource_interval_range: Vec2,
    pub resource_cluster_size_range: Vec2,
    pub resource_spawn_chance: f32,
    pub cave_resource_spawn_chance: f32,
    pub floating_ice_chunk_count: u32,
    pub island_count: u32,
    pub ice_spire_count: u32,
    pub abyss_island_count: u32,
    pub abyss_island_size_min: Point,
    pub abyss_island_size_max: Point,
    pub abyss_island_cave_probability: f32,
    pub abyss_resource_clusters_min: u32,
    pub abyss_resource_clusters_max: u32,
    pub sea_floor_depth: i32,
    pub sea_floor_variance: u32,
    pub mountain_count_min: u32,
    pub mountain_count_max: u32,
    pub mountain_height_min: u32,
    pub mountain_height_max: u32,
    pub ruin_count: u32,
    pub min_ruin_count: u32,
    pub max_ruin_count: u32,
    pub min_wreck_count: u32,
    pub max_wreck_count: u32,
    pub min_corpse_count: u32,
    pub max_corpse_count: u32,
    pub thalamus_probability: f32,
    pub wreck_hull_flooding_chance: f32,
    pub wreck_flooding_hull_min_water_percentage: f32,
    pub wreck_flooding_hull_max_water_percentage: f32,
    pub force_beacon_station: String,
    pub bottom_hole_probability: f32,
    pub water_particle_scale: f32,
    pub water_particle_velocity: Vec2,
    pub wall_texture_size: f32,
    pub wall_edge_texture_width: f32,
    pub flash_interval: Vec2,
    pub flash_color: Color,
    pub play_noise_loop_in_outpost_level: bool,
    pub water_ambience_volume: f32,
    pub wall_edge_expand_outwards_amount: f32,
    pub wall_edge_expand_inwards_amount: f32,
}

impl LevelGenerationParametersProperties {
    pub fn new(element: Node) -> Self {
        Self {
            ty: element
                .attribute_ignore_ascii_case("type")
                .map_or(LevelType::LocationConnection, |v| v.parse().unwrap()),
            commonness: element
                .attribute_ignore_ascii_case("commonness")
                .map_or(100.0, |v| v.parse().unwrap()),
            min_level_difficulty: element
                .attribute_ignore_ascii_case("minleveldifficulty")
                .map_or(0.0, |v| v.parse().unwrap()),
            max_level_difficulty: element
                .attribute_ignore_ascii_case("maxleveldifficulty")
                .map_or(100.0, |v| v.parse().unwrap()),
            ambient_light_color: element
                .attribute_ignore_ascii_case("ambientlightcolor")
                .map_or(
                    Color::Simple {
                        r: 27.0 / 255.0,
                        g: 30.0 / 255.0,
                        b: 36.0 / 255.0,
                        a: 1.0,
                    },
                    |v| v.parse().unwrap(),
                ),
            background_texture_color: element
                .attribute_ignore_ascii_case("backgroundtexturecolor")
                .map_or(
                    Color::Simple {
                        r: 20.0 / 255.0,
                        g: 40.0 / 255.0,
                        b: 50.0 / 255.0,
                        a: 1.0,
                    },
                    |v| v.parse().unwrap(),
                ),
            background_color: element
                .attribute_ignore_ascii_case("backgroundcolor")
                .map_or(
                    Color::Simple {
                        r: 20.0 / 255.0,
                        g: 40.0 / 255.0,
                        b: 50.0 / 255.0,
                        a: 1.0,
                    },
                    |v| v.parse().unwrap(),
                ),
            wall_color: element.attribute_ignore_ascii_case("wallcolor").map_or(
                Color::Simple {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                |v| v.parse().unwrap(),
            ),
            water_particle_color: element
                .attribute_ignore_ascii_case("waterparticlecolor")
                .map_or(
                    Color::Simple {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    |v| v.parse().unwrap(),
                ),
            start_position: element
                .attribute_ignore_ascii_case("startposition")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            end_position: element
                .attribute_ignore_ascii_case("endposition")
                .map_or(Vec2::new(1.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            force_outpost_position: element
                .attribute_ignore_ascii_case("forceoutpostposition")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            create_hole_next_to_end: element
                .attribute_ignore_ascii_case("createholenexttoend")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            create_hole_to_abyss: element
                .attribute_ignore_ascii_case("createholetoabyss")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            no_level_geometry: element
                .attribute_ignore_ascii_case("nolevelgeometry")
                .is_some_and(|v| v.to_lowercase().parse().unwrap()),
            level_object_amount: element
                .attribute_ignore_ascii_case("levelobjectamount")
                .map_or(1000, |v| v.parse().unwrap()),
            background_creature_amount: element
                .attribute_ignore_ascii_case("backgroundcreatureamount")
                .map_or(80, |v| v.parse().unwrap()),
            min_width: element
                .attribute_ignore_ascii_case("minwidth")
                .map_or(100_000, |v| v.parse().unwrap()),
            max_width: element
                .attribute_ignore_ascii_case("maxwidth")
                .map_or(100_000, |v| v.parse().unwrap()),
            height: element
                .attribute_ignore_ascii_case("height")
                .map_or(50000, |v| v.parse().unwrap()),
            initial_depth_min: element
                .attribute_ignore_ascii_case("initialdepthmin")
                .map_or(80000, |v| v.parse().unwrap()),
            initial_depth_max: element
                .attribute_ignore_ascii_case("initialdepthmax")
                .map_or(80000, |v| v.parse().unwrap()),
            min_tunnel_radius: element
                .attribute_ignore_ascii_case("mintunnelradius")
                .map_or(6500, |v| v.parse().unwrap()),
            side_tunnel_count: element
                .attribute_ignore_ascii_case("sidetunnelcount")
                .map_or(Point { x: 0, y: 1 }, |v| v.parse::<Point>().unwrap()),
            side_tunnel_variance: element
                .attribute_ignore_ascii_case("sidetunnelvariance")
                .map_or(0.5, |v| v.parse().unwrap()),
            min_side_tunnel_radius: element
                .attribute_ignore_ascii_case("minsidetunnelradius")
                .map_or(Point { x: 2000, y: 6000 }, |v| v.parse::<Point>().unwrap()),
            voronoi_site_interval: element
                .attribute_ignore_ascii_case("voronoisiteinterval")
                .map_or(Point { x: 3000, y: 3000 }, |v| v.parse::<Point>().unwrap()),
            voronoi_site_variance: element
                .attribute_ignore_ascii_case("voronoisitevariance")
                .map_or(Point { x: 700, y: 700 }, |v| v.parse::<Point>().unwrap()),
            cell_subdivision_length: element
                .attribute_ignore_ascii_case("cellsubdivisionlength")
                .map_or(5000, |v| v.parse().unwrap()),
            cell_rounding_amount: element
                .attribute_ignore_ascii_case("cellroundingamount")
                .map_or(0.5, |v| v.parse().unwrap()),
            cell_irregularity: element
                .attribute_ignore_ascii_case("cellirregularity")
                .map_or(0.1, |v| v.parse().unwrap()),
            main_path_node_interval_range: element
                .attribute_ignore_ascii_case("mainpathnodeintervalrange")
                .map_or(Vec2::new(5000.0, 10000.0), |v| {
                    v.parse::<Vector2>().unwrap().0
                }),
            main_path_variance: element
                .attribute_ignore_ascii_case("mainpathvariance")
                .map_or(0.5, |v| v.parse().unwrap()),
            cave_count: element
                .attribute_ignore_ascii_case("cavecount")
                .map_or(5, |v| v.parse().unwrap()),
            item_count: element
                .attribute_ignore_ascii_case("itemcount")
                .map_or(100, |v| v.parse().unwrap()),
            resource_interval_range: element
                .attribute_ignore_ascii_case("resourceintervalrange")
                .map_or(Vec2::new(19200.0, 38400.0), |v| {
                    v.parse::<Vector2>().unwrap().0
                }),
            cave_resource_interval_range: element
                .attribute_ignore_ascii_case("caveresourceintervalrange")
                .map_or(Vec2::new(9600.0, 19200.0), |v| {
                    v.parse::<Vector2>().unwrap().0
                }),
            resource_cluster_size_range: element
                .attribute_ignore_ascii_case("resourceclustersizerange")
                .map_or(Vec2::new(3.0, 6.0), |v| v.parse::<Vector2>().unwrap().0),
            resource_spawn_chance: element
                .attribute_ignore_ascii_case("resourcespawnchance")
                .map_or(0.3, |v| v.parse().unwrap()),
            cave_resource_spawn_chance: element
                .attribute_ignore_ascii_case("caveresourcespawnchance")
                .map_or(1.0, |v| v.parse().unwrap()),
            floating_ice_chunk_count: element
                .attribute_ignore_ascii_case("floatingicechunkcount")
                .map_or(0, |v| v.parse().unwrap()),
            island_count: element
                .attribute_ignore_ascii_case("islandcount")
                .map_or(0, |v| v.parse().unwrap()),
            ice_spire_count: element
                .attribute_ignore_ascii_case("icespirecount")
                .map_or(0, |v| v.parse().unwrap()),
            abyss_island_count: element
                .attribute_ignore_ascii_case("abyssislandcount")
                .map_or(5, |v| v.parse().unwrap()),
            abyss_island_size_min: element
                .attribute_ignore_ascii_case("abyssislandsizemin")
                .map_or(Point { x: 4000, y: 7000 }, |v| v.parse::<Point>().unwrap()),
            abyss_island_size_max: element
                .attribute_ignore_ascii_case("abyssislandsizemax")
                .map_or(Point { x: 8000, y: 10000 }, |v| v.parse::<Point>().unwrap()),
            abyss_island_cave_probability: element
                .attribute_ignore_ascii_case("abyssislandcaveprobability")
                .map_or(0.5, |v| v.parse().unwrap()),
            abyss_resource_clusters_min: element
                .attribute_ignore_ascii_case("abyss_esourceclustersmin")
                .map_or(10, |v| v.parse().unwrap()),
            abyss_resource_clusters_max: element
                .attribute_ignore_ascii_case("abyssresourceclustersmax")
                .map_or(40, |v| v.parse().unwrap()),
            sea_floor_depth: element
                .attribute_ignore_ascii_case("seafloordepth")
                .map_or(-300_000, |v| v.parse().unwrap()),
            sea_floor_variance: element
                .attribute_ignore_ascii_case("seafloorvariance")
                .map_or(1000, |v| v.parse().unwrap()),
            mountain_count_min: element
                .attribute_ignore_ascii_case("mountaincountmin")
                .map_or(0, |v| v.parse().unwrap()),
            mountain_count_max: element
                .attribute_ignore_ascii_case("mountaincountmax")
                .map_or(0, |v| v.parse().unwrap()),
            mountain_height_min: element
                .attribute_ignore_ascii_case("mountainheightmin")
                .map_or(1000, |v| v.parse().unwrap()),
            mountain_height_max: element
                .attribute_ignore_ascii_case("mountainheightmax")
                .map_or(5000, |v| v.parse().unwrap()),
            ruin_count: element
                .attribute_ignore_ascii_case("ruincount")
                .map_or(1, |v| v.parse().unwrap()),
            min_ruin_count: element
                .attribute_ignore_ascii_case("minruincount")
                .map_or(0, |v| v.parse().unwrap()),
            max_ruin_count: element
                .attribute_ignore_ascii_case("maxruincount")
                .map_or(0, |v| v.parse().unwrap()),
            min_wreck_count: element
                .attribute_ignore_ascii_case("minwreckcount")
                .map_or(1, |v| v.parse().unwrap()),
            max_wreck_count: element
                .attribute_ignore_ascii_case("maxwreckcount")
                .map_or(1, |v| v.parse().unwrap()),
            min_corpse_count: element
                .attribute_ignore_ascii_case("mincorpsecount")
                .map_or(1, |v| v.parse().unwrap()),
            max_corpse_count: element
                .attribute_ignore_ascii_case("maxcorpsecount")
                .map_or(5, |v| v.parse().unwrap()),
            thalamus_probability: element
                .attribute_ignore_ascii_case("thalamusprobability")
                .map_or(0.0, |v| v.parse().unwrap()),
            wreck_hull_flooding_chance: element
                .attribute_ignore_ascii_case("wreckhullfloodingchance")
                .map_or(0.5, |v| v.parse().unwrap()),
            wreck_flooding_hull_min_water_percentage: element
                .attribute_ignore_ascii_case("wreckfloodinghullminwaterpercentage")
                .map_or(0.1, |v| v.parse().unwrap()),
            wreck_flooding_hull_max_water_percentage: element
                .attribute_ignore_ascii_case("wreckfloodinghullmaxwaterpercentage")
                .map_or(1.0, |v| v.parse().unwrap()),
            force_beacon_station: element
                .attribute_ignore_ascii_case("forcebeaconstation")
                .unwrap_or("")
                .to_owned(),
            bottom_hole_probability: element
                .attribute_ignore_ascii_case("bottomholeprobability")
                .map_or(0.4, |v| v.parse().unwrap()),
            water_particle_scale: element
                .attribute_ignore_ascii_case("waterparticlescale")
                .map_or(1.0, |v| v.parse().unwrap()),
            water_particle_velocity: element
                .attribute_ignore_ascii_case("waterparticlevelocity")
                .map_or(Vec2::new(0.0, 10.0), |v| v.parse::<Vector2>().unwrap().0),
            wall_texture_size: element
                .attribute_ignore_ascii_case("walltexturesize")
                .map_or(2048.0, |v| v.parse().unwrap()),
            wall_edge_texture_width: element
                .attribute_ignore_ascii_case("walledgetexturewidth")
                .map_or(2048.0, |v| v.parse().unwrap()),
            flash_interval: element
                .attribute_ignore_ascii_case("flashinterval")
                .map_or(Vec2::new(0.0, 0.0), |v| v.parse::<Vector2>().unwrap().0),
            flash_color: element.attribute_ignore_ascii_case("flashcolor").map_or(
                Color::Simple {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
                |v| v.parse().unwrap(),
            ),
            play_noise_loop_in_outpost_level: element
                .attribute_ignore_ascii_case("playnoiseloopinoutpostlevel")
                .is_some_and(|v| v.to_lowercase().parse().unwrap()),
            water_ambience_volume: element
                .attribute_ignore_ascii_case("waterambiencevolume")
                .map_or(1.0, |v| v.parse().unwrap()),
            wall_edge_expand_outwards_amount: element
                .attribute_ignore_ascii_case("walledgeexpandoutwardsamount")
                .map_or(120.0, |v| v.parse().unwrap()),
            wall_edge_expand_inwards_amount: element
                .attribute_ignore_ascii_case("walledgeexpandinwardsamount")
                .map_or(1000.0, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LevelType {
    LocationConnection,
    Outpost,
}

impl FromStr for LevelType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "locationconnection" => Ok(Self::LocationConnection),
            "outpost" => Ok(Self::Outpost),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
