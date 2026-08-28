use ecolor::Color32;

use crate::elements::Widget;

#[repr(C)]
pub struct ColorPicker<'a> {
    color: &'a mut Color32,
}

impl<'a> ColorPicker<'a> {
    #[inline]
    pub fn new(color: &'a mut Color32) -> Self {
        Self { color }
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for ColorPicker<'_> {
    #[inline]
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.color_edit_button_srgba(self.color)
    }
}

impl<'a> From<ColorPicker<'a>> for Widget<'a> {
    #[inline]
    fn from(value: ColorPicker<'a>) -> Self {
        Self::ColorPicker(value)
    }
}
