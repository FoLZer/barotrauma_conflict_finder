use roxmltree::Node;

use crate::{
    shared::{prefabs::event_manager_settings::EventManagerSettings, util::Overridable},
    shared::util::XmlContentFile,
};

#[derive(Debug)]
pub struct EventManagerSettingsFile {
    pub event_manager_settings: Vec<Overridable<EventManagerSettings>>,
}

impl EventManagerSettingsFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            event_manager_settings: Vec::new(),
        };

        r.load_from_node(element, false);
        r.event_manager_settings.shrink_to_fit();

        r
    }

    fn matches_singular(identifier: &str) -> bool {
        !Self::matches_plural(identifier)
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("EventManagerSettings")
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
            self.event_manager_settings.push(Overridable {
                value: EventManagerSettings::new(element),
                is_override: overriding,
            });
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

impl XmlContentFile for EventManagerSettingsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
