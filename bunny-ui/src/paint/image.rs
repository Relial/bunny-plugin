use abi_stable::std_types::RVec;
use egui::{Color32, Vec2};

#[derive(Clone, Default, PartialEq, Eq)]
#[repr(C)]
pub struct ColorImage {
    pub pixels: RVec<Color32>,
    pub size: [usize; 2],
    pub source_size: Vec2,
}

#[cfg(feature = "manager")]
impl From<egui::ColorImage> for ColorImage {
    fn from(value: egui::ColorImage) -> Self {
        let egui::ColorImage {
            size,
            source_size,
            pixels,
        } = value;
        Self {
            pixels: pixels.into(),
            size,
            source_size,
        }
    }
}
