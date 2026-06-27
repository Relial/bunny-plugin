use egui::Ui;

use crate::{elements::Widget, widget_text::WidgetText};

#[repr(C)]
pub struct CheckBox<'a> {
    text: WidgetText,
    value: &'a mut bool,
}

impl<'a> CheckBox<'a> {
    pub fn new(value: &'a mut bool, text: impl Into<WidgetText>) -> Self {
        Self {
            value,
            text: text.into(),
        }
    }
}

impl egui::Widget for CheckBox<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let checked = self.value;
        ui.checkbox(checked, self.text)
    }
}

impl<'a> From<CheckBox<'a>> for Widget<'a> {
    fn from(value: CheckBox<'a>) -> Self {
        Self::CheckBox(value)
    }
}
