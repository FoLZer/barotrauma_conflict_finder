use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::string_component::StringComponent;

#[derive(Debug)]
pub struct ConcatComponent {
    pub string_comp: StringComponent,

    pub max_output_length: u32,
    pub separator: String,
}

impl ConcatComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            string_comp: StringComponent::from_xml(element),

            max_output_length: element
                .attribute_ignore_ascii_case("maxoutputlength")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            separator: element
                .attribute_ignore_ascii_case("separator")
                .map(|v| v.to_owned())
                .unwrap(),
        }
    }
}
