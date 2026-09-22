use abi_stable::std_types::ROption::{self, RNone, RSome};
use ecolor::Color32;

use crate::{WidgetText, paint::corner_radius::CornerRadius, widgets::Widget};

#[repr(C)]
pub enum ProgressBarText<'a> {
    Custom(WidgetText<'a>),
    Percentage,
}

#[repr(C)]
pub struct ProgressBar<'a> {
    text: ROption<ProgressBarText<'a>>,
    desired_width: ROption<f32>,
    desired_height: ROption<f32>,
    fill: ROption<Color32>,
    corner_radius: ROption<CornerRadius>,
    progress: f32,
    animate: bool,
}

impl<'a> ProgressBar<'a> {
    #[inline]
    pub fn new(progress: f32) -> Self {
        Self {
            text: RNone,
            desired_width: RNone,
            desired_height: RNone,
            fill: RNone,
            corner_radius: RNone,
            progress,
            animate: false,
        }
    }

    #[inline]
    pub fn desired_width(mut self, desired_width: f32) -> Self {
        self.desired_width = RSome(desired_width);
        self
    }

    #[inline]
    pub fn desired_height(mut self, desired_height: f32) -> Self {
        self.desired_height = RSome(desired_height);
        self
    }

    #[inline]
    pub fn fill(mut self, color: Color32) -> Self {
        self.fill = RSome(color);
        self
    }

    #[inline]
    pub fn text(mut self, text: impl Into<WidgetText<'a>>) -> Self {
        self.text = RSome(ProgressBarText::Custom(text.into()));
        self
    }

    #[inline]
    pub fn show_percentage(mut self) -> Self {
        self.text = RSome(ProgressBarText::Percentage);
        self
    }

    #[inline]
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = RSome(corner_radius.into());
        self
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for ProgressBar<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut bar = egui::ProgressBar::new(self.progress).animate(self.animate);
        if let RSome(width) = self.desired_width {
            bar = bar.desired_width(width);
        }
        if let RSome(height) = self.desired_height {
            bar = bar.desired_height(height);
        }
        if let RSome(text) = self.text {
            bar = match text {
                ProgressBarText::Custom(widget_text) => bar.text(widget_text),
                ProgressBarText::Percentage => bar.show_percentage(),
            };
        }
        if let RSome(fill) = self.fill {
            bar = bar.fill(fill);
        }
        if let RSome(corner_radius) = self.corner_radius {
            bar = bar.corner_radius(corner_radius)
        }

        bar.ui(ui)
    }
}

impl<'a> From<ProgressBar<'a>> for Widget<'a> {
    #[inline]
    fn from(value: ProgressBar<'a>) -> Self {
        Self::ProgressBar(value)
    }
}
