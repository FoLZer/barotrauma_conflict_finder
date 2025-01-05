use glam::Vec2;
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::ItemComponent;

#[derive(Debug)]
pub struct ItemContainerComponent {
    pub item: ItemComponent,

    pub capacity: u32,
    pub max_stack_size: u32,
    pub hide_items: bool,
    pub item_pos: Option<Vec2>,
    pub item_interval: Option<Vec2>,
    pub items_per_row: u32,
    pub draw_inventory: bool,
    pub allow_drag_and_drop: bool,
    pub allow_swapping_contained_items: bool,
    pub quick_use_moves_items_inside: bool,
    pub auto_interact_with_contained: bool,
    pub allow_access: bool,
    pub access_only_when_broken: bool,
    pub allow_access_when_dropped: bool,
    pub slots_per_row: u32,
    pub containable_restrictions: Vec<String>,
    pub auto_fill: bool,
    pub item_rotation: f32,
    pub spawn_with_id: Option<String>,
    pub spawn_with_id_when_broken: bool,
    pub auto_inject: bool,
    pub auto_inject_threshold: f32,
    pub remove_contained_items_on_deconstruct: bool,

    pub extra_stack_size: u32,
}

impl ItemContainerComponent {
    pub fn from_xml(element: &Node) -> Self {
        //TODO: elements
        Self {
            item: ItemComponent::from_xml(element),

            capacity: element
                .attribute_ignore_ascii_case("capacity")
                .map_or(5, |v| v.parse().unwrap()),
            max_stack_size: element
                .attribute_ignore_ascii_case("maxstacksize")
                .map_or(64, |v| v.parse().unwrap()),
            hide_items: element
                .attribute_ignore_ascii_case("hideitems")
                .map_or(true, |v| v.parse().unwrap()),
            item_pos: element
                .attribute_ignore_ascii_case("itempos")
                .map(|v| v.parse::<Vector2>().unwrap().0),
            item_interval: element
                .attribute_ignore_ascii_case("iteminterval")
                .map(|v| v.parse::<Vector2>().unwrap().0),
            items_per_row: element
                .attribute_ignore_ascii_case("itemsperrow")
                .map_or(100, |v| v.parse().unwrap()),
            draw_inventory: element
                .attribute_ignore_ascii_case("drawinventory")
                .map_or(true, |v| v.parse().unwrap()),
            allow_drag_and_drop: element
                .attribute_ignore_ascii_case("allowdraganddrop")
                .map_or(true, |v| v.parse().unwrap()),
            allow_swapping_contained_items: element
                .attribute_ignore_ascii_case("allowswappingcontaineditems")
                .map_or(true, |v| v.parse().unwrap()),
            quick_use_moves_items_inside: element
                .attribute_ignore_ascii_case("quickusemovesitemsinside")
                .map_or(true, |v| v.to_lowercase().parse().unwrap()),
            auto_interact_with_contained: element
                .attribute_ignore_ascii_case("autointeractwithcontained")
                .map_or(false, |v| v.parse().unwrap()),
            allow_access: element
                .attribute_ignore_ascii_case("allowaccess")
                .map_or(true, |v| v.parse().unwrap()),
            access_only_when_broken: element
                .attribute_ignore_ascii_case("accessonlywhenbroken")
                .map_or(false, |v| v.parse().unwrap()),
            allow_access_when_dropped: element
                .attribute_ignore_ascii_case("allowaccesswhendropped")
                .map_or(true, |v| v.parse().unwrap()),
            slots_per_row: element
                .attribute_ignore_ascii_case("slotsperrow")
                .map_or(5, |v| v.parse().unwrap()),
            containable_restrictions: element
                .attribute_ignore_ascii_case("containablerestrictions")
                .map(|v| v.split(',').map(|v| v.to_owned()).collect())
                .unwrap(),
            auto_fill: element
                .attribute_ignore_ascii_case("autofill")
                .map(|v| v.to_lowercase().parse().unwrap())
                .unwrap(),
            item_rotation: element
                .attribute_ignore_ascii_case("itemrotation")
                .map_or(0.0, |v| v.parse().unwrap()),
            spawn_with_id: element
                .attribute_ignore_ascii_case("spawnwithid")
                .map(|v| v.to_owned()),
            spawn_with_id_when_broken: element
                .attribute_ignore_ascii_case("spawnwithidwhenbroken")
                .map_or(false, |v| v.parse().unwrap()),
            auto_inject: element
                .attribute_ignore_ascii_case("autoinject")
                .map_or(false, |v| v.parse().unwrap()),
            auto_inject_threshold: element
                .attribute_ignore_ascii_case("autoinjectthreshold")
                .map_or(0.5, |v| v.parse().unwrap()),
            remove_contained_items_on_deconstruct: element
                .attribute_ignore_ascii_case("removecontaineditemsondeconstruct")
                .map_or(false, |v| v.parse().unwrap()),

            extra_stack_size: element
                .attribute_ignore_ascii_case("extrastacksize")
                .map_or(0, |v| v.parse().unwrap()),
        }
    }
}
