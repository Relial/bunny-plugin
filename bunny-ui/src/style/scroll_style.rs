use vtable::{VRef, VRefMut};

use crate::{Margin, style::ScrollFadeStyle, vtable::style::scroll_style::ScrollStyleFfiVTable};

pub trait BunnyScrollStyle {
    fn as_ref(&self) -> VRef<'_, ScrollStyleFfiVTable>;

    #[inline]
    fn floating(&self) -> bool {
        self.as_ref().floating()
    }

    #[inline]
    fn content_margin(&self) -> Margin {
        self.as_ref().content_margin()
    }

    #[inline]
    fn bar_width(&self) -> f32 {
        self.as_ref().bar_width()
    }

    #[inline]
    fn handle_min_length(&self) -> f32 {
        self.as_ref().handle_min_length()
    }

    #[inline]
    fn bar_inner_margin(&self) -> f32 {
        self.as_ref().bar_inner_margin()
    }

    #[inline]
    fn bar_outer_margin(&self) -> f32 {
        self.as_ref().bar_outer_margin()
    }

    #[inline]
    fn floating_width(&self) -> f32 {
        self.as_ref().floating_width()
    }

    #[inline]
    fn floating_allocated_width(&self) -> f32 {
        self.as_ref().floating_allocated_width()
    }

    #[inline]
    fn foreground_color(&self) -> bool {
        self.as_ref().foreground_color()
    }

    #[inline]
    fn dormant_background_opacity(&self) -> f32 {
        self.as_ref().dormant_background_opacity()
    }

    #[inline]
    fn active_background_opacity(&self) -> f32 {
        self.as_ref().active_background_opacity()
    }

    #[inline]
    fn interact_background_opacity(&self) -> f32 {
        self.as_ref().interact_background_opacity()
    }

    #[inline]
    fn dormant_handle_opacity(&self) -> f32 {
        self.as_ref().dormant_handle_opacity()
    }

    #[inline]
    fn active_handle_opacity(&self) -> f32 {
        self.as_ref().active_handle_opacity()
    }

    #[inline]
    fn interact_handle_opacity(&self) -> f32 {
        self.as_ref().interact_handle_opacity()
    }

    #[inline]
    fn fade(&self) -> ScrollFadeStyle {
        self.as_ref().fade()
    }
}

#[repr(C)]
pub struct BunnyScrollStyleRef<'a> {
    inner: VRef<'a, ScrollStyleFfiVTable>,
}

impl<'a> BunnyScrollStyleRef<'a> {
    #[inline]
    pub fn new(scroll_style: &'a egui::style::ScrollStyle) -> Self {
        Self {
            inner: VRef::new(scroll_style),
        }
    }
}

impl BunnyScrollStyle for BunnyScrollStyleRef<'_> {
    fn as_ref(&self) -> VRef<'_, ScrollStyleFfiVTable> {
        self.inner
    }
}

#[repr(C)]
pub struct BunnyScrollStyleMut<'a> {
    inner: VRefMut<'a, ScrollStyleFfiVTable>,
}

impl<'a> BunnyScrollStyleMut<'a> {
    #[inline]
    pub fn new(scroll_style: &'a mut egui::style::ScrollStyle) -> Self {
        Self {
            inner: VRefMut::new(scroll_style),
        }
    }
}

impl BunnyScrollStyleMut<'_> {
    #[inline]
    pub fn set_floating(&mut self, floating: bool) {
        self.inner.set_floating(floating);
    }

    #[inline]
    pub fn set_content_margin(&mut self, content_margin: impl Into<Margin>) {
        self.inner.set_content_margin(content_margin.into());
    }

    #[inline]
    pub fn set_bar_width(&mut self, bar_width: f32) {
        self.inner.set_bar_width(bar_width);
    }

    #[inline]
    pub fn set_handle_min_length(&mut self, handle_min_length: f32) {
        self.inner.set_handle_min_length(handle_min_length);
    }

    #[inline]
    pub fn set_bar_inner_margin(&mut self, bar_inner_margin: f32) {
        self.inner.set_bar_inner_margin(bar_inner_margin);
    }

    #[inline]
    pub fn set_bar_outer_margin(&mut self, bar_outer_margin: f32) {
        self.inner.set_bar_outer_margin(bar_outer_margin);
    }

    #[inline]
    pub fn set_floating_width(&mut self, floating_width: f32) {
        self.inner.set_floating_width(floating_width);
    }

    #[inline]
    pub fn set_floating_allocated_width(&mut self, floating_allocated_width: f32) {
        self.inner
            .set_floating_allocated_width(floating_allocated_width);
    }

    #[inline]
    pub fn set_foreground_color(&mut self, foreground_color: bool) {
        self.inner.set_foreground_color(foreground_color);
    }

    #[inline]
    pub fn set_dormant_background_opacity(&mut self, dormant_background_opacity: f32) {
        self.inner
            .set_dormant_background_opacity(dormant_background_opacity);
    }

    #[inline]
    pub fn set_active_background_opacity(&mut self, active_background_opacity: f32) {
        self.inner
            .set_active_background_opacity(active_background_opacity);
    }

    #[inline]
    pub fn set_interact_background_opacity(&mut self, interact_background_opacity: f32) {
        self.inner
            .set_interact_background_opacity(interact_background_opacity);
    }

    #[inline]
    pub fn set_dormant_handle_opacity(&mut self, dormant_handle_opacity: f32) {
        self.inner
            .set_dormant_handle_opacity(dormant_handle_opacity);
    }

    #[inline]
    pub fn set_active_handle_opacity(&mut self, active_handle_opacity: f32) {
        self.inner.set_active_handle_opacity(active_handle_opacity);
    }

    #[inline]
    pub fn set_interact_handle_opacity(&mut self, interact_handle_opacity: f32) {
        self.inner
            .set_interact_handle_opacity(interact_handle_opacity);
    }

    #[inline]
    pub fn set_fade(&mut self, fade: ScrollFadeStyle) {
        self.inner.set_fade(fade);
    }
}

impl BunnyScrollStyle for BunnyScrollStyleMut<'_> {
    fn as_ref(&self) -> VRef<'_, ScrollStyleFfiVTable> {
        self.inner.borrow()
    }
}
