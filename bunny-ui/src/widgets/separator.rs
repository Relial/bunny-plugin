use abi_stable::std_types::ROption::{self, RNone, RSome};
use egui::Ui;

use crate::elements::Widget;

#[repr(C)]
pub struct Separator {
    spacing: ROption<f32>,
    grow: f32,
    is_horizontal_line: ROption<bool>,
}

impl Separator {
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = RSome(spacing);
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.is_horizontal_line = RSome(true);
        self
    }

    pub fn vertical(mut self) -> Self {
        self.is_horizontal_line = RSome(false);
        self
    }

    pub fn grow(mut self, grow: f32) -> Self {
        self.grow += grow;
        self
    }

    pub fn shrink(mut self, shrink: f32) -> Self {
        self.grow -= shrink;
        self
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            spacing: RNone,
            grow: 0.0,
            is_horizontal_line: RNone,
        }
    }
}

impl egui::Widget for Separator {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let mut separator = egui::Separator::default();
        if let RSome(spacing) = self.spacing {
            separator = separator.spacing(spacing);
        }
        if self.grow > 0.0 {
            separator = separator.grow(self.grow)
        } else if self.grow < 0.0 {
            separator = separator.shrink(self.grow)
        }
        if let RSome(horizontal) = self.is_horizontal_line {
            if horizontal {
                separator = separator.horizontal();
            } else {
                separator = separator.vertical();
            }
        }
        ui.add(separator)
    }
}

impl From<Separator> for Widget<'_> {
    fn from(value: Separator) -> Self {
        Self::Separator(value)
    }
}
