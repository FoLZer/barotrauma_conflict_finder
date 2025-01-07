use std::str::FromStr;

use roxmltree::Node;

use crate::shared::{prefabs::item_prefab::DoesNotExistError, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct TrigonometricFunctionComponent {
    pub item: ItemComponent,

    pub function: FunctionType,
    pub use_radians: bool,
}

impl TrigonometricFunctionComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            function: element
                .attribute_ignore_ascii_case("function")
                .map_or(FunctionType::Sin, |v| v.parse().unwrap()),
            use_radians: element
                .attribute_ignore_ascii_case("useradians")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum FunctionType {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
}

impl FromStr for FunctionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            "asin" => Ok(Self::Asin),
            "acos" => Ok(Self::Acos),
            "atan" => Ok(Self::Atan),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
