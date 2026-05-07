use egui::Color32;

use crate::{
    margin::Margin, paint::{corner_radius::CornerRadius, stroke::Stroke}, shadow::Shadow
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Frame {
    pub inner_margin: Margin,
    pub fill: Color32,
    pub stroke: Stroke,
    pub corner_radius: CornerRadius,
    pub outer_margin: Margin,
    pub shadow: Shadow,
}

impl Frame {
    pub const NONE: Self = Self {
        inner_margin: Margin::ZERO,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::NONE,
        corner_radius: CornerRadius::ZERO,
        outer_margin: Margin::ZERO,
        shadow: Shadow::NONE,
    };

    pub const fn new() -> Self {
        Self::NONE
    }
}

impl Frame {
    #[inline]
    pub fn inner_margin(mut self, inner_margin: impl Into<Margin>) -> Self {
        self.inner_margin = inner_margin.into();
        self
    }

    #[inline]
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = fill;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    #[inline]
    pub fn outer_margin(mut self, outer_margin: impl Into<Margin>) -> Self {
        self.outer_margin = outer_margin.into();
        self
    }

    #[inline]
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = shadow;
        self
    }

    #[inline]
    pub fn multiply_with_opacity(mut self, opacity: f32) -> Self {
        self.fill = self.fill.gamma_multiply(opacity);
        self.stroke.color = self.stroke.color.gamma_multiply(opacity);
        self.shadow.color = self.shadow.color.gamma_multiply(opacity);
        self
    }
}

impl From<Frame> for egui::Frame {
    fn from(value: Frame) -> Self {
        Self {
            inner_margin: value.inner_margin.into(),
            fill: value.fill,
            stroke: value.stroke.into(),
            corner_radius: value.corner_radius.into(),
            outer_margin: value.outer_margin.into(),
            shadow: value.shadow.into(),
        }
    }
}
