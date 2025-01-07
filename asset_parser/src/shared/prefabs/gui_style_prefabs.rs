use std::{borrow::Borrow, collections::HashMap, hash::Hash, str::FromStr};

use bitfield_struct::bitfield;
use glam::{Vec2, Vec4};
use roxmltree::Node;

use crate::shared::{submarine_info::Vector2, util::NodeExp};

use super::{
    item_assembly_prefab::Rect,
    item_prefab::{BarotraumaSprite, Color, DoesNotExistError, Vector4},
    level_object_prefab::TransitionMode,
};

#[derive(Debug, Clone)]
pub struct GUIFontPrefab {
    pub font_overrides_map: MultiKeyHashMap<String, ScalableFont>,
    pub force_upper_case: bool,
    pub fallback_font: ScalableFont,
}

impl GUIFontPrefab {
    pub const FALLBACK_FONT_SIZE: Size = Size::ConstantScale(14);

    pub fn new(element: Node) -> Self {
        let font_overrides_map = {
            let mut values = Vec::new();
            let mut language_name_to_index = HashMap::new();
            for child in element
                .children()
                .filter(Node::is_element)
                .filter(|child| child.tag_name().name().eq_ignore_ascii_case("override"))
            {
                let languages = child
                    .attribute_ignore_ascii_case("language")
                    .map(|v| v.split(',').map(|v| v.to_owned()))
                    .unwrap();
                let font_path = child
                    .attribute_ignore_ascii_case("file")
                    .map(|v| v.to_owned())
                    .unwrap();
                let mut resolutions_to_size = Vec::new();
                for child in element
                    .children()
                    .filter(Node::is_element)
                    .filter(|child| child.tag_name().name().eq_ignore_ascii_case("size"))
                {
                    let max_resolution = child
                        .attribute_ignore_ascii_case("maxresolution")
                        .map(|v| v.parse::<Vector2>().unwrap().0);
                    let raw_size = {
                        let value = child.attribute_ignore_ascii_case("size").unwrap();

                        if let Some(f) = value.strip_suffix("vw").map(|v| v.parse::<f32>().unwrap())
                        {
                            Size::RelativeWidth(f)
                        } else if let Some(f) =
                            value.strip_suffix("vh").map(|v| v.parse::<f32>().unwrap())
                        {
                            Size::RelativeHeight(f)
                        } else {
                            Size::ConstantScale(value.parse::<u32>().unwrap())
                        }
                    };
                    resolutions_to_size.push((max_resolution, raw_size));
                }
                let dynamic_loading = child
                    .attribute_ignore_ascii_case("dynamicloading")
                    .map(|v| v.parse::<bool>().unwrap())
                    .unwrap();
                let shcc = SpeciallyHandledCharCategory::from_xml(child);
                let font = ScalableFont {
                    file_path: font_path,
                    resolutions_to_size,
                    dynamic_loading,
                    shcc,
                };
                let index = match values.iter().position(|v| v == &font) {
                    Some(index) => index,
                    None => {
                        values.push(font);
                        values.len() - 1
                    }
                };
                language_name_to_index.extend(languages.map(|a| (a, index)));
            }
            //SAFETY: safe if constructed using methods above
            unsafe { MultiKeyHashMap::new(values, language_name_to_index) }
        };

        let fallback_font_path = element
            .attribute_ignore_ascii_case("file")
            .map(|v| v.to_owned())
            .unwrap();
        let mut default_resolutions_to_size = Vec::new();
        for child in element
            .children()
            .filter(Node::is_element)
            .filter(|child| child.tag_name().name().eq_ignore_ascii_case("size"))
        {
            let max_resolution = child
                .attribute_ignore_ascii_case("maxresolution")
                .map(|v| v.parse::<Vector2>().unwrap().0);
            let raw_size = {
                let value = child.attribute_ignore_ascii_case("size").unwrap();

                if let Some(f) = value.strip_suffix("vw").map(|v| v.parse::<f32>().unwrap()) {
                    Size::RelativeWidth(f)
                } else if let Some(f) = value.strip_suffix("vh").map(|v| v.parse::<f32>().unwrap())
                {
                    Size::RelativeHeight(f)
                } else {
                    Size::ConstantScale(value.parse::<u32>().unwrap())
                }
            };
            default_resolutions_to_size.push((max_resolution, raw_size));
        }
        let fallback_dynamic_loading = element
            .attribute_ignore_ascii_case("dynamicloading")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let fallback_shcc = SpeciallyHandledCharCategory::from_xml(element);
        let force_upper_case = element
            .attribute_ignore_ascii_case("forceuppercase")
            .map_or(false, |v| v.parse::<bool>().unwrap());
        let fallback_font = ScalableFont {
            file_path: fallback_font_path,
            resolutions_to_size: default_resolutions_to_size,
            dynamic_loading: fallback_dynamic_loading,
            shcc: fallback_shcc,
        };

        Self {
            font_overrides_map,
            force_upper_case,
            fallback_font,
        }
    }

    pub fn get_font_by_language(&self, language: &str) -> &ScalableFont {
        match self.font_overrides_map.get(language) {
            Some(value) => value,
            None => &self.fallback_font,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiKeyHashMap<K, V> {
    values: Vec<V>,
    key_to_value_index: HashMap<K, usize>,
}

impl<K: Eq + PartialEq + Hash, V> MultiKeyHashMap<K, V> {
    /// # Safety
    ///
    /// You must ensure that all indexes in key_to_value_index are valid
    pub unsafe fn new(values: Vec<V>, key_to_value_index: HashMap<K, usize>) -> Self {
        Self {
            values,
            key_to_value_index,
        }
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        // Safety: this should always be true if modified using functions provided by this struct
        self.key_to_value_index
            .get(key)
            .map(|index| unsafe { self.values.get(*index).unwrap_unchecked() })
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Size {
    RelativeWidth(f32),
    RelativeHeight(f32),
    ConstantScale(u32),
}

#[bitfield(u8)]
#[derive(PartialEq)]
pub struct SpeciallyHandledCharCategory {
    pub cjk: bool,
    pub cyrillic: bool,
    pub japanese: bool,
    #[bits(5)]
    _unused: u8,
}

impl SpeciallyHandledCharCategory {
    pub fn from_xml(element: Node) -> Self {
        let mut current = Self::new();
        /* Filtered
        if element
            .attribute_ignore_ascii_case("isCJK")
            .unwrap()
            .parse::<bool>()
            .unwrap()
        {
            current |= Self::CJK;
        }
        */
        if element
            .attribute_ignore_ascii_case("isCyrillic")
            .map_or(false, |v| v.parse::<bool>().unwrap())
        {
            current.set_cyrillic(true);
        }
        /* Filtered
        if element
            .attribute_ignore_ascii_case("isJapanese")
            .unwrap()
            .parse::<bool>()
            .unwrap()
        {
            current |= Self::Japanese;
        }
        */
        current
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ScalableFont {
    pub file_path: String,
    pub resolutions_to_size: Vec<(Option<Vec2>, Size)>,
    pub dynamic_loading: bool,
    pub shcc: SpeciallyHandledCharCategory,
}

impl ScalableFont {
    pub fn get_font_size_by_resolution(&self, prefab: &GUIFontPrefab, resolution: &Vec2) -> Size {
        for (res, size) in &self.resolutions_to_size {
            let Some(res) = res else {
                return *size;
            };
            if res.x <= resolution.x && res.y <= resolution.y {
                return *size;
            }
        }

        for (res, size) in &prefab.fallback_font.resolutions_to_size {
            let Some(res) = res else {
                return *size;
            };
            if res.x <= resolution.x && res.y <= resolution.y {
                return *size;
            }
        }

        GUIFontPrefab::FALLBACK_FONT_SIZE
    }
}

#[derive(Debug, Clone)]
pub struct GUISpritePrefab {
    pub sprite: UISprite,
}

impl GUISpritePrefab {
    pub fn new(element: Node) -> Self {
        Self {
            sprite: UISprite::new(element),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UISprite {
    pub sprite: BarotraumaSprite,
    pub maintain_aspect_ratio: bool,
    pub maintain_border_aspect_ratio: bool,
    pub tile: bool,
    pub cross_fade_in: bool,
    pub cross_fade_out: bool,
    pub transition_mode: Option<TransitionMode>,
    pub slices: Option<UISpriteSlices>,
}

impl UISprite {
    pub fn new(element: Node) -> Self {
        let sprite = BarotraumaSprite::new(element);
        let maintain_aspect_ratio = element
            .attribute_ignore_ascii_case("maintainaspectratio")
            .map_or(false, |v| v.parse().unwrap());
        let maintain_border_aspect_ratio = element
            .attribute_ignore_ascii_case("maintainborderaspectratio")
            .map_or(false, |v| v.parse().unwrap());
        let tile = element
            .attribute_ignore_ascii_case("tile")
            .map_or(true, |v| v.parse().unwrap());
        let cross_fade_in = element
            .attribute_ignore_ascii_case("crossfadein")
            .map_or(false, |v| v.parse().unwrap());
        let cross_fade_out = element
            .attribute_ignore_ascii_case("crossfadeout")
            .map_or(false, |v| v.parse().unwrap());
        let transition_mode = element
            .attribute_ignore_ascii_case("transition")
            .map(|v| v.parse::<TransitionMode>().unwrap());
        let slices = element.attribute_ignore_ascii_case("slice").map(|v| {
            let slice_vec = v.parse::<Vector4>().unwrap().0;
            let min_border_scale = element
                .attribute_ignore_ascii_case("minborderscale")
                .map_or(0.1, |v| v.parse::<f32>().unwrap());
            let max_border_scale = element
                .attribute_ignore_ascii_case("maxborderscale")
                .map_or(10.0, |v| v.parse::<f32>().unwrap());
            let slice = Rect {
                x: slice_vec.x as i32,
                y: slice_vec.y as i32,
                width: (slice_vec.z - slice_vec.x) as u32,
                height: (slice_vec.w - slice_vec.y) as u32,
            };
            UISpriteSlices {
                slice,
                sprite_source_rect: sprite.source_rect.clone().unwrap(),
                min_border_scale,
                max_border_scale,
            }
        });

        Self {
            sprite,
            maintain_aspect_ratio,
            maintain_border_aspect_ratio,
            tile,
            cross_fade_in,
            cross_fade_out,
            transition_mode,
            slices,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UISpriteSlices {
    slice: Rect,
    sprite_source_rect: Rect,
    pub min_border_scale: f32,
    pub max_border_scale: f32,
}

impl UISpriteSlices {
    pub fn top_left(&self) -> Rect {
        Rect {
            x: self.sprite_source_rect.x,
            y: self.sprite_source_rect.y,
            width: (self.slice.x - self.sprite_source_rect.x)
                .try_into()
                .unwrap(),
            height: (self.slice.y - self.sprite_source_rect.y)
                .try_into()
                .unwrap(),
        }
    }

    pub fn top_mid(&self) -> Rect {
        Rect {
            x: self.slice.x,
            y: self.sprite_source_rect.y,
            width: self.slice.width,
            height: (self.slice.y - self.sprite_source_rect.y)
                .try_into()
                .unwrap(),
        }
    }

    pub fn top_right(&self) -> Rect {
        Rect {
            x: self.slice.x + self.slice.width as i32,
            y: self.sprite_source_rect.y,
            width: ((self.sprite_source_rect.x + self.sprite_source_rect.width as i32)
                - (self.slice.x + self.slice.width as i32))
                .try_into()
                .unwrap(),
            height: (self.slice.y - self.sprite_source_rect.y)
                .try_into()
                .unwrap(),
        }
    }

    pub fn mid_left(&self) -> Rect {
        Rect {
            x: self.sprite_source_rect.x,
            y: self.slice.y,
            width: (self.slice.x - self.sprite_source_rect.x)
                .try_into()
                .unwrap(),
            height: self.slice.height,
        }
    }

    pub fn center(&self) -> Rect {
        self.slice.clone()
    }

    pub fn mid_right(&self) -> Rect {
        Rect {
            x: self.slice.x + self.slice.width as i32,
            y: self.slice.y,
            width: ((self.sprite_source_rect.x + self.sprite_source_rect.width as i32)
                - (self.slice.x + self.slice.width as i32))
                .try_into()
                .unwrap(),
            height: self.slice.height,
        }
    }

    pub fn bottom_left(&self) -> Rect {
        Rect {
            x: self.sprite_source_rect.x,
            y: self.slice.y + self.slice.height as i32,
            width: (self.slice.x - self.sprite_source_rect.x)
                .try_into()
                .unwrap(),
            height: ((self.sprite_source_rect.y + self.sprite_source_rect.height as i32)
                - (self.slice.y + self.slice.height as i32))
                .try_into()
                .unwrap(),
        }
    }

    pub fn bottom_mid(&self) -> Rect {
        Rect {
            x: self.slice.x,
            y: self.slice.y + self.slice.height as i32,
            width: self.slice.width,
            height: ((self.sprite_source_rect.y + self.sprite_source_rect.height as i32)
                - (self.slice.y + self.slice.height as i32))
                .try_into()
                .unwrap(),
        }
    }

    pub fn bottom_right(&self) -> Rect {
        Rect {
            x: self.slice.x + self.slice.width as i32,
            y: self.slice.y + self.slice.height as i32,
            width: ((self.sprite_source_rect.x + self.sprite_source_rect.width as i32)
                - (self.slice.x + self.slice.width as i32))
                .try_into()
                .unwrap(),
            height: ((self.sprite_source_rect.y + self.sprite_source_rect.height as i32)
                - (self.slice.y + self.slice.height as i32))
                .try_into()
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GUISpriteSheetPrefab {
    pub sprite_sheet: SpriteSheet,
}

impl GUISpriteSheetPrefab {
    pub fn new(element: Node) -> Self {
        Self {
            sprite_sheet: SpriteSheet::new(element),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub sprite: BarotraumaSprite,
    pub column_count: u32,
    pub row_count: u32,
    pub origin: Vec2,
    pub empty_frames: u32,
}

impl SpriteSheet {
    pub fn new(element: Node) -> Self {
        let sprite = BarotraumaSprite::new(element);
        let column_count = element
            .attribute_ignore_ascii_case("columns")
            .map_or(1, |v| v.parse::<u32>().unwrap().max(1));
        let row_count = element
            .attribute_ignore_ascii_case("rows")
            .map_or(1, |v| v.parse::<u32>().unwrap().max(1));
        let origin = element
            .attribute_ignore_ascii_case("origin")
            .map_or(Vec2::new(0.5, 0.5), |v| v.parse::<Vector2>().unwrap().0);
        let empty_frames = element
            .attribute_ignore_ascii_case("emptyframes")
            .map_or(0, |v| v.parse::<u32>().unwrap());
        Self {
            sprite,
            column_count,
            row_count,
            origin,
            empty_frames,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GUIColorPrefab {
    pub color: Color,
}

impl GUIColorPrefab {
    pub fn new(element: Node) -> Self {
        Self {
            color: element
                .attribute_ignore_ascii_case("color")
                .map(|v| v.parse().unwrap())
                .unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct GUICursorPrefab {
    pub sprites: HashMap<CursorState, BarotraumaSprite>,
}

impl GUICursorPrefab {
    pub fn new(element: Node) -> Self {
        let mut sprites = HashMap::with_capacity(NUM_CURSOR_STATES);
        for child in element.children().filter(Node::is_element) {
            let state = element
                .attribute_ignore_ascii_case("state")
                .map_or(CursorState::Default, |v| v.parse().unwrap());
            sprites.insert(state, BarotraumaSprite::new(child));
        }

        Self { sprites }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CursorState {
    Default,
    Hand,
    Move,
    IBeam,
    Draggin,
    Waiting,
    WaitingBackground,
}
const NUM_CURSOR_STATES: usize = 7;

impl FromStr for CursorState {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Default" => Ok(Self::Default),
            "Hand" => Ok(Self::Hand),
            "Move" => Ok(Self::Move),
            "IBeam" => Ok(Self::IBeam),
            "Draggin" => Ok(Self::Draggin),
            "Waiting" => Ok(Self::Waiting),
            "WaitingBackground" => Ok(Self::WaitingBackground),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(Debug)]

pub struct GUIComponentStyle {
    pub padding: Vec4,
    pub color: Color,
    pub hover_color: Color,
    pub selected_color: Color,
    pub disabled_color: Color,
    pub pressed_color: Color,
    pub outline_color: Color,
    pub text_color: Color,
    pub hover_text_color: Color,
    pub disabled_text_color: Color,
    pub selected_text_color: Color,
    pub sprite_cross_fade_time: Option<f32>,
    pub color_cross_fade_time: Option<f32>,
    pub color_transition_mode: Option<TransitionMode>,
    pub fallback_state: Option<SpriteFallBackState>,
    pub font: Option<String>,
    pub force_upper_case: bool,
    pub sprites: HashMap<ComponentState, Vec<UISprite>>,
    #[allow(clippy::type_complexity)]
    pub resolutions_to_size: Vec<(Option<Vec2>, (Option<Size>, Option<Size>))>,
    pub child_styles: HashMap<String, GUIComponentStyle>,
}

impl GUIComponentStyle {
    pub fn new(element: Node) -> Self {
        let padding = element
            .attribute_ignore_ascii_case("padding")
            .map_or(Vec4::ZERO, |v| v.parse::<Vector4>().unwrap().0);
        let color = element.attribute_ignore_ascii_case("color").map_or(
            Color::Simple {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            |v| v.parse().unwrap(),
        );
        let hover_color = element
            .attribute_ignore_ascii_case("hovercolor")
            .map_or(color.clone(), |v| v.parse().unwrap());
        let selected_color = element
            .attribute_ignore_ascii_case("selectedcolor")
            .map_or(color.clone(), |v| v.parse().unwrap());
        let disabled_color = element
            .attribute_ignore_ascii_case("disabledcolor")
            .map_or(color.clone(), |v| v.parse().unwrap());
        let pressed_color = element
            .attribute_ignore_ascii_case("pressedcolor")
            .map_or(color.clone(), |v| v.parse().unwrap());
        let outline_color = element.attribute_ignore_ascii_case("outlinecolor").map_or(
            Color::Simple {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            |v| v.parse().unwrap(),
        );

        let text_color = element.attribute_ignore_ascii_case("textcolor").map_or(
            Color::Simple {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            |v| v.parse().unwrap(),
        );
        let hover_text_color = element
            .attribute_ignore_ascii_case("hover_textcolor")
            .map_or(text_color.clone(), |v| v.parse().unwrap());
        let disabled_text_color = element
            .attribute_ignore_ascii_case("disabled_textcolor")
            .map_or(text_color.clone(), |v| v.parse().unwrap());
        let selected_text_color = element
            .attribute_ignore_ascii_case("selected_textcolor")
            .map_or(text_color.clone(), |v| v.parse().unwrap());
        let sprite_cross_fade_time = element
            .attribute_ignore_ascii_case("spritefadetime")
            .map(|v| v.parse::<f32>().unwrap());
        let color_cross_fade_time = element
            .attribute_ignore_ascii_case("colorfadetime")
            .map(|v| v.parse::<f32>().unwrap());

        let color_transition_mode = element
            .attribute_ignore_ascii_case("colortransition")
            .map(|v| v.parse::<TransitionMode>().unwrap());
        let fallback_state = element
            .attribute_ignore_ascii_case("fallbackstate")
            .map(|v| v.parse::<SpriteFallBackState>().unwrap());

        let font = element.attribute("font").map(|v| v.to_owned());
        let force_upper_case = element
            .attribute_ignore_ascii_case("forceuppercase")
            .map_or(false, |v| v.parse().unwrap());

        let mut sprites: HashMap<ComponentState, Vec<UISprite>> = HashMap::new();
        let mut resolutions_to_size = Vec::new();
        let mut child_styles = HashMap::new();
        for child in element.children().filter(Node::is_element) {
            let elem_name = child.tag_name().name().to_lowercase();
            match elem_name.as_str() {
                "sprite" => {
                    let new_sprite = UISprite::new(child);
                    if let Some(state) = element
                        .attribute_ignore_ascii_case("state")
                        .map(|v| v.parse::<ComponentState>().unwrap())
                    {
                        if let Some(v) = sprites.get_mut(&state) {
                            v.push(new_sprite);
                        } else {
                            sprites.insert(state, vec![new_sprite]);
                        }
                    } else {
                        sprites.insert(ComponentState::None, vec![new_sprite.clone()]);
                        sprites.insert(ComponentState::Hover, vec![new_sprite.clone()]);
                        sprites.insert(ComponentState::Pressed, vec![new_sprite.clone()]);
                        sprites.insert(ComponentState::Selected, vec![new_sprite.clone()]);
                        sprites.insert(ComponentState::HoverSelected, vec![new_sprite]);
                    }
                }
                "size" => {
                    let max_resolution = child
                        .attribute_ignore_ascii_case("maxresolution")
                        .map(|v| v.parse::<Vector2>().unwrap().0);
                    let width = {
                        child.attribute_ignore_ascii_case("width").map(|value| {
                            if let Some(f) =
                                value.strip_suffix("vw").map(|v| v.parse::<f32>().unwrap())
                            {
                                Size::RelativeWidth(f)
                            } else if let Some(f) =
                                value.strip_suffix("vh").map(|v| v.parse::<f32>().unwrap())
                            {
                                Size::RelativeHeight(f)
                            } else {
                                Size::ConstantScale(value.parse::<u32>().unwrap())
                            }
                        })
                    };
                    let height = {
                        child.attribute_ignore_ascii_case("height").map(|value| {
                            if let Some(f) =
                                value.strip_suffix("vw").map(|v| v.parse::<f32>().unwrap())
                            {
                                Size::RelativeWidth(f)
                            } else if let Some(f) =
                                value.strip_suffix("vh").map(|v| v.parse::<f32>().unwrap())
                            {
                                Size::RelativeHeight(f)
                            } else {
                                Size::ConstantScale(value.parse::<u32>().unwrap())
                            }
                        })
                    };
                    resolutions_to_size.push((max_resolution, (width, height)));
                }
                _ => {
                    if child_styles.contains_key(&elem_name) {
                        panic!()
                    } else {
                        child_styles.insert(elem_name, GUIComponentStyle::new(child));
                    }
                }
            }
        }
        if sprites.contains_key(&ComponentState::Hover)
            && !sprites.contains_key(&ComponentState::HoverSelected)
        {
            sprites.insert(
                ComponentState::HoverSelected,
                unsafe { sprites.get(&ComponentState::Hover).unwrap_unchecked() }.clone(),
            );
        }

        Self {
            padding,
            color,
            hover_color,
            selected_color,
            disabled_color,
            pressed_color,
            outline_color,
            text_color,
            hover_text_color,
            disabled_text_color,
            selected_text_color,
            sprite_cross_fade_time,
            color_cross_fade_time,
            color_transition_mode,
            fallback_state,
            font,
            force_upper_case,
            sprites,
            resolutions_to_size,
            child_styles,
        }
    }
}

#[derive(Debug)]
pub enum SpriteFallBackState {
    None,
    Hover,
    Pressed,
    Selected,
    HoverSelected,
    Toggle,
}

impl FromStr for SpriteFallBackState {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "hover" => Ok(Self::Hover),
            "pressed" => Ok(Self::Pressed),
            "selected" => Ok(Self::Selected),
            "hoverselected" => Ok(Self::HoverSelected),
            "toggle" => Ok(Self::Toggle),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ComponentState {
    None,
    Hover,
    Pressed,
    Selected,
    HoverSelected,
}

impl FromStr for ComponentState {
    type Err = DoesNotExistError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Hover" => Ok(Self::Hover),
            "Pressed" => Ok(Self::Pressed),
            "Selected" => Ok(Self::Selected),
            "HoverSelected" => Ok(Self::HoverSelected),
            _ => Err(DoesNotExistError(s.to_owned())),
        }
    }
}
