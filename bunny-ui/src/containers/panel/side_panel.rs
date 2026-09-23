use abi_stable::std_types::ROption::{self, RNone, RSome};
use emath::{NumExt, Rangef};

#[cfg(feature = "manager")]
use crate::{
    BunnyResponse,
    closure::{PanelAnimatedBetweenClosure, PluginClosure},
};
use crate::{Id, containers::Frame, response::BunnyInnerResponse, ui::BunnyUi};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
enum PanelSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[repr(C)]
pub struct Panel {
    frame: ROption<Frame>,
    id: Id,
    default_size: ROption<f32>,
    size_range: Rangef,
    side: PanelSide,
    resizable: bool,
    show_separator_line: bool,
}

impl Panel {
    #[inline]
    pub fn left(id: impl Into<Id>) -> Self {
        Self::new(PanelSide::Left, id)
    }

    #[inline]
    pub fn right(id: impl Into<Id>) -> Self {
        Self::new(PanelSide::Right, id)
    }

    #[inline]
    pub fn top(id: impl Into<Id>) -> Self {
        Self::new(PanelSide::Top, id)
    }

    #[inline]
    pub fn bottom(id: impl Into<Id>) -> Self {
        Self::new(PanelSide::Bottom, id)
    }

    #[inline]
    fn new(side: PanelSide, id: impl Into<Id>) -> Self {
        let default_size: ROption<f32> = match side {
            PanelSide::Left | PanelSide::Right => RSome(200.0),
            PanelSide::Top | PanelSide::Bottom => RNone,
        };
        let size_range: Rangef = match side {
            PanelSide::Left | PanelSide::Right => Rangef::new(96.0, f32::INFINITY),
            PanelSide::Top | PanelSide::Bottom => Rangef::new(20.0, f32::INFINITY),
        };
        Self {
            frame: RNone,
            side,
            id: id.into(),
            default_size,
            size_range,
            resizable: true,
            show_separator_line: true,
        }
    }

    #[inline]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    #[inline]
    pub fn show_separator_line(mut self, show_separator_line: bool) -> Self {
        self.show_separator_line = show_separator_line;
        self
    }

    #[inline]
    pub fn default_size(mut self, default_size: f32) -> Self {
        self.default_size = RSome(default_size);
        self.size_range = Rangef::new(
            self.size_range.min.at_most(default_size),
            self.size_range.max.at_least(default_size),
        );
        self
    }

    #[inline]
    pub fn min_size(mut self, min_size: f32) -> Self {
        self.size_range = Rangef::new(min_size, self.size_range.max.at_least(min_size));
        self
    }

    #[inline]
    pub fn max_size(mut self, max_size: f32) -> Self {
        self.size_range = Rangef::new(self.size_range.min.at_most(max_size), max_size);
        self
    }

    #[inline]
    pub fn size_range(mut self, size_range: impl Into<Rangef>) -> Self {
        let size_range = size_range.into();
        self.default_size = self
            .default_size
            .map(|default_size| clamp_to_range(default_size, size_range));
        self.size_range = size_range;
        self
    }

    #[inline]
    pub fn exact_size(mut self, size: f32) -> Self {
        self.default_size = RSome(size);
        self.size_range = Rangef::point(size);
        self.resizable = false;
        self
    }

    #[inline]
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = RSome(frame);
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        ui.panel_show(self, add_contents)
    }

    #[inline]
    pub fn show_animated<R>(
        self,
        ui: &mut BunnyUi,
        is_expanded: bool,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> Option<BunnyInnerResponse<R>> {
        ui.panel_show_animated(self, is_expanded, add_contents)
    }

    #[inline]
    pub fn show_animated_between<R>(
        ui: &mut BunnyUi,
        is_expanded: bool,
        collapsed_panel: Self,
        expanded_panel: Self,
        add_contents: impl FnMut(&mut BunnyUi, f32) -> R,
    ) -> BunnyInnerResponse<R> {
        ui.panel_show_animated_between(is_expanded, collapsed_panel, expanded_panel, add_contents)
    }
}

#[cfg(feature = "manager")]
impl From<Panel> for egui::Panel {
    fn from(value: Panel) -> Self {
        let Panel {
            frame,
            side,
            id,
            default_size,
            size_range,
            resizable,
            show_separator_line,
        } = value;
        let mut panel = match side {
            PanelSide::Left => egui::Panel::left(id),
            PanelSide::Right => egui::Panel::right(id),
            PanelSide::Top => egui::Panel::top(id),
            PanelSide::Bottom => egui::Panel::bottom(id),
        }
        .size_range(size_range)
        .resizable(resizable)
        .show_separator_line(show_separator_line);
        if let RSome(frame) = frame {
            panel = panel.frame(frame.into());
        }
        if let RSome(default_size) = default_size {
            panel = panel.default_size(default_size);
        }
        panel
    }
}

#[cfg(feature = "manager")]
impl Panel {
    pub(crate) fn show_impl(self, ui: &mut egui::Ui, contents: PluginClosure) -> BunnyResponse {
        let panel: egui::Panel = self.into();
        let response = panel
            .show_inside(ui, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(response)
    }

    pub(crate) fn show_animated_impl(
        self,
        ui: &mut egui::Ui,
        is_expanded: bool,
        contents: PluginClosure,
    ) -> ROption<BunnyResponse> {
        let panel: egui::Panel = self.into();
        let response = panel.show_animated_inside(ui, is_expanded, |ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
        response
            .map(|inner| BunnyResponse::new(inner.response))
            .into()
    }

    pub(crate) fn show_animated_between_impl(
        ui: &mut egui::Ui,
        is_expanded: bool,
        collapsed_panel: Self,
        expanded_panel: Self,
        contents: PanelAnimatedBetweenClosure,
    ) -> BunnyResponse {
        let collapsed: egui::Panel = collapsed_panel.into();
        let expanded: egui::Panel = expanded_panel.into();
        let response = egui::Panel::show_animated_between_inside(
            ui,
            is_expanded,
            collapsed,
            expanded,
            |ui, how_expanded| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b, how_expanded);
            },
        )
        .response;
        BunnyResponse::new(response)
    }
}

fn clamp_to_range(x: f32, range: Rangef) -> f32 {
    let range = range.as_positive();
    x.clamp(range.min, range.max)
}
