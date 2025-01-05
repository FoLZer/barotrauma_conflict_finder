use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::sound_prefab::{
            BackgroundMusicPrefabs, DamageSoundPrefab, GUISoundPrefab, SoundPrefab,
        },
        util::Overridable,
    },
};

#[derive(Debug)]
pub struct SoundsFile {
    pub sound_prefabs: Vec<Overridable<SoundPrefab>>,
    pub damage_sound_prefabs: Vec<Overridable<DamageSoundPrefab>>,
    pub background_music_prefabs: Vec<Overridable<BackgroundMusicPrefabs>>,
    pub gui_sound_prefabs: Vec<Overridable<GUISoundPrefab>>,
}

impl SoundsFile {
    pub fn new(element: Node) -> Self {
        let mut r = Self {
            sound_prefabs: Vec::new(),
            damage_sound_prefabs: Vec::new(),
            background_music_prefabs: Vec::new(),
            gui_sound_prefabs: Vec::new(),
        };

        r.load_from_node(element, false);
        r.sound_prefabs.shrink_to_fit();
        r.damage_sound_prefabs.shrink_to_fit();
        r.background_music_prefabs.shrink_to_fit();
        r.gui_sound_prefabs.shrink_to_fit();

        r
    }

    fn matches_singular(identifier: &str) -> bool {
        !Self::matches_plural(identifier)
    }

    fn matches_plural(identifier: &str) -> bool {
        identifier.eq_ignore_ascii_case("sounds")
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
            match elem_name {
                "music" => {
                    self.background_music_prefabs.push(Overridable {
                        value: BackgroundMusicPrefabs::new(element),
                        is_override: overriding,
                    });
                }
                "damagesound" => {
                    self.damage_sound_prefabs.push(Overridable {
                        value: DamageSoundPrefab::new(element),
                        is_override: overriding,
                    });
                }
                "guisound" => {
                    self.gui_sound_prefabs.push(Overridable {
                        value: GUISoundPrefab::new(element),
                        is_override: overriding,
                    });
                }
                _ => {
                    self.sound_prefabs.push(Overridable {
                        value: SoundPrefab::new(element),
                        is_override: overriding,
                    });
                }
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

impl XmlContentFile for SoundsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}
