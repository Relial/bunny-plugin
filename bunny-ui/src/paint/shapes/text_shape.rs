use abi_stable::std_types::ROption::{self, RNone, RSome};
use ecolor::Color32;
use emath::Pos2;
use mint::Point2;

use crate::{
    galley::BunnyGalley,
    paint::{shapes::shape::Shape, stroke::Stroke},
};

#[repr(C)]
pub struct TextShape {
    pub galley: BunnyGalley,
    pub pos: Pos2,
    pub underline: Stroke,
    pub override_text_color: ROption<Color32>,
    pub fallback_color: Color32,
    pub opacity_factor: f32,
    pub angle: f32,
}

impl TextShape {
    #[inline]
    pub fn new(pos: impl Into<Point2<f32>>, galley: BunnyGalley, fallback_color: Color32) -> Self {
        Self {
            pos: pos.into().into(),
            galley,
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
impl From<TextShape> for egui::epaint::TextShape {
    fn from(value: TextShape) -> Self {
        let TextShape {
            galley,
            pos,
            underline,
            override_text_color,
            fallback_color,
            opacity_factor,
            angle,
        } = value;
        egui::epaint::TextShape {
            pos,
            galley: galley.into_inner(),
            underline: underline.into(),
            fallback_color,
            override_text_color: override_text_color.into(),
            opacity_factor,
            angle,
        }
    }
}
