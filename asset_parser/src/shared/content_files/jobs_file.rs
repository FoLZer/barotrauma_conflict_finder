use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::job_prefab::{ItemRepairPriority, JobPrefab},
        util::Overridable,
    },
};

#[derive(Debug)]
pub struct JobsFile {
    pub item_repair_priorities: Vec<Overridable<ItemRepairPriority>>,
    pub jobs: Vec<Overridable<JobPrefab>>,
}

impl JobsFile {
    fn load_elements(&mut self, element: Node, overriding: bool) {
        for child in element.children().filter(Node::is_element) {
            let elem_name = child.tag_name().name();
            if elem_name.eq_ignore_ascii_case("ItemRepairPriorities") {
                for child in child.children().filter(Node::is_element) {
                    self.item_repair_priorities.push(Overridable {
                        value: ItemRepairPriority::new(child),
                        is_override: overriding,
                    });
                }
            } else if elem_name.eq_ignore_ascii_case("override") {
                self.load_elements(child, true);
            } else {
                self.jobs.push(Overridable {
                    value: JobPrefab::new(child),
                    is_override: overriding,
                })
            }
        }
    }

    pub fn new(element: Node) -> Self {
        let mut r = Self {
            item_repair_priorities: Vec::new(),
            jobs: Vec::new(),
        };

        r.load_elements(element, false);

        r
    }
}

impl XmlContentFile for JobsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
