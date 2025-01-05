use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::event_prefab::{EventPrefab, EventSet, EventSprite, TraitorEventPrefab},
        util::Overridable,
    },
};

#[derive(Debug)]
pub struct RandomEventsFile {
    pub traitor_event_prefabs: Vec<Overridable<TraitorEventPrefab>>,
    pub event_prefabs: Vec<Overridable<EventPrefab>>,
    pub event_sprites: Vec<Overridable<EventSprite>>,
    pub event_sets: Vec<Overridable<EventSet>>,
}

impl RandomEventsFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            traitor_event_prefabs: Vec::new(),
            event_prefabs: Vec::new(),
            event_sprites: Vec::new(),
            event_sets: Vec::new(),
        };
        r.load_from_node(element, false);
        r.traitor_event_prefabs.shrink_to_fit();
        r.event_prefabs.shrink_to_fit();
        r.event_sprites.shrink_to_fit();
        r.event_sets.shrink_to_fit();

        r
    }

    fn load_from_node(&mut self, element: Node, overriding: bool) {
        let elem_name = element.tag_name().name();
        if elem_name.eq_ignore_ascii_case("override") {
            element
                .children()
                .filter(Node::is_element)
                .for_each(|child| self.load_from_node(child, true))
        } else if elem_name.eq_ignore_ascii_case("randomevents") {
            element
                .children()
                .filter(Node::is_element)
                .for_each(|child| self.load_from_node(child, overriding))
        } else if elem_name.eq_ignore_ascii_case("eventprefabs") {
            for child in element.children().filter(Node::is_element) {
                if child.tag_name().name().eq_ignore_ascii_case("traitorevent") {
                    self.traitor_event_prefabs.push(Overridable {
                        value: TraitorEventPrefab::new(child),
                        is_override: overriding,
                    });
                } else {
                    self.event_prefabs.push(Overridable {
                        value: EventPrefab::new(child, None),
                        is_override: overriding,
                    });
                }
            }
        } else if elem_name.eq_ignore_ascii_case("eventsprites") {
            self.event_sprites
                .extend(
                    element
                        .children()
                        .filter(Node::is_element)
                        .map(|child| Overridable {
                            value: EventSprite::new(child),
                            is_override: overriding,
                        }),
                );
        } else if elem_name.eq_ignore_ascii_case("eventset") {
            self.event_sets.push(Overridable {
                value: EventSet::new(element),
                is_override: overriding,
            });
        } else if elem_name.eq_ignore_ascii_case("clear") {
            todo!();
        } else {
            dbg!(elem_name);
            panic!() //TODO:
        }
    }
}

impl XmlContentFile for RandomEventsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
