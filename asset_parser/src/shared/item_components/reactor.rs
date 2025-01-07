use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct ReactorComponent {
    pub powered: PoweredComponent,

    pub power_on: bool,
    pub last_user_was_player: bool,
    pub max_power_output: f32,
    pub meltdown_delay: f32,
    pub fire_delay: f32,
    pub temperature: f32,
    pub fission_rate: f32,
    pub turbine_output: f32,
    pub fuel_consumption_rate: f32,
    pub temperature_critical: bool,
    pub auto_temp: bool,
    pub available_fuel: f32,
    pub load: f32,
    pub target_fission_rate: f32,
    pub target_turbine_output: f32,
    pub correct_turbine_output: f32,
    pub explosion_damages_other_subs: bool,
}

impl ReactorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            power_on: element
                .attribute_ignore_ascii_case("poweron")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            last_user_was_player: element
                .attribute_ignore_ascii_case("lastuserwasplayer")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            max_power_output: element
                .attribute_ignore_ascii_case("maxpoweroutput")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            meltdown_delay: element
                .attribute_ignore_ascii_case("meltdowndelay")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            fire_delay: element
                .attribute_ignore_ascii_case("firedelay")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            temperature: element
                .attribute_ignore_ascii_case("temperature")
                .map_or(0.0, |v| v.parse().unwrap()),
            fission_rate: element
                .attribute_ignore_ascii_case("fissionrate")
                .map_or(0.0, |v| v.parse().unwrap()),
            turbine_output: element
                .attribute_ignore_ascii_case("turbineoutput")
                .map_or(0.0, |v| v.parse().unwrap()),
            fuel_consumption_rate: element
                .attribute_ignore_ascii_case("fuelconsumptionrate")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            temperature_critical: element
                .attribute_ignore_ascii_case("temperaturecritical")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            auto_temp: element
                .attribute_ignore_ascii_case("autotemp")
                .map_or(false, |v| v.to_lowercase().parse().unwrap()),
            available_fuel: element
                .attribute_ignore_ascii_case("availablefuel")
                .map_or(0.0, |v| v.parse().unwrap()),
            load: element
                .attribute_ignore_ascii_case("load")
                .map_or(0.0, |v| v.parse().unwrap()),
            target_fission_rate: element
                .attribute_ignore_ascii_case("targetfissionrate")
                .map_or(0.0, |v| v.parse().unwrap()),
            target_turbine_output: element
                .attribute_ignore_ascii_case("targetturbineoutput")
                .map_or(0.0, |v| v.parse().unwrap()),
            correct_turbine_output: element
                .attribute_ignore_ascii_case("correctturbineoutput")
                .map_or(0.0, |v| v.parse().unwrap()),
            explosion_damages_other_subs: element
                .attribute_ignore_ascii_case("explosiondamagesothersubs")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
