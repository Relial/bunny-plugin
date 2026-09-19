use abi_stable::std_types::{
    RBox,
    ROption::{self, RNone, RSome},
};

use crate::{
    Id,
    containers::popup::{Popup, PopupAnchor, PopupKind},
    elements::Container,
    layout::Layout,
    response::{InnerResponse, Response},
    sense::Sense,
    ui_old::BunnyUi,
};

#[repr(C)]
pub struct Tooltip<'a> {
    pub popup: Popup<'a>,
    parent: Id,
    for_enabled: ROption<bool>,
    set_width: bool,
}

impl<'a> Tooltip<'a> {
    pub fn always_open(parent: impl Into<Id>, anchor: impl Into<PopupAnchor>) -> Self {
        Self {
            popup: Popup::new(parent, anchor)
                .kind(PopupKind::Tooltip)
                .gap(4.0)
                .sense(Sense::hover()),
            parent,
            for_enabled: RNone,
            set_width: false,
        }
    }

    pub fn for_widget(response: &Response) -> Self {
        let popup = Popup::from_response(response)
            .kind(PopupKind::Tooltip)
            .gap(4.0)
            .sense(Sense::hover());
        Self {
            popup,
            parent: response.id,
            for_enabled: RNone,
            set_width: false,
        }
    }

    pub fn for_enabled(response: &Response) -> Self {
        let mut tooltip = Self::for_widget(response);
        tooltip.for_enabled = RSome(true);
        tooltip
    }

    pub fn for_disabled(response: &Response) -> Self {
        let mut tooltip = Self::for_widget(response);
        tooltip.for_enabled = RSome(false);
        tooltip
    }

    #[inline]
    pub fn at_pointer(mut self) -> Self {
        self.popup = self.popup.at_pointer();
        self
    }

    #[inline]
    pub fn gap(mut self, gap: f32) -> Self {
        self.popup = self.popup.gap(gap);
        self
    }

    #[inline]
    pub fn layout(mut self, layout: Layout) -> Self {
        self.popup = self.popup.layout(layout);
        self
    }

    #[inline]
    pub fn width(mut self, width: f32) -> Self {
        self.set_width = true;
        self.popup = self.popup.width(width);
        self
    }

    pub fn show<R>(
        self,
        ui: &mut BunnyUi<'a>,
        content: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        let mut new = ui.new_child(None);
        let ret = content(&mut new);
        let response = ui.response(self.popup.get_id());
        let inner = InnerResponse::new(ret, response.cloned().unwrap_or_default());
        ui.add_component(
            self.popup.get_id(),
            Container::Tooltip(RBox::new(TooltipComponent {
                contents: new,
                tooltip: self,
            })),
        );
        inner
    }
}

#[repr(C)]
pub struct TooltipComponent<'a> {
    tooltip: Tooltip<'a>,
    contents: BunnyUi<'a>,
}

#[cfg(feature = "manager")]
impl crate::elements::UiContainer for TooltipComponent<'_> {
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
        if !self.tooltip.popup.is_open(ui) {
            return Response::empty(id, pointer_state);
        }

        let mut popup = self.tooltip.popup.egui(ui, id);
        if !self.tooltip.set_width {
            popup = popup.width(ui.global_style().spacing.tooltip_width);
        }
        let response = responses
            .get(&self.tooltip.parent)
            .and_then(|r| ui.read_response(r.egui_id));

        popup = match self.tooltip.for_enabled {
            RSome(true) => popup.open(
                response
                    .map(|r| r.enabled() && egui::Tooltip::should_show_tooltip(&r, true))
                    .unwrap_or(false),
            ),
            RSome(false) => popup.open(
                response
                    .map(|r| !r.enabled() && egui::Tooltip::should_show_tooltip(&r, true))
                    .unwrap_or(false),
            ),
            RNone => popup,
        };

        let inner = popup.show(|ui| {
            self.contents.ui(ui, responses, pointer_state.clone());
        });
        if let Some(inner) = inner {
            Response::new(id, inner.response, pointer_state)
        } else {
            Response::empty(id, pointer_state)
        }
    }
}
