use roxmltree::Node;

use crate::shared::{
    prefabs::cave_generation_params::CaveGenerationParams,
    util::{Overridable, PrefabWithKey, XmlContentFile},
};

#[derive(Debug)]
pub struct CaveGenerationParamsFile {
    pub cave_generation_params: Vec<Overridable<PrefabWithKey<CaveGenerationParams>>>,
}

impl CaveGenerationParamsFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            cave_generation_params: Vec::new(),
        };

        r.load_from_node(element, false);
        r.cave_generation_params.shrink_to_fit();

        r
    }

    fn matches_singular(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("cave")
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("cavegenerationparameters")
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
            self.cave_generation_params.push(Overridable {
                value: {
                    let v = CaveGenerationParams::new(element);

                    PrefabWithKey::new(&v.identifier.clone(), v)
                },
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

impl XmlContentFile for CaveGenerationParamsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
