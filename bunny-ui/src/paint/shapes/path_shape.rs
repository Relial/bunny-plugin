use abi_stable::std_types::RVec;
use egui::{Color32, Pos2};

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
    pub fn line(points: impl Into<RVec<Pos2>>, stroke: impl Into<PathStroke>) -> Self {
        Self {
            points: points.into(),
            closed: false,
            fill: Default::default(),
            stroke: stroke.into(),
        }
    }

    #[inline]
    pub fn closed_line(points: impl Into<RVec<Pos2>>, stroke: impl Into<PathStroke>) -> Self {
        Self {
            points: points.into(),
            closed: true,
            fill: Default::default(),
            stroke: stroke.into(),
        }
    }

    #[inline]
    pub fn convex_polygon(
        points: impl Into<RVec<Pos2>>,
        fill: impl Into<Color32>,
        stroke: impl Into<PathStroke>,
    ) -> Self {
        Self {
            points: points.into(),
            closed: true,
            fill: fill.into(),
            stroke: stroke.into(),
        }
    }
}

impl From<PathShape> for Shape<'_> {
    fn from(value: PathShape) -> Self {
        Shape::Path(value)
    }
}

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
