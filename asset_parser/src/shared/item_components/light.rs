use roxmltree::Node;

use crate::shared::{prefabs::item_prefab::Color, util::NodeExp};

use super::powered::PoweredComponent;

#[derive(Debug)]
pub struct LightComponent {
    pub powered: PoweredComponent,

    pub range: f32,
    pub cast_shadows: bool,
    pub draw_behind_subs: bool,
    pub is_on: bool,
    pub flicker: f32,
    pub flicker_speed: f32,
    pub pulse_frequency: f32,
    pub pulse_amount: f32,
    pub blink_frequency: f32,
    pub light_color: Color,
    pub ignore_continuous_toggle: bool,
    pub alpha_blend: bool,
}

impl LightComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            powered: PoweredComponent::from_xml(element),

            range: element
                .attribute_ignore_ascii_case("range")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            cast_shadows: element
                .attribute_ignore_ascii_case("castshadows")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            draw_behind_subs: element
                .attribute_ignore_ascii_case("drawbehindsubs")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            is_on: element
                .attribute_ignore_ascii_case("ison")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            flicker: element
                .attribute_ignore_ascii_case("flicker")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            flicker_speed: element
                .attribute_ignore_ascii_case("flickerspeed")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            pulse_frequency: element
                .attribute_ignore_ascii_case("pulsefrequency")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            pulse_amount: element
                .attribute_ignore_ascii_case("pulseamount")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            blink_frequency: element
                .attribute_ignore_ascii_case("blinkfrequency")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            light_color: element
                .attribute_ignore_ascii_case("lightcolor")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            ignore_continuous_toggle: element
                .attribute_ignore_ascii_case("ignorecontinuoustoggle")
                .map_or(false, |v| v.parse().unwrap()),
            alpha_blend: element
                .attribute_ignore_ascii_case("alphablend")
                .map_or(true, |v| v.parse().unwrap()),
        }
    }
}
