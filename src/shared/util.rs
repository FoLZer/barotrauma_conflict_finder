use md5::{Digest, Md5, digest::Update};
use roxmltree::Node;

pub trait NodeExp<'a> {
    fn attribute_ignore_ascii_case(&self, name: &str) -> Option<&'a str>;
    fn has_attribute_ignore_ascii_case(&self, name: &str) -> bool;
}

impl<'a, 'input: 'a> NodeExp<'a> for Node<'a, 'input> {
    fn attribute_ignore_ascii_case(&self, name: &str) -> Option<&'a str> {
        self.attributes()
            .find(|a| a.name().eq_ignore_ascii_case(name))
            .map(|a| a.value())
    }

    fn has_attribute_ignore_ascii_case(&self, name: &str) -> bool {
        self.attributes()
            .any(|a| a.name().eq_ignore_ascii_case(name))
    }
}

#[derive(Debug, Clone)]
pub struct Overridable<T> {
    pub value: T,
    pub is_override: bool,
}

#[derive(Debug, Clone)]
pub struct PrefabWithKey<T> {
    pub key: u32,
    pub prefab: T,
}

impl<T> PrefabWithKey<T> {
    //identifier must be lowercase to be the same as vanilla barotrauma
    pub fn new(identifier: &str, prefab: T) -> Self {
        Self {
            key: {
                let hash: [u8; 16] = Md5::new().chain(identifier).finalize().into();

                ((identifier.len() as u32 & 0xFF) << 24)
                    | ((hash[hash.len() - 3] as u32) << 16)
                    | ((hash[hash.len() - 2] as u32) << 8)
                    | (hash[hash.len() - 1] as u32)
            },
            prefab,
        }
    }

    pub fn get_identifier(&self) -> u32 {
        self.key
    }
}

pub trait XmlContentFile {
    fn from_xml(element: Node) -> Self;
}

