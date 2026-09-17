use crate::{WidgetText, widgets::Widget};

#[repr(C)]
pub struct Link {
    text: WidgetText,
}

impl Link {
    #[inline]
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self { text: text.into() }
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for Link {
    #[inline]
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.link(self.text)
    }
}

impl From<Link> for Widget<'_> {
    #[inline]
    fn from(value: Link) -> Self {
        Self::Link(value)
    }
}
