use egui::Color32;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ColorMode {
    Solid(Color32),
}

impl Default for ColorMode {
    fn default() -> Self {
        Self::TRANSPARENT
    }
}

impl ColorMode {
    pub const TRANSPARENT: Self = Self::Solid(Color32::TRANSPARENT);
}

impl From<ColorMode> for egui::epaint::ColorMode {
    fn from(value: ColorMode) -> Self {
        match value {
            ColorMode::Solid(color32) => Self::Solid(color32),
        }
    }
}
