use abi_stable::std_types::RArc;
use abi_stable::std_types::ROption::RSome;
use egui::response::Flags;
use egui::{Id, Pos2, Rect, Vec2};

use crate::containers::popup::{Popup, PopupKind};
use crate::containers::tooltip::Tooltip;
use crate::input::PointerButton;
use crate::input_state::PointerState;
use crate::ui::BunnyUi;
use crate::widget_text::WidgetText;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Response {
    pub id: Id,
    pub egui_id: Id,
    pub rect: Rect,
    pub interact_rect: Rect,
    pub flags: Flags,
    pub pointer_state: RArc<PointerState>,
}

impl Response {
    pub fn new(id: Id, egui_resp: egui::Response, pointer_state: RArc<PointerState>) -> Self {
        Self {
            id,
            egui_id: egui_resp.id,
            rect: egui_resp.rect,
            interact_rect: egui_resp.interact_rect,
            flags: egui_resp.flags,
            pointer_state,
        }
    }

    pub fn rect_only(id: Id, rect: Rect, pointer_state: RArc<PointerState>) -> Self {
        Self {
            id,
            egui_id: id,
            rect,
            interact_rect: rect,
            flags: Flags::empty(),
            pointer_state,
        }
    }

    pub fn empty(id: Id, pointer_state: RArc<PointerState>) -> Self {
        Self {
            id,
            pointer_state,
            ..Default::default()
        }
    }

    #[inline(always)]
    pub fn clicked(&self) -> bool {
        self.flags.contains(Flags::FAKE_PRIMARY_CLICKED) || self.clicked_by(PointerButton::Primary)
    }

    #[inline]
    pub fn clicked_by(&self, button: PointerButton) -> bool {
        self.flags.contains(Flags::CLICKED) && self.pointer_state.button_clicked(button)
    }

    #[inline]
    pub fn secondary_clicked(&self) -> bool {
        self.clicked_by(PointerButton::Secondary)
    }

    #[inline]
    pub fn middle_clicked(&self) -> bool {
        self.clicked_by(PointerButton::Middle)
    }

    #[inline]
    pub fn double_clicked(&self) -> bool {
        self.double_clicked_by(PointerButton::Primary)
    }

    #[inline]
    pub fn triple_clicked(&self) -> bool {
        self.triple_clicked_by(PointerButton::Primary)
    }

    #[inline]
    pub fn double_clicked_by(&self, button: PointerButton) -> bool {
        self.flags.contains(Flags::CLICKED) && self.pointer_state.button_double_clicked(button)
    }

    #[inline]
    pub fn triple_clicked_by(&self, button: PointerButton) -> bool {
        self.flags.contains(Flags::CLICKED) && self.pointer_state.button_triple_clicked(button)
    }

    pub fn clicked_elsewhere(&self) -> bool {
        let (pointer_interact_pos, any_click) = (
            self.pointer_state.interact_pos(),
            self.pointer_state.any_click(),
        );

        if any_click {
            if self.contains_pointer() || self.hovered() {
                false
            } else if let RSome(pos) = pointer_interact_pos {
                !self.interact_rect.contains(pos)
            } else {
                false
            }
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn enabled(&self) -> bool {
        self.flags.contains(Flags::ENABLED)
    }

    #[inline(always)]
    pub fn hovered(&self) -> bool {
        self.flags.contains(Flags::HOVERED)
    }

    #[inline(always)]
    pub fn contains_pointer(&self) -> bool {
        self.flags.contains(Flags::CONTAINS_POINTER)
    }

    #[inline(always)]
    pub fn highlighted(&self) -> bool {
        self.flags.contains(Flags::HIGHLIGHTED)
    }

    #[inline]
    pub fn drag_started(&self) -> bool {
        self.flags.contains(Flags::DRAG_STARTED)
    }

    #[inline]
    pub fn drag_started_by(&self, button: PointerButton) -> bool {
        self.drag_started() && self.pointer_state.button_down(button)
    }

    #[inline(always)]
    pub fn dragged(&self) -> bool {
        self.flags.contains(Flags::DRAGGED)
    }

    #[inline]
    pub fn dragged_by(&self, button: PointerButton) -> bool {
        self.dragged() && self.pointer_state.button_down(button)
    }

    #[inline]
    pub fn drag_stopped(&self) -> bool {
        self.flags.contains(Flags::DRAG_STOPPED)
    }

    pub fn drag_stopped_by(&self, button: PointerButton) -> bool {
        self.drag_stopped() && self.pointer_state.button_released(button)
    }

    #[inline]
    pub fn drag_delta(&self) -> Vec2 {
        if self.dragged() {
            self.pointer_state.delta()
        } else {
            Vec2::ZERO
        }
    }

    #[inline]
    pub fn total_drag_delta(&self) -> Option<Vec2> {
        if self.dragged() {
            self.pointer_state.total_drag_delta()
        } else {
            None
        }
    }

    #[inline]
    pub fn hover_pos(&self) -> Option<Pos2> {
        if self.hovered() {
            self.pointer_state.latest_pos().into()
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn is_pointer_button_down_on(&self) -> bool {
        self.flags.contains(Flags::IS_POINTER_BUTTON_DOWN_ON)
    }

    #[inline(always)]
    pub fn changed(&self) -> bool {
        self.flags.contains(Flags::CHANGED)
    }

    #[inline(always)]
    pub fn mark_changed(&mut self) {
        self.flags.set(Flags::CHANGED, true);
    }

    pub fn show_tooltip_text(&self, ui: &mut BunnyUi, text: impl Into<WidgetText>) {
        self.show_tooltip_ui(ui, |ui| {
            ui.label(text);
        });
    }

    pub fn on_hover_text(self, ui: &mut BunnyUi, text: impl Into<WidgetText>) -> Self {
        self.on_hover_ui(ui, |ui| {
            ui.label(text);
        })
    }

    pub fn on_hover_text_at_pointer(self, ui: &mut BunnyUi, text: impl Into<WidgetText>) -> Self {
        self.on_hover_ui_at_pointer(ui, |ui| {
            ui.label(text);
        })
    }
}

impl<'a> Response {
    pub fn on_hover_ui(
        self,
        ui: &mut BunnyUi<'a>,
        add_contents: impl FnOnce(&mut BunnyUi<'a>),
    ) -> Self {
        Tooltip::for_enabled(&self).show(ui, add_contents);
        self
    }

    pub fn on_disabled_hover_ui(
        self,
        ui: &mut BunnyUi<'a>,
        add_contents: impl FnOnce(&mut BunnyUi<'a>),
    ) -> Self {
        Tooltip::for_disabled(&self).show(ui, add_contents);
        self
    }

    pub fn on_hover_ui_at_pointer(
        self,
        ui: &mut BunnyUi<'a>,
        add_contents: impl FnOnce(&mut BunnyUi<'a>),
    ) -> Self {
        Tooltip::for_enabled(&self)
            .at_pointer()
            .gap(12.0)
            .show(ui, add_contents);
        self
    }

    pub fn show_tooltip_ui(
        &self,
        ui: &mut BunnyUi<'a>,
        add_contents: impl FnOnce(&mut BunnyUi<'a>),
    ) {
        Popup::from_response(self)
            .kind(PopupKind::Tooltip)
            .show(ui, add_contents);
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            id: Id::NULL,
            egui_id: Id::NULL,
            rect: Rect::ZERO,
            interact_rect: Rect::ZERO,
            flags: Flags::empty(),
            pointer_state: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct InnerResponse<R> {
    pub inner: R,
    pub response: Response,
}

impl<R> InnerResponse<R> {
    #[inline]
    pub fn new(inner: R, response: Response) -> Self {
        Self { inner, response }
    }
}
