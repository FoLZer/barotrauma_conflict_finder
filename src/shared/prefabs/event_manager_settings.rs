use roxmltree::Node;

use crate::shared::util::NodeExp;

#[derive(Debug)]
pub struct EventManagerSettings {
    pub identifier: String,
    pub event_threshold_increase: f32,
    pub default_event_threshold: f32,
    pub event_cooldown: f32,
    pub min_level_difficulty: f32,
    pub max_level_difficulty: f32,
    pub freeze_duration_when_crew_away: f32,
}

impl EventManagerSettings {
    pub fn new(element: Node) -> Self {
        let identifier = element.tag_name().name().to_lowercase();
        let event_threshold_increase = element
            .attribute_ignore_ascii_case("eventthresholdincrease")
            .map_or(0.0005, |v| v.parse::<f32>().unwrap());
        let default_event_threshold = element
            .attribute_ignore_ascii_case("defaulteventthreshold")
            .map_or(0.2, |v| v.parse::<f32>().unwrap());
        let event_cooldown = element
            .attribute_ignore_ascii_case("eventcooldown")
            .map_or(360.0, |v| v.parse::<f32>().unwrap());
        let min_level_difficulty = element
            .attribute_ignore_ascii_case("minleveldifficulty")
            .map_or(0.0, |v| v.parse::<f32>().unwrap());
        let max_level_difficulty = element
            .attribute_ignore_ascii_case("maxleveldifficulty")
            .map_or(100.0, |v| v.parse::<f32>().unwrap());
        let freeze_duration_when_crew_away = element
            .attribute_ignore_ascii_case("freezedurationwhencrewaway")
            .map_or(60.0 * 10.0, |v| v.parse::<f32>().unwrap());

        Self {
            identifier,
            event_threshold_increase,
            default_event_threshold,
            event_cooldown,
            min_level_difficulty,
            max_level_difficulty,
            freeze_duration_when_crew_away,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}
