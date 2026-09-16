use egui::{Color32, Rangef};

use crate::paint::{corner_radius::CornerRadius, stroke::Stroke};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum TextStyle {
    Small,
    Body,
    Monospace,
    Button,
    Heading,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct ScrollFadeStyle {
    pub strength: f32,
    pub size: f32,
}

impl Default for ScrollFadeStyle {
    fn default() -> Self {
        Self {
            strength: 0.5,
            size: 20.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct ScrollAnimation {
    pub duration: Rangef,
    pub points_per_second: f32,
}

impl Default for ScrollAnimation {
    fn default() -> Self {
        Self {
            points_per_second: 1000.0,
            duration: Rangef::new(0.1, 0.3),
        }
    }
}

impl ScrollAnimation {
    pub fn new(points_per_second: f32, duration: Rangef) -> Self {
        Self {
            points_per_second,
            duration,
        }
    }

    pub fn none() -> Self {
        Self {
            points_per_second: f32::INFINITY,
            duration: Rangef::new(0.0, 0.0),
        }
    }

    pub fn duration(t: f32) -> Self {
        Self {
            points_per_second: f32::INFINITY,
            duration: Rangef::new(t, t),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct TextCursorStyle {
    pub stroke: Stroke,
    pub on_duration: f32,
    pub off_duration: f32,
    pub preview: bool,
    pub blink: bool,
}

impl Default for TextCursorStyle {
    fn default() -> Self {
        Self {
            stroke: Stroke::new(2.0, Color32::from_rgb(192, 222, 255)),
            preview: false,
            blink: true,
            on_duration: 0.5,
            off_duration: 0.5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Selection {
    pub stroke: Stroke,
    pub bg_fill: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum HandleShape {
    Rect { aspect_ratio: f32 },
    Circle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct WidgetVisuals {
    pub bg_stroke: Stroke,
    pub fg_stroke: Stroke,
    pub bg_fill: Color32,
    pub weak_bg_fill: Color32,
    pub corner_radius: CornerRadius,
    pub expansion: f32,
}

impl WidgetVisuals {
    #[inline(always)]
    pub fn text_color(&self) -> Color32 {
        self.fg_stroke.color
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum NumericColorSpace {
    GammaByte,
    Linear,
}

#[cfg(feature = "manager")]
impl From<WidgetVisuals> for egui::style::WidgetVisuals {
    #[inline]
    fn from(value: WidgetVisuals) -> Self {
        Self {
            bg_fill: value.bg_fill,
            weak_bg_fill: value.weak_bg_fill,
            bg_stroke: value.bg_stroke.into(),
            corner_radius: value.corner_radius.into(),
            fg_stroke: value.fg_stroke.into(),
            expansion: value.expansion,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::WidgetVisuals> for WidgetVisuals {
    #[inline]
    fn from(value: egui::style::WidgetVisuals) -> Self {
        Self {
            bg_fill: value.bg_fill,
            weak_bg_fill: value.weak_bg_fill,
            bg_stroke: value.bg_stroke.into(),
            corner_radius: value.corner_radius.into(),
            fg_stroke: value.fg_stroke.into(),
            expansion: value.expansion,
        }
    }
}

#[cfg(feature = "manager")]
impl From<NumericColorSpace> for egui::style::NumericColorSpace {
    #[inline]
    fn from(value: NumericColorSpace) -> Self {
        match value {
            NumericColorSpace::GammaByte => Self::GammaByte,
            NumericColorSpace::Linear => Self::Linear,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::NumericColorSpace> for NumericColorSpace {
    #[inline]
    fn from(value: egui::style::NumericColorSpace) -> Self {
        match value {
            egui::style::NumericColorSpace::GammaByte => Self::GammaByte,
            egui::style::NumericColorSpace::Linear => Self::Linear,
        }
    }
}

#[cfg(feature = "manager")]
impl From<Selection> for egui::style::Selection {
    #[inline]
    fn from(value: Selection) -> Self {
        Self {
            bg_fill: value.bg_fill,
            stroke: value.stroke.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::Selection> for Selection {
    #[inline]
    fn from(value: egui::style::Selection) -> Self {
        Selection {
            bg_fill: value.bg_fill,
            stroke: value.stroke.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<HandleShape> for egui::style::HandleShape {
    #[inline]
    fn from(value: HandleShape) -> Self {
        match value {
            HandleShape::Circle => Self::Circle,
            HandleShape::Rect { aspect_ratio } => Self::Rect { aspect_ratio },
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::HandleShape> for HandleShape {
    #[inline]
    fn from(value: egui::style::HandleShape) -> Self {
        match value {
            egui::style::HandleShape::Circle => Self::Circle,
            egui::style::HandleShape::Rect { aspect_ratio } => Self::Rect { aspect_ratio },
        }
    }
}

#[cfg(feature = "manager")]
impl From<TextCursorStyle> for egui::style::TextCursorStyle {
    #[inline]
    fn from(value: TextCursorStyle) -> Self {
        Self {
            stroke: value.stroke.into(),
            preview: value.preview,
            blink: value.blink,
            on_duration: value.on_duration,
            off_duration: value.off_duration,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::TextCursorStyle> for TextCursorStyle {
    #[inline]
    fn from(value: egui::style::TextCursorStyle) -> Self {
        Self {
            stroke: value.stroke.into(),
            preview: value.preview,
            blink: value.blink,
            on_duration: value.on_duration,
            off_duration: value.off_duration,
        }
    }
}

#[cfg(feature = "manager")]
impl From<ScrollFadeStyle> for egui::style::ScrollFadeStyle {
    #[inline]
    fn from(value: ScrollFadeStyle) -> Self {
        Self {
            strength: value.strength,
            size: value.size,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::ScrollFadeStyle> for ScrollFadeStyle {
    #[inline]
    fn from(value: egui::style::ScrollFadeStyle) -> Self {
        Self {
            strength: value.strength,
            size: value.size,
        }
    }
}

#[cfg(feature = "manager")]
impl From<ScrollAnimation> for egui::style::ScrollAnimation {
    #[inline]
    fn from(value: ScrollAnimation) -> Self {
        Self {
            points_per_second: value.points_per_second,
            duration: value.duration,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::ScrollAnimation> for ScrollAnimation {
    #[inline]
    fn from(value: egui::style::ScrollAnimation) -> Self {
        Self {
            points_per_second: value.points_per_second,
            duration: value.duration,
        }
    }
}

#[cfg(feature = "manager")]
impl From<TextStyle> for egui::TextStyle {
    #[inline]
    fn from(value: TextStyle) -> Self {
        match value {
            TextStyle::Small => Self::Small,
            TextStyle::Body => Self::Body,
            TextStyle::Monospace => Self::Monospace,
            TextStyle::Button => Self::Button,
            TextStyle::Heading => Self::Heading,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::TextStyle> for TextStyle {
    #[inline]
    fn from(value: egui::TextStyle) -> Self {
        match value {
            egui::TextStyle::Small => Self::Small,
            egui::TextStyle::Body => Self::Body,
            egui::TextStyle::Monospace => Self::Monospace,
            egui::TextStyle::Button => Self::Button,
            egui::TextStyle::Heading => Self::Heading,
            egui::TextStyle::Name(_) => panic!("Unsupported TextStyle: Name"),
        }
    }
}
