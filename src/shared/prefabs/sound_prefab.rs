use std::str::FromStr;

use glam::Vec2;
use roxmltree::Node;

use crate::shared::{
    content_files::level_generation_parameters_file::SourcePoolIndex, submarine_info::Vector2,
    util::NodeExp,
};

use super::item_prefab::DoesNotExistError;

#[derive(Debug)]
pub struct SoundPrefab {
    pub identifier: String,
    pub sound_path: String,
    pub volume: f32,
    pub range: f32,
    pub source_pool_index: SourcePoolIndex,
}

impl SoundPrefab {
    pub fn new(element: Node) -> Self {
        let sound_path = element
            .attribute_ignore_ascii_case("file")
            .unwrap()
            .to_owned();
        let identifier = element
            .attribute_ignore_ascii_case("identifier")
            .or(element
                .attribute_ignore_ascii_case("path")
                .map(|v| v.rsplit_once('.').map(|v| v.0).unwrap_or(v)))
            .map_or(
                sound_path
                    .rsplit_once('.')
                    .map(|v| v.0.to_owned())
                    .unwrap_or(sound_path.clone()),
                |v| v.to_owned(),
            );

        let volume = element
            .attribute_ignore_ascii_case("volume")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let range = element
            .attribute_ignore_ascii_case("range")
            .map_or(1.0, |v| v.parse::<f32>().unwrap());
        let source_pool_index = element
            .attribute_ignore_ascii_case("sourcepool")
            .map_or(SourcePoolIndex::Default, |v| v.parse().unwrap());

        Self {
            identifier,
            sound_path,
            volume,
            range,
            source_pool_index,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub struct DamageSoundPrefab {
    pub sound_prefab: SoundPrefab,
    pub damage_range: Vec2,
    pub damage_type: Option<String>,
    pub ignore_muffling: bool,
    pub required_tag: Option<String>,
}

impl DamageSoundPrefab {
    pub fn new(element: Node) -> Self {
        Self {
            sound_prefab: SoundPrefab::new(element),
            damage_range: element
                .attribute_ignore_ascii_case("damagerange")
                .map_or(Vec2::ZERO, |v| v.parse::<Vector2>().unwrap().0),
            damage_type: element
                .attribute_ignore_ascii_case("damagetype")
                .map(|v| v.to_owned()),
            ignore_muffling: element
                .attribute_ignore_ascii_case("ignoremuffling")
                .map_or(false, |v| v.parse().unwrap()),
            required_tag: element
                .attribute_ignore_ascii_case("requiredtag")
                .map(|v| v.to_owned()),
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.sound_prefab.identifier
    }
}

#[derive(Debug)]
pub struct BackgroundMusicPrefabs {
    pub sound_prefab: SoundPrefab,
    pub ty: String,
    pub intensity_range: Vec2,
    pub duck_volume: bool,
    pub mute_intensity_tracks: bool,
    pub force_intensity_track: Option<f32>,
    pub volume: f32,
    pub continue_from_previous_time: bool,
}

impl BackgroundMusicPrefabs {
    pub fn new(element: Node) -> Self {
        Self {
            sound_prefab: SoundPrefab::new(element),
            ty: element
                .attribute_ignore_ascii_case("type")
                .map(|v| v.parse().unwrap())
                .unwrap(),
            intensity_range: element
                .attribute_ignore_ascii_case("intensityrange")
                .map_or(Vec2::new(0.0, 100.0), |v| v.parse::<Vector2>().unwrap().0),
            duck_volume: element
                .attribute_ignore_ascii_case("duckvolume")
                .map_or(false, |v| v.parse().unwrap()),
            mute_intensity_tracks: element
                .attribute_ignore_ascii_case("muteintensitytracks")
                .map_or(false, |v| v.parse().unwrap()),
            force_intensity_track: element
                .attribute_ignore_ascii_case("forceintensitytrack")
                .map(|v| v.parse().unwrap()),
            volume: element
                .attribute_ignore_ascii_case("volume")
                .map_or(1.0, |v| v.parse().unwrap()),
            continue_from_previous_time: element
                .attribute_ignore_ascii_case("continuefromprevioustime")
                .map_or(false, |v| v.parse().unwrap()),
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.sound_prefab.identifier
    }
}

#[derive(Debug)]
pub struct GUISoundPrefab {
    pub sound_prefab: SoundPrefab,
    pub ty: GUISoundType,
}

impl GUISoundPrefab {
    pub fn new(element: Node) -> Self {
        Self {
            sound_prefab: SoundPrefab::new(element),
            ty: element
                .attribute_ignore_ascii_case("guisoundtype")
                .map_or(GUISoundType::UIMessage, |v| v.parse().unwrap()),
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.sound_prefab.identifier
    }
}

#[derive(Debug)]
pub enum GUISoundType {
    UIMessage,
    ChatMessage,
    RadioMessage,
    DeadMessage,
    Select,
    PickItem,
    PickItemFail,
    DropItem,
    PopupMenu,
    Decrease,
    Increase,
    UISwitch,
    TickBox,
    ConfirmTransaction,
    Cart,
}
impl FromStr for GUISoundType {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UIMessage" => Ok(Self::UIMessage),
            "ChatMessage" => Ok(Self::ChatMessage),
            "RadioMessage" => Ok(Self::RadioMessage),
            "DeadMessage" => Ok(Self::DeadMessage),
            "Select" => Ok(Self::Select),
            "PickItem" => Ok(Self::PickItem),
            "PickItemFail" => Ok(Self::PickItemFail),
            "DropItem" => Ok(Self::DropItem),
            "PopupMenu" => Ok(Self::PopupMenu),
            "Decrease" => Ok(Self::Decrease),
            "Increase" => Ok(Self::Increase),
            "UISwitch" => Ok(Self::UISwitch),
            "TickBox" => Ok(Self::TickBox),
            "ConfirmTransaction" => Ok(Self::ConfirmTransaction),
            "Cart" => Ok(Self::Cart),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
