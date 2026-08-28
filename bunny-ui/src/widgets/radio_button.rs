use crate::{elements::Widget, widget_text::WidgetText};

#[repr(C)]
pub struct RadioButton {
    text: WidgetText,
    checked: bool,
}

impl RadioButton {
    #[inline]
    pub fn new(checked: bool, text: impl Into<WidgetText>) -> Self {
        Self {
            checked,
            text: text.into(),
        }
    }
}

impl From<RadioButton> for Widget<'_> {
    #[inline]
    fn from(value: RadioButton) -> Self {
        Self::RadioButton(value)
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for RadioButton {
    #[inline]
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.radio(self.checked, self.text)
    }
}
