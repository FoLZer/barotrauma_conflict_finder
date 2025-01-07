use roxmltree::Node;

use crate::shared::util::{NodeExp, XmlContentFile};

#[derive(Debug)]
pub struct NPCConversationFile {
    pub language: String,
    pub conversations: Vec<NPCConversation>,
}

impl NPCConversationFile {
    pub fn new(mut element: Node) -> Self {
        if element.tag_name().name().eq_ignore_ascii_case("override") {
            element = element.first_element_child().unwrap();
        }

        let language = element
            .attribute_ignore_ascii_case("language")
            .unwrap_or("English")
            .to_owned();
        let conversations = element
            .children()
            .filter(Node::is_element)
            .filter(|v| v.tag_name().name().eq_ignore_ascii_case("conversation"))
            .map(|child| NPCConversation::new(child))
            .collect();

        Self {
            language,
            conversations,
        }
    }
}

impl XmlContentFile for NPCConversationFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}

#[derive(Debug)]
pub struct NPCConversation {
    pub line: String,
    pub speaker_index: u32,
    pub allowed_jobs: Vec<String>,
    pub flags: Vec<String>,
    pub min_intensity: Option<f32>,
    pub max_intensity: Option<f32>,
    pub responses: Vec<NPCConversation>,
    pub require_next_line: bool,
}

impl NPCConversation {
    pub fn new(element: Node) -> Self {
        let line = element
            .attribute_ignore_ascii_case("line")
            .unwrap()
            .to_owned();
        let speaker_index = element
            .attribute_ignore_ascii_case("speaker")
            .map(|v| v.parse::<u32>().unwrap())
            .unwrap();
        let allowed_jobs = element
            .attribute_ignore_ascii_case("allowedjobs")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let flags = element
            .attribute_ignore_ascii_case("speakertags")
            .map(|v| {
                v.split(',')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let min_intensity = element
            .attribute_ignore_ascii_case("minintensity")
            .map(|v| v.parse::<f32>().unwrap());
        let max_intensity = element
            .attribute_ignore_ascii_case("maxintensity")
            .map(|v| v.parse::<f32>().unwrap());
        let responses = element
            .children()
            .filter(Node::is_element)
            .map(NPCConversation::new)
            .collect::<Vec<_>>();
        let require_next_line = element
            .attribute_ignore_ascii_case("require_next_line")
            .is_some_and(|v| v.parse::<bool>().unwrap());

        Self {
            line,
            speaker_index,
            allowed_jobs,
            flags,
            min_intensity,
            max_intensity,
            responses,
            require_next_line,
        }
    }
}
