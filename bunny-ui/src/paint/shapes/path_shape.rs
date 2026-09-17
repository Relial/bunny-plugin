use abi_stable::std_types::RVec;
use ecolor::Color32;
use emath::Pos2;
use mint::Point2;

use crate::paint::{shapes::shape::Shape, stroke::PathStroke};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct PathShape {
    pub points: RVec<Pos2>,
    pub stroke: PathStroke,
    pub fill: Color32,
    pub closed: bool,
}

impl PathShape {
    #[inline]
    pub fn line<P: Into<Point2<f32>>>(
        points: impl IntoIterator<Item = P>,
        stroke: impl Into<PathStroke>,
    ) -> Self {
        Self {
            points: points.into_iter().map(|p| p.into().into()).collect(),
            closed: false,
            fill: Default::default(),
            stroke: stroke.into(),
        }
    }

    #[inline]
    pub fn closed_line<P: Into<Point2<f32>>>(
        points: impl IntoIterator<Item = P>,
        stroke: impl Into<PathStroke>,
    ) -> Self {
        Self {
            points: points.into_iter().map(|p| p.into().into()).collect(),
            closed: true,
            fill: Default::default(),
            stroke: stroke.into(),
        }
    }

    #[inline]
    pub fn convex_polygon<P: Into<Point2<f32>>>(
        points: impl IntoIterator<Item = P>,
        fill: impl Into<Color32>,
        stroke: impl Into<PathStroke>,
    ) -> Self {
        Self {
            points: points.into_iter().map(|p| p.into().into()).collect(),
            closed: true,
            fill: fill.into(),
            stroke: stroke.into(),
        }
    }
}

impl From<PathShape> for Shape {
    #[inline]
    fn from(value: PathShape) -> Self {
        Shape::Path(value)
    }
}

#[cfg(feature = "manager")]
impl From<PathShape> for egui::epaint::PathShape {
    fn from(value: PathShape) -> Self {
        let PathShape {
            points,
            closed,
            fill,
            stroke,
        } = value;
        Self {
            points: points.into(),
            closed,
            fill,
            stroke: stroke.into(),
        }
    }
}
