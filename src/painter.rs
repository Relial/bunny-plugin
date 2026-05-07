use abi_stable::std_types::RVec;
use egui::{Rect, Ui};
use tracing::error;

use crate::paint::shapes::shape::Shape;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Painter<'a> {
    pub shapes: RVec<Shape<'a>>,
    pub clip_rect: Rect,
    pub opacity_factor: f32,
}

impl Painter<'_> {
    pub fn new(clip_rect: Rect) -> Self {
        Self {
            shapes: RVec::new(),
            clip_rect,
            opacity_factor: 1.0,
        }
    }

    pub fn set_clip_rect(&mut self, clip_rect: Rect) {
        self.clip_rect = clip_rect;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
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

impl Painter<'_> {
    pub fn ui(self, ui: &mut Ui) {
        let painter = ui.painter();
        let shapes = self.shapes.into_iter().filter_map(|s| match s.to_egui(ui) {
            Ok(s) => Some(s),
            Err(e) => {
                error!("Failed to convert Shape to Egui: {e}");
                None
            }
        });
        painter.extend(shapes);
    }
}
