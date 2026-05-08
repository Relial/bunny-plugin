use egui::Ui;

use crate::{elements::Widget, num::Num};

#[repr(C)]
pub struct Slider<'a> {
    value: &'a mut Num,
    range: [Num; 2],
}

impl<'a> Slider<'a> {
    pub fn new(value: &'a mut Num, range: [Num; 2]) -> Self {
        Self { value, range }
    }
}

impl egui::Widget for Slider<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let value = self.value;
        let widget = match value {
            Num::Integer(i) => {
                let range = self.range[0].into()..=self.range[1].into();
                egui::Slider::new(i, range)
            }
            Num::Float(f) => {
                let range = self.range[0].into()..=self.range[1].into();
                egui::Slider::new(f, range)
            }
        };
        ui.add(widget)
    }
}

impl<'a> From<Slider<'a>> for Widget<'a> {
    fn from(value: Slider<'a>) -> Self {
        Self::Slider(value)
    }
}
