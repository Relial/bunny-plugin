use egui::{Color32, Pos2, emath::RectTransform};

use crate::paint::stroke::PathStroke;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct QuadraticBezierShape {
    pub points: [Pos2; 3],
    pub stroke: PathStroke,
    pub fill: Color32,
    pub closed: bool,
}

impl QuadraticBezierShape {
    pub fn from_points_stroke(
        points: [Pos2; 3],
        closed: bool,
        fill: Color32,
        stroke: impl Into<PathStroke>,
    ) -> Self {
        Self {
            points,
            closed,
            fill,
            stroke: stroke.into(),
        }
    }

    pub fn transform(&self, transform: &RectTransform) -> Self {
        let mut points = [Pos2::default(); 3];
        for (i, origin_point) in self.points.iter().enumerate() {
            points[i] = transform * *origin_point;
        }
        Self {
            points,
            closed: self.closed,
            fill: self.fill,
            stroke: self.stroke,
        }
    }

    pub fn sample(&self, t: f32) -> Pos2 {
        let h = 1.0 - t;
        let a = t * t;
        let b = 2.0 * t * h;
        let c = h * h;
        let result = self.points[2].to_vec2() * a
            + self.points[1].to_vec2() * b
            + self.points[0].to_vec2() * c;
        result.to_pos2()
    }
}

#[cfg(feature = "manager")]
impl From<QuadraticBezierShape> for egui::epaint::QuadraticBezierShape {
    fn from(value: QuadraticBezierShape) -> Self {
        let QuadraticBezierShape {
            points,
            closed,
            fill,
            stroke,
        } = value;
        Self {
            points,
            closed,
            fill,
            stroke: stroke.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct CubicBezierShape {
    pub points: [Pos2; 4],
    pub stroke: PathStroke,
    pub fill: Color32,
    pub closed: bool,
}

impl CubicBezierShape {
    pub fn from_points_stroke(
        points: [Pos2; 4],
        closed: bool,
        fill: Color32,
        stroke: impl Into<PathStroke>,
    ) -> Self {
        Self {
            points,
            closed,
            fill,
            stroke: stroke.into(),
        }
    }

    pub fn transform(&self, transform: &RectTransform) -> Self {
        let mut points = [Pos2::default(); 4];
        for (i, origin_point) in self.points.iter().enumerate() {
            points[i] = transform * *origin_point;
        }
        Self {
            points,
            closed: self.closed,
            fill: self.fill,
            stroke: self.stroke,
        }
    }

    pub fn sample(&self, t: f32) -> Pos2 {
        let h = 1.0 - t;
        let a = t * t * t;
        let b = 3.0 * t * t * h;
        let c = 3.0 * t * h * h;
        let d = h * h * h;
        let result = self.points[3].to_vec2() * a
            + self.points[2].to_vec2() * b
            + self.points[1].to_vec2() * c
            + self.points[0].to_vec2() * d;
        result.to_pos2()
    }
}

#[cfg(feature = "manager")]
impl From<CubicBezierShape> for egui::epaint::CubicBezierShape {
    fn from(value: CubicBezierShape) -> Self {
        let CubicBezierShape {
            points,
            closed,
            fill,
            stroke,
        } = value;
        Self {
            points,
            closed,
            fill,
            stroke: stroke.into(),
        }
    }
}
