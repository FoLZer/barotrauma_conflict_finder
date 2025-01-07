use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::ItemComponent;

#[derive(Debug)]
pub struct CustomInterfaceComponent {
    pub item: ItemComponent,

    pub labels: Vec<String>,
    pub signals: Vec<String>,
    pub element_states: Option<Vec<bool>>,
}

impl CustomInterfaceComponent {
    pub fn from_xml(element: &Node) -> Self {
        //TODO: custom elements
        Self {
            item: ItemComponent::from_xml(element),

            labels: element
                .attribute_ignore_ascii_case("labels")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap(),
            signals: element
                .attribute_ignore_ascii_case("signals")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap(),
            element_states: element
                .attribute_ignore_ascii_case("elementstates")
                .map(|v| {
                    v.split(',')
                        .map(|v| v.to_lowercase().parse().unwrap())
                        .collect()
                }),
        }
    }
}
