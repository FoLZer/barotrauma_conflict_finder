use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct GeneticMaterialComponent {
    pub item: ItemComponent,

    pub effect: String,
    pub tainted_effect: String,
    pub tainted: bool,
    pub selected_tainted_effect: String,
    pub condition_increase_on_combine_min: f32,
    pub condition_increase_on_combine_max: f32,
    pub name_identifier: Option<String>,
}

impl GeneticMaterialComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            effect: element
                .attribute_ignore_ascii_case("effect")
                .map(|v| v.to_owned())
                .unwrap(),
            tainted_effect: element
                .attribute_ignore_ascii_case("taintedeffect")
                .map(|v| v.to_owned())
                .unwrap(),
            tainted: element
                .attribute_ignore_ascii_case("tainted")
                .map_or(false, |v| v.parse().unwrap()),
            selected_tainted_effect: element
                .attribute_ignore_ascii_case("selectedtaintedeffect")
                .map(|v| v.to_owned())
                .unwrap(),
            condition_increase_on_combine_min: element
                .attribute_ignore_ascii_case("conditionincreaseoncombinemin")
                .map_or(3.0, |v| v.parse().unwrap()),
            condition_increase_on_combine_max: element
                .attribute_ignore_ascii_case("conditionincreaseoncombinemax")
                .map_or(8.0, |v| v.to_lowercase().parse().unwrap()),
            name_identifier: element
                .attribute_ignore_ascii_case("nameidentifier")
                .map(|v| v.to_owned()),
        }
    }
}
