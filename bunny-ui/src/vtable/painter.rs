use egui::{Painter, Rect};
use vtable::{VRef, VRefMut, vtable};

use crate::{LayerId, painter::BunnyPainter};

#[vtable]
#[repr(C)]
pub struct PainterFfiVTable {
    // new
    // with_layer_id
    with_clip_rect: fn(VRef<PainterFfiVTable>, rect: Rect) -> BunnyPainter,
    set_layer_id: fn(VRefMut<PainterFfiVTable>, layer_id: LayerId),
    set_opacity: fn(VRefMut<PainterFfiVTable>, opacity: f32),
    multiply_opacity: fn(VRefMut<PainterFfiVTable>, opacity: f32),
    opacity: fn(VRef<PainterFfiVTable>) -> f32,
    is_visible: fn(VRef<PainterFfiVTable>) -> bool,
    set_invisible: fn(VRefMut<PainterFfiVTable>),
    // ctx
    pixels_per_point: fn(VRef<PainterFfiVTable>) -> f32,
    // fonts
    // fonts_mut
    layer_id: fn(VRef<PainterFfiVTable>) -> LayerId,
    clip_rect: fn(VRef<PainterFfiVTable>) -> Rect,
    shrink_clip_rect: fn(VRefMut<PainterFfiVTable>, new_clip_rect: Rect),
    set_clip_rect: fn(VRefMut<PainterFfiVTable>, clip_rect: Rect),
    round_to_pixel_center: fn(VRef<PainterFfiVTable>, point: f32) -> f32,

    drop: fn(VRefMut<PainterFfiVTable>),
}

impl PainterFfi for Painter {
    #[inline]
    fn with_clip_rect(&self, rect: Rect) -> BunnyPainter {
        let painter = self.with_clip_rect(rect);
        BunnyPainter::new(painter)
    }

    #[inline]
    fn set_layer_id(&mut self, layer_id: LayerId) {
        self.set_layer_id(layer_id.into());
    }

    #[inline]
    fn set_opacity(&mut self, opacity: f32) {
        self.set_opacity(opacity);
    }

    #[inline]
    fn multiply_opacity(&mut self, opacity: f32) {
        self.multiply_opacity(opacity);
    }

    #[inline]
    fn opacity(&self) -> f32 {
        self.opacity()
    }

    #[inline]
    fn is_visible(&self) -> bool {
        self.is_visible()
    }

    #[inline]
    fn set_invisible(&mut self) {
        self.set_invisible();
    }

    #[inline]
    fn pixels_per_point(&self) -> f32 {
        self.pixels_per_point()
    }

    #[inline]
    fn layer_id(&self) -> LayerId {
        self.layer_id().into()
    }

    #[inline]
    fn clip_rect(&self) -> Rect {
        self.clip_rect()
    }

    #[inline]
    fn shrink_clip_rect(&mut self, new_clip_rect: Rect) {
        self.shrink_clip_rect(new_clip_rect);
    }

    #[inline]
    fn set_clip_rect(&mut self, clip_rect: Rect) {
        self.set_clip_rect(clip_rect);
    }

    #[inline]
    fn round_to_pixel_center(&self, point: f32) -> f32 {
        self.round_to_pixel_center(point)
    }
}

PainterFfiVTable_static!(static PAINTERFFI_VT for Painter);
