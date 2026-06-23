use std::{collections::BTreeMap, sync::Arc};

use abi_stable::std_types::RString;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FontId {
    pub size: f32,
    pub family: FontFamily,
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

    pub const fn monospace(size: f32) -> Self {
        Self::new(size, FontFamily::Monospace)
    }
}

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

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontFamily {
    #[default]
    Proportional,
    Monospace,
    Name(RString),
}

impl FontFamily {
    pub fn to_egui(self, font_data: &BTreeMap<String, Arc<egui::FontData>>) -> egui::FontFamily {
        match self {
            FontFamily::Proportional => egui::FontFamily::Proportional,
            FontFamily::Monospace => egui::FontFamily::Monospace,
            FontFamily::Name(name) => {
                if font_data.contains_key(name.as_str()) {
                    egui::FontFamily::Name(name.as_str().into())
                } else {
                    egui::FontFamily::Proportional
                }
            }
        }
    }
}

impl std::fmt::Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FontFamily::Proportional => "Proportional",
            FontFamily::Monospace => "Monospace",
            FontFamily::Name(name) => name.as_str(),
        };
        write!(f, "{s}")
    }
}
