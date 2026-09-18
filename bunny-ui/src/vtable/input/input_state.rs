use abi_stable::std_types::RVec;
use egui::{Rect, Vec2};
use vtable::{VRef, vtable};

use crate::{Event, Key, KeyboardShortcut, Modifiers, input::BunnyPointerState};

#[vtable]
#[repr(C)]
pub struct InputStateFfiVTable {
    // raw
    pointer: fn(VRef<InputStateFfiVTable>) -> BunnyPointerState,
    smooth_scroll_delta: fn(VRef<InputStateFfiVTable>) -> Vec2,
    pixels_per_point: fn(VRef<InputStateFfiVTable>) -> f32,
    max_texture_side: fn(VRef<InputStateFfiVTable>) -> usize,
    time: fn(VRef<InputStateFfiVTable>) -> f64,
    unstable_dt: fn(VRef<InputStateFfiVTable>) -> f32,
    predicted_dt: fn(VRef<InputStateFfiVTable>) -> f32,
    stable_dt: fn(VRef<InputStateFfiVTable>) -> f32,
    focused: fn(VRef<InputStateFfiVTable>) -> bool,
    modifiers: fn(VRef<InputStateFfiVTable>) -> Modifiers,
    keys_down: fn(VRef<InputStateFfiVTable>) -> RVec<Key>,
    events: fn(VRef<InputStateFfiVTable>) -> RVec<Event>,

    // begin_pass
    // viewport
    content_rect: fn(VRef<InputStateFfiVTable>) -> Rect,
    viewport_rect: fn(VRef<InputStateFfiVTable>) -> Rect,
    // safe_area_insets
    zoom_delta: fn(VRef<InputStateFfiVTable>) -> f32,
    zoom_delta_2d: fn(VRef<InputStateFfiVTable>) -> Vec2,
    rotation_delta: fn(VRef<InputStateFfiVTable>) -> f32,
    translation_delta: fn(VRef<InputStateFfiVTable>) -> Vec2,
    is_scrolling: fn(VRef<InputStateFfiVTable>) -> bool,
    time_since_last_scroll: fn(VRef<InputStateFfiVTable>) -> f32,
    count_and_consume_key:
        fn(VRefMut<InputStateFfiVTable>, modifiers: Modifiers, logical_key: Key) -> usize,
    consume_key: fn(VRefMut<InputStateFfiVTable>, modifiers: Modifiers, logical_key: Key) -> bool,
    consume_shortcut: fn(VRefMut<InputStateFfiVTable>, shortcut: KeyboardShortcut) -> bool,
    key_pressed: fn(VRef<InputStateFfiVTable>, desired_key: Key) -> bool,
    num_presses: fn(VRef<InputStateFfiVTable>, desired_key: Key) -> usize,
    key_down: fn(VRef<InputStateFfiVTable>, desired_key: Key) -> bool,
    key_released: fn(VRef<InputStateFfiVTable>, desired_key: Key) -> bool,
    physical_pixel_size: fn(VRef<InputStateFfiVTable>) -> f32,
    aim_radius: fn(VRef<InputStateFfiVTable>) -> f32,
}

impl InputStateFfi for egui::InputState {
    #[inline]
    fn pointer(&self) -> BunnyPointerState<'_> {
        BunnyPointerState::new(&self.pointer)
    }

    #[inline]
    fn smooth_scroll_delta(&self) -> Vec2 {
        self.smooth_scroll_delta
    }

    #[inline]
    fn pixels_per_point(&self) -> f32 {
        self.pixels_per_point
    }

    #[inline]
    fn max_texture_side(&self) -> usize {
        self.max_texture_side
    }

    #[inline]
    fn time(&self) -> f64 {
        self.time
    }

    #[inline]
    fn unstable_dt(&self) -> f32 {
        self.unstable_dt
    }

    #[inline]
    fn predicted_dt(&self) -> f32 {
        self.predicted_dt
    }

    #[inline]
    fn stable_dt(&self) -> f32 {
        self.stable_dt
    }

    #[inline]
    fn focused(&self) -> bool {
        self.focused
    }

    #[inline]
    fn modifiers(&self) -> Modifiers {
        self.modifiers.into()
    }

    #[inline]
    fn keys_down(&self) -> RVec<Key> {
        self.keys_down.iter().copied().map(|k| k.into()).collect()
    }

    #[inline]
    fn events(&self) -> RVec<Event> {
        self.events.iter().map(|e| e.into()).collect()
    }

    #[inline]
    fn content_rect(&self) -> Rect {
        self.content_rect()
    }

    #[inline]
    fn viewport_rect(&self) -> Rect {
        self.viewport_rect()
    }

    #[inline]
    fn zoom_delta(&self) -> f32 {
        self.zoom_delta()
    }

    #[inline]
    fn zoom_delta_2d(&self) -> Vec2 {
        self.zoom_delta_2d()
    }

    #[inline]
    fn rotation_delta(&self) -> f32 {
        self.rotation_delta()
    }

    #[inline]
    fn translation_delta(&self) -> Vec2 {
        self.translation_delta()
    }

    #[inline]
    fn is_scrolling(&self) -> bool {
        self.is_scrolling()
    }

    #[inline]
    fn time_since_last_scroll(&self) -> f32 {
        self.time_since_last_scroll()
    }

    #[inline]
    fn count_and_consume_key(&mut self, modifiers: Modifiers, logical_key: Key) -> usize {
        self.count_and_consume_key(modifiers.into(), logical_key.into())
    }

    #[inline]
    fn consume_key(&mut self, modifiers: Modifiers, logical_key: Key) -> bool {
        self.consume_key(modifiers.into(), logical_key.into())
    }

    #[inline]
    fn consume_shortcut(&mut self, shortcut: KeyboardShortcut) -> bool {
        self.consume_shortcut(&shortcut.into())
    }

    #[inline]
    fn key_pressed(&self, desired_key: Key) -> bool {
        self.key_pressed(desired_key.into())
    }

    #[inline]
    fn num_presses(&self, desired_key: Key) -> usize {
        self.num_presses(desired_key.into())
    }

    #[inline]
    fn key_down(&self, desired_key: Key) -> bool {
        self.key_down(desired_key.into())
    }

    #[inline]
    fn key_released(&self, desired_key: Key) -> bool {
        self.key_released(desired_key.into())
    }

    #[inline]
    fn physical_pixel_size(&self) -> f32 {
        self.physical_pixel_size()
    }

    #[inline]
    fn aim_radius(&self) -> f32 {
        self.aim_radius()
    }
}

InputStateFfiVTable_static!(static INPUTSTATEFFI_VT for egui::InputState);
