use abi_stable::std_types::{RStr, RVec};
use ecolor::Color32;
use emath::{Pos2, Rangef, Rect, Vec2};
use vtable::{VRef, VRefMut, vtable};

use crate::{
    Align2, BunnyGalley, BunnyPainter, LayerId, ShapeIdx,
    paint::{CornerRadius, FontId, LayoutJob, PathStroke, Shape, Stroke, StrokeKind, TextureId},
};

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

    add: fn(VRef<PainterFfiVTable>, shape: Shape) -> ShapeIdx,
    extend: fn(VRef<PainterFfiVTable>, shapes: RVec<Shape>),
    set: fn(VRef<PainterFfiVTable>, idx: ShapeIdx, shape: Shape),
    // for_each_shape
    debug_rect: fn(VRef<PainterFfiVTable>, rect: Rect, color: Color32, text: RStr),
    error: fn(VRef<PainterFfiVTable>, pos: Pos2, text: RStr) -> Rect,
    debug_text:
        fn(VRef<PainterFfiVTable>, pos: Pos2, anchor: Align2, color: Color32, text: RStr) -> Rect,

    line_segment: fn(VRef<PainterFfiVTable>, points: [Pos2; 2], stroke: Stroke) -> ShapeIdx,
    line: fn(VRef<PainterFfiVTable>, points: RVec<Pos2>, stroke: PathStroke) -> ShapeIdx,
    hline: fn(VRef<PainterFfiVTable>, x: Rangef, y: f32, stroke: Stroke) -> ShapeIdx,
    vline: fn(VRef<PainterFfiVTable>, x: f32, y: Rangef, stroke: Stroke) -> ShapeIdx,
    circle: fn(
        VRef<PainterFfiVTable>,
        center: Pos2,
        radius: f32,
        fill_color: Color32,
        stroke: Stroke,
    ) -> ShapeIdx,
    circle_filled:
        fn(VRef<PainterFfiVTable>, center: Pos2, radius: f32, fill_color: Color32) -> ShapeIdx,
    circle_stroke:
        fn(VRef<PainterFfiVTable>, center: Pos2, radius: f32, stroke: Stroke) -> ShapeIdx,
    rect: fn(
        VRef<PainterFfiVTable>,
        rect: Rect,
        corner_radius: CornerRadius,
        fill_color: Color32,
        stroke: Stroke,
        stroke_kind: StrokeKind,
    ) -> ShapeIdx,
    rect_filled: fn(
        VRef<PainterFfiVTable>,
        rect: Rect,
        corner_radius: CornerRadius,
        fill_color: Color32,
    ) -> ShapeIdx,
    rect_stroke: fn(
        VRef<PainterFfiVTable>,
        rect: Rect,
        corner_radius: CornerRadius,
        stroke: Stroke,
        stroke_kind: StrokeKind,
    ) -> ShapeIdx,
    arrow: fn(VRef<PainterFfiVTable>, origin: Pos2, vec: Vec2, stroke: Stroke),
    image: fn(
        VRef<PainterFfiVTable>,
        texture_id: TextureId,
        rect: Rect,
        uv: Rect,
        tint: Color32,
    ) -> ShapeIdx,

    text: fn(
        VRef<PainterFfiVTable>,
        pos: Pos2,
        anchor: Align2,
        text: RStr,
        font_id: FontId,
        text_color: Color32,
    ) -> Rect,
    layout: fn(
        VRef<PainterFfiVTable>,
        text: RStr,
        font_id: FontId,
        color: Color32,
        wrap_width: f32,
    ) -> BunnyGalley,
    layout_no_wrap:
        fn(VRef<PainterFfiVTable>, text: RStr, font_id: FontId, color: Color32) -> BunnyGalley,
    layout_job: fn(VRef<PainterFfiVTable>, layout_job: LayoutJob) -> BunnyGalley,
    galley: fn(VRef<PainterFfiVTable>, pos: Pos2, galley: BunnyGalley, fallback_color: Color32),
    galley_with_override_text_color:
        fn(VRef<PainterFfiVTable>, pos: Pos2, galley: BunnyGalley, text_color: Color32),

    drop: fn(VRefMut<PainterFfiVTable>),
}

#[cfg(feature = "manager")]
impl PainterFfi for egui::Painter {
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

    #[inline]
    fn add(&self, shape: Shape) -> ShapeIdx {
        self.add(shape).into()
    }

    #[inline]
    fn extend(&self, shapes: RVec<Shape>) {
        self.extend(shapes.into_iter().map(|s| s.into()));
    }

    #[inline]
    fn set(&self, idx: ShapeIdx, shape: Shape) {
        self.set(idx.into(), shape);
    }

    #[inline]
    fn debug_rect(&self, rect: Rect, color: Color32, text: RStr<'_>) {
        self.debug_rect(rect, color, text);
    }

    #[inline]
    fn error(&self, pos: Pos2, text: RStr<'_>) -> Rect {
        self.error(pos, text)
    }

    #[inline]
    fn debug_text(&self, pos: Pos2, anchor: Align2, color: Color32, text: RStr<'_>) -> Rect {
        self.debug_text(pos, anchor.into(), color, text)
    }

    #[inline]
    fn line_segment(&self, points: [Pos2; 2], stroke: Stroke) -> ShapeIdx {
        self.line_segment(points, stroke).into()
    }

    #[inline]
    fn line(&self, points: RVec<Pos2>, stroke: PathStroke) -> ShapeIdx {
        self.line(points.into(), stroke).into()
    }

    #[inline]
    fn hline(&self, x: Rangef, y: f32, stroke: Stroke) -> ShapeIdx {
        self.hline(x, y, stroke).into()
    }

    #[inline]
    fn vline(&self, x: f32, y: Rangef, stroke: Stroke) -> ShapeIdx {
        self.vline(x, y, stroke).into()
    }

    #[inline]
    fn circle(&self, center: Pos2, radius: f32, fill_color: Color32, stroke: Stroke) -> ShapeIdx {
        self.circle(center, radius, fill_color, stroke).into()
    }

    #[inline]
    fn circle_filled(&self, center: Pos2, radius: f32, fill_color: Color32) -> ShapeIdx {
        self.circle_filled(center, radius, fill_color).into()
    }

    #[inline]
    fn circle_stroke(&self, center: Pos2, radius: f32, stroke: Stroke) -> ShapeIdx {
        self.circle_stroke(center, radius, stroke).into()
    }

    #[inline]
    fn rect(
        &self,
        rect: Rect,
        corner_radius: CornerRadius,
        fill_color: Color32,
        stroke: Stroke,
        stroke_kind: StrokeKind,
    ) -> ShapeIdx {
        self.rect(rect, corner_radius, fill_color, stroke, stroke_kind.into())
            .into()
    }

    #[inline]
    fn rect_filled(
        &self,
        rect: Rect,
        corner_radius: CornerRadius,
        fill_color: Color32,
    ) -> ShapeIdx {
        self.rect_filled(rect, corner_radius, fill_color).into()
    }

    #[inline]
    fn rect_stroke(
        &self,
        rect: Rect,
        corner_radius: CornerRadius,
        stroke: Stroke,
        stroke_kind: StrokeKind,
    ) -> ShapeIdx {
        self.rect_stroke(rect, corner_radius, stroke, stroke_kind.into())
            .into()
    }

    #[inline]
    fn arrow(&self, origin: Pos2, vec: Vec2, stroke: Stroke) {
        self.arrow(origin, vec, stroke);
    }

    #[inline]
    fn image(&self, texture_id: TextureId, rect: Rect, uv: Rect, tint: Color32) -> ShapeIdx {
        self.image(texture_id.into(), rect, uv, tint).into()
    }

    #[inline]
    fn text(
        &self,
        pos: Pos2,
        anchor: Align2,
        text: RStr<'_>,
        font_id: FontId,
        text_color: Color32,
    ) -> Rect {
        self.text(pos, anchor.into(), text, font_id.into(), text_color)
    }

    #[inline]
    fn layout(
        &self,
        text: RStr<'_>,
        font_id: FontId,
        color: Color32,
        wrap_width: f32,
    ) -> BunnyGalley {
        BunnyGalley::new(self.layout(text.into(), font_id.into(), color, wrap_width))
    }

    #[inline]
    fn layout_no_wrap(&self, text: RStr<'_>, font_id: FontId, color: Color32) -> BunnyGalley {
        BunnyGalley::new(self.layout_no_wrap(text.into(), font_id.into(), color))
    }

    #[inline]
    fn layout_job(&self, layout_job: LayoutJob) -> BunnyGalley {
        BunnyGalley::new(self.layout_job(layout_job.into()))
    }

    #[inline]
    fn galley(&self, pos: Pos2, galley: BunnyGalley, fallback_color: Color32) {
        self.galley(pos, galley.into_inner(), fallback_color);
    }

    #[inline]
    fn galley_with_override_text_color(&self, pos: Pos2, galley: BunnyGalley, text_color: Color32) {
        self.galley_with_override_text_color(pos, galley.into_inner(), text_color);
    }
}

#[cfg(feature = "manager")]
PainterFfiVTable_static!(static PAINTERFFI_VT for egui::Painter);
