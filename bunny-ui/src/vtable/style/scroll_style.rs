use vtable::vtable;

use crate::{Margin, style::ScrollFadeStyle};

#[vtable]
#[repr(C)]
pub struct ScrollStyleFfiVTable {
    floating: fn(VRef<ScrollStyleFfiVTable>) -> bool,
    content_margin: fn(VRef<ScrollStyleFfiVTable>) -> Margin,
    bar_width: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    handle_min_length: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    bar_inner_margin: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    bar_outer_margin: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    floating_width: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    floating_allocated_width: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    foreground_color: fn(VRef<ScrollStyleFfiVTable>) -> bool,
    dormant_background_opacity: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    active_background_opacity: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    interact_background_opacity: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    dormant_handle_opacity: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    active_handle_opacity: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    interact_handle_opacity: fn(VRef<ScrollStyleFfiVTable>) -> f32,
    fade: fn(VRef<ScrollStyleFfiVTable>) -> ScrollFadeStyle,

    set_floating: fn(VRefMut<ScrollStyleFfiVTable>, floating: bool),
    set_content_margin: fn(VRefMut<ScrollStyleFfiVTable>, margin: Margin),
    set_bar_width: fn(VRefMut<ScrollStyleFfiVTable>, width: f32),
    set_handle_min_length: fn(VRefMut<ScrollStyleFfiVTable>, handle_min_length: f32),
    set_bar_inner_margin: fn(VRefMut<ScrollStyleFfiVTable>, margin: f32),
    set_bar_outer_margin: fn(VRefMut<ScrollStyleFfiVTable>, margin: f32),
    set_floating_width: fn(VRefMut<ScrollStyleFfiVTable>, width: f32),
    set_floating_allocated_width: fn(VRefMut<ScrollStyleFfiVTable>, width: f32),
    set_foreground_color: fn(VRefMut<ScrollStyleFfiVTable>, color: bool),
    set_dormant_background_opacity: fn(VRefMut<ScrollStyleFfiVTable>, opacity: f32),
    set_active_background_opacity: fn(VRefMut<ScrollStyleFfiVTable>, opacity: f32),
    set_interact_background_opacity: fn(VRefMut<ScrollStyleFfiVTable>, opacity: f32),
    set_dormant_handle_opacity: fn(VRefMut<ScrollStyleFfiVTable>, opacity: f32),
    set_active_handle_opacity: fn(VRefMut<ScrollStyleFfiVTable>, opacity: f32),
    set_interact_handle_opacity: fn(VRefMut<ScrollStyleFfiVTable>, opacity: f32),
    set_fade: fn(VRefMut<ScrollStyleFfiVTable>, fade: ScrollFadeStyle),
}

#[cfg(feature = "manager")]
impl ScrollStyleFfi for egui::style::ScrollStyle {
    #[inline]
    fn floating(&self) -> bool {
        self.floating
    }

    #[inline]
    fn content_margin(&self) -> Margin {
        self.content_margin.into()
    }

    #[inline]
    fn bar_width(&self) -> f32 {
        self.bar_width
    }

    #[inline]
    fn handle_min_length(&self) -> f32 {
        self.handle_min_length
    }

    #[inline]
    fn bar_inner_margin(&self) -> f32 {
        self.bar_inner_margin
    }

    #[inline]
    fn bar_outer_margin(&self) -> f32 {
        self.bar_outer_margin
    }

    #[inline]
    fn floating_width(&self) -> f32 {
        self.floating_width
    }

    #[inline]
    fn floating_allocated_width(&self) -> f32 {
        self.floating_allocated_width
    }

    #[inline]
    fn foreground_color(&self) -> bool {
        self.foreground_color
    }

    #[inline]
    fn dormant_background_opacity(&self) -> f32 {
        self.dormant_background_opacity
    }

    #[inline]
    fn active_background_opacity(&self) -> f32 {
        self.active_background_opacity
    }

    #[inline]
    fn interact_background_opacity(&self) -> f32 {
        self.interact_background_opacity
    }

    #[inline]
    fn dormant_handle_opacity(&self) -> f32 {
        self.dormant_handle_opacity
    }

    #[inline]
    fn active_handle_opacity(&self) -> f32 {
        self.active_handle_opacity
    }

    #[inline]
    fn interact_handle_opacity(&self) -> f32 {
        self.interact_handle_opacity
    }

    #[inline]
    fn fade(&self) -> ScrollFadeStyle {
        self.fade.into()
    }

    #[inline]
    fn set_floating(&mut self, floating: bool) {
        self.floating = floating
    }

    #[inline]
    fn set_content_margin(&mut self, margin: Margin) {
        self.content_margin = margin.into()
    }

    #[inline]
    fn set_bar_width(&mut self, width: f32) {
        self.bar_width = width
    }

    #[inline]
    fn set_handle_min_length(&mut self, handle_min_length: f32) {
        self.handle_min_length = handle_min_length
    }

    #[inline]
    fn set_bar_inner_margin(&mut self, margin: f32) {
        self.bar_inner_margin = margin
    }

    #[inline]
    fn set_bar_outer_margin(&mut self, margin: f32) {
        self.bar_outer_margin = margin
    }

    #[inline]
    fn set_floating_width(&mut self, width: f32) {
        self.floating_width = width
    }

    #[inline]
    fn set_floating_allocated_width(&mut self, width: f32) {
        self.floating_allocated_width = width
    }

    #[inline]
    fn set_foreground_color(&mut self, color: bool) {
        self.foreground_color = color
    }

    #[inline]
    fn set_dormant_background_opacity(&mut self, opacity: f32) {
        self.dormant_background_opacity = opacity
    }

    #[inline]
    fn set_active_background_opacity(&mut self, opacity: f32) {
        self.active_background_opacity = opacity
    }

    #[inline]
    fn set_interact_background_opacity(&mut self, opacity: f32) {
        self.interact_background_opacity = opacity
    }

    #[inline]
    fn set_dormant_handle_opacity(&mut self, opacity: f32) {
        self.dormant_handle_opacity = opacity
    }

    #[inline]
    fn set_active_handle_opacity(&mut self, opacity: f32) {
        self.active_handle_opacity = opacity
    }

    #[inline]
    fn set_interact_handle_opacity(&mut self, opacity: f32) {
        self.interact_handle_opacity = opacity
    }

    #[inline]
    fn set_fade(&mut self, fade: ScrollFadeStyle) {
        self.fade = fade.into()
    }
}

#[cfg(feature = "manager")]
ScrollStyleFfiVTable_static!(static SCROLLSTYLEFFI_VT for egui::style::ScrollStyle);
