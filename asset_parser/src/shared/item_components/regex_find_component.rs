use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct RegExFindComponent {
    pub item: ItemComponent,

    pub max_output_length: u32,
    pub output: String,
    pub use_capture_group: bool,
    pub output_empty_capture_group: bool,
    pub false_output: String,
    pub continuous_output: bool,
    pub expression: String,
}

impl RegExFindComponent {
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
            use_capture_group: element
                .attribute_ignore_ascii_case("usecapturegroup")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            output_empty_capture_group: element
                .attribute_ignore_ascii_case("outputemptycapturegroup")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            false_output: element
                .attribute_ignore_ascii_case("falseoutput")
                .map(|v| v.to_owned())
                .unwrap(),
            continuous_output: element
                .attribute_ignore_ascii_case("continuousoutput")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            expression: element
                .attribute_ignore_ascii_case("expression")
                .map(|v| v.to_owned())
                .unwrap(),
        }
    }
}
