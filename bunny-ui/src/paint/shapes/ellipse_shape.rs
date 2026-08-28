use ecolor::Color32;
use emath::{Pos2, Rot2, Vec2};

use crate::paint::{shapes::shape::Shape, stroke::Stroke};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct EllipseShape {
    pub center: Pos2,
    pub radius: Vec2,
    pub stroke: Stroke,
    pub fill: Color32,
    pub angle: f32,
}

impl EllipseShape {
    #[inline]
    pub fn filled(center: Pos2, radius: Vec2, fill_color: impl Into<Color32>) -> Self {
        Self {
            center,
            radius,
            fill: fill_color.into(),
            stroke: Default::default(),
            angle: 0.0,
        }
    }

    #[inline]
    pub fn stroke(center: Pos2, radius: Vec2, stroke: impl Into<Stroke>) -> Self {
        Self {
            center,
            radius,
            fill: Default::default(),
            stroke: stroke.into(),
            angle: 0.0,
        }
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
        self.center = pivot + rot * (self.center - pivot);
        self
    }
}

impl From<EllipseShape> for Shape<'_> {
    #[inline]
    fn from(value: EllipseShape) -> Self {
        Self::Ellipse(value)
    }
}

#[cfg(feature = "manager")]
impl From<EllipseShape> for egui::epaint::EllipseShape {
    fn from(value: EllipseShape) -> Self {
        let EllipseShape {
            center,
            radius,
            fill,
            stroke,
            angle,
        } = value;
        Self {
            center,
            radius,
            fill,
            stroke: stroke.into(),
            angle,
        }
    }
}
