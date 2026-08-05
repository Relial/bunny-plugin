use crate::{elements::Widget, widget_text::WidgetText};

#[repr(C)]
pub struct RadioButton {
    text: WidgetText,
    checked: bool,
}

impl RadioButton {
    pub fn new(checked: bool, text: impl Into<WidgetText>) -> Self {
        Self {
            checked,
            text: text.into(),
        }
    }
}

impl From<RadioButton> for Widget<'_> {
    fn from(value: RadioButton) -> Self {
        Self::RadioButton(value)
    }
}

impl egui::Widget for RadioButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.radio(self.checked, self.text)
    }
}
