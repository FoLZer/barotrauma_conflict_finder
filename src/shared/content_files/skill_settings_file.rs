use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{prefabs::skill_settings::SkillSettings, util::Overridable},
};

#[derive(Debug)]
pub struct SkillSettingsFile {
    pub skill_settings: Overridable<SkillSettings>,
}

impl SkillSettingsFile {
    pub fn new(element: Node) -> Self {
        if element.tag_name().name().eq_ignore_ascii_case("override") {
            Self {
                skill_settings: Overridable {
                    value: SkillSettings::new(element.children().find(Node::is_element).unwrap()),
                    is_override: true,
                },
            }
        } else {
            Self {
                skill_settings: Overridable {
                    value: SkillSettings::new(element),
                    is_override: false,
                },
            }
        }
    }
}

impl XmlContentFile for SkillSettingsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
