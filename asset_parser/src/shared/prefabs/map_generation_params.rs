use std::{collections::HashMap, num::ParseIntError, ops::Sub, str::FromStr};

use glam::{DVec2, Vec2};
use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::{
    gui_style_prefabs::SpriteSheet,
    item_prefab::{BarotraumaSprite, Color},
};

#[derive(Debug)]
pub struct MapGenerationParams {
    pub show_locations: bool,
    pub show_level_type_names: bool,
    pub show_overlay: bool,
    pub difficulty_zones: u32,
    pub width: u32,
    pub height: u32,
    pub small_level_connection_length: f32,
    pub large_level_connection_length: f32,
    pub voronoi_site_interval: Point,
    pub voronoi_site_variance: Point,
    pub min_connection_distance: f32,
    pub min_location_distance: f32,
    pub connection_indicator_iteration_multiplier: f32,
    pub connection_indicator_displacement_multiplier: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub map_tile_scale: f32,
    pub location_icon_size: f32,
    pub location_connection_width: f32,
    pub indicator_color: Color,
    pub connection_color: Color,
    pub highlighted_connection_color: Color,
    pub unvisited_connection_color: Color,
    pub gate_count: Vec<u32>,
    pub connection_sprite: Option<BarotraumaSprite>,
    pub passed_connection_sprite: Option<BarotraumaSprite>,
    pub map_tiles: HashMap<String, Vec<BarotraumaSprite>>,
    pub fog_of_war_sprite: Option<BarotraumaSprite>,
    pub current_location_indicator: Option<BarotraumaSprite>,
    pub selected_location_indicator: Option<BarotraumaSprite>,
    pub decorative_graph_sprite: Option<SpriteSheet>,
    pub mission_icon: Option<BarotraumaSprite>,
    pub type_change_icon: Option<BarotraumaSprite>,
    pub radiation_params: Option<RadiationParams>,
}

impl MapGenerationParams {
    pub fn new(element: Node) -> Self {
        let show_locations = element
            .attribute_ignore_ascii_case("showlocations")
            .map_or(true, |v| v.parse().unwrap());
        let show_level_type_names = element
            .attribute_ignore_ascii_case("showleveltypenames")
            .map_or(true, |v| v.parse().unwrap());
        let show_overlay = element
            .attribute_ignore_ascii_case("showoverlay")
            .map_or(true, |v| v.parse().unwrap());
        let difficulty_zones = element
            .attribute_ignore_ascii_case("difficultyzones")
            .map_or(6, |v| v.parse::<u32>().unwrap());
        let width = element
            .attribute_ignore_ascii_case("width")
            .map_or(8000, |v| v.parse::<u32>().unwrap());
        let height = element
            .attribute_ignore_ascii_case("height")
            .map_or(500, |v| v.parse::<u32>().unwrap());
        let small_level_connection_length = element
            .attribute_ignore_ascii_case("smalllevelconnectionlength")
            .map_or(20.0, |v| v.parse::<f32>().unwrap());
        let large_level_connection_length = element
            .attribute_ignore_ascii_case("largelevelconnectionlength")
            .map_or(200.0, |v| v.parse::<f32>().unwrap());
        let voronoi_site_interval = element
            .attribute_ignore_ascii_case("voronoisiteinterval")
            .map_or(Point { x: 20, y: 20 }, |v| v.parse::<Point>().unwrap());
        let voronoi_site_variance = element
            .attribute_ignore_ascii_case("voronoisitevariance")
            .map_or(Point { x: 5, y: 5 }, |v| v.parse::<Point>().unwrap());
        let min_connection_distance = element
            .attribute_ignore_ascii_case("minconnectiondistance")
            .map_or(10.0, |v| v.parse::<f32>().unwrap());
        let min_location_distance = element
            .attribute_ignore_ascii_case("minlocationdistance")
            .map_or(5.0, |v| v.parse::<f32>().unwrap());
        let connection_indicator_iteration_multiplier = element
            .attribute_ignore_ascii_case("connectionindicatoriterationmultiplier")
            .map_or(0.1, |v| v.parse::<f32>().unwrap());
        let connection_indicator_displacement_multiplier = element
            .attribute_ignore_ascii_case("connectionindicatordisplacementmultiplier")
            .map_or(0.1, |v| v.parse::<f32>().unwrap());
        let min_zoom = element
            .attribute_ignore_ascii_case("minzoom")
            .map_or(0.75, |v| v.parse::<f32>().unwrap());
        let max_zoom = element
            .attribute_ignore_ascii_case("maxzoom")
            .map_or(1.5, |v| v.parse::<f32>().unwrap());
        let map_tile_scale = element
            .attribute_ignore_ascii_case("maptilescale")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let location_icon_size = element
            .attribute_ignore_ascii_case("locationiconsize")
            .map_or(15.0, |v| v.parse::<f32>().unwrap());
        let location_connection_width = element
            .attribute_ignore_ascii_case("locationconnectionwidth")
            .map_or(5.0, |v| v.parse::<f32>().unwrap());
        let indicator_color = element
            .attribute_ignore_ascii_case("indicatorcolor")
            .map_or(
                Color::Simple {
                    r: 220.0 / 255.0,
                    g: 220.0 / 255.0,
                    b: 100.0 / 255.0,
                    a: 1.0,
                },
                |v| v.parse::<Color>().unwrap(),
            );
        let connection_color = element
            .attribute_ignore_ascii_case("connectioncolor")
            .map_or(
                Color::Simple {
                    r: 150.0 / 255.0,
                    g: 150.0 / 255.0,
                    b: 150.0 / 255.0,
                    a: 1.0,
                },
                |v| v.parse::<Color>().unwrap(),
            );
        let highlighted_connection_color = element
            .attribute_ignore_ascii_case("highlightedconnectioncolor")
            .map_or(
                Color::Simple {
                    r: 150.0 / 255.0,
                    g: 150.0 / 255.0,
                    b: 150.0 / 255.0,
                    a: 1.0,
                },
                |v| v.parse::<Color>().unwrap(),
            );
        let unvisited_connection_color = element
            .attribute_ignore_ascii_case("unvisitedconnectioncolor")
            .map_or(
                Color::Simple {
                    r: 150.0 / 255.0,
                    g: 150.0 / 255.0,
                    b: 150.0 / 255.0,
                    a: 1.0,
                },
                |v| v.parse::<Color>().unwrap(),
            );

        let gate_count = element.attribute_ignore_ascii_case("gatecount").map_or(
            vec![1; difficulty_zones as usize],
            |v| {
                v.split(',')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            },
        );

        let mut connection_sprite = None;
        let mut passed_connection_sprite = None;
        let mut map_tiles: HashMap<String, Vec<BarotraumaSprite>> = HashMap::new();
        let mut fog_of_war_sprite = None;
        let mut current_location_indicator = None;
        let mut selected_location_indicator = None;
        let mut decorative_graph_sprite = None;
        let mut mission_icon = None;
        let mut type_change_icon = None;
        let mut radiation_params = None;
        for child in element.children().filter(Node::is_element) {
            match child.tag_name().name().to_lowercase().as_str() {
                "connectionsprite" => {
                    connection_sprite = Some(BarotraumaSprite::new(child));
                }
                "passedconnectionsprite" => {
                    passed_connection_sprite = Some(BarotraumaSprite::new(child));
                }
                "maptile" => {
                    let biome = child
                        .attribute_ignore_ascii_case("biome")
                        .map(|v| v.to_owned())
                        .unwrap();
                    if let Some(v) = map_tiles.get_mut(&biome) {
                        v.push(BarotraumaSprite::new(child));
                    } else {
                        map_tiles.insert(biome, vec![BarotraumaSprite::new(child)]);
                    }
                }
                "fogofwarsprite" => {
                    fog_of_war_sprite = Some(BarotraumaSprite::new(child));
                }
                "locationindicator" | "currentlocationindicator" => {
                    current_location_indicator = Some(BarotraumaSprite::new(child));
                }
                "selectedlocationindicator" => {
                    selected_location_indicator = Some(BarotraumaSprite::new(child));
                }
                "decorativegraphsprite" => {
                    decorative_graph_sprite = Some(SpriteSheet::new(child));
                }
                "missionicon" => {
                    mission_icon = Some(BarotraumaSprite::new(child));
                }
                "typechangeicon" => {
                    type_change_icon = Some(BarotraumaSprite::new(child));
                }
                "radiation_params" => {
                    radiation_params = Some(RadiationParams::new(child));
                }
                _ => (),
            }
        }

        Self {
            show_locations,
            show_level_type_names,
            show_overlay,
            difficulty_zones,
            width,
            height,
            small_level_connection_length,
            large_level_connection_length,
            voronoi_site_interval,
            voronoi_site_variance,
            min_connection_distance,
            min_location_distance,
            connection_indicator_iteration_multiplier,
            connection_indicator_displacement_multiplier,
            min_zoom,
            max_zoom,
            map_tile_scale,
            location_icon_size,
            location_connection_width,
            indicator_color,
            connection_color,
            highlighted_connection_color,
            unvisited_connection_color,
            gate_count,
            connection_sprite,
            passed_connection_sprite,
            map_tiles,
            fog_of_war_sprite,
            current_location_indicator,
            selected_location_indicator,
            decorative_graph_sprite,
            mission_icon,
            type_change_icon,
            radiation_params,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn distance_squared(&self, other: &Point) -> u64 {
        let dx = self.x.abs_diff(other.x) as u64;
        let dy = self.y.abs_diff(other.y) as u64;
        dx * dx + dy * dy
    }

    pub fn length_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',').map(|v| v.trim());
        let x = split
            .next()
            .ok_or(ParsePointError::NotEnoughComponents)?
            .parse::<i32>()?;
        let y = split
            .next()
            .ok_or(ParsePointError::NotEnoughComponents)?
            .parse::<i32>()?;
        Ok(Self { x, y })
    }
}

impl From<Point> for Vec2 {
    fn from(value: Point) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<Point> for DVec2 {
    fn from(value: Point) -> Self {
        Self {
            x: value.x as f64,
            y: value.y as f64,
        }
    }
}

impl From<Vec2> for Point {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl From<DVec2> for Point {
    fn from(value: DVec2) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
pub enum ParsePointError {
    NotEnoughComponents,
    ParseIntError(ParseIntError),
}
impl From<ParseIntError> for ParsePointError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

#[derive(Debug)]
pub struct RadiationParams {
    pub starting_radiation: f32,
    pub radiation_step: f32,
    pub critical_radiation_threshold: u32,
    pub minimum_outpost_amount: u32,
    pub animation_speed: f32,
    pub radiation_damage_delay: f32,
    pub radiation_damage_amount: f32,
    pub max_radiation: f32,
    pub radiation_area_color: Color,
    pub radiation_border_tint: Color,
    pub border_animation_speed: f32,
}

impl RadiationParams {
    pub fn new(element: Node) -> Self {
        Self {
            starting_radiation: element
                .attribute_ignore_ascii_case("startinganimation")
                .map_or(-100.0, |v| v.parse().unwrap()),
            radiation_step: element
                .attribute_ignore_ascii_case("radiationstep")
                .map_or(100.0, |v| v.parse().unwrap()),
            critical_radiation_threshold: element
                .attribute_ignore_ascii_case("criticalradiationthreshold")
                .map_or(10, |v| v.parse().unwrap()),
            minimum_outpost_amount: element
                .attribute_ignore_ascii_case("minimumoutpostamount")
                .map_or(3, |v| v.parse().unwrap()),
            animation_speed: element
                .attribute_ignore_ascii_case("animationspeed")
                .map_or(3.0, |v| v.parse().unwrap()),
            radiation_damage_delay: element
                .attribute_ignore_ascii_case("radiationdamagedelay")
                .map_or(10.0, |v| v.parse().unwrap()),
            radiation_damage_amount: element
                .attribute_ignore_ascii_case("radiationdamageamount")
                .map_or(1.0, |v| v.parse().unwrap()),
            max_radiation: element
                .attribute_ignore_ascii_case("maxradiation")
                .map_or(-1.0, |v| v.parse().unwrap()),
            radiation_area_color: element
                .attribute_ignore_ascii_case("radiationareacolor")
                .map_or(
                    Color::Simple {
                        r: 139.0 / 255.0,
                        g: 0.0,
                        b: 0.0,
                        a: 85.0 / 255.0,
                    },
                    |v| v.parse().unwrap(),
                ),
            radiation_border_tint: element
                .attribute_ignore_ascii_case("radiationbordertint")
                .map_or(
                    Color::Simple {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    |v| v.parse().unwrap(),
                ),
            border_animation_speed: element
                .attribute_ignore_ascii_case("borderanimationspeed")
                .map_or(16.66, |v| v.parse().unwrap()),
        }
    }
}
