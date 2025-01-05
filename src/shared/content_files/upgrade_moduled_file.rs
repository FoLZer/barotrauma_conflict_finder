use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::upgrade_prefab::{UpgradeCategory, UpgradePrefab},
        util::Overridable,
    },
};

#[derive(Debug)]
pub struct UpgradeModulesFile {
    pub categories: Vec<Overridable<UpgradeCategory>>,
    pub prefabs: Vec<Overridable<UpgradePrefab>>,
}

impl UpgradeModulesFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            categories: Vec::new(),
            prefabs: Vec::new(),
        };

        r.load_from_node(element, false);
        r.categories.shrink_to_fit();
        r.prefabs.shrink_to_fit();

        r
    }

    fn matches_singular(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("upgrademodule")
            || identifier.eq_ignore_ascii_case("upgradecategory")
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("upgrademodules")
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
            if elem_name.eq_ignore_ascii_case("upgradecategory") {
                self.categories.push(Overridable {
                    value: UpgradeCategory::new(element),
                    is_override: overriding,
                });
            } else {
                self.prefabs.push(Overridable {
                    value: UpgradePrefab::new(element),
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

impl XmlContentFile for UpgradeModulesFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
