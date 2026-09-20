use egui::Vec2;
use mint::Vector2;
use vtable::{VRef, VRefMut};

use crate::{
    Margin,
    style::{BunnyScrollStyleMut, BunnyScrollStyleRef, ScrollStyle},
    vtable::style::spacing::SpacingFfiVTable,
};

pub trait BunnySpacing {
    fn as_ref(&self) -> VRef<'_, SpacingFfiVTable>;

    #[inline]
    fn item_spacing(&self) -> Vec2 {
        self.as_ref().item_spacing()
    }

    #[inline]
    fn window_margin(&self) -> Margin {
        self.as_ref().window_margin()
    }

    #[inline]
    fn button_padding(&self) -> Vec2 {
        self.as_ref().button_padding()
    }

    #[inline]
    fn menu_margin(&self) -> Margin {
        self.as_ref().menu_margin()
    }

    #[inline]
    fn indent(&self) -> f32 {
        self.as_ref().indent()
    }

    #[inline]
    fn interact_size(&self) -> Vec2 {
        self.as_ref().interact_size()
    }

    #[inline]
    fn slider_width(&self) -> f32 {
        self.as_ref().slider_width()
    }

    #[inline]
    fn slider_rail_height(&self) -> f32 {
        self.as_ref().slider_rail_height()
    }

    #[inline]
    fn combo_width(&self) -> f32 {
        self.as_ref().combo_width()
    }

    #[inline]
    fn text_edit_width(&self) -> f32 {
        self.as_ref().text_edit_width()
    }

    #[inline]
    fn icon_width(&self) -> f32 {
        self.as_ref().icon_width()
    }

    #[inline]
    fn icon_width_inner(&self) -> f32 {
        self.as_ref().icon_width_inner()
    }

    #[inline]
    fn icon_spacing(&self) -> f32 {
        self.as_ref().icon_spacing()
    }

    #[inline]
    fn default_area_size(&self) -> Vec2 {
        self.as_ref().default_area_size()
    }

    #[inline]
    fn tooltip_width(&self) -> f32 {
        self.as_ref().tooltip_width()
    }

    #[inline]
    fn menu_width(&self) -> f32 {
        self.as_ref().menu_width()
    }

    #[inline]
    fn indent_ends_with_horizontal_line(&self) -> bool {
        self.as_ref().indent_ends_with_horizontal_line()
    }

    #[inline]
    fn combo_height(&self) -> f32 {
        self.as_ref().combo_height()
    }
}

#[repr(transparent)]
pub struct BunnySpacingRef<'a> {
    inner: VRef<'a, SpacingFfiVTable>,
}

impl<'a> BunnySpacingRef<'a> {
    #[inline]
    pub fn new(spacing: &'a egui::style::Spacing) -> Self {
        Self {
            inner: VRef::new(spacing),
        }
    }
}

impl BunnySpacingRef<'_> {
    #[inline]
    pub fn scroll_style(&self) -> BunnyScrollStyleRef<'_> {
        self.inner.scroll_style()
    }

    #[inline]
    pub fn scroll_style_clone(&self) -> ScrollStyle {
        self.inner.scroll_style_clone()
    }
}

impl BunnySpacing for BunnySpacingRef<'_> {
    #[inline]
    fn as_ref(&self) -> VRef<'_, SpacingFfiVTable> {
        self.inner
    }
}

#[repr(transparent)]
pub struct BunnySpacingMut<'a> {
    inner: VRefMut<'a, SpacingFfiVTable>,
}

impl<'a> BunnySpacingMut<'a> {
    #[inline]
    pub fn new(spacing: &'a mut egui::style::Spacing) -> Self {
        Self {
            inner: VRefMut::new(spacing),
        }
    }
}

impl BunnySpacingMut<'_> {
    #[inline]
    pub fn set_item_spacing(&mut self, item_spacing: impl Into<Vector2<f32>>) {
        self.inner.set_item_spacing(item_spacing.into().into());
    }

    #[inline]
    pub fn set_window_margin(&mut self, margin: impl Into<Margin>) {
        self.inner.set_window_margin(margin.into());
    }

    #[inline]
    pub fn set_button_padding(&mut self, button_padding: impl Into<Vector2<f32>>) {
        self.inner.set_button_padding(button_padding.into().into());
    }

    #[inline]
    pub fn set_menu_margin(&mut self, margin: impl Into<Margin>) {
        self.inner.set_menu_margin(margin.into());
    }

    #[inline]
    pub fn set_indent(&mut self, indent: f32) {
        self.inner.set_indent(indent);
    }

    #[inline]
    pub fn set_interact_size(&mut self, interact_size: impl Into<Vector2<f32>>) {
        self.inner.set_interact_size(interact_size.into().into());
    }

    #[inline]
    pub fn set_slider_width(&mut self, width: f32) {
        self.inner.set_slider_width(width);
    }

    #[inline]
    pub fn set_slider_rail_height(&mut self, height: f32) {
        self.inner.set_slider_rail_height(height);
    }

    #[inline]
    pub fn set_combo_width(&mut self, width: f32) {
        self.inner.set_combo_width(width);
    }

    #[inline]
    pub fn set_text_edit_width(&mut self, width: f32) {
        self.inner.set_text_edit_width(width);
    }

    #[inline]
    pub fn set_icon_width(&mut self, width: f32) {
        self.inner.set_icon_width(width);
    }

    #[inline]
    pub fn set_icon_width_inner(&mut self, width_inner: f32) {
        self.inner.set_icon_width_inner(width_inner);
    }

    #[inline]
    pub fn set_icon_spacing(&mut self, icon_spacing: f32) {
        self.inner.set_icon_spacing(icon_spacing);
    }

    #[inline]
    pub fn set_default_area_size(&mut self, default_area_size: impl Into<Vector2<f32>>) {
        self.inner
            .set_default_area_size(default_area_size.into().into());
    }

    #[inline]
    pub fn set_tooltip_width(&mut self, width: f32) {
        self.inner.set_tooltip_width(width);
    }

    #[inline]
    pub fn set_menu_width(&mut self, width: f32) {
        self.inner.set_menu_width(width);
    }

    #[inline]
    pub fn set_menu_spacing(&mut self, menu_spacing: f32) {
        self.inner.set_menu_spacing(menu_spacing);
    }

    #[inline]
    pub fn set_indent_ends_with_horizontal_line(&mut self, indent_ends_with_horizontal_line: bool) {
        self.inner
            .set_indent_ends_with_horizontal_line(indent_ends_with_horizontal_line);
    }

    #[inline]
    pub fn set_combo_height(&mut self, height: f32) {
        self.inner.set_combo_height(height);
    }
}

impl BunnySpacingMut<'_> {
    #[inline]
    pub fn scroll_style(&self) -> BunnyScrollStyleRef<'_> {
        self.inner.scroll_style()
    }

    #[inline]
    pub fn scroll_style_mut(&mut self) -> BunnyScrollStyleMut<'_> {
        self.inner.scroll_style_mut()
    }

    #[inline]
    pub fn scroll_style_clone(&self) -> ScrollStyle {
        self.inner.scroll_style_clone()
    }

    #[inline]
    pub fn set_scroll_style(&mut self, scroll_style: ScrollStyle) {
        self.inner.set_scroll_style(scroll_style);
    }
}

impl BunnySpacing for BunnySpacingMut<'_> {
    #[inline]
    fn as_ref(&self) -> VRef<'_, SpacingFfiVTable> {
        self.inner.borrow()
    }
}
