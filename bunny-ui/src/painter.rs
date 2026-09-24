use abi_stable::std_types::RVec;
use ecolor::Color32;
use emath::{Rangef, Rect};
use mint::{Point2, Vector2};
use vtable::{VBox, VRef};

use crate::{
    Align2, BunnyGalley, LayerId, ShapeIdx,
    paint::{CornerRadius, FontId, LayoutJob, PathStroke, Shape, Stroke, StrokeKind, TextureId},
    vtable::painter::PainterFfiVTable,
};

pub trait BunnyPaint {
    fn as_ref(&self) -> VRef<'_, PainterFfiVTable>;

    #[inline]
    fn with_clip_rect(&self, rect: Rect) -> BunnyPainter {
        self.as_ref().with_clip_rect(rect)
    }

    #[inline]
    fn opacity(&self) -> f32 {
        self.as_ref().opacity()
    }

    #[inline]
    fn is_visible(&self) -> bool {
        self.as_ref().is_visible()
    }

    #[inline]
    fn pixels_per_point(&self) -> f32 {
        self.as_ref().pixels_per_point()
    }

    #[inline]
    fn layer_id(&self) -> LayerId {
        self.as_ref().layer_id()
    }

    #[inline]
    fn clip_rect(&self) -> Rect {
        self.as_ref().clip_rect()
    }

    #[inline]
    fn round_to_pixel_center(&self, point: f32) -> f32 {
        self.as_ref().round_to_pixel_center(point)
    }

    #[inline]
    fn add(&self, shape: impl Into<Shape>) -> ShapeIdx {
        self.as_ref().add(shape.into())
    }

    #[inline]
    fn extend(&self, shapes: impl Into<RVec<Shape>>) {
        self.as_ref().extend(shapes.into());
    }

    #[inline]
    fn set(&self, idx: ShapeIdx, shape: impl Into<Shape>) {
        self.as_ref().set(idx, shape.into());
    }

    #[inline]
    fn debug_rect(&self, rect: Rect, color: impl Into<Color32>, text: impl AsRef<str>) {
        self.as_ref()
            .debug_rect(rect, color.into(), text.as_ref().into());
    }

    #[inline]
    fn error(&self, pos: impl Into<Point2<f32>>, text: impl AsRef<str>) -> Rect {
        self.as_ref().error(pos.into().into(), text.as_ref().into())
    }

    #[inline]
    fn debug_text(
        &self,
        pos: impl Into<Point2<f32>>,
        anchor: Align2,
        color: impl Into<Color32>,
        text: impl AsRef<str>,
    ) -> Rect {
        self.as_ref().debug_text(
            pos.into().into(),
            anchor,
            color.into(),
            text.as_ref().into(),
        )
    }

    #[inline]
    fn line_segment(
        &self,
        points: [impl Into<Point2<f32>>; 2],
        stroke: impl Into<Stroke>,
    ) -> ShapeIdx {
        self.as_ref()
            .line_segment(points.map(|p| p.into().into()), stroke.into())
    }

    #[inline]
    fn line<P: Into<Point2<f32>>>(
        &self,
        points: impl IntoIterator<Item = P>,
        stroke: PathStroke,
    ) -> ShapeIdx {
        self.as_ref().line(
            points.into_iter().map(|p| p.into().into()).collect(),
            stroke,
        )
    }

    #[inline]
    fn hline(&self, x: impl Into<Rangef>, y: f32, stroke: impl Into<Stroke>) -> ShapeIdx {
        self.as_ref().hline(x.into(), y, stroke.into())
    }

    #[inline]
    fn vline(&self, x: f32, y: impl Into<Rangef>, stroke: impl Into<Stroke>) -> ShapeIdx {
        self.as_ref().vline(x, y.into(), stroke.into())
    }

    #[inline]
    fn circle(
        &self,
        center: impl Into<Point2<f32>>,
        radius: f32,
        fill_color: impl Into<Color32>,
        stroke: impl Into<Stroke>,
    ) -> ShapeIdx {
        self.as_ref().circle(
            center.into().into(),
            radius,
            fill_color.into(),
            stroke.into(),
        )
    }

    #[inline]
    fn circle_filled(
        &self,
        center: impl Into<Point2<f32>>,
        radius: f32,
        fill_color: impl Into<Color32>,
    ) -> ShapeIdx {
        self.as_ref()
            .circle_filled(center.into().into(), radius, fill_color.into())
    }

    #[inline]
    fn circle_stroke(
        &self,
        center: impl Into<Point2<f32>>,
        radius: f32,
        stroke: impl Into<Stroke>,
    ) -> ShapeIdx {
        self.as_ref()
            .circle_stroke(center.into().into(), radius, stroke.into())
    }

    #[inline]
    fn rect(
        &self,
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        fill_color: impl Into<Color32>,
        stroke: impl Into<Stroke>,
        stroke_kind: StrokeKind,
    ) -> ShapeIdx {
        self.as_ref().rect(
            rect,
            corner_radius.into(),
            fill_color.into(),
            stroke.into(),
            stroke_kind,
        )
    }

    #[inline]
    fn rect_filled(
        &self,
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        fill_color: impl Into<Color32>,
    ) -> ShapeIdx {
        self.as_ref()
            .rect_filled(rect, corner_radius.into(), fill_color.into())
    }

    #[inline]
    fn rect_stroke(
        &self,
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        stroke: impl Into<Stroke>,
        stroke_kind: StrokeKind,
    ) -> ShapeIdx {
        self.as_ref()
            .rect_stroke(rect, corner_radius.into(), stroke.into(), stroke_kind)
    }

    #[inline]
    fn arrow(
        &self,
        origin: impl Into<Point2<f32>>,
        vec: impl Into<Vector2<f32>>,
        stroke: impl Into<Stroke>,
    ) {
        self.as_ref()
            .arrow(origin.into().into(), vec.into().into(), stroke.into());
    }

    #[inline]
    fn image(
        &self,
        texture_id: impl Into<TextureId>,
        rect: Rect,
        uv: Rect,
        tint: impl Into<Color32>,
    ) -> ShapeIdx {
        self.as_ref()
            .image(texture_id.into(), rect, uv, tint.into())
    }

    #[inline]
    fn text(
        &self,
        pos: impl Into<Point2<f32>>,
        anchor: Align2,
        text: impl AsRef<str>,
        font_id: FontId,
        text_color: impl Into<Color32>,
    ) -> Rect {
        self.as_ref().text(
            pos.into().into(),
            anchor,
            text.as_ref().into(),
            font_id,
            text_color.into(),
        )
    }

    #[inline]
    fn layout(
        &self,
        text: impl AsRef<str>,
        font_id: FontId,
        color: impl Into<Color32>,
        wrap_width: f32,
    ) -> BunnyGalley {
        self.as_ref()
            .layout(text.as_ref().into(), font_id, color.into(), wrap_width)
    }

    #[inline]
    fn layout_no_wrap(
        &self,
        text: impl AsRef<str>,
        font_id: FontId,
        color: impl Into<Color32>,
    ) -> BunnyGalley {
        self.as_ref()
            .layout_no_wrap(text.as_ref().into(), font_id, color.into())
    }

    #[inline]
    fn layout_job(&self, layout_job: LayoutJob) -> BunnyGalley {
        self.as_ref().layout_job(layout_job)
    }

    #[inline]
    fn galley(
        &self,
        pos: impl Into<Point2<f32>>,
        galley: BunnyGalley,
        fallback_color: impl Into<Color32>,
    ) {
        self.as_ref()
            .galley(pos.into().into(), galley, fallback_color.into());
    }

    #[inline]
    fn galley_with_override_text_color(
        &self,
        pos: impl Into<Point2<f32>>,
        galley: BunnyGalley,
        text_color: impl Into<Color32>,
    ) {
        self.as_ref()
            .galley_with_override_text_color(pos.into().into(), galley, text_color.into());
    }
}

#[repr(transparent)]
pub struct BunnyPainterRef<'a> {
    inner: VRef<'a, PainterFfiVTable>,
}

#[cfg(feature = "manager")]
impl<'a> BunnyPainterRef<'a> {
    #[inline]
    pub fn new(painter: &'a egui::Painter) -> Self {
        Self {
            inner: VRef::new(painter),
        }
    }
}

impl<'a> BunnyPaint for BunnyPainterRef<'a> {
    #[inline(always)]
    fn as_ref(&self) -> VRef<'_, PainterFfiVTable> {
        self.inner
    }
}

#[repr(transparent)]
pub struct BunnyPainter {
    inner: VBox<PainterFfiVTable>,
}

#[cfg(feature = "manager")]
impl BunnyPainter {
    #[inline]
    pub fn new(painter: egui::Painter) -> Self {
        Self {
            inner: VBox::new(painter),
        }
    }
}

impl BunnyPaint for BunnyPainter {
    #[inline(always)]
    fn as_ref(&self) -> VRef<'_, PainterFfiVTable> {
        self.inner.borrow()
    }
}

impl BunnyPainter {
    #[inline]
    pub fn set_layer_id(&mut self, layer_id: LayerId) {
        self.inner.set_layer_id(layer_id);
    }

    #[inline]
    pub fn set_opacity(&mut self, opacity: f32) {
        self.inner.set_opacity(opacity);
    }

    #[inline]
    pub fn multiply_opacity(&mut self, opacity: f32) {
        self.inner.multiply_opacity(opacity);
    }

    #[inline]
    pub fn set_invisible(&mut self) {
        self.inner.set_invisible();
    }

    #[inline]
    pub fn shrink_clip_rect(&mut self, new_clip_rect: Rect) {
        self.inner.shrink_clip_rect(new_clip_rect);
    }

    #[inline]
    pub fn set_clip_rect(&mut self, clip_rect: Rect) {
        self.inner.set_clip_rect(clip_rect);
    }
}
