use egui::Color32;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
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

#[cfg(feature = "manager")]
impl From<ColorMode> for egui::epaint::ColorMode {
    fn from(value: ColorMode) -> Self {
        match value {
            ColorMode::Solid(color32) => Self::Solid(color32),
        }
    }
}
