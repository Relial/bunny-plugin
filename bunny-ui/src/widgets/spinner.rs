use abi_stable::std_types::ROption::{self, RSome};
use egui::Color32;

use crate::elements::Widget;

#[derive(Default)]
#[repr(C)]
pub struct Spinner {
    size: ROption<f32>,
    color: ROption<Color32>,
}

impl Spinner {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn size(mut self, size: f32) -> Self {
        self.size = RSome(size);
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = RSome(color.into());
        self
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for Spinner {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut spinner = egui::Spinner::new();
        if let RSome(size) = self.size {
            spinner = spinner.size(size);
        }
        if let RSome(color) = self.color {
            spinner = spinner.color(color);
        }

        spinner.ui(ui)
    }
}

impl From<Spinner> for Widget<'_> {
    fn from(value: Spinner) -> Self {
        Self::Spinner(value)
    }
}
