use emath::Rect;

use crate::{elements::Widget, sense::Sense};

#[repr(C)]
pub struct Interact {
    rect: Rect,
    sense: Sense,
}

impl Interact {
    #[inline]
    pub fn new(rect: Rect, sense: Sense) -> Self {
        Self { rect, sense }
    }
}

impl From<Interact> for Widget<'_> {
    #[inline]
    fn from(value: Interact) -> Self {
        Self::Interact(value)
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for Interact {
    #[inline]
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.interact(self.rect, ui.next_auto_id(), self.sense.into())
    }
}
