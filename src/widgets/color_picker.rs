use egui::Color32;

use crate::elements::Widget;

pub struct ColorPicker<'a> {
    color: &'a mut Color32,
}

impl<'a> ColorPicker<'a> {
    pub fn new(color: &'a mut Color32) -> Self {
        Self { color }
    }
}

impl egui::Widget for ColorPicker<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.color_edit_button_srgba(self.color)
    }
}

impl<'a> From<ColorPicker<'a>> for Widget<'a> {
    fn from(value: ColorPicker<'a>) -> Self {
        Self::ColorPicker(value)
    }
}
