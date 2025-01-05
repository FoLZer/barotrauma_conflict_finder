use roxmltree::Node;

use crate::{
    shared::{
        prefabs::order_prefab::{OrderCategoryIcon, OrderPrefab},
        util::Overridable,
    },
    shared::util::XmlContentFile,
};

#[derive(Debug)]
pub struct OrdersFile {
    pub order_prefabs: Vec<Overridable<OrderPrefab>>,
    pub order_category_icons: Vec<Overridable<OrderCategoryIcon>>,
}

impl OrdersFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            order_prefabs: Vec::new(),
            order_category_icons: Vec::new(),
        };

        r.load_from_node(element, false);
        r.order_prefabs.shrink_to_fit();
        r.order_category_icons.shrink_to_fit();

        r
    }

    fn matches_singular(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("order") || identifier.eq_ignore_ascii_case("ordercategory")
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("orders")
    }

    fn load_from_node(&mut self, element: Node, overriding: bool) {
        let elem_name = element.tag_name().name();
        if elem_name.eq_ignore_ascii_case("override") {
            element
                .children()
                .filter(Node::is_element)
                .for_each(|child| self.load_from_node(child, true));
        } else if elem_name.eq_ignore_ascii_case("clear") {
            todo!();
            //self.prefabs.add_override_file(OverrideFile {
            //    hash: self.hash.clone(),
            //    content_package_index: todo!(),
            //});
        } else if Self::matches_singular(elem_name) {
            if elem_name.eq_ignore_ascii_case("order") {
                self.order_prefabs.push(Overridable {
                    value: OrderPrefab::new(element),
                    is_override: overriding,
                });
            } else {
                self.order_category_icons.push(Overridable {
                    value: OrderCategoryIcon::new(element),
                    is_override: overriding,
                });
            }
        } else if Self::matches_plural(elem_name) {
            element
                .children()
                .filter(Node::is_element)
                .for_each(|child| self.load_from_node(child, overriding));
        } else {
            dbg!(elem_name);
            panic!() //TODO:
        }
    }
}

impl XmlContentFile for OrdersFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
