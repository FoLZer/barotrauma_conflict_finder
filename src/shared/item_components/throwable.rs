use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::holdable::HoldableComponent;

#[derive(Debug)]
pub struct ThrowableComponent {
    pub holdable: HoldableComponent,

    pub throw_force: f32,
}

impl ThrowableComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            holdable: HoldableComponent::from_xml(element),

            throw_force: element
                .attribute_ignore_ascii_case("throwforce")
                .map_or(1.0, |v| v.parse().unwrap()),
        }
    }
}
