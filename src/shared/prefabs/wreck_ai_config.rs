use roxmltree::Node;

use crate::shared::util::NodeExp;

#[derive(Debug)]
pub struct WreckAIConfig {
    pub identifier: String,
    pub defensive_agent: Option<String>,
    pub offensive_agent: Option<String>,
    pub brain: Option<String>,
    pub spawner: Option<String>,
    pub brain_room_background: Option<String>,
    pub brain_room_vertical_wall: Option<String>,
    pub brain_room_horizontal_wall: Option<String>,
    pub agent_spawn_delay: f32,
    pub agent_spawn_delay_random_factor: f32,
    pub agent_spawn_delay_difficulty_multiplier: f32,
    pub agent_spawn_count_difficulty_multiplier: f32,
    pub min_agents_per_brain_room: u32,
    pub max_agents_per_room: u32,
    pub min_agents_outside: u32,
    pub max_agents_outside: u32,
    pub min_agents_inside: u32,
    pub max_agents_inside: u32,
    pub max_agent_count: u32,
    pub min_water_level: f32,
    pub kill_agents_when_entity_dies: bool,
    pub dead_entity_color_multiplier: f32,
    pub dead_entity_color_fade_out_time: f32,
    pub forbidden_ammunition: Vec<String>,
}

impl WreckAIConfig {
    pub fn new(element: Node) -> Self {
        Self {
            identifier: element
                .attribute_ignore_ascii_case("Entity")
                .map(|v| v.to_owned())
                .unwrap(),
            defensive_agent: element
                .attribute_ignore_ascii_case("defensiveagent")
                .map(|v| v.to_owned()),
            offensive_agent: element
                .attribute_ignore_ascii_case("offensiveagent")
                .map(|v| v.to_owned()),
            brain: element
                .attribute_ignore_ascii_case("brain")
                .map(|v| v.to_owned()),
            spawner: element
                .attribute_ignore_ascii_case("spawner")
                .map(|v| v.to_owned()),
            brain_room_background: element
                .attribute_ignore_ascii_case("brainroombackground")
                .map(|v| v.to_owned()),
            brain_room_vertical_wall: element
                .attribute_ignore_ascii_case("brainroomverticalwall")
                .map(|v| v.to_owned()),
            brain_room_horizontal_wall: element
                .attribute_ignore_ascii_case("brainroomhorizontalwall")
                .map(|v| v.to_owned()),
            agent_spawn_delay: element
                .attribute_ignore_ascii_case("agentspawndelay")
                .map_or(60.0, |v| v.parse().unwrap()),
            agent_spawn_delay_random_factor: element
                .attribute_ignore_ascii_case("agentspawndelayrandomfactor")
                .map_or(0.5, |v| v.parse().unwrap()),
            agent_spawn_delay_difficulty_multiplier: element
                .attribute_ignore_ascii_case("agentspawndelaydifficultymultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            agent_spawn_count_difficulty_multiplier: element
                .attribute_ignore_ascii_case("agentspawncountdifficultymultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            min_agents_per_brain_room: element
                .attribute_ignore_ascii_case("minagentsperbrainroom")
                .map_or(0, |v| v.parse().unwrap()),
            max_agents_per_room: element
                .attribute_ignore_ascii_case("maxagentsperroom")
                .map_or(3, |v| v.parse().unwrap()),
            min_agents_outside: element
                .attribute_ignore_ascii_case("minagentsoutside")
                .map_or(2, |v| v.parse().unwrap()),
            max_agents_outside: element
                .attribute_ignore_ascii_case("maxagentsoutside")
                .map_or(5, |v| v.parse().unwrap()),
            min_agents_inside: element
                .attribute_ignore_ascii_case("minagentsinside")
                .map_or(3, |v| v.parse().unwrap()),
            max_agents_inside: element
                .attribute_ignore_ascii_case("maxagentsinside")
                .map_or(10, |v| v.parse().unwrap()),
            max_agent_count: element
                .attribute_ignore_ascii_case("maxagentcount")
                .map_or(15, |v| v.parse().unwrap()),
            min_water_level: element
                .attribute_ignore_ascii_case("minwaterlevel")
                .map_or(100.0, |v| v.parse().unwrap()),
            kill_agents_when_entity_dies: element
                .attribute_ignore_ascii_case("killagentswhenentitydies")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            dead_entity_color_multiplier: element
                .attribute_ignore_ascii_case("deadentitycolormultiplier")
                .map_or(1.0, |v| v.parse().unwrap()),
            dead_entity_color_fade_out_time: element
                .attribute_ignore_ascii_case("deadentitycolorfadeouttime")
                .map_or(1.0, |v| v.parse().unwrap()),
            forbidden_ammunition: element
                .attribute_ignore_ascii_case("forbiddenammunition")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap(),
        }
    }
}
