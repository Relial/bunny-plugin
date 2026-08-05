use std::f32;

use abi_stable::{
    external_types::RRwLock,
    rvec,
    std_types::{RArc, RString, RVec},
};
use egui::{
    Color32, Pos2, Rangef, Rect, Rgba, Vec2,
    emath::{GuiRounding, Rot2},
};

use crate::{
    align::Align2,
    image_source::ImageSource,
    paint::{
        corner_radius::CornerRadius,
        paintlist::{ClippedShape, PaintList, ShapeIdx},
        shape_transform::adjust_colors,
        shapes::{circle_shape::CircleShape, rect_shape::RectShape, shape::Shape},
        stroke::{PathStroke, Stroke, StrokeKind},
        text::{
            fonts::FontId,
            text_layout_types::{LayoutJob, TextFormat},
        },
    },
};

#[repr(C)]
#[derive(Clone)]
pub struct Painter<'a> {
    paint_list: RArc<RRwLock<PaintList<'a>>>,
    clip_rect: Rect,
    opacity_factor: f32,
    pixels_per_point: f32,
}

impl<'a> Painter<'a> {
    pub fn new(
        paint_list: RArc<RRwLock<PaintList<'a>>>,
        clip_rect: Rect,
        pixels_per_point: f32,
    ) -> Self {
        Self {
            paint_list,
            clip_rect,
            opacity_factor: 1.0,
            pixels_per_point,
        }
    }

    pub fn with_clip_rect(&self, clip_rect: Rect) -> Self {
        let mut painter = self.clone();
        painter.set_clip_rect(clip_rect);
        painter
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

    #[inline]
    pub fn opacity(&self) -> f32 {
        self.opacity_factor
    }

    #[inline]
    pub fn pixels_per_point(&self) -> f32 {
        self.pixels_per_point
    }

    #[inline]
    pub fn clip_rect(&self) -> Rect {
        self.clip_rect
    }

    #[inline]
    pub fn shrink_clip_rect(&mut self, new_clip_rect: Rect) {
        self.clip_rect = self.clip_rect.intersect(new_clip_rect);
    }

    #[inline]
    pub fn set_clip_rect(&mut self, clip_rect: Rect) {
        self.clip_rect = clip_rect;
    }

    #[inline]
    pub fn round_to_pixel_center(&self, point: f32) -> f32 {
        point.round_to_pixel_center(self.pixels_per_point())
    }
}

impl<'a> Painter<'a> {
    fn paint_list<R>(&self, writer: impl FnOnce(&mut PaintList<'a>) -> R) -> R {
        let mut list = self.paint_list.write();
        writer(&mut list)
    }

    fn transform_shape(&self, shape: &mut Shape) {
        if self.opacity_factor < 1.0 {
            multiply_opacity(shape, self.opacity_factor);
        }
    }

    pub fn add(&self, shape: impl Into<Shape<'a>>) -> ShapeIdx {
        if self.opacity_factor == 0.0 {
            self.paint_list(|l| l.add(self.clip_rect, Shape::Noop))
        } else {
            let mut shape = shape.into();
            self.transform_shape(&mut shape);
            self.paint_list(|l| l.add(self.clip_rect, shape))
        }
    }

    pub fn extend<I: IntoIterator<Item = Shape<'a>>>(&self, shapes: I) {
        if self.opacity_factor == 0.0 {
            return;
        }
        if self.opacity_factor < 1.0 {
            let shapes = shapes.into_iter().map(|mut shape| {
                self.transform_shape(&mut shape);
                shape
            });
            self.paint_list(|l| l.extend(self.clip_rect, shapes))
        } else {
            self.paint_list(|l| l.extend(self.clip_rect, shapes))
        }
    }

    pub fn set(&self, idx: ShapeIdx, shape: impl Into<Shape<'a>>) {
        let mut shape = shape.into();
        self.transform_shape(&mut shape);
        self.paint_list(|l| l.set(idx, self.clip_rect, shape));
    }

    pub fn for_each_shape(&self, mut reader: impl FnMut(&ClippedShape)) {
        self.paint_list(|l| {
            for c in l.all_entries() {
                reader(c)
            }
        });
    }
}

impl<'a> Painter<'a> {
    pub fn debug_rect(&self, rect: Rect, color: Color32, text: impl Into<RString>) -> ShapeIdx {
        self.rect(
            rect,
            0.0,
            color.additive().linear_multiply(0.015),
            (1.0, color),
            StrokeKind::Outside,
        );
        self.text(
            rect.min,
            Align2::LEFT_TOP,
            text,
            FontId::monospace(12.0),
            color,
        )
    }

    pub fn error(&self, pos: Pos2, text: impl std::fmt::Display) -> ShapeIdx {
        let color = Color32::RED;
        self.debug_text(pos, Align2::LEFT_TOP, color, format!("🔥 {text}"))
    }

    pub fn debug_text(
        &self,
        pos: Pos2,
        anchor: Align2,
        color: Color32,
        text: impl Into<RString>,
    ) -> ShapeIdx {
        let is_text_bright = color.is_additive() || Rgba::from(color).intensity() > 0.5;
        let bg_color = if is_text_bright {
            Color32::from_black_alpha(150)
        } else {
            Color32::from_white_alpha(150)
        };
        let text_format = TextFormat {
            font_id: FontId::monospace(12.0),
            color,
            background: bg_color,
            expand_bg: 2.0,
            ..Default::default()
        };
        let job = LayoutJob::single_section(text, text_format);
        let shape = Shape::text_with_layout_job(pos, anchor, job, color);
        self.add(shape)
    }
}

impl<'a> Painter<'a> {
    pub fn line_segment(&self, points: [Pos2; 2], stroke: impl Into<Stroke>) -> ShapeIdx {
        self.add(Shape::LineSegment {
            points,
            stroke: stroke.into(),
        })
    }

    pub fn line(&self, points: impl Into<RVec<Pos2>>, stroke: impl Into<PathStroke>) -> ShapeIdx {
        self.add(Shape::line(points, stroke))
    }

    pub fn hline(&self, x: impl Into<Rangef>, y: f32, stroke: impl Into<Stroke>) -> ShapeIdx {
        self.add(Shape::hline(x, y, stroke))
    }

    pub fn vline(&self, x: f32, y: impl Into<Rangef>, stroke: impl Into<Stroke>) -> ShapeIdx {
        self.add(Shape::vline(x, y, stroke))
    }

    pub fn circle(
        &self,
        center: Pos2,
        radius: f32,
        fill_color: impl Into<Color32>,
        stroke: impl Into<Stroke>,
    ) -> ShapeIdx {
        self.add(CircleShape {
            center,
            radius,
            fill: fill_color.into(),
            stroke: stroke.into(),
        })
    }

    pub fn circle_filled(
        &self,
        center: Pos2,
        radius: f32,
        fill_color: impl Into<Color32>,
    ) -> ShapeIdx {
        self.add(Shape::circle_filled(center, radius, fill_color))
    }

    pub fn circle_stroke(&self, center: Pos2, radius: f32, stroke: impl Into<Stroke>) -> ShapeIdx {
        self.add(Shape::circle_stroke(center, radius, stroke))
    }

    pub fn rect(
        &self,
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        fill_color: impl Into<Color32>,
        stroke: impl Into<Stroke>,
        stroke_kind: StrokeKind,
    ) -> ShapeIdx {
        self.add(RectShape::new(
            rect,
            corner_radius,
            fill_color,
            stroke,
            stroke_kind,
        ))
    }

    pub fn rect_filled(
        &self,
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        fill_color: impl Into<Color32>,
    ) -> ShapeIdx {
        self.add(Shape::rect_filled(rect, corner_radius, fill_color))
    }

    pub fn rect_stroke(
        &self,
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        stroke: impl Into<Stroke>,
        stroke_kind: StrokeKind,
    ) -> ShapeIdx {
        self.add(Shape::rect_stroke(rect, corner_radius, stroke, stroke_kind))
    }

    pub fn arrow(&self, origin: Pos2, vec: Vec2, stroke: impl Into<Stroke>) -> ShapeIdx {
        let rot = Rot2::from_angle(f32::consts::TAU / 10.0);
        let tip_length = vec.length() / 4.0;
        let tip = origin + vec;
        let dir = vec.normalized();
        let stroke = stroke.into();
        let s1 = Shape::line_segment([origin, tip], stroke);
        let s2 = Shape::line_segment([tip, tip - tip_length * (rot * dir)], stroke);
        let s3 = Shape::line_segment([tip, tip - tip_length * (rot.inverse() * dir)], stroke);
        let combined = Shape::Vec(rvec![s1, s2, s3]);
        self.add(combined)
    }

    pub fn image(
        &self,
        texture_source: ImageSource<'a>,
        rect: Rect,
        uv: Rect,
        tint: Color32,
    ) -> ShapeIdx {
        self.add(Shape::image(texture_source, rect, uv, tint))
    }
}

impl<'a> Painter<'a> {
    pub fn text(
        &self,
        pos: Pos2,
        anchor: Align2,
        text: impl Into<RString>,
        font_id: FontId,
        text_color: Color32,
    ) -> ShapeIdx {
        let shape = Shape::text(pos, anchor, text, font_id, text_color);
        self.add(shape)
    }

    pub fn text_with_layout_job(
        &self,
        pos: Pos2,
        anchor: Align2,
        layout_job: LayoutJob,
        fallback_color: Color32,
    ) -> ShapeIdx {
        let shape = Shape::text_with_layout_job(pos, anchor, layout_job, fallback_color);
        self.add(shape)
    }
}

fn multiply_opacity(shape: &mut Shape, opacity: f32) {
    adjust_colors(shape, move |color| {
        if *color != Color32::TRANSPARENT {
            *color = color.gamma_multiply(opacity);
        }
    });
}
