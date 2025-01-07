use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct ModuloComponent {
    pub item: ItemComponent,

    pub modulus: f32,
}

impl ModuloComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            modulus: element
                .attribute_ignore_ascii_case("modulus")
                .map(|v| v.parse().unwrap())
                .unwrap(),
        }
    }
}
