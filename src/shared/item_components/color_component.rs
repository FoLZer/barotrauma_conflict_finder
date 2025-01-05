use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct ColorComponent {
    pub item: ItemComponent,

    pub use_hsv: bool,
}

impl ColorComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            use_hsv: element
                .attribute_ignore_ascii_case("usehsv")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}
