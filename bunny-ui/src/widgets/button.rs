use abi_stable::std_types::ROption::{self, RNone, RSome};
use ecolor::Color32;

use crate::{
    WidgetText,
    paint::{corner_radius::CornerRadius, stroke::Stroke},
    widgets::Widget,
};

#[repr(C)]
pub struct Button {
    text: WidgetText,
    stroke: ROption<Stroke>,
    fill: ROption<Color32>,
    corner_radius: ROption<CornerRadius>,
    frame: ROption<bool>,
    frame_when_inactive: bool,
    selected: bool,
    small: bool,
}

impl Button {
    #[inline]
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            fill: RNone,
            stroke: RNone,
            small: false,
            frame: RNone,
            frame_when_inactive: true,
            corner_radius: RNone,
            selected: false,
        }
    }

    #[inline]
    pub fn selectable(selected: bool, text: impl Into<WidgetText>) -> Self {
        Self::new(text)
            .selected(selected)
            .frame_when_inactive(selected)
            .frame(true)
    }

    #[inline]
    pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
        self.fill = RSome(fill.into());
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = RSome(stroke.into());
        self.frame = RSome(true);
        self
    }

    #[inline]
    pub fn small(mut self) -> Self {
        self.small = true;
        self
    }

    #[inline]
    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = RSome(frame);
        self
    }

    #[inline]
    pub fn frame_when_inactive(mut self, frame_when_inactive: bool) -> Self {
        self.frame_when_inactive = frame_when_inactive;
        self
    }

    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = RSome(corner_radius.into());
        self
    }

    #[inline]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for Button {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut button = egui::Button::new(self.text).selected(self.selected);
        if let RSome(fill) = self.fill {
            button = button.fill(fill);
        }
        if let RSome(stroke) = self.stroke {
            button = button.stroke(stroke);
        }
        if self.small {
            button = button.small();
        }
        if let RSome(frame) = self.frame {
            button = button.frame(frame);
        }
        if let RSome(corner_radius) = self.corner_radius {
            button = button.corner_radius(corner_radius);
        }
        ui.add(button)
    }
}

impl From<Button> for Widget {
    #[inline]
    fn from(value: Button) -> Self {
        Self::Button(value)
    }
}
