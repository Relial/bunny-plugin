use egui::{Rect, Sense, Ui};

use crate::elements::Widget;

#[repr(C)]
pub struct Interact {
    rect: Rect,
    sense: Sense,
}

impl Interact {
    pub fn new(rect: Rect, sense: Sense) -> Self {
        Self { rect, sense }
    }
}

impl From<Interact> for Widget<'_> {
    fn from(value: Interact) -> Self {
        Self::Interact(value)
    }
}

impl egui::Widget for Interact {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.interact(self.rect, ui.next_auto_id(), self.sense)
    }
}
