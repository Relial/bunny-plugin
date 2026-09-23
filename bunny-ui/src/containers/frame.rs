use ecolor::Color32;

#[cfg(feature = "manager")]
use crate::{BunnyResponse, closure::PluginClosure};
use crate::{
    Margin, Shadow,
    paint::{corner_radius::CornerRadius, stroke::Stroke},
    response::BunnyInnerResponse,
    style::{BunnyStyleRef, BunnyVisuals},
    ui::BunnyUi,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Frame {
    pub stroke: Stroke,
    pub shadow: Shadow,
    pub inner_margin: Margin,
    pub fill: Color32,
    pub corner_radius: CornerRadius,
    pub outer_margin: Margin,
}

impl Frame {
    pub const NONE: Self = Self {
        inner_margin: Margin::ZERO,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::NONE,
        corner_radius: CornerRadius::ZERO,
        outer_margin: Margin::ZERO,
        shadow: Shadow::NONE,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::NONE
    }

    pub fn group(style: &BunnyStyleRef) -> Self {
        let frame_style = style.frame_style();
        Self::new()
            .inner_margin(6)
            .corner_radius(frame_style.noninteractive_corner_radius)
            .stroke(frame_style.noninteractive_bg_stroke)
    }

    pub fn side_top_panel(style: &BunnyStyleRef) -> Self {
        Self::new()
            .inner_margin(Margin::symmetric(8, 2))
            .fill(style.visuals().panel_fill())
    }

    pub fn central_panel(style: &BunnyStyleRef) -> Self {
        Self::new()
            .inner_margin(8)
            .fill(style.visuals().panel_fill())
    }

    pub fn window(style: &BunnyStyleRef) -> Self {
        let frame_style = style.frame_style();
        Self::new()
            .inner_margin(frame_style.window_margin)
            .corner_radius(frame_style.window_corner_radius)
            .shadow(frame_style.window_shadow)
            .fill(frame_style.window_fill)
            .stroke(frame_style.window_stroke)
    }

    pub fn menu(style: &BunnyStyleRef) -> Self {
        let frame_style = style.frame_style();
        Self::new()
            .inner_margin(frame_style.menu_margin)
            .corner_radius(frame_style.menu_corner_radius)
            .shadow(frame_style.popup_shadow)
            .fill(frame_style.window_fill)
            .stroke(frame_style.window_stroke)
    }

    pub fn popup(style: &BunnyStyleRef) -> Self {
        let frame_style = style.frame_style();
        Self::new()
            .inner_margin(frame_style.menu_margin)
            .corner_radius(frame_style.menu_corner_radius)
            .shadow(frame_style.popup_shadow)
            .fill(frame_style.window_fill)
            .stroke(frame_style.window_stroke)
    }

    pub fn canvas(style: &BunnyStyleRef) -> Self {
        let frame_style = style.frame_style();
        Self::new()
            .inner_margin(2)
            .corner_radius(frame_style.noninteractive_corner_radius)
            .fill(frame_style.extreme_bg_color)
            .stroke(frame_style.window_stroke)
    }

    pub fn dark_canvas(style: &BunnyStyleRef) -> Self {
        Self::canvas(style).fill(Color32::from_black_alpha(250))
    }
}

impl Frame {
    #[inline]
    pub fn inner_margin(mut self, inner_margin: impl Into<Margin>) -> Self {
        self.inner_margin = inner_margin.into();
        self
    }

    #[inline]
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = fill;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = stroke.into();
        self
    }

    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = corner_radius.into();
        self
    }

    #[inline]
    pub fn outer_margin(mut self, outer_margin: impl Into<Margin>) -> Self {
        self.outer_margin = outer_margin.into();
        self
    }

    #[inline]
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = shadow;
        self
    }

    #[inline]
    pub fn multiply_with_opacity(mut self, opacity: f32) -> Self {
        self.fill = self.fill.gamma_multiply(opacity);
        self.stroke.color = self.stroke.color.gamma_multiply(opacity);
        self.shadow.color = self.shadow.color.gamma_multiply(opacity);
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        ui.frame_show(self, add_contents)
    }
}

// impl Frame {
//     #[inline]
//     pub fn total_margin(&self) -> MarginF32 {
//         MarginF32::from(self.inner_margin)
//             + MarginF32::from(self.stroke.width)
//             + MarginF32::from(self.outer_margin)
//     }

//     #[inline]
//     pub fn fill_rect(&self, content_rect: Rect) -> Rect {
//         content_rect + self.inner_margin
//     }

//     #[inline]
//     pub fn widget_rect(&self, content_rect: Rect) -> Rect {
//         content_rect + self.inner_margin + MarginF32::from(self.stroke.width)
//     }

//     #[inline]
//     pub fn outer_rect(&self, content_rect: Rect) -> Rect {
//         content_rect + self.inner_margin + MarginF32::from(self.stroke.width) + self.outer_margin
//     }
// }

#[cfg(feature = "manager")]
impl From<Frame> for egui::Frame {
    #[inline]
    fn from(value: Frame) -> Self {
        Self {
            inner_margin: value.inner_margin.into(),
            fill: value.fill,
            stroke: value.stroke.into(),
            corner_radius: value.corner_radius.into(),
            outer_margin: value.outer_margin.into(),
            shadow: value.shadow.into(),
        }
    }
}

#[repr(C)]
pub(crate) struct FrameStyle {
    pub noninteractive_bg_stroke: Stroke,
    pub window_shadow: Shadow,
    pub window_stroke: Stroke,
    pub popup_shadow: Shadow,
    pub noninteractive_corner_radius: CornerRadius,
    pub window_margin: Margin,
    pub window_corner_radius: CornerRadius,
    pub window_fill: Color32,
    pub menu_margin: Margin,
    pub menu_corner_radius: CornerRadius,
    pub extreme_bg_color: Color32,
}

#[cfg(feature = "manager")]
impl FrameStyle {
    pub fn from_style(style: &egui::Style) -> Self {
        let spacing = &style.spacing;
        let visuals = &style.visuals;
        Self {
            noninteractive_corner_radius: visuals.widgets.noninteractive.corner_radius.into(),
            noninteractive_bg_stroke: visuals.widgets.noninteractive.bg_stroke.into(),
            window_margin: spacing.window_margin.into(),
            window_corner_radius: visuals.window_corner_radius.into(),
            window_shadow: visuals.window_shadow.into(),
            window_fill: visuals.window_fill,
            window_stroke: visuals.window_stroke.into(),
            menu_margin: spacing.menu_margin.into(),
            menu_corner_radius: visuals.menu_corner_radius.into(),
            popup_shadow: visuals.popup_shadow.into(),
            extreme_bg_color: visuals.extreme_bg_color,
        }
    }
}

#[cfg(feature = "manager")]
impl Frame {
    #[inline]
    pub(crate) fn show_impl(self, ui: &mut egui::Ui, contents: PluginClosure) -> BunnyResponse {
        let frame: egui::Frame = self.into();
        let response = frame
            .show(ui, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(response)
    }
}
