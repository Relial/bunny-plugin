use abi_stable::std_types::RArc;
use emath::{Pos2, Rect, Vec2};

use crate::Id;
use crate::containers::popup::{Popup, PopupKind};
use crate::containers::tooltip::Tooltip;
use crate::input::PointerButton;
use crate::input_state::PointerState;
use crate::ui::BunnyUi;
use crate::widget_text::WidgetText;

#[cfg(feature = "manager")]
type EguiId = egui::Id;
#[cfg(not(feature = "manager"))]
type EguiId = u64;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Response {
    pub pointer_state: RArc<PointerState>,
    pub rect: Rect,
    pub interact_rect: Rect,
    pub id: Id,
    pub(crate) egui_id: EguiId,
    pub flags: Flags,
}

impl Response {
    #[cfg(feature = "manager")]
    pub fn new(id: Id, egui_resp: egui::Response, pointer_state: RArc<PointerState>) -> Self {
        Self {
            id,
            egui_id: egui_resp.id,
            rect: egui_resp.rect,
            interact_rect: egui_resp.interact_rect,
            flags: egui_resp.flags.into(),
            pointer_state,
        }
    }

    #[cfg(feature = "manager")]
    pub fn rect_only(id: Id, rect: Rect, pointer_state: RArc<PointerState>) -> Self {
        Self {
            id,
            egui_id: EguiId::NULL,
            rect,
            interact_rect: rect,
            flags: Flags::empty(),
            pointer_state,
        }
    }

    #[cfg(feature = "manager")]
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
            } else if let Some(pos) = pointer_interact_pos {
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
            self.pointer_state.latest_pos()
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
    #[cfg(feature = "manager")]
    fn default() -> Self {
        Self {
            id: Id::NULL,
            egui_id: EguiId::NULL,
            rect: Rect::ZERO,
            interact_rect: Rect::ZERO,
            flags: Flags::empty(),
            pointer_state: Default::default(),
        }
    }

    #[cfg(not(feature = "manager"))]
    fn default() -> Self {
        Self {
            id: Id::NULL,
            egui_id: u64::MAX,
            rect: Rect::ZERO,
            interact_rect: Rect::ZERO,
            flags: Flags::empty(),
            pointer_state: Default::default(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
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

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Flags(u16);

bitflags::bitflags! {
    impl Flags: u16 {
        /// Was the widget enabled?
        /// If `false`, there was no interaction attempted (not even hover).
        const ENABLED = 1<<0;

        /// The pointer is above this widget with no other blocking it.
        const CONTAINS_POINTER = 1<<1;

        /// The pointer is hovering above this widget or the widget was clicked/tapped this frame.
        const HOVERED = 1<<2;

        /// The widget is highlighted via a call to [`Response::highlight`] or
        /// [`Context::highlight_widget`].
        const HIGHLIGHTED = 1<<3;

        /// This widget was clicked this frame.
        ///
        /// Which pointer and how many times we don't know,
        /// and ask [`crate::InputState`] about at runtime.
        ///
        /// This is only set to true if the widget was clicked
        /// by an actual mouse.
        const CLICKED = 1<<4;

        /// This widget should act as if clicked due
        /// to something else than a click.
        ///
        /// This is set to true if the widget has keyboard focus and
        /// the user hit the Space or Enter key.
        const FAKE_PRIMARY_CLICKED = 1<<5;

        /// This widget was long-pressed on a touch screen to simulate a secondary click.
        const LONG_TOUCHED = 1<<6;

        /// The widget started being dragged this frame.
        const DRAG_STARTED = 1<<7;

        /// The widget is being dragged.
        const DRAGGED = 1<<8;

        /// The widget was being dragged, but now it has been released.
        const DRAG_STOPPED = 1<<9;

        /// Is the pointer button currently down on this widget?
        /// This is true if the pointer is pressing down or dragging a widget
        const IS_POINTER_BUTTON_DOWN_ON = 1<<10;

        /// Was the underlying data changed?
        ///
        /// e.g. the slider was dragged, text was entered in a [`TextEdit`](crate::TextEdit) etc.
        /// Always `false` for something like a [`Button`](crate::Button).
        ///
        /// Note that this can be `true` even if the user did not interact with the widget,
        /// for instance if an existing slider value was clamped to the given range.
        const CHANGED = 1<<11;

        /// Should this container be closed?
        const CLOSE = 1<<12;
    }
}

#[cfg(feature = "manager")]
impl From<Flags> for egui::response::Flags {
    #[inline]
    fn from(value: Flags) -> Self {
        Self::from_bits_retain(value.bits())
    }
}

#[cfg(feature = "manager")]
impl From<egui::response::Flags> for Flags {
    #[inline]
    fn from(value: egui::response::Flags) -> Self {
        Self::from_bits_retain(value.bits())
    }
}
