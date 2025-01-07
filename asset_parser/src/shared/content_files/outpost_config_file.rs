use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{prefabs::generation_configs::OutpostGenerationParams, util::Overridable},
};

#[derive(Debug)]
pub struct OutpostConfigFile {
    pub outpost_generation_params: Vec<Overridable<OutpostGenerationParams>>,
}

impl OutpostConfigFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            outpost_generation_params: Vec::new(),
        };

        r.load_from_node(element, false);
        r.outpost_generation_params.shrink_to_fit();

        r
    }

    fn matches_singular(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("OutpostConfig")
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("OutpostGenerationParameters")
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
            self.outpost_generation_params.push(Overridable {
                value: OutpostGenerationParams::new(element),
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

impl XmlContentFile for OutpostConfigFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
