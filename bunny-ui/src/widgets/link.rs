use crate::{WidgetText, widgets::Widget};

#[repr(C)]
pub struct Link<'a> {
    text: WidgetText<'a>,
}

impl<'a> Link<'a> {
    #[inline]
    pub fn new(text: impl Into<WidgetText<'a>>) -> Self {
        Self { text: text.into() }
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for Link<'_> {
    #[inline]
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.link(self.text)
    }
}

impl<'a> From<Link<'a>> for Widget<'a> {
    #[inline]
    fn from(value: Link<'a>) -> Self {
        Self::Link(value)
    }
}
