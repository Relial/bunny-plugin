use abi_stable::std_types::RBox;
use ecolor::Color32;

use crate::{
    elements::Container,
    layout::Layout,
    margin::Margin,
    paint::{corner_radius::CornerRadius, stroke::Stroke},
    response::InnerResponse,
    shadow::Shadow,
    style::Style,
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

    pub fn group(style: &Style) -> Self {
        let visuals = style.visuals();
        Self::new()
            .inner_margin(6)
            .corner_radius(visuals.widgets.noninteractive.corner_radius)
            .stroke(visuals.widgets.noninteractive.bg_stroke)
    }

    pub fn side_top_panel(style: &Style) -> Self {
        Self::new()
            .inner_margin(Margin::symmetric(8, 2))
            .fill(style.visuals().panel_fill)
    }

    pub fn central_panel(style: &Style) -> Self {
        Self::new().inner_margin(8).fill(style.visuals().panel_fill)
    }

    pub fn window(style: &Style) -> Self {
        let visuals = style.visuals();
        Self::new()
            .inner_margin(style.spacing().window_margin)
            .corner_radius(visuals.window_corner_radius)
            .shadow(visuals.window_shadow)
            .fill(visuals.window_fill)
            .stroke(visuals.window_stroke)
    }

    pub fn menu(style: &Style) -> Self {
        let visuals = style.visuals();
        Self::new()
            .inner_margin(style.spacing().menu_margin)
            .corner_radius(visuals.menu_corner_radius)
            .shadow(visuals.popup_shadow)
            .fill(visuals.window_fill)
            .stroke(visuals.window_stroke)
    }

    pub fn popup(style: &Style) -> Self {
        let visuals = style.visuals();
        Self::new()
            .inner_margin(style.spacing().menu_margin)
            .corner_radius(visuals.menu_corner_radius)
            .shadow(visuals.popup_shadow)
            .fill(visuals.window_fill)
            .stroke(visuals.window_stroke)
    }

    pub fn canvas(style: &Style) -> Self {
        let visuals = style.visuals();
        Self::new()
            .inner_margin(2)
            .corner_radius(visuals.widgets.noninteractive.corner_radius)
            .fill(visuals.extreme_bg_color)
            .stroke(visuals.window_stroke)
    }

    pub fn dark_canvas(style: &Style) -> Self {
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

    pub fn show<'a, R>(
        self,
        ui: &mut BunnyUi<'a>,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        let mut new = ui.new_child(Some(Layout::default()));
        let ret = add_contents(&mut new);
        let response = ui.add_component_auto_id(Container::Frame(RBox::new(FrameComponent {
            contents: new,
            frame: self,
        })));
        InnerResponse::new(ret, response)
    }
}

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
pub struct FrameComponent<'a> {
    contents: BunnyUi<'a>,
    frame: Frame,
}

impl<'a> From<FrameComponent<'a>> for Container<'a> {
    #[inline]
    fn from(value: FrameComponent<'a>) -> Self {
        Self::Frame(RBox::new(value))
    }
}

#[cfg(feature = "manager")]
impl crate::elements::UiContainer for FrameComponent<'_> {
    fn ui(
        self,
        ui: &mut egui::Ui,
        responses: &mut abi_stable::std_types::RHashMap<
            crate::Id,
            crate::response::Response,
            rapidhash::fast::RandomState,
        >,
        pointer_state: abi_stable::std_types::RArc<crate::input_state::PointerState>,
        id: crate::Id,
    ) -> crate::response::Response {
        let frame: egui::Frame = self.frame.into();
        let resp = frame
            .show(ui, |ui| {
                self.contents.ui(ui, responses, pointer_state.clone());
            })
            .response;
        crate::response::Response::new(id, resp, pointer_state)
    }
}
