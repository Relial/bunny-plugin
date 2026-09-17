use abi_stable::std_types::{
    RBox,
    ROption::{self, RNone, RSome},
};
use ecolor::Color32;
use emath::{Pos2, Rect, Rot2};
use mint::Point2;

use crate::paint::{
    TextureId,
    brush::Brush,
    corner_radius::CornerRadius,
    shapes::shape::Shape,
    stroke::{Stroke, StrokeKind},
};

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct RectShape {
    pub brush: ROption<RBox<Brush>>,
    pub rect: Rect,
    pub stroke: Stroke,
    pub corner_radius: CornerRadius,
    pub fill: Color32,
    pub stroke_kind: StrokeKind,
    pub blur_width: f32,
    pub angle: f32,
    pub round_to_pixels: ROption<bool>,
}

impl RectShape {
    #[inline]
    pub fn new(
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        fill_color: impl Into<Color32>,
        stroke: impl Into<Stroke>,
        stroke_kind: StrokeKind,
    ) -> Self {
        Self {
            rect,
            corner_radius: corner_radius.into(),
            fill: fill_color.into(),
            stroke: stroke.into(),
            stroke_kind,
            round_to_pixels: RNone,
            blur_width: 0.0,
            brush: Default::default(),
            angle: 0.0,
        }
    }

    #[inline]
    pub fn filled(
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        fill_color: impl Into<Color32>,
    ) -> Self {
        Self::new(
            rect,
            corner_radius,
            fill_color,
            Stroke::NONE,
            StrokeKind::Outside,
        )
    }

    #[inline]
    pub fn stroke(
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        stroke: impl Into<Stroke>,
        stroke_kind: StrokeKind,
    ) -> Self {
        let fill = Color32::TRANSPARENT;
        Self::new(rect, corner_radius, fill, stroke, stroke_kind)
    }

    #[inline]
    pub fn with_stroke_kind(mut self, stroke_kind: StrokeKind) -> Self {
        self.stroke_kind = stroke_kind;
        self
    }

    #[inline]
    pub fn with_round_to_pixels(mut self, round_to_pixels: bool) -> Self {
        self.round_to_pixels = RSome(round_to_pixels);
        self
    }

    #[inline]
    pub fn with_blur_width(mut self, blur_width: f32) -> Self {
        self.blur_width = blur_width;
        self
    }

    #[inline]
    pub fn with_texture(mut self, fill_texture_id: TextureId, uv: Rect) -> Self {
        self.brush = RSome(RBox::new(Brush {
            fill_texture_id,
            uv,
        }));
        self
    }

    #[inline]
    pub fn with_angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    #[inline]
    pub fn with_angle_and_pivot(mut self, angle: f32, pivot: impl Into<Point2<f32>>) -> Self {
        let pivot: Pos2 = pivot.into().into();
        self.angle = angle;
        let rot = Rot2::from_angle(angle);
        let center = self.rect.center();
        let new_center = pivot + rot * (center - pivot);
        self.rect = self.rect.translate(new_center - center);
        self
    }
}

impl From<RectShape> for Shape {
    #[inline]
    fn from(value: RectShape) -> Self {
        Self::Rect(value)
    }
}

#[cfg(feature = "manager")]
impl From<RectShape> for egui::epaint::RectShape {
    fn from(value: RectShape) -> Self {
        let RectShape {
            rect,
            corner_radius,
            fill,
            stroke,
            stroke_kind,
            round_to_pixels,
            blur_width,
            brush,
            angle,
        } = value;
        let brush: Option<std::sync::Arc<egui::epaint::Brush>> = brush
            .map(|b| std::sync::Arc::new(RBox::into_inner(b).into()))
            .into();
        Self {
            rect,
            corner_radius: corner_radius.into(),
            fill,
            stroke: stroke.into(),
            stroke_kind: stroke_kind.into(),
            round_to_pixels: round_to_pixels.into_option(),
            blur_width,
            brush,
            angle,
        }
    }
}
