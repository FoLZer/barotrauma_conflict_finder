use std::num::ParseIntError;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::util::NodeExp;

use super::{item_prefab::MapEntityProperties, map_generation_params::Point};

#[derive(Debug)]
pub struct ItemAssemblyPrefab {
    pub identifier: String,
    pub map_entity_properties: MapEntityProperties,
    pub name: Option<String>,
    pub description: Option<String>,
    pub contained_item_ids: Vec<u32>,
    pub display_entities: Vec<(String, Rect)>,
    pub bounds: Option<Rect>,
}

impl ItemAssemblyPrefab {
    pub fn new(element: Node) -> Self {
        let map_entity_properties = MapEntityProperties::new(element);
        let name = element
            .attribute_ignore_ascii_case("name")
            .map(std::borrow::ToOwned::to_owned);
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .map(|v| v.to_owned())
            .or(name.clone())
            .unwrap();
        let description = element
            .attribute_ignore_ascii_case("description")
            .map(std::borrow::ToOwned::to_owned);

        let mut contained_item_ids = Vec::new();
        for child in element.children().filter(Node::is_element) {
            let Some(container_element) = child.children().find(|child| {
                child
                    .tag_name()
                    .name()
                    .eq_ignore_ascii_case("itemcontainer")
            }) else {
                continue;
            };
            let contained_string = container_element
                .attribute_ignore_ascii_case("contained")
                .unwrap();
            for item_id_string in contained_string.split(',') {
                for id_str in item_id_string.split(';') {
                    if let Ok(id) = id_str.parse::<u32>() {
                        contained_item_ids.push(id);
                    }
                }
            }
        }

        let (mut min_x, mut min_y, mut max_x, mut max_y): (
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
        ) = (None, None, None, None);
        let mut display_entities = Vec::new();
        for child in element.children().filter(Node::is_element) {
            let id = child
                .attribute_ignore_ascii_case("ID")
                .map(|v| v.parse::<u32>().unwrap());
            if id.is_some_and(|id| id > 0 && contained_item_ids.contains(&id)) {
                continue;
            }
            let identifier = child
                .attribute_ignore_ascii_case("identifier")
                .map(std::borrow::ToOwned::to_owned)
                .unwrap_or(child.tag_name().name().to_lowercase());
            let rect = child.attribute_ignore_ascii_case("rect").map_or(
                Rect {
                    x: 0i32,
                    y: 0i32,
                    width: 0,
                    height: 0,
                },
                |v| Rect::from_str(v, false).unwrap(),
            );
            if !child
                .children()
                .any(|child| child.tag_name().name().eq_ignore_ascii_case("wire"))
            {
                min_x = Some(if let Some(v) = min_x {
                    v.min(rect.x)
                } else {
                    rect.x
                });
                min_y = Some(if let Some(v) = min_y {
                    v.min(rect.y - rect.height as i32)
                } else {
                    rect.y - rect.height as i32
                });
                max_x = Some(if let Some(v) = max_x {
                    v.min(rect.x + rect.width as i32)
                } else {
                    rect.x + rect.width as i32
                });
                max_y = Some(if let Some(v) = max_y {
                    v.min(rect.y)
                } else {
                    rect.y
                });
                if !child
                    .attribute_ignore_ascii_case("hideinassemblypreview")
                    .is_some_and(|v| v.parse().unwrap())
                {
                    display_entities.push((identifier, rect));
                }
            }
        }

        let bounds =
            min_x
                .zip(min_y)
                .zip(max_x.zip(max_y))
                .map(|((min_x, min_y), (max_x, max_y))| Rect {
                    x: min_x,
                    y: min_y,
                    width: (max_x - min_x).try_into().unwrap(),
                    height: (max_y - min_y).try_into().unwrap(),
                });

        contained_item_ids.shrink_to_fit();
        display_entities.shrink_to_fit();

        Self {
            identifier,
            map_entity_properties,
            name,
            description,
            contained_item_ids,
            display_entities,
            bounds,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn from_points(location: Point, size: (u32, u32)) -> Self {
        Self {
            x: location.x,
            y: location.y,
            width: size.0,
            height: size.1,
        }
    }
}

impl Rect {
    pub fn location(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn set_location(&mut self, value: Point) {
        self.x = value.x;
        self.y = value.y;
    }

    pub fn contains(&self, value: Point) -> bool {
        (((self.x <= value.x) && (value.x < (self.x + self.width as i32))) && (self.y <= value.y))
            && (value.y < (self.y + self.height as i32))
    }

    pub fn intersects(&self, value: Rect) -> bool {
        value.left() < self.right()
            && self.left() < value.right()
            && value.top() < self.bottom()
            && self.top() < value.bottom()
    }

    pub fn from_str(s: &str, require_size: bool) -> Result<Self, ParseRectError> {
        let mut spl = s.split(',');
        let x = spl
            .next()
            .ok_or(ParseRectError::NotEnoughComponents)?
            .parse()?;
        let y = spl
            .next()
            .ok_or(ParseRectError::NotEnoughComponents)?
            .parse()?;
        let width = if require_size {
            spl.next()
                .ok_or(ParseRectError::NotEnoughComponents)?
                .parse()?
        } else if let Some(s) = spl.next() {
            s.parse()?
        } else {
            0
        };
        let height = if require_size {
            spl.next()
                .ok_or(ParseRectError::NotEnoughComponents)?
                .parse()?
        } else if let Some(s) = spl.next() {
            s.parse()?
        } else {
            0
        };
        Ok(Self {
            x,
            y,
            width,
            height,
        })
    }

    pub fn inflate_vec(&mut self, amount: Vec2) {
        self.inflate(amount.x as i32, amount.y as i32)
    }

    pub fn inflate(&mut self, horizontal_amount: i32, vertical_amount: i32) {
        self.x -= horizontal_amount;
        self.y -= vertical_amount;
        self.width = self
            .width
            .checked_add_signed(horizontal_amount * 2)
            .unwrap();
        self.height = self.height.checked_add_signed(vertical_amount * 2).unwrap();
    }

    pub fn top(&self) -> i32 {
        self.y
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.height as i32
    }

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn right(&self) -> i32 {
        self.x + self.width as i32
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.width as i32 / 2,
            y: self.y + self.height as i32 / 2,
        }
    }

    pub fn union(self, other: Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        Rect {
            x,
            y,
            width: (self.right().max(other.right()) - x).try_into().unwrap(),
            height: (self.bottom().max(other.bottom()) - y).try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum ParseRectError {
    NotEnoughComponents,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseRectError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
