use abi_stable::std_types::RVec;
use egui::{Rect, Ui, emath::TSTransform};
use tracing::{debug, warn};

use crate::paint::shapes::shape::Shape;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShapeIdx(pub usize);

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct PaintList<'a>(RVec<ClippedShape<'a>>);

impl Default for PaintList<'_> {
    fn default() -> Self {
        Self(RVec::new())
    }
}

impl<'a> PaintList<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let shapes_clip_rects: Vec<(egui::Shape, Rect)> = self
            .0
            .drain(..)
            .filter_map(|s| match s.shape.to_egui(ui) {
                Ok(egui_shape) => Some((egui_shape, s.clip_rect)),
                Err(e) => {
                    debug!("Failed to convert Shape to Egui: {e}");
                    None
                }
            })
            .collect();
        ui.graphics_mut(|g| {
            let paint_list = g.entry(ui.layer_id());
            for (shape, clip_rect) in shapes_clip_rects {
                paint_list.add(clip_rect, shape);
            }
        });
    }
}

impl<'a> PaintList<'a> {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn next_idx(&self) -> ShapeIdx {
        ShapeIdx(self.0.len())
    }

    #[inline(always)]
    pub fn add(&mut self, clip_rect: Rect, shape: Shape<'a>) -> ShapeIdx {
        let idx = self.next_idx();
        self.0.push(ClippedShape { clip_rect, shape });
        idx
    }

    pub fn extend<I: IntoIterator<Item = Shape<'a>>>(&mut self, clip_rect: Rect, shapes: I) {
        self.0.extend(
            shapes
                .into_iter()
                .map(|shape| ClippedShape { clip_rect, shape }),
        );
    }

    #[inline(always)]
    pub fn set(&mut self, idx: ShapeIdx, clip_rect: Rect, shape: Shape<'a>) {
        if self.0.len() <= idx.0 {
            warn!("Index {} is out of bounds for PaintList", idx.0);
            return;
        }
        self.0[idx.0] = ClippedShape { clip_rect, shape };
    }

    #[inline(always)]
    pub fn reset_shape(&mut self, idx: ShapeIdx) {
        self.0[idx.0].shape = Shape::Noop;
    }

    pub fn mutate_shape(&mut self, idx: ShapeIdx, f: impl FnOnce(&mut ClippedShape)) {
        self.0.get_mut(idx.0).map(f);
    }

    pub fn transform(&mut self, transform: TSTransform) {
        for ClippedShape { clip_rect, shape } in &mut self.0 {
            *clip_rect = transform.mul_rect(*clip_rect);
            shape.transform(transform);
        }
    }

    pub fn transform_range(&mut self, start: ShapeIdx, end: ShapeIdx, transform: TSTransform) {
        for ClippedShape { clip_rect, shape } in &mut self.0[start.0..end.0] {
            *clip_rect = transform.mul_rect(*clip_rect);
            shape.transform(transform);
        }
    }

    pub fn all_entries(&self) -> impl ExactSizeIterator<Item = &ClippedShape<'a>> {
        self.0.iter()
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct ClippedShape<'a> {
    pub clip_rect: Rect,
    pub shape: Shape<'a>,
}
