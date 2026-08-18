use ecolor::Color32;
use emath::Pos2;

use crate::paint::{shapes::shape::Shape, stroke::Stroke};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct CircleShape {
    pub center: Pos2,
    pub stroke: Stroke,
    pub radius: f32,
    pub fill: Color32,
}

impl CircleShape {
    #[inline]
    pub fn filled(center: Pos2, radius: f32, fill_color: impl Into<Color32>) -> Self {
        Self {
            center,
            radius,
            fill: fill_color.into(),
            stroke: Default::default(),
        }
    }

    #[inline]
    pub fn stroke(center: Pos2, radius: f32, stroke: impl Into<Stroke>) -> Self {
        Self {
            center,
            radius,
            fill: Default::default(),
            stroke: stroke.into(),
        }
    }
}

impl From<CircleShape> for Shape<'_> {
    fn from(value: CircleShape) -> Self {
        Self::Circle(value)
    }
}

#[cfg(feature = "manager")]
impl From<CircleShape> for egui::epaint::CircleShape {
    fn from(value: CircleShape) -> Self {
        let CircleShape {
            center,
            radius,
            fill,
            stroke,
        } = value;
        Self {
            center,
            radius,
            fill,
            stroke: stroke.into(),
        }
    }
}
