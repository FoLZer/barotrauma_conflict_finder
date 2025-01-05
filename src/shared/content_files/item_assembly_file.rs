use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::item_assembly_prefab::ItemAssemblyPrefab,
        util::{NodeExp, Overridable},
    },
};

#[derive(Debug)]
pub struct ItemAssemblyFile {
    pub item_assemblies: Vec<Overridable<ItemAssemblyPrefab>>,
}

impl ItemAssemblyFile {
    pub fn new(element: Node) -> Self {
        let mut item_assemblies = Self::load_from_node(element, false);
        item_assemblies.shrink_to_fit();
        Self { item_assemblies }
    }

    fn matches_singular(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("itemassembly")
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("itemassemblies")
    }

    fn load_from_node(element: Node, overriding: bool) -> Vec<Overridable<ItemAssemblyPrefab>> {
        let elem_name = element.tag_name().name();
        if elem_name.eq_ignore_ascii_case("override") {
            element
                .children()
                .filter(Node::is_element)
                .flat_map(|child| Self::load_from_node(child, true))
                .collect()
        } else if elem_name.eq_ignore_ascii_case("clear") {
            todo!();
            //self.prefabs.add_override_file(OverrideFile {
            //    hash: self.hash.clone(),
            //    content_package_index: todo!(),
            //});
        } else if Self::matches_singular(elem_name) {
            if element.has_attribute_ignore_ascii_case("identifier")
                || element.has_attribute_ignore_ascii_case("name")
            {
                vec![Overridable {
                    value: ItemAssemblyPrefab::new(element),
                    is_override: overriding,
                }]
            } else {
                println!(
                    "Failed to create an ItemAssemblyPrefab since it doesn't contain an identifier."
                );
                vec![]
            }
        } else if Self::matches_plural(elem_name) {
            element
                .children()
                .filter(Node::is_element)
                .flat_map(|child| Self::load_from_node(child, overriding))
                .collect()
        } else {
            dbg!(elem_name);
            panic!() //TODO:
        }
    }
}

impl XmlContentFile for ItemAssemblyFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
