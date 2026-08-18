use crate::{elements::Widget, widget_text::WidgetText};

#[repr(C)]
pub struct Link {
    text: WidgetText,
}

impl Link {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self { text: text.into() }
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for Link {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.link(self.text)
    }
}

impl From<Link> for Widget<'_> {
    fn from(value: Link) -> Self {
        Self::Link(value)
    }
}
