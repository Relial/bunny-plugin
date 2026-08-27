#[cfg(feature = "manager")]
use std::{collections::BTreeMap, sync::Arc};

use abi_stable::std_types::{RArc, RHashMap, RString, RVec};
use rapidhash::fast::RandomState;

use crate::style::TextStyle;

#[derive(Clone, Debug)]
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
impl FontId {
    pub(crate) fn convert_to_egui(
        self,
        font_data: &BTreeMap<String, Arc<egui::FontData>>,
    ) -> egui::FontId {
        let FontId { size, family } = self;
        egui::FontId {
            size,
            family: family.to_egui(font_data),
        }
    }
}

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub enum FontFamily {
    #[default]
    Proportional,
    Monospace,
    Custom(CustomFont),
}

#[cfg(feature = "manager")]
impl FontFamily {
    pub fn to_egui(self, font_data: &BTreeMap<String, Arc<egui::FontData>>) -> egui::FontFamily {
        match self {
            FontFamily::Proportional => egui::FontFamily::Proportional,
            FontFamily::Monospace => egui::FontFamily::Monospace,
            FontFamily::Custom(custom_font) => unsafe {
                Arc::increment_strong_count(custom_font.arc_ptr);
                let name = Arc::from_raw(custom_font.arc_ptr);
                if font_data.contains_key(name.as_ref()) {
                    egui::FontFamily::Name(name)
                } else {
                    egui::FontFamily::Proportional
                }
            },
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
#[derive(Clone, Copy, Debug)]
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
impl FontSelection {
    pub fn convert_to_egui(self, ui: &egui::Ui) -> egui::FontSelection {
        match self {
            FontSelection::Default => egui::FontSelection::Default,
            FontSelection::FontId(font_id) => {
                let id = ui.fonts(|i| {
                    let data = &i.definitions().font_data;
                    font_id.convert_to_egui(data)
                });
                egui::FontSelection::FontId(id)
            }
            FontSelection::Style(text_style) => egui::FontSelection::Style(text_style.into()),
        }
    }
}
