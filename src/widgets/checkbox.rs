use egui::Ui;

use crate::{elements::Widget, widget_text::WidgetText};

#[repr(C)]
pub struct CheckBox<'a> {
    text: Option<WidgetText>,
    checked: &'a mut bool,
    indeterminate: bool,
}

impl<'a> CheckBox<'a> {
    pub fn new(checked: &'a mut bool, text: impl Into<WidgetText>) -> Self {
        Self {
            text: Some(text.into()),
            checked,
            indeterminate: false,
        }
    }

    pub fn without_text(checked: &'a mut bool) -> Self {
        Self {
            text: None,
            checked,
            indeterminate: false,
        }
    }

    #[inline]
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }
}

impl egui::Widget for CheckBox<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let mut checkbox = if let Some(text) = self.text {
            egui::Checkbox::new(self.checked, text)
        } else {
            egui::Checkbox::without_text(self.checked)
        };
        checkbox = checkbox.indeterminate(self.indeterminate);
        ui.add(checkbox)
    }
}

impl<'a> From<CheckBox<'a>> for Widget<'a> {
    fn from(value: CheckBox<'a>) -> Self {
        Self::CheckBox(value)
    }
}
