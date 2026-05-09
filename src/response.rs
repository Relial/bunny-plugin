use abi_stable::std_types::RArc;
use egui::Rect;
use egui::response::Flags;

use crate::elements::Id;
use crate::input::{PointerButton, PointerState};

#[repr(C)]
#[derive(Clone)]
pub struct Response {
    pub id: Id,
    pub rect: Rect,
    pub interact_rect: Rect,
    pub flags: Flags,
    pub input: RArc<PointerState>,
}

impl Response {
    pub fn new(id: Id, egui_resp: egui::Response, pointer_state: RArc<PointerState>) -> Self {
        Self {
            id,
            rect: egui_resp.rect,
            interact_rect: egui_resp.interact_rect,
            flags: egui_resp.flags,
            input: pointer_state,
        }
    }

    pub fn rect_only(id: Id, rect: Rect) -> Self {
        Self {
            id,
            rect,
            interact_rect: rect,
            ..Default::default()
        }
    }

    #[inline(always)]
    pub fn clicked(&self) -> bool {
        self.flags.contains(Flags::FAKE_PRIMARY_CLICKED) || self.clicked_by(PointerButton::Primary)
    }

    #[inline]
    pub fn clicked_by(&self, button: PointerButton) -> bool {
        self.flags.contains(Flags::CLICKED) && self.input.button_clicked(button)
    }

    #[inline]
    pub fn secondary_clicked(&self) -> bool {
        self.clicked_by(PointerButton::Secondary)
    }

    #[inline]
    pub fn middle_clicked(&self) -> bool {
        self.clicked_by(PointerButton::Middle)
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
        self.drag_started() && self.input.button_down(button)
    }

    #[inline(always)]
    pub fn dragged(&self) -> bool {
        self.flags.contains(Flags::DRAGGED)
    }

    #[inline]
    pub fn dragged_by(&self, button: PointerButton) -> bool {
        self.dragged() && self.input.button_down(button)
    }

    #[inline]
    pub fn drag_stopped(&self) -> bool {
        self.flags.contains(Flags::DRAG_STOPPED)
    }

    pub fn drag_stopped_by(&self, button: PointerButton) -> bool {
        self.drag_stopped() && self.input.button_released(button)
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
}

impl Default for Response {
    fn default() -> Self {
        Self {
            id: Id::NULL,
            rect: Rect::ZERO,
            interact_rect: Rect::ZERO,
            flags: Flags::empty(),
            input: RArc::new(PointerState::default()),
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
