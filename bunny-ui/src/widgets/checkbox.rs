use crate::{elements::Widget, widget_text::WidgetText};

#[repr(C)]
pub struct CheckBox {
    text: Option<WidgetText>,
    checked: bool,
    indeterminate: bool,
}

impl CheckBox {
    pub fn new(checked: bool, text: impl Into<WidgetText>) -> Self {
        Self {
            text: Some(text.into()),
            checked,
            indeterminate: false,
        }
    }

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
impl egui::Widget for CheckBox {
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

impl From<CheckBox> for Widget<'_> {
    fn from(value: CheckBox) -> Self {
        Self::CheckBox(value)
    }
}
