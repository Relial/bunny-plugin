use crate::{WidgetText, widgets::Widget};

#[repr(C)]
pub struct RadioButton<'a> {
    text: WidgetText<'a>,
    checked: bool,
}

impl<'a> RadioButton<'a> {
    #[inline]
    pub fn new(checked: bool, text: impl Into<WidgetText<'a>>) -> Self {
        Self {
            checked,
            text: text.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for RadioButton<'_> {
    #[inline]
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.radio(self.checked, self.text)
    }
}

impl<'a> From<RadioButton<'a>> for Widget<'a> {
    #[inline]
    fn from(value: RadioButton<'a>) -> Self {
        Self::RadioButton(value)
    }
}
