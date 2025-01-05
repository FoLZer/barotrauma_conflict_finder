use roxmltree::Node;

use crate::shared::{
    prefabs::{
        affliction_prefab::{AfflictionPrefab, AfflictionPrefabType},
        cpr_settings_prefab::CPRSettings,
        item_prefab::BarotraumaSprite,
    },
    util::{Overridable, XmlContentFile},
};

#[derive(Debug)]
pub struct AfflictionsFile {
    pub cpr_settings: Vec<Overridable<CPRSettings>>,
    pub damage_overlays: Vec<Overridable<DamageOverlay>>,
    pub affliction_prefabs: Vec<Overridable<AfflictionPrefab>>,
}

impl AfflictionsFile {
    fn parse_element(&mut self, element: Node, overriding: bool) {
        let elem_name = element.tag_name().name();
        if elem_name.eq_ignore_ascii_case("override") {
            element
                .children()
                .filter(Node::is_element)
                .for_each(|child| self.parse_element(child, true));
        } else if elem_name.eq_ignore_ascii_case("afflictions") {
            element
                .children()
                .filter(Node::is_element)
                .for_each(|child| self.parse_element(child, overriding));
        } else if elem_name.eq_ignore_ascii_case("cprsettings") {
            let cpr_settings = CPRSettings::new(element);
            self.cpr_settings.push(Overridable {
                value: cpr_settings,
                is_override: overriding,
            });
        } else if elem_name.eq_ignore_ascii_case("damageoverlay") {
            let damage_overlay = Overridable {
                value: DamageOverlay::new(element),
                is_override: overriding,
            };
            self.damage_overlays.push(damage_overlay);
        } else {
            let prefab_type = match elem_name {
                "Psychosis" | "AfflictionPsychosis" => AfflictionPrefabType::Psychosis,
                "Bleeding" | "AfflictionBleeding" => AfflictionPrefabType::Bleeding,
                "Husk" | "AfflictionHusk" => AfflictionPrefabType::Husk,
                "SpaceHerpes" | "AfflictionSpaceHerpes" => AfflictionPrefabType::SpaceHerpes,
                "BuffDurationIncrease" => AfflictionPrefabType::BuffDurationIncrease,
                _ => AfflictionPrefabType::Normal,
            };
            self.affliction_prefabs.push(Overridable {
                value: AfflictionPrefab::new(element, prefab_type),
                is_override: overriding,
            });
        }
    }

    pub fn new(element: Node) -> Self {
        let mut r = Self {
            cpr_settings: Vec::new(),
            damage_overlays: Vec::new(),
            affliction_prefabs: Vec::new(),
        };
        r.parse_element(element, false);
        r.cpr_settings.shrink_to_fit();
        r.damage_overlays.shrink_to_fit();
        r.affliction_prefabs.shrink_to_fit();

        r
    }
}

impl XmlContentFile for AfflictionsFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}

#[derive(Debug)]
pub struct DamageOverlay(pub BarotraumaSprite);

impl DamageOverlay {
    pub fn new(element: Node) -> Self {
        Self(BarotraumaSprite::new(element))
    }
}
