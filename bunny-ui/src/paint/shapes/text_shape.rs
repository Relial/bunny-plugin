use abi_stable::std_types::{
    RBox,
    ROption::{self, RNone, RSome},
};
use ecolor::Color32;
use emath::Pos2;
use mint::Point2;

use crate::{
    Align2,
    paint::{shapes::shape::Shape, stroke::Stroke, text::text_layout_types::LayoutJob},
};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TextShape {
    pub layout_job: RBox<LayoutJob>,
    pub pos: Pos2,
    pub anchor: Align2,
    pub underline: Stroke,
    pub override_text_color: ROption<Color32>,
    pub fallback_color: Color32,
    pub opacity_factor: f32,
    pub angle: f32,
}

impl TextShape {
    #[inline]
    pub fn new(
        pos: impl Into<Point2<f32>>,
        layout_job: LayoutJob,
        anchor: Align2,
        fallback_color: Color32,
    ) -> Self {
        Self {
            pos: pos.into().into(),
            layout_job: RBox::new(layout_job),
            anchor,
            underline: Stroke::NONE,
            fallback_color,
            override_text_color: RNone,
            opacity_factor: 1.0,
            angle: 0.0,
        }
    }

    #[inline]
    pub fn with_underline(mut self, underline: Stroke) -> Self {
        self.underline = underline;
        self
    }

    #[inline]
    pub fn with_override_text_color(mut self, override_text_color: Color32) -> Self {
        self.override_text_color = RSome(override_text_color);
        self
    }

    #[inline]
    pub fn with_angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    #[inline]
    pub fn with_opacity_factor(mut self, opacity_factor: f32) -> Self {
        self.opacity_factor = opacity_factor;
        self
    }
}

impl From<TextShape> for Shape {
    #[inline(always)]
    fn from(value: TextShape) -> Self {
        Self::Text(value)
    }
}

#[cfg(feature = "manager")]
impl TextShape {
    pub fn to_egui(self, ctx: &egui::Context) -> egui::epaint::TextShape {
        let TextShape {
            pos,
            layout_job,
            anchor,
            underline,
            fallback_color,
            override_text_color,
            opacity_factor,
            angle,
        } = self;
        let galley = ctx.fonts_mut(|f| f.layout_job(RBox::into_inner(layout_job).into()));
        let rect = anchor.anchor_size(pos, galley.size());
        egui::epaint::TextShape {
            pos: rect.min,
            galley,
            underline: underline.into(),
            fallback_color,
            override_text_color: override_text_color.into(),
            opacity_factor,
            angle,
        }
    }
}
