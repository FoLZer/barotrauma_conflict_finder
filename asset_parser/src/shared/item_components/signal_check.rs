use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct SignalCheckComponent {
    pub item: ItemComponent,

    pub max_output_length: u32,
    pub output: String,
    pub false_output: String,
    pub target_signal: String,
}

impl SignalCheckComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            max_output_length: element
                .attribute_ignore_ascii_case("maxoutputlength")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            output: element
                .attribute_ignore_ascii_case("output")
                .map(|v| v.to_owned())
                .unwrap(),
            false_output: element
                .attribute_ignore_ascii_case("falseoutput")
                .map(|v| v.to_owned())
                .unwrap(),
            target_signal: element
                .attribute_ignore_ascii_case("targetsignal")
                .map(|v| v.to_owned())
                .unwrap(),
        }
    }
}
