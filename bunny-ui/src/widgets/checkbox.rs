use crate::{WidgetText, widgets::Widget};

#[repr(C)]
pub struct CheckBox<'a> {
    text: Option<WidgetText<'a>>,
    checked: bool,
    indeterminate: bool,
}

impl<'a> CheckBox<'a> {
    #[inline]
    pub fn new(checked: bool, text: impl Into<WidgetText<'a>>) -> Self {
        Self {
            text: Some(text.into()),
            checked,
            indeterminate: false,
        }
    }

    #[inline]
    pub fn without_text(checked: bool) -> Self {
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

#[cfg(feature = "manager")]
impl egui::Widget for CheckBox<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut temp = self.checked;
        let checkbox = if let Some(text) = self.text {
            egui::Checkbox::new(&mut temp, text)
        } else {
            egui::Checkbox::without_text(&mut temp)
        }
        .indeterminate(self.indeterminate);
        ui.add(checkbox)
    }
}

impl<'a> From<CheckBox<'a>> for Widget<'a> {
    #[inline]
    fn from(value: CheckBox<'a>) -> Self {
        Self::CheckBox(value)
    }
}
