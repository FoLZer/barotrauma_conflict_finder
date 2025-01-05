use std::collections::HashMap;

use roxmltree::Node;

use crate::shared::util::{NodeExp, XmlContentFile};

#[derive(Debug)]
pub struct TextFile {
    pub language_name: String,
    pub translated_name: Option<String>,
    pub no_whitespace: bool,
    pub texts: HashMap<String, Vec<String>>,
}

impl TextFile {
    pub fn new(element: Node) -> Self {
        let language_name = element
            .attribute_ignore_ascii_case("language")
            .unwrap()
            .to_owned();
        let translated_name = element
            .attribute_ignore_ascii_case("translatedname")
            .map(std::borrow::ToOwned::to_owned);
        let no_whitespace = element
            .attribute_ignore_ascii_case("nowhitespace")
            .is_some_and(|v| v.to_lowercase().parse().unwrap());
        let mut texts: HashMap<String, Vec<String>> = HashMap::new();
        for child in element.children().filter(Node::is_element) {
            let elem_name = child.tag_name().name().to_lowercase();
            if let Some(v) = texts.get_mut(&elem_name) {
                v.push(child.text().map_or(String::new(), |v| {
                    v.replace("\\n", "\n")
                        .replace("&amp;", "&")
                        .replace("&lt;", "<")
                        .replace("&gt;", ">")
                        .replace("&quot;", "\"")
                        .replace("&apos;", "'")
                }));
            } else {
                texts.insert(elem_name, vec![child.text().map_or(String::new(), |v| {
                    v.replace("\\n", "\n")
                        .replace("&amp;", "&")
                        .replace("&lt;", "<")
                        .replace("&gt;", ">")
                        .replace("&quot;", "\"")
                        .replace("&apos;", "'")
                })]);
            }
        }
        texts.values_mut().for_each(std::vec::Vec::shrink_to_fit);
        texts.shrink_to_fit();

        Self {
            language_name,
            translated_name,
            no_whitespace,
            texts,
        }
    }
}

impl XmlContentFile for TextFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
