use shared::fonts::CustomFont;

use crate::style::TextStyle;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct FontId {
    pub family: FontFamily,
    pub size: f32,
}

impl Default for FontId {
    #[inline]
    fn default() -> Self {
        Self {
            size: 14.0,
            family: FontFamily::Proportional,
        }
    }
}

impl FontId {
    #[inline]
    pub const fn new(size: f32, family: FontFamily) -> Self {
        Self { size, family }
    }

    #[inline]
    pub const fn proportional(size: f32) -> Self {
        Self::new(size, FontFamily::Proportional)
    }

    #[inline]
    pub const fn monospace(size: f32) -> Self {
        Self::new(size, FontFamily::Monospace)
    }

    #[inline]
    pub const fn custom(custom_font: CustomFont, size: f32) -> Self {
        Self::new(size, FontFamily::Custom(custom_font))
    }
}

#[cfg(feature = "manager")]
impl From<FontId> for egui::FontId {
    #[inline]
    fn from(value: FontId) -> Self {
        let FontId { family, size } = value;
        Self {
            size,
            family: family.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::FontId> for FontId {
    fn from(value: egui::FontId) -> Self {
        let egui::FontId { size, family } = value;
        Self {
            family: family.into(),
            size,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub enum FontFamily {
    #[default]
    Proportional,
    Monospace,
    Custom(CustomFont),
}

#[cfg(feature = "manager")]
impl From<FontFamily> for egui::FontFamily {
    #[inline]
    fn from(value: FontFamily) -> Self {
        match value {
            FontFamily::Proportional => Self::Proportional,
            FontFamily::Monospace => Self::Monospace,
            FontFamily::Custom(custom_font) => {
                let name = custom_font.into_name();
                Self::Name(name)
            }
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::FontFamily> for FontFamily {
    fn from(value: egui::FontFamily) -> Self {
        match value {
            egui::FontFamily::Proportional => Self::Proportional,
            egui::FontFamily::Monospace => Self::Monospace,
            egui::FontFamily::Name(_) => Self::Proportional, // Let's just not deal with it!
        }
    }
}

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub enum FontSelection {
    #[default]
    Default,
    FontId(FontId),
    Style(TextStyle),
}

impl From<FontId> for FontSelection {
    #[inline(always)]
    fn from(value: FontId) -> Self {
        Self::FontId(value)
    }
}

impl From<TextStyle> for FontSelection {
    #[inline(always)]
    fn from(value: TextStyle) -> Self {
        Self::Style(value)
    }
}

#[cfg(feature = "manager")]
impl From<FontSelection> for egui::FontSelection {
    fn from(value: FontSelection) -> Self {
        match value {
            FontSelection::Default => Self::Default,
            FontSelection::FontId(font_id) => Self::FontId(font_id.into()),
            FontSelection::Style(text_style) => Self::Style(text_style.into()),
        }
    }
}
