use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct ExponentiationComponent {
    pub item: ItemComponent,

    pub exponent: f32,
}

impl ExponentiationComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            exponent: element
                .attribute_ignore_ascii_case("exponent")
                .map_or(1.0, |v| v.parse().unwrap()),
        }
    }
}
