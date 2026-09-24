use abi_stable::std_types::ROption;
use ecolor::Color32;
use vtable::vtable;

use crate::{
    HandleShape, NumericColorSpace, Selection, Shadow, TextCursorStyle, WidgetVisuals,
    paint::{CornerRadius, Stroke, TextOptions},
};

#[vtable]
#[repr(C)]
pub struct VisualsFfiVTable {
    dark_mode: fn(VRef<VisualsFfiVTable>) -> bool,
    text_options: fn(VRef<VisualsFfiVTable>) -> TextOptions,
    override_text_color: fn(VRef<VisualsFfiVTable>) -> ROption<Color32>,
    weak_text_alpha: fn(VRef<VisualsFfiVTable>) -> f32,
    weak_text_color: fn(VRef<VisualsFfiVTable>) -> ROption<Color32>,
    widgets_noninteractive: fn(VRef<VisualsFfiVTable>) -> WidgetVisuals,
    widgets_inactive: fn(VRef<VisualsFfiVTable>) -> WidgetVisuals,
    widgets_hovered: fn(VRef<VisualsFfiVTable>) -> WidgetVisuals,
    widgets_active: fn(VRef<VisualsFfiVTable>) -> WidgetVisuals,
    widgets_open: fn(VRef<VisualsFfiVTable>) -> WidgetVisuals,
    selection: fn(VRef<VisualsFfiVTable>) -> Selection,
    hyperlink_color: fn(VRef<VisualsFfiVTable>) -> Color32,
    faint_bg_color: fn(VRef<VisualsFfiVTable>) -> Color32,
    extreme_bg_color: fn(VRef<VisualsFfiVTable>) -> Color32,
    text_edit_bg_color: fn(VRef<VisualsFfiVTable>) -> ROption<Color32>,
    code_bg_color: fn(VRef<VisualsFfiVTable>) -> Color32,
    warn_fg_color: fn(VRef<VisualsFfiVTable>) -> Color32,
    error_fg_color: fn(VRef<VisualsFfiVTable>) -> Color32,
    window_corner_radius: fn(VRef<VisualsFfiVTable>) -> CornerRadius,
    window_shadow: fn(VRef<VisualsFfiVTable>) -> Shadow,
    window_fill: fn(VRef<VisualsFfiVTable>) -> Color32,
    window_stroke: fn(VRef<VisualsFfiVTable>) -> Stroke,
    window_highlight_topmost: fn(VRef<VisualsFfiVTable>) -> bool,
    menu_corner_radius: fn(VRef<VisualsFfiVTable>) -> CornerRadius,
    panel_fill: fn(VRef<VisualsFfiVTable>) -> Color32,
    popup_shadow: fn(VRef<VisualsFfiVTable>) -> Shadow,
    resize_corner_size: fn(VRef<VisualsFfiVTable>) -> f32,
    text_cursor: fn(VRef<VisualsFfiVTable>) -> TextCursorStyle,
    clip_rect_margin: fn(VRef<VisualsFfiVTable>) -> f32,
    button_frame: fn(VRef<VisualsFfiVTable>) -> bool,
    collapsing_header_frame: fn(VRef<VisualsFfiVTable>) -> bool,
    indent_has_left_vline: fn(VRef<VisualsFfiVTable>) -> bool,
    striped: fn(VRef<VisualsFfiVTable>) -> bool,
    slider_trailing_fill: fn(VRef<VisualsFfiVTable>) -> bool,
    handle_shape: fn(VRef<VisualsFfiVTable>) -> HandleShape,
    // interact_cursor
    image_loading_spinners: fn(VRef<VisualsFfiVTable>) -> bool,
    numeric_color_space: fn(VRef<VisualsFfiVTable>) -> NumericColorSpace,
    disabled_alpha: fn(VRef<VisualsFfiVTable>) -> f32,

    set_dark_mode: fn(VRefMut<VisualsFfiVTable>, dark_mode: bool),
    set_text_options: fn(VRefMut<VisualsFfiVTable>, text_options: TextOptions),
    set_override_text_color: fn(VRefMut<VisualsFfiVTable>, color: ROption<Color32>),
    set_weak_text_alpha: fn(VRefMut<VisualsFfiVTable>, alpha: f32),
    set_weak_text_color: fn(VRefMut<VisualsFfiVTable>, color: ROption<Color32>),
    set_widgets_noninteractive: fn(VRefMut<VisualsFfiVTable>, noninteractive: WidgetVisuals),
    set_widgets_inactive: fn(VRefMut<VisualsFfiVTable>, inactive: WidgetVisuals),
    set_widgets_hovered: fn(VRefMut<VisualsFfiVTable>, hovered: WidgetVisuals),
    set_widgets_active: fn(VRefMut<VisualsFfiVTable>, active: WidgetVisuals),
    set_widgets_open: fn(VRefMut<VisualsFfiVTable>, open: WidgetVisuals),
    set_selection: fn(VRefMut<VisualsFfiVTable>, selection: Selection),
    set_hyperlink_color: fn(VRefMut<VisualsFfiVTable>, color: Color32),
    set_faint_bg_color: fn(VRefMut<VisualsFfiVTable>, color: Color32),
    set_extreme_bg_color: fn(VRefMut<VisualsFfiVTable>, color: Color32),
    set_text_edit_bg_color: fn(VRefMut<VisualsFfiVTable>, color: ROption<Color32>),
    set_code_bg_color: fn(VRefMut<VisualsFfiVTable>, color: Color32),
    set_warn_fg_color: fn(VRefMut<VisualsFfiVTable>, color: Color32),
    set_error_fg_color: fn(VRefMut<VisualsFfiVTable>, color: Color32),
    set_window_corner_radius: fn(VRefMut<VisualsFfiVTable>, corner_radius: CornerRadius),
    set_window_shadow: fn(VRefMut<VisualsFfiVTable>, shadow: Shadow),
    set_window_fill: fn(VRefMut<VisualsFfiVTable>, color: Color32),
    set_window_stroke: fn(VRefMut<VisualsFfiVTable>, stroke: Stroke),
    set_window_highlight_topmost: fn(VRefMut<VisualsFfiVTable>, highlight: bool),
    set_menu_corner_radius: fn(VRefMut<VisualsFfiVTable>, corner_radius: CornerRadius),
    set_panel_fill: fn(VRefMut<VisualsFfiVTable>, color: Color32),
    set_popup_shadow: fn(VRefMut<VisualsFfiVTable>, shadow: Shadow),
    set_resize_corner_size: fn(VRefMut<VisualsFfiVTable>, corner_size: f32),
    set_text_cursor: fn(VRefMut<VisualsFfiVTable>, text_cursor: TextCursorStyle),
    set_clip_rect_margin: fn(VRefMut<VisualsFfiVTable>, margin: f32),
    set_button_frame: fn(VRefMut<VisualsFfiVTable>, button_frame: bool),
    set_collapsing_header_frame: fn(VRefMut<VisualsFfiVTable>, collapsing_header_frame: bool),
    set_indent_has_left_vline: fn(VRefMut<VisualsFfiVTable>, indent_has_left_vline: bool),
    set_striped: fn(VRefMut<VisualsFfiVTable>, striped: bool),
    set_slider_trailing_fill: fn(VRefMut<VisualsFfiVTable>, slider_trailing_fill: bool),
    set_handle_shape: fn(VRefMut<VisualsFfiVTable>, handle_shape: HandleShape),
    set_image_loading_spinners: fn(VRefMut<VisualsFfiVTable>, image_loading_spinners: bool),
    set_numeric_color_space: fn(VRefMut<VisualsFfiVTable>, numeric_color_space: NumericColorSpace),
    set_disabled_alpha: fn(VRefMut<VisualsFfiVTable>, alpha: f32),
}

#[cfg(feature = "manager")]
impl VisualsFfi for egui::style::Visuals {
    #[inline]
    fn dark_mode(&self) -> bool {
        self.dark_mode
    }

    #[inline]
    fn text_options(&self) -> TextOptions {
        self.text_options.into()
    }

    #[inline]
    fn override_text_color(&self) -> ROption<Color32> {
        self.override_text_color.into()
    }

    #[inline]
    fn weak_text_alpha(&self) -> f32 {
        self.weak_text_alpha
    }

    #[inline]
    fn weak_text_color(&self) -> ROption<Color32> {
        self.weak_text_color.into()
    }

    #[inline]
    fn widgets_noninteractive(&self) -> WidgetVisuals {
        self.widgets.noninteractive.into()
    }

    #[inline]
    fn widgets_inactive(&self) -> WidgetVisuals {
        self.widgets.inactive.into()
    }

    #[inline]
    fn widgets_hovered(&self) -> WidgetVisuals {
        self.widgets.hovered.into()
    }

    #[inline]
    fn widgets_active(&self) -> WidgetVisuals {
        self.widgets.active.into()
    }

    #[inline]
    fn widgets_open(&self) -> WidgetVisuals {
        self.widgets.open.into()
    }

    #[inline]
    fn selection(&self) -> Selection {
        self.selection.into()
    }

    #[inline]
    fn hyperlink_color(&self) -> Color32 {
        self.hyperlink_color
    }

    #[inline]
    fn faint_bg_color(&self) -> Color32 {
        self.faint_bg_color
    }

    #[inline]
    fn extreme_bg_color(&self) -> Color32 {
        self.extreme_bg_color
    }

    #[inline]
    fn text_edit_bg_color(&self) -> ROption<Color32> {
        self.text_edit_bg_color.into()
    }

    #[inline]
    fn code_bg_color(&self) -> Color32 {
        self.code_bg_color
    }

    #[inline]
    fn warn_fg_color(&self) -> Color32 {
        self.warn_fg_color
    }

    #[inline]
    fn error_fg_color(&self) -> Color32 {
        self.error_fg_color
    }

    #[inline]
    fn window_corner_radius(&self) -> CornerRadius {
        self.window_corner_radius.into()
    }

    #[inline]
    fn window_shadow(&self) -> Shadow {
        self.window_shadow.into()
    }

    #[inline]
    fn window_fill(&self) -> Color32 {
        self.window_fill
    }

    #[inline]
    fn window_stroke(&self) -> Stroke {
        self.window_stroke.into()
    }

    #[inline]
    fn window_highlight_topmost(&self) -> bool {
        self.window_highlight_topmost
    }

    #[inline]
    fn menu_corner_radius(&self) -> CornerRadius {
        self.menu_corner_radius.into()
    }

    #[inline]
    fn panel_fill(&self) -> Color32 {
        self.panel_fill
    }

    #[inline]
    fn popup_shadow(&self) -> Shadow {
        self.popup_shadow.into()
    }

    #[inline]
    fn resize_corner_size(&self) -> f32 {
        self.resize_corner_size
    }

    #[inline]
    fn text_cursor(&self) -> TextCursorStyle {
        self.text_cursor.clone().into()
    }

    #[inline]
    fn clip_rect_margin(&self) -> f32 {
        self.clip_rect_margin
    }

    #[inline]
    fn button_frame(&self) -> bool {
        self.button_frame
    }

    #[inline]
    fn collapsing_header_frame(&self) -> bool {
        self.collapsing_header_frame
    }

    #[inline]
    fn indent_has_left_vline(&self) -> bool {
        self.indent_has_left_vline
    }

    #[inline]
    fn striped(&self) -> bool {
        self.striped
    }

    #[inline]
    fn slider_trailing_fill(&self) -> bool {
        self.slider_trailing_fill
    }

    #[inline]
    fn handle_shape(&self) -> HandleShape {
        self.handle_shape.into()
    }

    #[inline]
    fn image_loading_spinners(&self) -> bool {
        self.image_loading_spinners
    }

    #[inline]
    fn numeric_color_space(&self) -> NumericColorSpace {
        self.numeric_color_space.into()
    }

    #[inline]
    fn disabled_alpha(&self) -> f32 {
        self.disabled_alpha
    }

    #[inline]
    fn set_dark_mode(&mut self, dark_mode: bool) {
        self.dark_mode = dark_mode
    }

    #[inline]
    fn set_text_options(&mut self, text_options: TextOptions) {
        self.text_options = text_options.into()
    }

    #[inline]
    fn set_override_text_color(&mut self, color: ROption<Color32>) {
        self.override_text_color = color.into()
    }

    #[inline]
    fn set_weak_text_alpha(&mut self, alpha: f32) {
        self.weak_text_alpha = alpha
    }

    #[inline]
    fn set_weak_text_color(&mut self, color: ROption<Color32>) {
        self.weak_text_color = color.into()
    }

    #[inline]
    fn set_widgets_noninteractive(&mut self, noninteractive: WidgetVisuals) {
        self.widgets.noninteractive = noninteractive.into()
    }

    #[inline]
    fn set_widgets_inactive(&mut self, inactive: WidgetVisuals) {
        self.widgets.inactive = inactive.into()
    }

    #[inline]
    fn set_widgets_hovered(&mut self, hovered: WidgetVisuals) {
        self.widgets.hovered = hovered.into()
    }

    #[inline]
    fn set_widgets_active(&mut self, active: WidgetVisuals) {
        self.widgets.active = active.into()
    }

    #[inline]
    fn set_widgets_open(&mut self, open: WidgetVisuals) {
        self.widgets.open = open.into()
    }

    #[inline]
    fn set_selection(&mut self, selection: Selection) {
        self.selection = selection.into()
    }

    #[inline]
    fn set_hyperlink_color(&mut self, color: Color32) {
        self.hyperlink_color = color
    }

    #[inline]
    fn set_faint_bg_color(&mut self, color: Color32) {
        self.faint_bg_color = color
    }

    #[inline]
    fn set_extreme_bg_color(&mut self, color: Color32) {
        self.extreme_bg_color = color
    }

    #[inline]
    fn set_text_edit_bg_color(&mut self, color: ROption<Color32>) {
        self.text_edit_bg_color = color.into()
    }

    #[inline]
    fn set_code_bg_color(&mut self, color: Color32) {
        self.code_bg_color = color
    }

    #[inline]
    fn set_warn_fg_color(&mut self, color: Color32) {
        self.warn_fg_color = color
    }

    #[inline]
    fn set_error_fg_color(&mut self, color: Color32) {
        self.error_fg_color = color
    }

    #[inline]
    fn set_window_corner_radius(&mut self, corner_radius: CornerRadius) {
        self.window_corner_radius = corner_radius.into()
    }

    #[inline]
    fn set_window_shadow(&mut self, shadow: Shadow) {
        self.window_shadow = shadow.into()
    }

    #[inline]
    fn set_window_fill(&mut self, color: Color32) {
        self.window_fill = color
    }

    #[inline]
    fn set_window_stroke(&mut self, stroke: Stroke) {
        self.window_stroke = stroke.into()
    }

    #[inline]
    fn set_window_highlight_topmost(&mut self, highlight: bool) {
        self.window_highlight_topmost = highlight
    }

    #[inline]
    fn set_menu_corner_radius(&mut self, corner_radius: CornerRadius) {
        self.menu_corner_radius = corner_radius.into()
    }

    #[inline]
    fn set_panel_fill(&mut self, color: Color32) {
        self.panel_fill = color
    }

    #[inline]
    fn set_popup_shadow(&mut self, shadow: Shadow) {
        self.popup_shadow = shadow.into()
    }

    #[inline]
    fn set_resize_corner_size(&mut self, corner_size: f32) {
        self.resize_corner_size = corner_size
    }

    #[inline]
    fn set_text_cursor(&mut self, text_cursor: TextCursorStyle) {
        self.text_cursor = text_cursor.into()
    }

    #[inline]
    fn set_clip_rect_margin(&mut self, margin: f32) {
        self.clip_rect_margin = margin
    }

    #[inline]
    fn set_button_frame(&mut self, button_frame: bool) {
        self.button_frame = button_frame
    }

    #[inline]
    fn set_collapsing_header_frame(&mut self, collapsing_header_frame: bool) {
        self.collapsing_header_frame = collapsing_header_frame
    }

    #[inline]
    fn set_indent_has_left_vline(&mut self, indent_has_left_vline: bool) {
        self.indent_has_left_vline = indent_has_left_vline
    }

    #[inline]
    fn set_striped(&mut self, striped: bool) {
        self.striped = striped
    }

    #[inline]
    fn set_slider_trailing_fill(&mut self, slider_trailing_fill: bool) {
        self.slider_trailing_fill = slider_trailing_fill
    }

    #[inline]
    fn set_handle_shape(&mut self, handle_shape: HandleShape) {
        self.handle_shape = handle_shape.into()
    }

    #[inline]
    fn set_image_loading_spinners(&mut self, image_loading_spinners: bool) {
        self.image_loading_spinners = image_loading_spinners
    }

    #[inline]
    fn set_numeric_color_space(&mut self, numeric_color_space: NumericColorSpace) {
        self.numeric_color_space = numeric_color_space.into()
    }

    #[inline]
    fn set_disabled_alpha(&mut self, alpha: f32) {
        self.disabled_alpha = alpha
    }
}

#[cfg(feature = "manager")]
VisualsFfiVTable_static!(static VISUALSFFI_VT for egui::style::Visuals);
