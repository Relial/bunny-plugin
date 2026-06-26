use abi_stable::std_types::{
    RBox,
    ROption::{self, RSome},
};
use egui::Color32;

use crate::{elements::Widget, paint::corner_radius::CornerRadius, widget_text::WidgetText};

#[repr(C)]
pub enum ProgressBarText {
    Custom(WidgetText),
    Percentage,
}

#[repr(C)]
pub struct ProgressBar {
    progress: f32,
    desired_width: ROption<f32>,
    desired_height: ROption<f32>,
    text: ROption<ProgressBarText>,
    fill: ROption<Color32>,
    animate: bool,
    corner_radius: ROption<CornerRadius>,
}

impl ProgressBar {}

impl egui::Widget for ProgressBar {
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

impl From<ProgressBar> for Widget<'_> {
    fn from(value: ProgressBar) -> Self {
        Self::ProgressBar(RBox::new(value))
    }
}
