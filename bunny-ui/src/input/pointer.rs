use egui::{Pos2, Rect, Vec2};
use vtable::VRef;

use crate::{PointerButton, vtable::input::pointer::PointerStateFfiVTable};

#[repr(C)]
pub struct BunnyPointerState<'a> {
    inner: VRef<'a, PointerStateFfiVTable>,
}

impl<'a> BunnyPointerState<'a> {
    #[inline]
    pub fn new(pointer_state: &'a egui::PointerState) -> Self {
        Self {
            inner: VRef::new(pointer_state),
        }
    }
}

impl<'a> BunnyPointerState<'a> {
    #[inline]
    pub fn delta(&self) -> Vec2 {
        self.inner.delta()
    }

    #[inline]
    pub fn motion(&self) -> Option<Vec2> {
        self.inner.motion().into_option()
    }

    #[inline]
    pub fn velocity(&self) -> Vec2 {
        self.inner.velocity()
    }

    #[inline]
    pub fn direction(&self) -> Vec2 {
        self.inner.direction()
    }

    #[inline]
    pub fn press_origin(&self) -> Option<Pos2> {
        self.inner.press_origin().into_option()
    }

    #[inline]
    pub fn total_drag_delta(&self) -> Option<Vec2> {
        self.inner.total_drag_delta().into_option()
    }

    #[inline]
    pub fn press_start_time(&self) -> Option<f64> {
        self.inner.press_start_time().into_option()
    }

    #[inline]
    pub fn latest_pos(&self) -> Option<Pos2> {
        self.inner.latest_pos().into_option()
    }

    #[inline]
    pub fn hover_pos(&self) -> Option<Pos2> {
        self.inner.hover_pos().into_option()
    }

    #[inline]
    pub fn interact_pos(&self) -> Option<Pos2> {
        self.inner.interact_pos().into_option()
    }

    #[inline]
    pub fn has_pointer(&self) -> bool {
        self.inner.has_pointer()
    }

    #[inline]
    pub fn is_still(&self) -> bool {
        self.inner.is_still()
    }

    #[inline]
    pub fn is_moving(&self) -> bool {
        self.inner.is_moving()
    }

    #[inline]
    pub fn time_since_last_movement(&self) -> f32 {
        self.inner.time_since_last_movement()
    }

    #[inline]
    pub fn time_since_last_click(&self) -> f32 {
        self.inner.time_since_last_click()
    }

    #[inline]
    pub fn any_pressed(&self) -> bool {
        self.inner.any_pressed()
    }

    #[inline]
    pub fn any_released(&self) -> bool {
        self.inner.any_released()
    }

    #[inline]
    pub fn button_pressed(&self, button: PointerButton) -> bool {
        self.inner.button_pressed(button)
    }

    #[inline]
    pub fn button_released(&self, button: PointerButton) -> bool {
        self.inner.button_released(button)
    }

    #[inline]
    pub fn primary_pressed(&self) -> bool {
        self.inner.primary_pressed()
    }

    #[inline]
    pub fn secondary_pressed(&self) -> bool {
        self.inner.secondary_pressed()
    }

    #[inline]
    pub fn primary_released(&self) -> bool {
        self.inner.primary_released()
    }

    #[inline]
    pub fn secondary_released(&self) -> bool {
        self.inner.secondary_released()
    }

    #[inline]
    pub fn any_down(&self) -> bool {
        self.inner.any_down()
    }

    #[inline]
    pub fn any_click(&self) -> bool {
        self.inner.any_click()
    }

    #[inline]
    pub fn button_clicked(&self, button: PointerButton) -> bool {
        self.inner.button_clicked(button)
    }

    #[inline]
    pub fn button_double_clicked(&self, button: PointerButton) -> bool {
        self.inner.button_double_clicked(button)
    }

    #[inline]
    pub fn button_triple_clicked(&self, button: PointerButton) -> bool {
        self.inner.button_triple_clicked(button)
    }

    #[inline]
    pub fn primary_clicked(&self) -> bool {
        self.inner.primary_clicked()
    }

    #[inline]
    pub fn secondary_clicked(&self) -> bool {
        self.inner.secondary_clicked()
    }

    #[inline]
    pub fn button_down(&self, button: PointerButton) -> bool {
        self.inner.button_down(button)
    }

    #[inline]
    pub fn could_any_button_be_click(&self) -> bool {
        self.inner.could_any_button_be_click()
    }

    #[inline]
    pub fn is_decidedly_dragging(&self) -> bool {
        self.inner.is_decidedly_dragging()
    }

    #[inline]
    pub fn primary_down(&self) -> bool {
        self.inner.primary_down()
    }

    #[inline]
    pub fn secondary_down(&self) -> bool {
        self.inner.secondary_down()
    }

    #[inline]
    pub fn middle_down(&self) -> bool {
        self.inner.middle_down()
    }

    #[inline]
    pub fn is_moving_towards_rect(&self, rect: &Rect) -> bool {
        self.inner.is_moving_towards_rect(rect)
    }
}
