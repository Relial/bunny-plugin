use abi_stable::std_types::{
    RBox,
    ROption::{self, RNone, RSome},
};
use egui::{Color32, Pos2, Rect, emath::Rot2};

use crate::{
    image_source::ImageSource,
    paint::{
        brush::Brush,
        corner_radius::CornerRadius,
        shapes::shape::Shape,
        stroke::{Stroke, StrokeKind},
    },
};

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct RectShape<'a> {
    pub brush: ROption<RBox<Brush<'a>>>,
    pub rect: Rect,
    pub stroke: Stroke,
    pub corner_radius: CornerRadius,
    pub fill: Color32,
    pub stroke_kind: StrokeKind,
    pub blur_width: f32,
    pub angle: f32,
    pub round_to_pixels: ROption<bool>,
}

impl<'a> RectShape<'a> {
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
    pub fn with_texture(mut self, fill_texture_source: ImageSource<'a>, uv: Rect) -> Self {
        self.brush = RSome(RBox::new(Brush {
            fill_texture_source,
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
    pub fn with_angle_and_pivot(mut self, angle: f32, pivot: Pos2) -> Self {
        self.angle = angle;
        let rot = Rot2::from_angle(angle);
        let center = self.rect.center();
        let new_center = pivot + rot * (center - pivot);
        self.rect = self.rect.translate(new_center - center);
        self
    }
}

impl<'a> From<RectShape<'a>> for Shape<'a> {
    fn from(value: RectShape<'a>) -> Self {
        Self::Rect(value)
    }
}

#[cfg(feature = "manager")]
impl<'a> RectShape<'a> {
    pub fn to_egui(self, ctx: &egui::Context) -> anyhow::Result<egui::epaint::RectShape> {
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
        } = self;
        let brush: Option<std::sync::Arc<egui::epaint::Brush>> = if let RSome(brush) = brush {
            let egui_brush = RBox::into_inner(brush).to_egui(ctx)?;
            Some(std::sync::Arc::new(egui_brush))
        } else {
            None
        };
        Ok(egui::epaint::RectShape {
            rect,
            corner_radius: corner_radius.into(),
            fill,
            stroke: stroke.into(),
            stroke_kind: stroke_kind.into(),
            round_to_pixels: round_to_pixels.into(),
            blur_width,
            brush,
            angle,
        })
    }
}
