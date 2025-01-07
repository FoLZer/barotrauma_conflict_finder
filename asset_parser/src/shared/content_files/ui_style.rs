use derive_builder::Builder;
use roxmltree::Node;

use crate::{
    shared::util::XmlContentFile,
    shared::{
        prefabs::gui_style_prefabs::{
            GUIColorPrefab, GUIComponentStyle, GUICursorPrefab, GUIFontPrefab, GUISpritePrefab,
            GUISpriteSheetPrefab,
        },
        util::Overridable,
    },
};

#[derive(Debug)]
pub struct UIStyleFile {
    pub fonts: GUIFonts,
    pub sprites: GUISprites,
    pub sprite_sheets: GUISpriteSheets,
    pub colors: GUIColors,
    pub cursors: Vec<Overridable<GUICursorPrefab>>,
    pub components: Vec<Overridable<GUIComponentStyle>>,
}

impl XmlContentFile for UIStyleFile {
    fn from_xml(element: Node) -> Self {
        Self::new(element)
    }
}

#[derive(Default)]
pub struct UIStyleFileBuilder {
    pub fonts: GUIFontsBuilder,
    pub sprites: GUISpritesBuilder,
    pub sprite_sheets: GUISpriteSheetsBuilder,
    pub colors: GUIColorsBuilder,
    pub cursors: Vec<Overridable<GUICursorPrefab>>,
    pub components: Vec<Overridable<GUIComponentStyle>>,
}

impl UIStyleFileBuilder {
    pub fn load_from_x_element(&mut self, element: Node, overriding: bool) {
        let elem_name = element.tag_name().name().to_owned();
        let elem_name_font_suffix = if elem_name.ends_with("Font") {
            elem_name.clone()
        } else {
            format!("{}Font", elem_name)
        };
        if elem_name == "override" {
            for child in element.children().filter(Node::is_element) {
                self.load_from_x_element(child, true);
            }
        } else if self.fonts.does_field_exist(&elem_name_font_suffix) {
            self.fonts.set_field_by_name(
                &elem_name_font_suffix,
                Some(Overridable {
                    value: GUIFontPrefab::new(element),
                    is_override: overriding,
                }),
            );
        } else if self.sprites.does_field_exist(&elem_name) {
            self.sprites.set_field_by_name(
                &elem_name,
                Some(Overridable {
                    value: GUISpritePrefab::new(element),
                    is_override: overriding,
                }),
            );
        } else if self.sprite_sheets.does_field_exist(&elem_name) {
            self.sprite_sheets.set_field_by_name(
                &elem_name,
                Some(Overridable {
                    value: GUISpriteSheetPrefab::new(element),
                    is_override: overriding,
                }),
            );
        } else if self.colors.does_field_exist(&elem_name) {
            self.colors.set_field_by_name(
                &elem_name,
                Some(Overridable {
                    value: GUIColorPrefab::new(element),
                    is_override: overriding,
                }),
            );
        } else if elem_name == "cursor" {
            self.cursors.push(Overridable {
                value: GUICursorPrefab::new(element),
                is_override: overriding,
            });
        } else if elem_name == "style" {
            for child in element.children().filter(Node::is_element) {
                self.load_from_x_element(child, overriding);
            }
        } else {
            self.components.push(Overridable {
                value: GUIComponentStyle::new(element),
                is_override: overriding,
            });
        }
    }

    pub fn build(self) -> UIStyleFile {
        UIStyleFile {
            fonts: self.fonts.build().unwrap(),
            sprites: self.sprites.build().unwrap(),
            sprite_sheets: self.sprite_sheets.build().unwrap(),
            colors: self.colors.build().unwrap(),
            cursors: self.cursors,
            components: self.components,
        }
    }
}

impl UIStyleFile {
    pub fn new(element: Node) -> Self {
        let mut r = UIStyleFileBuilder::default();

        r.load_from_x_element(element, false);

        r.build()
    }
}

/*
macro_rules! construct_builder {
    (
        $(#[$outer:meta])*
        $struct_vis:vis struct $struct_name:ident: $target_type:ty {
            $(
                #[xmlname = $xml_name:literal]
                $field_vis:vis $field_name:ident: $field_type:ty,
            )*
        }
    ) => {
        $(#[$outer])*
        $struct_vis struct $struct_name {
            $(
                $field_vis $field_name: $field_type,
            )*
        }

        paste::paste! {
            impl [<$struct_name Builder>] {
                pub fn does_field_exist(&self, field_name: &str) -> bool {
                    match field_name {
                        $($xml_name => true,)*
                        _ => false
                    }
                }

                pub fn set_field_by_name(&mut self, field_name: &str, value: $target_type) {
                    match field_name {
                        $($xml_name => {self.$field_name(value);},)*
                        _ => panic!()
                    }
                }
            }
        }
    };
}
*/

/*
construct_builder! {
    #[derive(Builder, Default, Debug)]
    #[builder(default)]
    pub struct GUIFonts: Option<Overridable<GUIFontPrefab>> {
        #[xmlname = "Font"]
        pub font: Option<Overridable<GUIFontPrefab>>,
        #[xmlname = "UnscaledSmallFont"]
        pub unscaled_small_font: Option<Overridable<GUIFontPrefab>>,
        #[xmlname = "SmallFont"]
        pub small_font: Option<Overridable<GUIFontPrefab>>,
        #[xmlname = "LargeFont"]
        pub large_font: Option<Overridable<GUIFontPrefab>>,
        #[xmlname = "SubHeadingFont"]
        pub sub_heading_font: Option<Overridable<GUIFontPrefab>>,
        #[xmlname = "DigitalFont"]
        pub digital_font: Option<Overridable<GUIFontPrefab>>,
        #[xmlname = "HotkeyFont"]
        pub hotkey_font: Option<Overridable<GUIFontPrefab>>,
        #[xmlname = "MonospacedFont"]
        pub monospaced_font: Option<Overridable<GUIFontPrefab>>,
    }
}
*/

#[derive(Builder, Default, Debug)]
#[builder(default)]
pub struct GUIFonts {
    pub font: Option<Overridable<GUIFontPrefab>>,
    pub unscaled_small_font: Option<Overridable<GUIFontPrefab>>,
    pub small_font: Option<Overridable<GUIFontPrefab>>,
    pub large_font: Option<Overridable<GUIFontPrefab>>,
    pub sub_heading_font: Option<Overridable<GUIFontPrefab>>,
    pub digital_font: Option<Overridable<GUIFontPrefab>>,
    pub hotkey_font: Option<Overridable<GUIFontPrefab>>,
    pub monospaced_font: Option<Overridable<GUIFontPrefab>>,
}
impl GUIFontsBuilder {
    pub fn does_field_exist(&self, field_name: &str) -> bool {
        matches!(
            field_name,
            "Font"
                | "UnscaledSmallFont"
                | "SmallFont"
                | "LargeFont"
                | "SubHeadingFont"
                | "DigitalFont"
                | "HotkeyFont"
                | "MonospacedFont"
        )
    }
    pub fn set_field_by_name(
        &mut self,
        field_name: &str,
        value: Option<Overridable<GUIFontPrefab>>,
    ) {
        match field_name {
            "Font" => {
                self.font(value);
            }
            "UnscaledSmallFont" => {
                self.unscaled_small_font(value);
            }
            "SmallFont" => {
                self.small_font(value);
            }
            "LargeFont" => {
                self.large_font(value);
            }
            "SubHeadingFont" => {
                self.sub_heading_font(value);
            }
            "DigitalFont" => {
                self.digital_font(value);
            }
            "HotkeyFont" => {
                self.hotkey_font(value);
            }
            "MonospacedFont" => {
                self.monospaced_font(value);
            }
            _ => {
                panic!()
            }
        }
    }
}

/*
construct_builder! {
    #[derive(Builder, Default, Debug)]
    #[builder(default)]
    pub struct GUISprites: Option<Overridable<GUISpritePrefab>> {
        #[xmlname = "SubmarineLocationIcon"]
        pub submarine_location_icon: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "Arrow"]
        pub arrow: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "SpeechBubbleIcon"]
        pub speech_bubble_icon: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "BrokenIcon"]
        pub broken_icon: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "YouAreHereCircle"]
        pub you_are_here_circle: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "Radiation"]
        pub radiation: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "UIGlow"]
        pub ui_glow: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "TalentGlow"]
        pub talent_glow: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "PingCircle"]
        pub ping_circle: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "UIGlowCircular"]
        pub ui_glow_circular: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "UIGlowSolidCircular"]
        pub ui_glow_solid_circular: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "UIThermalGlow"]
        pub ui_thermal_glow: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "ButtonPulse"]
        pub button_pulse: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "WalletPortraitBG"]
        pub wallet_portrait_background: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "CrewWalletIconSmall"]
        pub crew_wallet_icon_small: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "EndRoundButtonPulse"]
        pub end_round_button_pulse: Option<Overridable<GUISpritePrefab>>,
        #[xmlname = "IconOverflowIndicator"]
        pub icon_overflow_indicator: Option<Overridable<GUISpritePrefab>>,
    }
}
*/
#[derive(Builder, Default, Debug)]
#[builder(default)]
pub struct GUISprites {
    pub submarine_location_icon: Option<Overridable<GUISpritePrefab>>,
    pub arrow: Option<Overridable<GUISpritePrefab>>,
    pub speech_bubble_icon: Option<Overridable<GUISpritePrefab>>,
    pub broken_icon: Option<Overridable<GUISpritePrefab>>,
    pub you_are_here_circle: Option<Overridable<GUISpritePrefab>>,
    pub radiation: Option<Overridable<GUISpritePrefab>>,
    pub ui_glow: Option<Overridable<GUISpritePrefab>>,
    pub talent_glow: Option<Overridable<GUISpritePrefab>>,
    pub ping_circle: Option<Overridable<GUISpritePrefab>>,
    pub ui_glow_circular: Option<Overridable<GUISpritePrefab>>,
    pub ui_glow_solid_circular: Option<Overridable<GUISpritePrefab>>,
    pub ui_thermal_glow: Option<Overridable<GUISpritePrefab>>,
    pub button_pulse: Option<Overridable<GUISpritePrefab>>,
    pub wallet_portrait_background: Option<Overridable<GUISpritePrefab>>,
    pub crew_wallet_icon_small: Option<Overridable<GUISpritePrefab>>,
    pub end_round_button_pulse: Option<Overridable<GUISpritePrefab>>,
    pub icon_overflow_indicator: Option<Overridable<GUISpritePrefab>>,
}
impl GUISpritesBuilder {
    pub fn does_field_exist(&self, field_name: &str) -> bool {
        matches!(
            field_name,
            "SubmarineLocationIcon"
                | "Arrow"
                | "SpeechBubbleIcon"
                | "BrokenIcon"
                | "YouAreHereCircle"
                | "Radiation"
                | "UIGlow"
                | "TalentGlow"
                | "PingCircle"
                | "UIGlowCircular"
                | "UIGlowSolidCircular"
                | "UIThermalGlow"
                | "ButtonPulse"
                | "WalletPortraitBG"
                | "CrewWalletIconSmall"
                | "EndRoundButtonPulse"
                | "IconOverflowIndicator"
        )
    }
    pub fn set_field_by_name(
        &mut self,
        field_name: &str,
        value: Option<Overridable<GUISpritePrefab>>,
    ) {
        match field_name {
            "SubmarineLocationIcon" => {
                self.submarine_location_icon(value);
            }
            "Arrow" => {
                self.arrow(value);
            }
            "SpeechBubbleIcon" => {
                self.speech_bubble_icon(value);
            }
            "BrokenIcon" => {
                self.broken_icon(value);
            }
            "YouAreHereCircle" => {
                self.you_are_here_circle(value);
            }
            "Radiation" => {
                self.radiation(value);
            }
            "UIGlow" => {
                self.ui_glow(value);
            }
            "TalentGlow" => {
                self.talent_glow(value);
            }
            "PingCircle" => {
                self.ping_circle(value);
            }
            "UIGlowCircular" => {
                self.ui_glow_circular(value);
            }
            "UIGlowSolidCircular" => {
                self.ui_glow_solid_circular(value);
            }
            "UIThermalGlow" => {
                self.ui_thermal_glow(value);
            }
            "ButtonPulse" => {
                self.button_pulse(value);
            }
            "WalletPortraitBG" => {
                self.wallet_portrait_background(value);
            }
            "CrewWalletIconSmall" => {
                self.crew_wallet_icon_small(value);
            }
            "EndRoundButtonPulse" => {
                self.end_round_button_pulse(value);
            }
            "IconOverflowIndicator" => {
                self.icon_overflow_indicator(value);
            }
            _ => {
                panic!()
            }
        }
    }
}

/*
construct_builder! {
    #[derive(Builder, Default, Debug)]
    #[builder(default)]
    pub struct GUISpriteSheets: Option<Overridable<GUISpriteSheetPrefab>> {
        #[xmlname = "RadiationAnimSpriteSheet"]
        pub radiation_anim_sprite_sheet: Option<Overridable<GUISpriteSheetPrefab>>,
        #[xmlname = "SavingIndicator"]
        pub saving_indicator: Option<Overridable<GUISpriteSheetPrefab>>,
        #[xmlname = "GenericThrobber"]
        pub generic_throbber: Option<Overridable<GUISpriteSheetPrefab>>,
        #[xmlname = "FocusIndicator"]
        pub focus_indicator: Option<Overridable<GUISpriteSheetPrefab>>,
    }
}
*/
#[derive(Builder, Default, Debug)]
#[builder(default)]
pub struct GUISpriteSheets {
    pub radiation_anim_sprite_sheet: Option<Overridable<GUISpriteSheetPrefab>>,
    pub saving_indicator: Option<Overridable<GUISpriteSheetPrefab>>,
    pub generic_throbber: Option<Overridable<GUISpriteSheetPrefab>>,
    pub focus_indicator: Option<Overridable<GUISpriteSheetPrefab>>,
}
impl GUISpriteSheetsBuilder {
    pub fn does_field_exist(&self, field_name: &str) -> bool {
        matches!(
            field_name,
            "RadiationAnimSpriteSheet" | "SavingIndicator" | "GenericThrobber" | "FocusIndicator"
        )
    }
    pub fn set_field_by_name(
        &mut self,
        field_name: &str,
        value: Option<Overridable<GUISpriteSheetPrefab>>,
    ) {
        match field_name {
            "RadiationAnimSpriteSheet" => {
                self.radiation_anim_sprite_sheet(value);
            }
            "SavingIndicator" => {
                self.saving_indicator(value);
            }
            "GenericThrobber" => {
                self.generic_throbber(value);
            }
            "FocusIndicator" => {
                self.focus_indicator(value);
            }
            _ => {
                panic!()
            }
        }
    }
}

/*
construct_builder! {
    #[derive(Builder, Default, Debug)]
    #[builder(default)]
    pub struct GUIColors: Option<Overridable<GUIColorPrefab>> {
        #[xmlname = "Green"]
        pub green: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "Orange"]
        pub orange: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "Red"]
        pub red: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "Blue"]
        pub blue: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "Yellow"]
        pub yellow: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ModdedServerColor"]
        pub modded_server_color: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorInventoryEmpty"]
        pub color_inventory_empty: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorInventoryHalf"]
        pub color_inventory_half: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorInventoryFull"]
        pub color_inventory_full: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorInventoryBackground"]
        pub color_inventory_background: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorInventoryEmptyOverlay"]
        pub color_inventory_empty_overlay: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "TextColorNormal"]
        pub text_color_normal: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "TextColorBright"]
        pub text_color_bright: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "TextColorDark"]
        pub text_color_dark: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "TextColorDim"]
        pub text_color_dim: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ItemQualityColorPoor"]
        pub item_quality_color_poor: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ItemQualityColorNormal"]
        pub item_quality_color_normal: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ItemQualityColorGood"]
        pub item_quality_color_good: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ItemQualityColorExcellent"]
        pub item_quality_color_excellent: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ItemQualityColorMasterwork"]
        pub item_quality_color_masterwork: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorReputationVeryLow"]
        pub color_reputation_very_low: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorReputationLow"]
        pub color_reputation_low: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorReputationNeutral"]
        pub color_reputation_neutral: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorReputationHigh"]
        pub color_reputation_high: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "ColorReputationVeryHigh"]
        pub color_reputation_very_high: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "EquipmentSlotIconColor"]
        pub equipment_slot_icon_color: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "BuffColorLow"]
        pub buff_color_low: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "BuffColorMedium"]
        pub buff_color_medium: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "BuffColorHigh"]
        pub buff_color_high: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "DebuffColorLow"]
        pub debuff_color_low: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "DebuffColorMedium"]
        pub debuff_color_medium: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "DebuffColorHigh"]
        pub debuff_color_high: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "HealthBarColorLow"]
        pub health_bar_color_low: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "HealthBarColorMedium"]
        pub health_bar_color_medium: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "HealthBarColorHigh"]
        pub health_bar_color_high: Option<Overridable<GUIColorPrefab>>,
        #[xmlname = "HealthBarColorPoisoned"]
        pub health_bar_color_poisoned: Option<Overridable<GUIColorPrefab>>,
    }
}
*/
#[derive(Builder, Default, Debug)]
#[builder(default)]
pub struct GUIColors {
    pub green: Option<Overridable<GUIColorPrefab>>,
    pub orange: Option<Overridable<GUIColorPrefab>>,
    pub red: Option<Overridable<GUIColorPrefab>>,
    pub blue: Option<Overridable<GUIColorPrefab>>,
    pub yellow: Option<Overridable<GUIColorPrefab>>,
    pub modded_server_color: Option<Overridable<GUIColorPrefab>>,
    pub color_inventory_empty: Option<Overridable<GUIColorPrefab>>,
    pub color_inventory_half: Option<Overridable<GUIColorPrefab>>,
    pub color_inventory_full: Option<Overridable<GUIColorPrefab>>,
    pub color_inventory_background: Option<Overridable<GUIColorPrefab>>,
    pub color_inventory_empty_overlay: Option<Overridable<GUIColorPrefab>>,
    pub text_color_normal: Option<Overridable<GUIColorPrefab>>,
    pub text_color_bright: Option<Overridable<GUIColorPrefab>>,
    pub text_color_dark: Option<Overridable<GUIColorPrefab>>,
    pub text_color_dim: Option<Overridable<GUIColorPrefab>>,
    pub item_quality_color_poor: Option<Overridable<GUIColorPrefab>>,
    pub item_quality_color_normal: Option<Overridable<GUIColorPrefab>>,
    pub item_quality_color_good: Option<Overridable<GUIColorPrefab>>,
    pub item_quality_color_excellent: Option<Overridable<GUIColorPrefab>>,
    pub item_quality_color_masterwork: Option<Overridable<GUIColorPrefab>>,
    pub color_reputation_very_low: Option<Overridable<GUIColorPrefab>>,
    pub color_reputation_low: Option<Overridable<GUIColorPrefab>>,
    pub color_reputation_neutral: Option<Overridable<GUIColorPrefab>>,
    pub color_reputation_high: Option<Overridable<GUIColorPrefab>>,
    pub color_reputation_very_high: Option<Overridable<GUIColorPrefab>>,
    pub equipment_slot_icon_color: Option<Overridable<GUIColorPrefab>>,
    pub buff_color_low: Option<Overridable<GUIColorPrefab>>,
    pub buff_color_medium: Option<Overridable<GUIColorPrefab>>,
    pub buff_color_high: Option<Overridable<GUIColorPrefab>>,
    pub debuff_color_low: Option<Overridable<GUIColorPrefab>>,
    pub debuff_color_medium: Option<Overridable<GUIColorPrefab>>,
    pub debuff_color_high: Option<Overridable<GUIColorPrefab>>,
    pub health_bar_color_low: Option<Overridable<GUIColorPrefab>>,
    pub health_bar_color_medium: Option<Overridable<GUIColorPrefab>>,
    pub health_bar_color_high: Option<Overridable<GUIColorPrefab>>,
    pub health_bar_color_poisoned: Option<Overridable<GUIColorPrefab>>,
}
impl GUIColorsBuilder {
    pub fn does_field_exist(&self, field_name: &str) -> bool {
        matches!(
            field_name,
            "Green"
                | "Orange"
                | "Red"
                | "Blue"
                | "Yellow"
                | "ModdedServerColor"
                | "ColorInventoryEmpty"
                | "ColorInventoryHalf"
                | "ColorInventoryFull"
                | "ColorInventoryBackground"
                | "ColorInventoryEmptyOverlay"
                | "TextColorNormal"
                | "TextColorBright"
                | "TextColorDark"
                | "TextColorDim"
                | "ItemQualityColorPoor"
                | "ItemQualityColorNormal"
                | "ItemQualityColorGood"
                | "ItemQualityColorExcellent"
                | "ItemQualityColorMasterwork"
                | "ColorReputationVeryLow"
                | "ColorReputationLow"
                | "ColorReputationNeutral"
                | "ColorReputationHigh"
                | "ColorReputationVeryHigh"
                | "EquipmentSlotIconColor"
                | "BuffColorLow"
                | "BuffColorMedium"
                | "BuffColorHigh"
                | "DebuffColorLow"
                | "DebuffColorMedium"
                | "DebuffColorHigh"
                | "HealthBarColorLow"
                | "HealthBarColorMedium"
                | "HealthBarColorHigh"
                | "HealthBarColorPoisoned"
        )
    }
    pub fn set_field_by_name(
        &mut self,
        field_name: &str,
        value: Option<Overridable<GUIColorPrefab>>,
    ) {
        match field_name {
            "Green" => {
                self.green(value);
            }
            "Orange" => {
                self.orange(value);
            }
            "Red" => {
                self.red(value);
            }
            "Blue" => {
                self.blue(value);
            }
            "Yellow" => {
                self.yellow(value);
            }
            "ModdedServerColor" => {
                self.modded_server_color(value);
            }
            "ColorInventoryEmpty" => {
                self.color_inventory_empty(value);
            }
            "ColorInventoryHalf" => {
                self.color_inventory_half(value);
            }
            "ColorInventoryFull" => {
                self.color_inventory_full(value);
            }
            "ColorInventoryBackground" => {
                self.color_inventory_background(value);
            }
            "ColorInventoryEmptyOverlay" => {
                self.color_inventory_empty_overlay(value);
            }
            "TextColorNormal" => {
                self.text_color_normal(value);
            }
            "TextColorBright" => {
                self.text_color_bright(value);
            }
            "TextColorDark" => {
                self.text_color_dark(value);
            }
            "TextColorDim" => {
                self.text_color_dim(value);
            }
            "ItemQualityColorPoor" => {
                self.item_quality_color_poor(value);
            }
            "ItemQualityColorNormal" => {
                self.item_quality_color_normal(value);
            }
            "ItemQualityColorGood" => {
                self.item_quality_color_good(value);
            }
            "ItemQualityColorExcellent" => {
                self.item_quality_color_excellent(value);
            }
            "ItemQualityColorMasterwork" => {
                self.item_quality_color_masterwork(value);
            }
            "ColorReputationVeryLow" => {
                self.color_reputation_very_low(value);
            }
            "ColorReputationLow" => {
                self.color_reputation_low(value);
            }
            "ColorReputationNeutral" => {
                self.color_reputation_neutral(value);
            }
            "ColorReputationHigh" => {
                self.color_reputation_high(value);
            }
            "ColorReputationVeryHigh" => {
                self.color_reputation_very_high(value);
            }
            "EquipmentSlotIconColor" => {
                self.equipment_slot_icon_color(value);
            }
            "BuffColorLow" => {
                self.buff_color_low(value);
            }
            "BuffColorMedium" => {
                self.buff_color_medium(value);
            }
            "BuffColorHigh" => {
                self.buff_color_high(value);
            }
            "DebuffColorLow" => {
                self.debuff_color_low(value);
            }
            "DebuffColorMedium" => {
                self.debuff_color_medium(value);
            }
            "DebuffColorHigh" => {
                self.debuff_color_high(value);
            }
            "HealthBarColorLow" => {
                self.health_bar_color_low(value);
            }
            "HealthBarColorMedium" => {
                self.health_bar_color_medium(value);
            }
            "HealthBarColorHigh" => {
                self.health_bar_color_high(value);
            }
            "HealthBarColorPoisoned" => {
                self.health_bar_color_poisoned(value);
            }
            _ => {
                panic!()
            }
        }
    }
}
