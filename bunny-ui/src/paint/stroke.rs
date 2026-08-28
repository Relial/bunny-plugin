use ecolor::Color32;

use crate::paint::color::ColorMode;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Stroke {
    pub width: f32,
    pub color: Color32,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            width: 1.0,
            color: Color32::BLACK,
        }
    }
}

impl Stroke {
    pub const NONE: Self = Self {
        width: 0.0,
        color: Color32::TRANSPARENT,
    };

    #[inline]
    pub fn new(width: f32, color: impl Into<Color32>) -> Self {
        Self {
            width,
            color: color.into(),
        }
    }
}

impl<Color> From<(f32, Color)> for Stroke
where
    Color: Into<Color32>,
{
    #[inline(always)]
    fn from((width, color): (f32, Color)) -> Self {
        Self::new(width, color)
    }
}

#[cfg(feature = "manager")]
impl From<Stroke> for egui::Stroke {
    #[inline]
    fn from(value: Stroke) -> Self {
        Self {
            width: value.width,
            color: value.color,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::Stroke> for Stroke {
    #[inline]
    fn from(value: egui::Stroke) -> Self {
        Self {
            width: value.width,
            color: value.color,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum StrokeKind {
    Inside,
    Middle,
    Outside,
}

#[cfg(feature = "manager")]
impl From<StrokeKind> for egui::StrokeKind {
    #[inline]
    fn from(value: StrokeKind) -> Self {
        match value {
            StrokeKind::Inside => Self::Inside,
            StrokeKind::Middle => Self::Middle,
            StrokeKind::Outside => Self::Outside,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct PathStroke {
    pub color: ColorMode,
    pub width: f32,
    pub kind: StrokeKind,
}

impl Default for PathStroke {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

impl PathStroke {
    pub const NONE: Self = Self {
        width: 0.0,
        color: ColorMode::TRANSPARENT,
        kind: StrokeKind::Middle,
    };
}

#[cfg(feature = "manager")]
impl From<PathStroke> for egui::epaint::PathStroke {
    #[inline]
    fn from(value: PathStroke) -> Self {
        let PathStroke { width, color, kind } = value;
        Self {
            width,
            color: color.into(),
            kind: kind.into(),
        }
    }
}
