use std::sync::Arc;

use roxmltree::Node;

use crate::{shared::submarine_info::SubmarineInfo, shared::util::XmlContentFile};

#[derive(Debug)]
pub struct SubmarineFile {
    pub submarine_info: Arc<SubmarineInfo>,
}

impl SubmarineFile {
    pub fn new(element: Node) -> Self {
        Self {
            submarine_info: Arc::new(SubmarineInfo::new(element)),
        }
    }
}

impl XmlContentFile for SubmarineFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
