use ecolor::Color32;
use vtable::{VRef, VRefMut};

use crate::{
    Shadow,
    paint::{CornerRadius, Stroke, TextOptions},
    style::{HandleShape, NumericColorSpace, Selection, TextCursorStyle, WidgetVisuals},
    vtable::style::visuals::VisualsFfiVTable,
};

pub trait BunnyVisuals {
    fn as_ref(&self) -> VRef<'_, VisualsFfiVTable>;

    #[inline]
    fn dark_mode(&self) -> bool {
        self.as_ref().dark_mode()
    }

    #[inline]
    fn text_options(&self) -> TextOptions {
        self.as_ref().text_options()
    }

    #[inline]
    fn override_text_color(&self) -> Option<Color32> {
        self.as_ref().override_text_color().into_option()
    }

    #[inline]
    fn weak_text_alpha(&self) -> f32 {
        self.as_ref().weak_text_alpha()
    }

    #[inline]
    fn weak_text_color(&self) -> Option<Color32> {
        self.as_ref().weak_text_color().into_option()
    }

    #[inline]
    fn widgets_noninteractive(&self) -> WidgetVisuals {
        self.as_ref().widgets_noninteractive()
    }

    #[inline]
    fn widgets_inactive(&self) -> WidgetVisuals {
        self.as_ref().widgets_inactive()
    }

    #[inline]
    fn widgets_hovered(&self) -> WidgetVisuals {
        self.as_ref().widgets_hovered()
    }

    #[inline]
    fn widgets_active(&self) -> WidgetVisuals {
        self.as_ref().widgets_active()
    }

    #[inline]
    fn widgets_open(&self) -> WidgetVisuals {
        self.as_ref().widgets_open()
    }

    #[inline]
    fn selection(&self) -> Selection {
        self.as_ref().selection()
    }

    #[inline]
    fn hyperlink_color(&self) -> Color32 {
        self.as_ref().hyperlink_color()
    }

    #[inline]
    fn faint_bg_color(&self) -> Color32 {
        self.as_ref().faint_bg_color()
    }

    #[inline]
    fn extreme_bg_color(&self) -> Color32 {
        self.as_ref().extreme_bg_color()
    }

    #[inline]
    fn text_edit_bg_color(&self) -> Option<Color32> {
        self.as_ref().text_edit_bg_color().into_option()
    }

    #[inline]
    fn code_bg_color(&self) -> Color32 {
        self.as_ref().code_bg_color()
    }

    #[inline]
    fn warn_fg_color(&self) -> Color32 {
        self.as_ref().warn_fg_color()
    }

    #[inline]
    fn error_fg_color(&self) -> Color32 {
        self.as_ref().error_fg_color()
    }

    #[inline]
    fn window_corner_radius(&self) -> CornerRadius {
        self.as_ref().window_corner_radius()
    }

    #[inline]
    fn window_shadow(&self) -> Shadow {
        self.as_ref().window_shadow()
    }

    #[inline]
    fn window_fill(&self) -> Color32 {
        self.as_ref().window_fill()
    }

    #[inline]
    fn window_stroke(&self) -> Stroke {
        self.as_ref().window_stroke()
    }

    #[inline]
    fn window_highlight_topmost(&self) -> bool {
        self.as_ref().window_highlight_topmost()
    }

    #[inline]
    fn menu_corner_radius(&self) -> CornerRadius {
        self.as_ref().menu_corner_radius()
    }

    #[inline]
    fn panel_fill(&self) -> Color32 {
        self.as_ref().panel_fill()
    }

    #[inline]
    fn popup_shadow(&self) -> Shadow {
        self.as_ref().popup_shadow()
    }

    #[inline]
    fn resize_corner_size(&self) -> f32 {
        self.as_ref().resize_corner_size()
    }

    #[inline]
    fn text_cursor(&self) -> TextCursorStyle {
        self.as_ref().text_cursor()
    }

    #[inline]
    fn clip_rect_margin(&self) -> f32 {
        self.as_ref().clip_rect_margin()
    }

    #[inline]
    fn button_frame(&self) -> bool {
        self.as_ref().button_frame()
    }

    #[inline]
    fn collapsing_header_frame(&self) -> bool {
        self.as_ref().collapsing_header_frame()
    }

    #[inline]
    fn indent_has_left_vline(&self) -> bool {
        self.as_ref().indent_has_left_vline()
    }

    #[inline]
    fn striped(&self) -> bool {
        self.as_ref().striped()
    }

    #[inline]
    fn slider_trailing_fill(&self) -> bool {
        self.as_ref().slider_trailing_fill()
    }

    #[inline]
    fn handle_shape(&self) -> HandleShape {
        self.as_ref().handle_shape()
    }

    #[inline]
    fn image_loading_spinners(&self) -> bool {
        self.as_ref().image_loading_spinners()
    }

    #[inline]
    fn numeric_color_space(&self) -> NumericColorSpace {
        self.as_ref().numeric_color_space()
    }

    #[inline]
    fn disabled_alpha(&self) -> f32 {
        self.as_ref().disabled_alpha()
    }
}

#[repr(transparent)]
pub struct BunnyVisualsRef<'a> {
    inner: VRef<'a, VisualsFfiVTable>,
}

#[cfg(feature = "manager")]
impl<'a> BunnyVisualsRef<'a> {
    #[inline]
    pub fn new(visuals: &'a egui::style::Visuals) -> Self {
        Self {
            inner: VRef::new(visuals),
        }
    }
}

impl BunnyVisuals for BunnyVisualsRef<'_> {
    fn as_ref(&self) -> VRef<'_, VisualsFfiVTable> {
        self.inner
    }
}

#[repr(transparent)]
pub struct BunnyVisualsMut<'a> {
    inner: VRefMut<'a, VisualsFfiVTable>,
}

#[cfg(feature = "manager")]
impl<'a> BunnyVisualsMut<'a> {
    #[inline]
    pub fn new(visuals: &'a mut egui::style::Visuals) -> Self {
        Self {
            inner: VRefMut::new(visuals),
        }
    }
}

impl BunnyVisualsMut<'_> {
    #[inline]
    pub fn set_dark_mode(&mut self, dark_mode: bool) {
        self.inner.set_dark_mode(dark_mode);
    }

    #[inline]
    pub fn set_text_options(&mut self, text_options: TextOptions) {
        self.inner.set_text_options(text_options);
    }

    #[inline]
    pub fn set_override_text_color(&mut self, color: Option<Color32>) {
        self.inner.set_override_text_color(color.into());
    }

    #[inline]
    pub fn set_weak_text_alpha(&mut self, alpha: f32) {
        self.inner.set_weak_text_alpha(alpha);
    }

    #[inline]
    pub fn set_weak_text_color(&mut self, color: Option<Color32>) {
        self.inner.set_weak_text_color(color.into());
    }

    #[inline]
    pub fn set_widgets_noninteractive(&mut self, noninteractive: WidgetVisuals) {
        self.inner.set_widgets_noninteractive(noninteractive);
    }

    #[inline]
    pub fn set_widgets_inactive(&mut self, inactive: WidgetVisuals) {
        self.inner.set_widgets_inactive(inactive);
    }

    #[inline]
    pub fn set_widgets_hovered(&mut self, hovered: WidgetVisuals) {
        self.inner.set_widgets_hovered(hovered);
    }

    #[inline]
    pub fn set_widgets_active(&mut self, active: WidgetVisuals) {
        self.inner.set_widgets_active(active);
    }

    #[inline]
    pub fn set_widgets_open(&mut self, open: WidgetVisuals) {
        self.inner.set_widgets_open(open);
    }

    #[inline]
    pub fn set_selection(&mut self, selection: Selection) {
        self.inner.set_selection(selection);
    }

    #[inline]
    pub fn set_hyperlink_color(&mut self, color: Color32) {
        self.inner.set_hyperlink_color(color);
    }

    #[inline]
    pub fn set_faint_bg_color(&mut self, color: Color32) {
        self.inner.set_faint_bg_color(color);
    }

    #[inline]
    pub fn set_extreme_bg_color(&mut self, color: Color32) {
        self.inner.set_extreme_bg_color(color);
    }

    #[inline]
    pub fn set_text_edit_bg_color(&mut self, color: Option<Color32>) {
        self.inner.set_text_edit_bg_color(color.into());
    }

    #[inline]
    pub fn set_code_bg_color(&mut self, color: Color32) {
        self.inner.set_code_bg_color(color);
    }

    #[inline]
    pub fn set_warn_fg_color(&mut self, color: Color32) {
        self.inner.set_warn_fg_color(color);
    }

    #[inline]
    pub fn set_error_fg_color(&mut self, color: Color32) {
        self.inner.set_error_fg_color(color);
    }

    #[inline]
    pub fn set_window_corner_radius(&mut self, corner_radius: impl Into<CornerRadius>) {
        self.inner.set_window_corner_radius(corner_radius.into());
    }

    #[inline]
    pub fn set_window_shadow(&mut self, shadow: Shadow) {
        self.inner.set_window_shadow(shadow);
    }

    #[inline]
    pub fn set_window_fill(&mut self, color: Color32) {
        self.inner.set_window_fill(color);
    }

    #[inline]
    pub fn set_window_stroke(&mut self, stroke: impl Into<Stroke>) {
        self.inner.set_window_stroke(stroke.into());
    }

    #[inline]
    pub fn set_window_highlight_topmost(&mut self, highlight: bool) {
        self.inner.set_window_highlight_topmost(highlight);
    }

    #[inline]
    pub fn set_menu_corner_radius(&mut self, corner_radius: impl Into<CornerRadius>) {
        self.inner.set_menu_corner_radius(corner_radius.into());
    }

    #[inline]
    pub fn set_panel_fill(&mut self, color: Color32) {
        self.inner.set_panel_fill(color);
    }

    #[inline]
    pub fn set_popup_shadow(&mut self, shadow: Shadow) {
        self.inner.set_popup_shadow(shadow);
    }

    #[inline]
    pub fn set_resize_corner_size(&mut self, size: f32) {
        self.inner.set_resize_corner_size(size);
    }

    #[inline]
    pub fn set_text_cursor(&mut self, text_cursor: TextCursorStyle) {
        self.inner.set_text_cursor(text_cursor);
    }

    #[inline]
    pub fn set_clip_rect_margin(&mut self, margin: f32) {
        self.inner.set_clip_rect_margin(margin);
    }

    #[inline]
    pub fn set_button_frame(&mut self, button_frame: bool) {
        self.inner.set_button_frame(button_frame);
    }

    #[inline]
    pub fn set_collapsing_header_frame(&mut self, collapsing_header_frame: bool) {
        self.inner
            .set_collapsing_header_frame(collapsing_header_frame);
    }

    #[inline]
    pub fn set_indent_has_left_vline(&mut self, indent_has_left_vline: bool) {
        self.inner.set_indent_has_left_vline(indent_has_left_vline);
    }

    #[inline]
    pub fn set_striped(&mut self, striped: bool) {
        self.inner.set_striped(striped);
    }

    #[inline]
    pub fn set_slider_trailing_fill(&mut self, slider_trailing_fill: bool) {
        self.inner.set_slider_trailing_fill(slider_trailing_fill);
    }

    #[inline]
    pub fn set_handle_shape(&mut self, handle_shape: HandleShape) {
        self.inner.set_handle_shape(handle_shape);
    }

    #[inline]
    pub fn set_image_loading_spinners(&mut self, image_loading_spinners: bool) {
        self.inner
            .set_image_loading_spinners(image_loading_spinners);
    }

    #[inline]
    pub fn set_numeric_color_space(&mut self, numeric_color_space: NumericColorSpace) {
        self.inner.set_numeric_color_space(numeric_color_space);
    }

    #[inline]
    pub fn set_disabled_alpha(&mut self, alpha: f32) {
        self.inner.set_disabled_alpha(alpha);
    }
}

impl BunnyVisuals for BunnyVisualsMut<'_> {
    fn as_ref(&self) -> VRef<'_, VisualsFfiVTable> {
        self.inner.borrow()
    }
}
