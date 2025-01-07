use std::str::FromStr;

use roxmltree::Node;

use crate::shared::{prefabs::item_prefab::DoesNotExistError, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct FunctionComponent {
    pub item: ItemComponent,

    pub function: FunctionType,
}

impl FunctionComponent {
    pub fn from_xml(element: &Node) -> Self {
        Self {
            item: ItemComponent::from_xml(element),

            function: element
                .attribute_ignore_ascii_case("function")
                .map_or(FunctionType::Round, |v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
pub enum FunctionType {
    Round,
    Ceil,
    Floor,
    Factorial,
    AbsoluteValue,
    SquareRoot,
}

impl FromStr for FunctionType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "round" => Ok(Self::Round),
            "ceil" => Ok(Self::Ceil),
            "floor" => Ok(Self::Floor),
            "factorial" => Ok(Self::Factorial),
            "absolutevalue" => Ok(Self::AbsoluteValue),
            "squareroot" => Ok(Self::SquareRoot),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
