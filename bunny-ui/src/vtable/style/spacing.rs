use egui::{Spacing, Vec2};
use vtable::{VRef, VRefMut, vtable};

use crate::{
    Margin,
    style::{BunnyScrollStyleMut, BunnyScrollStyleRef, ScrollStyle},
};

#[vtable]
#[repr(C)]
pub struct SpacingFfiVTable {
    item_spacing: fn(VRef<SpacingFfiVTable>) -> Vec2,
    window_margin: fn(VRef<SpacingFfiVTable>) -> Margin,
    button_padding: fn(VRef<SpacingFfiVTable>) -> Vec2,
    menu_margin: fn(VRef<SpacingFfiVTable>) -> Margin,
    indent: fn(VRef<SpacingFfiVTable>) -> f32,
    interact_size: fn(VRef<SpacingFfiVTable>) -> Vec2,
    slider_width: fn(VRef<SpacingFfiVTable>) -> f32,
    slider_rail_height: fn(VRef<SpacingFfiVTable>) -> f32,
    combo_width: fn(VRef<SpacingFfiVTable>) -> f32,
    text_edit_width: fn(VRef<SpacingFfiVTable>) -> f32,
    icon_width: fn(VRef<SpacingFfiVTable>) -> f32,
    icon_width_inner: fn(VRef<SpacingFfiVTable>) -> f32,
    icon_spacing: fn(VRef<SpacingFfiVTable>) -> f32,
    default_area_size: fn(VRef<SpacingFfiVTable>) -> Vec2,
    tooltip_width: fn(VRef<SpacingFfiVTable>) -> f32,
    menu_width: fn(VRef<SpacingFfiVTable>) -> f32,
    indent_ends_with_horizontal_line: fn(VRef<SpacingFfiVTable>) -> bool,
    combo_height: fn(VRef<SpacingFfiVTable>) -> f32,

    set_item_spacing: fn(VRefMut<SpacingFfiVTable>, item_spacing: Vec2),
    set_window_margin: fn(VRefMut<SpacingFfiVTable>, window_margin: Margin),
    set_button_padding: fn(VRefMut<SpacingFfiVTable>, button_padding: Vec2),
    set_menu_margin: fn(VRefMut<SpacingFfiVTable>, menu_margin: Margin),
    set_indent: fn(VRefMut<SpacingFfiVTable>, indent: f32),
    set_interact_size: fn(VRefMut<SpacingFfiVTable>, interact_size: Vec2),
    set_slider_width: fn(VRefMut<SpacingFfiVTable>, slider_width: f32),
    set_slider_rail_height: fn(VRefMut<SpacingFfiVTable>, slider_rail_height: f32),
    set_combo_width: fn(VRefMut<SpacingFfiVTable>, combo_width: f32),
    set_text_edit_width: fn(VRefMut<SpacingFfiVTable>, text_edit_width: f32),
    set_icon_width: fn(VRefMut<SpacingFfiVTable>, icon_width: f32),
    set_icon_width_inner: fn(VRefMut<SpacingFfiVTable>, icon_width_inner: f32),
    set_icon_spacing: fn(VRefMut<SpacingFfiVTable>, icon_spacing: f32),
    set_default_area_size: fn(VRefMut<SpacingFfiVTable>, default_area_size: Vec2),
    set_tooltip_width: fn(VRefMut<SpacingFfiVTable>, tooltip_width: f32),
    set_menu_width: fn(VRefMut<SpacingFfiVTable>, menu_width: f32),
    set_menu_spacing: fn(VRefMut<SpacingFfiVTable>, menu_spacing: f32),
    set_indent_ends_with_horizontal_line:
        fn(VRefMut<SpacingFfiVTable>, indent_ends_with_horizontal_line: bool),
    set_combo_height: fn(VRefMut<SpacingFfiVTable>, combo_height: f32),

    scroll_style: fn(VRef<SpacingFfiVTable>) -> BunnyScrollStyleRef,
    scroll_style_mut: fn(VRefMut<SpacingFfiVTable>) -> BunnyScrollStyleMut,
    scroll_style_clone: fn(VRef<SpacingFfiVTable>) -> ScrollStyle,
    set_scroll_style: fn(VRefMut<SpacingFfiVTable>, scroll_style: ScrollStyle),
}

impl SpacingFfi for Spacing {
    #[inline]
    fn item_spacing(&self) -> Vec2 {
        self.item_spacing
    }

    #[inline]
    fn window_margin(&self) -> Margin {
        self.window_margin.into()
    }

    #[inline]
    fn button_padding(&self) -> Vec2 {
        self.button_padding
    }

    #[inline]
    fn menu_margin(&self) -> Margin {
        self.menu_margin.into()
    }

    #[inline]
    fn indent(&self) -> f32 {
        self.indent
    }

    #[inline]
    fn interact_size(&self) -> Vec2 {
        self.interact_size
    }

    #[inline]
    fn slider_width(&self) -> f32 {
        self.slider_width
    }

    #[inline]
    fn slider_rail_height(&self) -> f32 {
        self.slider_rail_height
    }

    #[inline]
    fn combo_width(&self) -> f32 {
        self.combo_width
    }

    #[inline]
    fn text_edit_width(&self) -> f32 {
        self.text_edit_width
    }

    #[inline]
    fn icon_width(&self) -> f32 {
        self.icon_width
    }

    #[inline]
    fn icon_width_inner(&self) -> f32 {
        self.icon_width_inner
    }

    #[inline]
    fn icon_spacing(&self) -> f32 {
        self.icon_spacing
    }

    #[inline]
    fn default_area_size(&self) -> Vec2 {
        self.default_area_size
    }

    #[inline]
    fn tooltip_width(&self) -> f32 {
        self.tooltip_width
    }

    #[inline]
    fn menu_width(&self) -> f32 {
        self.menu_width
    }

    #[inline]
    fn indent_ends_with_horizontal_line(&self) -> bool {
        self.indent_ends_with_horizontal_line
    }

    #[inline]
    fn combo_height(&self) -> f32 {
        self.combo_height
    }

    #[inline]
    fn set_item_spacing(&mut self, item_spacing: Vec2) {
        self.item_spacing = item_spacing
    }

    #[inline]
    fn set_window_margin(&mut self, window_margin: Margin) {
        self.window_margin = window_margin.into()
    }

    #[inline]
    fn set_button_padding(&mut self, button_padding: Vec2) {
        self.button_padding = button_padding
    }

    #[inline]
    fn set_menu_margin(&mut self, menu_margin: Margin) {
        self.menu_margin = menu_margin.into()
    }

    #[inline]
    fn set_indent(&mut self, indent: f32) {
        self.indent = indent
    }

    #[inline]
    fn set_interact_size(&mut self, interact_size: Vec2) {
        self.interact_size = interact_size
    }

    #[inline]
    fn set_slider_width(&mut self, slider_width: f32) {
        self.slider_width = slider_width
    }

    #[inline]
    fn set_slider_rail_height(&mut self, slider_rail_height: f32) {
        self.slider_rail_height = slider_rail_height
    }

    #[inline]
    fn set_combo_width(&mut self, combo_width: f32) {
        self.combo_width = combo_width
    }

    #[inline]
    fn set_text_edit_width(&mut self, text_edit_width: f32) {
        self.text_edit_width = text_edit_width
    }

    #[inline]
    fn set_icon_width(&mut self, icon_width: f32) {
        self.icon_width = icon_width
    }

    #[inline]
    fn set_icon_width_inner(&mut self, icon_width_inner: f32) {
        self.icon_width_inner = icon_width_inner
    }

    #[inline]
    fn set_icon_spacing(&mut self, icon_spacing: f32) {
        self.icon_spacing = icon_spacing
    }

    #[inline]
    fn set_default_area_size(&mut self, default_area_size: Vec2) {
        self.default_area_size = default_area_size
    }

    #[inline]
    fn set_tooltip_width(&mut self, tooltip_width: f32) {
        self.tooltip_width = tooltip_width
    }

    #[inline]
    fn set_menu_width(&mut self, menu_width: f32) {
        self.menu_width = menu_width
    }

    #[inline]
    fn set_menu_spacing(&mut self, menu_spacing: f32) {
        self.menu_spacing = menu_spacing
    }

    #[inline]
    fn set_indent_ends_with_horizontal_line(&mut self, indent_ends_with_horizontal_line: bool) {
        self.indent_ends_with_horizontal_line = indent_ends_with_horizontal_line
    }

    #[inline]
    fn set_combo_height(&mut self, combo_height: f32) {
        self.combo_height = combo_height
    }

    #[inline]
    fn scroll_style(&self) -> BunnyScrollStyleRef<'_> {
        BunnyScrollStyleRef::new(&self.scroll)
    }

    #[inline]
    fn scroll_style_mut(&mut self) -> BunnyScrollStyleMut<'_> {
        BunnyScrollStyleMut::new(&mut self.scroll)
    }

    #[inline]
    fn scroll_style_clone(&self) -> ScrollStyle {
        self.scroll.into()
    }

    #[inline]
    fn set_scroll_style(&mut self, scroll_style: ScrollStyle) {
        self.scroll = scroll_style.into()
    }
}

SpacingFfiVTable_static!(static SPACINGFFI_VT for Spacing);
