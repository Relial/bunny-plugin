#[cfg(feature = "manager")]
use std::sync::Arc;

use abi_stable::std_types::{RArc, RHashMap, RString, RVec};
use rapidhash::fast::RandomState;

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
            FontFamily::Custom(custom_font) => unsafe {
                Arc::increment_strong_count(custom_font.arc_ptr);
                let name = Arc::from_raw(custom_font.arc_ptr);
                Self::Name(name)
            },
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

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CustomFonts(RArc<CustomFontsImpl>);

impl CustomFonts {
    #[cfg(feature = "manager")]
    pub fn new(fonts: impl IntoIterator<Item = (RString, CustomFont)>) -> Self {
        Self(RArc::new(CustomFontsImpl::new(fonts)))
    }

    /// Get a font loaded by the manager by its filename without the extension
    #[inline]
    pub fn get_font(&self, name: impl AsRef<str>) -> Option<CustomFont> {
        self.0.get_font(name)
    }

    /// Fonts loaded by the manager
    #[inline]
    pub fn fonts(&self) -> &[NamedCustomFont] {
        self.0.fonts()
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
struct CustomFontsImpl {
    list: RVec<NamedCustomFont>,
    map: RHashMap<RString, CustomFont, RandomState>,
}

impl CustomFontsImpl {
    #[cfg(feature = "manager")]
    pub fn new(fonts: impl IntoIterator<Item = (RString, CustomFont)>) -> Self {
        let (map, list) = fonts
            .into_iter()
            .map(|(name, font)| ((name.clone(), font), NamedCustomFont::new(name, font)))
            .unzip();
        Self { list, map }
    }

    #[inline]
    pub fn get_font(&self, name: impl AsRef<str>) -> Option<CustomFont> {
        self.map.get(name.as_ref()).copied()
    }

    #[inline]
    pub fn fonts(&self) -> &[NamedCustomFont] {
        self.list.as_slice()
    }
}

/// A font loaded by the manager
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct CustomFont {
    arc_ptr: *const str,
}

#[cfg(feature = "manager")]
impl CustomFont {
    pub fn new(name: Arc<str>) -> Self {
        let arc_ptr = Arc::into_raw(name);
        Self { arc_ptr }
    }
}

/// A font loaded by the manager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct NamedCustomFont {
    name: RString,
    font: CustomFont,
}

impl NamedCustomFont {
    #[cfg(feature = "manager")]
    fn new(name: impl Into<RString>, font: CustomFont) -> Self {
        Self {
            name: name.into(),
            font,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn font(&self) -> CustomFont {
        self.font
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
