use abi_stable::std_types::ROption::{self, RNone, RSome};
use egui::Rect;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Painter {
    clip_rect: ROption<Rect>,
    opacity_factor: f32,
}

impl Default for Painter {
    fn default() -> Self {
        Self {
            clip_rect: RNone,
            opacity_factor: 1.0,
        }
    }
}

impl Painter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clip_rect(&mut self, rect: Rect) {
        self.clip_rect = RSome(rect);
    }

    pub fn opacity(&mut self, opacity: f32) {
        if opacity.is_finite() {
            self.opacity_factor = opacity.clamp(0.0, 1.0);
        }
    }

    pub fn multiply_opacity(&mut self, opacity: f32) {
        if opacity.is_finite() {
            self.opacity_factor *= opacity.clamp(0.0, 1.0);
        }
    }
}
