use abi_stable::std_types::ROption;
use egui::{Pos2, Rect, Vec2};
use vtable::vtable;

use crate::PointerButton;

#[vtable]
#[repr(C)]
pub struct PointerStateFfiVTable {
    delta: fn(VRef<PointerStateFfiVTable>) -> Vec2,
    motion: fn(VRef<PointerStateFfiVTable>) -> ROption<Vec2>,
    velocity: fn(VRef<PointerStateFfiVTable>) -> Vec2,
    direction: fn(VRef<PointerStateFfiVTable>) -> Vec2,
    press_origin: fn(VRef<PointerStateFfiVTable>) -> ROption<Pos2>,
    total_drag_delta: fn(VRef<PointerStateFfiVTable>) -> ROption<Vec2>,
    press_start_time: fn(VRef<PointerStateFfiVTable>) -> ROption<f64>,
    latest_pos: fn(VRef<PointerStateFfiVTable>) -> ROption<Pos2>,
    hover_pos: fn(VRef<PointerStateFfiVTable>) -> ROption<Pos2>,
    interact_pos: fn(VRef<PointerStateFfiVTable>) -> ROption<Pos2>,
    has_pointer: fn(VRef<PointerStateFfiVTable>) -> bool,
    is_still: fn(VRef<PointerStateFfiVTable>) -> bool,
    is_moving: fn(VRef<PointerStateFfiVTable>) -> bool,
    time_since_last_movement: fn(VRef<PointerStateFfiVTable>) -> f32,
    time_since_last_click: fn(VRef<PointerStateFfiVTable>) -> f32,
    any_pressed: fn(VRef<PointerStateFfiVTable>) -> bool,
    any_released: fn(VRef<PointerStateFfiVTable>) -> bool,
    button_pressed: fn(VRef<PointerStateFfiVTable>, button: PointerButton) -> bool,
    button_released: fn(VRef<PointerStateFfiVTable>, button: PointerButton) -> bool,
    primary_pressed: fn(VRef<PointerStateFfiVTable>) -> bool,
    secondary_pressed: fn(VRef<PointerStateFfiVTable>) -> bool,
    primary_released: fn(VRef<PointerStateFfiVTable>) -> bool,
    secondary_released: fn(VRef<PointerStateFfiVTable>) -> bool,
    any_down: fn(VRef<PointerStateFfiVTable>) -> bool,
    any_click: fn(VRef<PointerStateFfiVTable>) -> bool,
    button_clicked: fn(VRef<PointerStateFfiVTable>, button: PointerButton) -> bool,
    button_double_clicked: fn(VRef<PointerStateFfiVTable>, button: PointerButton) -> bool,
    button_triple_clicked: fn(VRef<PointerStateFfiVTable>, button: PointerButton) -> bool,
    primary_clicked: fn(VRef<PointerStateFfiVTable>) -> bool,
    secondary_clicked: fn(VRef<PointerStateFfiVTable>) -> bool,
    button_down: fn(VRef<PointerStateFfiVTable>, button: PointerButton) -> bool,
    could_any_button_be_click: fn(VRef<PointerStateFfiVTable>) -> bool,
    is_decidedly_dragging: fn(VRef<PointerStateFfiVTable>) -> bool,
    primary_down: fn(VRef<PointerStateFfiVTable>) -> bool,
    secondary_down: fn(VRef<PointerStateFfiVTable>) -> bool,
    middle_down: fn(VRef<PointerStateFfiVTable>) -> bool,
    is_moving_towards_rect: fn(VRef<PointerStateFfiVTable>, rect: &Rect) -> bool,
}

impl PointerStateFfi for egui::PointerState {
    #[inline]
    fn delta(&self) -> Vec2 {
        self.delta()
    }

    #[inline]
    fn motion(&self) -> ROption<Vec2> {
        self.motion().into()
    }

    #[inline]
    fn velocity(&self) -> Vec2 {
        self.velocity()
    }

    #[inline]
    fn direction(&self) -> Vec2 {
        self.direction()
    }

    #[inline]
    fn press_origin(&self) -> ROption<Pos2> {
        self.press_origin().into()
    }

    #[inline]
    fn total_drag_delta(&self) -> ROption<Vec2> {
        self.total_drag_delta().into()
    }

    #[inline]
    fn press_start_time(&self) -> ROption<f64> {
        self.press_start_time().into()
    }

    #[inline]
    fn latest_pos(&self) -> ROption<Pos2> {
        self.latest_pos().into()
    }

    #[inline]
    fn hover_pos(&self) -> ROption<Pos2> {
        self.hover_pos().into()
    }

    #[inline]
    fn interact_pos(&self) -> ROption<Pos2> {
        self.interact_pos().into()
    }

    #[inline]
    fn has_pointer(&self) -> bool {
        self.has_pointer()
    }

    #[inline]
    fn is_still(&self) -> bool {
        self.is_still()
    }

    #[inline]
    fn is_moving(&self) -> bool {
        self.is_moving()
    }

    #[inline]
    fn time_since_last_movement(&self) -> f32 {
        self.time_since_last_movement()
    }

    #[inline]
    fn time_since_last_click(&self) -> f32 {
        self.time_since_last_click()
    }

    #[inline]
    fn any_pressed(&self) -> bool {
        self.any_pressed()
    }

    #[inline]
    fn any_released(&self) -> bool {
        self.any_released()
    }

    #[inline]
    fn button_pressed(&self, button: PointerButton) -> bool {
        self.button_pressed(button.into())
    }

    #[inline]
    fn button_released(&self, button: PointerButton) -> bool {
        self.button_released(button.into())
    }

    #[inline]
    fn primary_pressed(&self) -> bool {
        self.primary_pressed()
    }

    #[inline]
    fn secondary_pressed(&self) -> bool {
        self.secondary_pressed()
    }

    #[inline]
    fn primary_released(&self) -> bool {
        self.primary_released()
    }

    #[inline]
    fn secondary_released(&self) -> bool {
        self.secondary_released()
    }

    #[inline]
    fn any_down(&self) -> bool {
        self.any_down()
    }

    #[inline]
    fn any_click(&self) -> bool {
        self.any_click()
    }

    #[inline]
    fn button_clicked(&self, button: PointerButton) -> bool {
        self.button_clicked(button.into())
    }

    #[inline]
    fn button_double_clicked(&self, button: PointerButton) -> bool {
        self.button_double_clicked(button.into())
    }

    #[inline]
    fn button_triple_clicked(&self, button: PointerButton) -> bool {
        self.button_triple_clicked(button.into())
    }

    #[inline]
    fn primary_clicked(&self) -> bool {
        self.primary_clicked()
    }

    #[inline]
    fn secondary_clicked(&self) -> bool {
        self.secondary_clicked()
    }

    #[inline]
    fn button_down(&self, button: PointerButton) -> bool {
        self.button_down(button.into())
    }

    #[inline]
    fn could_any_button_be_click(&self) -> bool {
        self.could_any_button_be_click()
    }

    #[inline]
    fn is_decidedly_dragging(&self) -> bool {
        self.is_decidedly_dragging()
    }

    #[inline]
    fn primary_down(&self) -> bool {
        self.primary_down()
    }

    #[inline]
    fn secondary_down(&self) -> bool {
        self.secondary_down()
    }

    #[inline]
    fn middle_down(&self) -> bool {
        self.middle_down()
    }

    #[inline]
    fn is_moving_towards_rect(&self, rect: &Rect) -> bool {
        self.is_moving_towards_rect(rect)
    }
}

PointerStateFfiVTable_static!(static POINTERSTATEFFI_VT for egui::PointerState);
