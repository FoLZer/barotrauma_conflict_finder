use roxmltree::Node;

pub struct ContentPackage {
    pub path: String,
}

impl ContentPackage {
    pub fn from_xml(element: Node) -> Self {
        Self {
            path: element.attribute("path").unwrap().to_owned(),
        }
    }
}

pub struct ContentPackages {
    pub core_package: ContentPackage,
    pub regular_packages: Vec<ContentPackage>,
}

pub struct PlayerConfigFile {
    pub content_packages: ContentPackages,
}

impl PlayerConfigFile {
    pub fn from_xml(element: Node) -> Self {
        let content_packages_node = element
            .children()
            .filter(Node::is_element)
            .find(|v| v.tag_name().name().eq_ignore_ascii_case("contentpackages"))
            .unwrap();
        let regular_packages_node = content_packages_node
            .children()
            .filter(Node::is_element)
            .find(|v| v.tag_name().name().eq_ignore_ascii_case("regularpackages"))
            .unwrap();
        Self {
            content_packages: ContentPackages {
                core_package: ContentPackage::from_xml(
                    content_packages_node
                        .children()
                        .filter(Node::is_element)
                        .find(|v| v.tag_name().name().eq_ignore_ascii_case("corepackage"))
                        .unwrap(),
                ),
                regular_packages: regular_packages_node
                    .children()
                    .filter(Node::is_element)
                    .map(|v| ContentPackage::from_xml(v))
                    .collect(),
            },
        }
    }
}
