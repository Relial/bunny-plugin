use egui::{Rect, Vec2};
use vtable::VRefMut;

use crate::{
    Event, Key, KeyboardShortcut, Modifiers, input::BunnyPointerState,
    vtable::input::input_state::InputStateFfiVTable,
};

#[repr(C)]
pub struct BunnyInputState<'a> {
    inner: VRefMut<'a, InputStateFfiVTable>,
}

impl<'a> BunnyInputState<'a> {
    #[inline]
    pub fn new(input_state: &'a mut egui::InputState) -> Self {
        Self {
            inner: VRefMut::new(input_state),
        }
    }
}

impl<'a> BunnyInputState<'a> {
    #[inline]
    pub fn pointer(&self) -> BunnyPointerState<'_> {
        self.inner.pointer()
    }

    #[inline]
    pub fn smooth_scroll_delta(&self) -> Vec2 {
        self.inner.smooth_scroll_delta()
    }

    #[inline]
    pub fn pixels_per_point(&self) -> f32 {
        self.inner.pixels_per_point()
    }

    #[inline]
    pub fn max_texture_side(&self) -> usize {
        self.inner.max_texture_side()
    }

    #[inline]
    pub fn time(&self) -> f64 {
        self.inner.time()
    }

    #[inline]
    pub fn unstable_dt(&self) -> f32 {
        self.inner.unstable_dt()
    }

    #[inline]
    pub fn predicted_dt(&self) -> f32 {
        self.inner.predicted_dt()
    }

    #[inline]
    pub fn stable_dt(&self) -> f32 {
        self.inner.stable_dt()
    }

    #[inline]
    pub fn focused(&self) -> bool {
        self.inner.focused()
    }

    #[inline]
    pub fn modifiers(&self) -> Modifiers {
        self.inner.modifiers()
    }

    #[inline]
    pub fn keys_down(&self) -> impl Iterator<Item = Key> {
        self.inner.keys_down().into_iter()
    }

    #[inline]
    pub fn events(&self) -> impl Iterator<Item = Event> {
        self.inner.events().into_iter()
    }
}

impl<'a> BunnyInputState<'a> {
    #[inline]
    pub fn content_rect(&self) -> Rect {
        self.inner.content_rect()
    }

    #[inline]
    pub fn viewport_rect(&self) -> Rect {
        self.inner.viewport_rect()
    }

    #[inline]
    pub fn zoom_delta(&self) -> f32 {
        self.inner.zoom_delta()
    }

    #[inline]
    pub fn zoom_delta_2d(&self) -> Vec2 {
        self.inner.zoom_delta_2d()
    }

    #[inline]
    pub fn rotation_delta(&self) -> f32 {
        self.inner.rotation_delta()
    }

    #[inline]
    pub fn translation_delta(&self) -> Vec2 {
        self.inner.translation_delta()
    }

    #[inline]
    pub fn is_scrolling(&self) -> bool {
        self.inner.is_scrolling()
    }

    #[inline]
    pub fn time_since_last_scroll(&self) -> f32 {
        self.inner.time_since_last_scroll()
    }

    #[inline]
    pub fn count_and_consume_key(&mut self, modifiers: Modifiers, logical_key: Key) -> usize {
        self.inner.count_and_consume_key(modifiers, logical_key)
    }

    #[inline]
    pub fn consume_key(&mut self, modifiers: Modifiers, logical_key: Key) -> bool {
        self.inner.consume_key(modifiers, logical_key)
    }

    #[inline]
    pub fn consume_shortcut(&mut self, shortcut: KeyboardShortcut) -> bool {
        self.inner.consume_shortcut(shortcut)
    }

    #[inline]
    pub fn key_pressed(&self, desired_key: Key) -> bool {
        self.inner.key_pressed(desired_key)
    }

    #[inline]
    pub fn num_presses(&self, desired_key: Key) -> usize {
        self.inner.num_presses(desired_key)
    }

    #[inline]
    pub fn key_down(&self, desired_key: Key) -> bool {
        self.inner.key_down(desired_key)
    }

    #[inline]
    pub fn key_released(&self, desired_key: Key) -> bool {
        self.inner.key_released(desired_key)
    }

    #[inline]
    pub fn physical_pixel_size(&self) -> f32 {
        self.inner.physical_pixel_size()
    }

    #[inline]
    pub fn aim_radius(&self) -> f32 {
        self.inner.aim_radius()
    }
}
