use std::{hash::Hash, mem::MaybeUninit};

use abi_stable::std_types::{RStr, RString, Tuple2};
use ecolor::Hsva;
use egui::{Color32, Id, Pos2, Rangef, Rect, Sense, Vec2};
use emath::TSTransform;
use mint::Vector2;
use vtable::VRefMut;

use crate::{
    Align, ImageSource, LayerId, Layout, RichText, SizeHint, UiBuilder, WidgetText,
    closure::{
        InputStateClosure, PanelAnimatedBetweenClosure, PluginClosure, ScrollAreaRowsClosure,
    },
    containers::{
        Area, BunnyCollapsingResponse, BunnyModalResponse, BunnyScrollAreaOutput, CentralPanel,
        CollapsingHeader, ComboBox, Frame, Grid, Modal, Panel, Popup, ScrollArea, Sides, Window,
    },
    galley::BunnyGalley,
    id::hash_id_salt,
    input::BunnyInputState,
    load::TexturePoll,
    paint::{
        text::{
            fonts::FontId,
            text_layout_types::{LayoutJob, TextWrapMode},
        },
        textures::TextureOptions,
    },
    painter::{BunnyPainter, BunnyPainterRef},
    response::{BunnyInnerResponse, BunnyResponse},
    style::{
        BunnyInteractionMut, BunnyInteractionRef, BunnySpacingMut, BunnySpacingRef, BunnyStyleMut,
        BunnyStyleRef, BunnyVisualsMut, BunnyVisualsRef, ScrollAnimation, Style, TextStyle,
    },
    vtable::ui::UiFfiVTable,
    widgets::{Widget, text_edit::bunny_string::BunnyString},
};

#[repr(transparent)]
pub struct BunnyUi<'a> {
    inner: VRefMut<'a, UiFfiVTable>,
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn new(ui: &'a mut egui::Ui) -> Self {
        Self {
            inner: VRefMut::new(ui),
        }
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn is_sizing_pass(&self) -> bool {
        self.inner.is_sizing_pass()
    }

    #[inline]
    pub fn id(&self) -> Id {
        self.inner.id()
    }

    #[inline]
    pub fn unique_id(&self) -> Id {
        self.inner.unique_id()
    }

    /// Immutable style reference for this BunnyUi and its children.
    #[inline]
    pub fn style(&self) -> BunnyStyleRef<'_> {
        self.inner.style()
    }

    /// Mutable style reference for this BunnyUi and its children.
    #[inline]
    pub fn style_mut(&mut self) -> BunnyStyleMut<'_> {
        self.inner.style_mut()
    }

    /// Get a full clone of this BunnyUi's style.
    /// This can be useful if you're going to change many style fields and set them at once with set_style()
    #[inline]
    pub fn style_clone(&self) -> Style {
        self.inner.style_clone()
    }

    /// Set the style of this BunnyUi and its children.
    #[inline]
    pub fn set_style(&mut self, style: &Style) {
        self.inner.set_style(style);
    }

    #[inline]
    pub fn reset_style(&mut self) {
        self.inner.reset_style();
    }

    #[inline]
    pub fn spacing(&self) -> BunnySpacingRef<'_> {
        self.inner.spacing()
    }

    #[inline]
    pub fn spacing_mut(&mut self) -> BunnySpacingMut<'_> {
        self.inner.spacing_mut()
    }

    #[inline]
    pub fn interaction(&self) -> BunnyInteractionRef<'_> {
        self.inner.interaction()
    }

    #[inline]
    pub fn interaction_mut(&mut self) -> BunnyInteractionMut<'_> {
        self.inner.interaction_mut()
    }

    #[inline]
    pub fn visuals(&self) -> BunnyVisualsRef<'_> {
        self.inner.visuals()
    }

    #[inline]
    pub fn visuals_mut(&mut self) -> BunnyVisualsMut<'_> {
        self.inner.visuals_mut()
    }

    #[inline]
    pub fn is_tooltip(&self) -> bool {
        self.inner.is_tooltip()
    }

    #[inline]
    pub fn painter(&self) -> BunnyPainterRef<'_> {
        self.inner.painter()
    }

    #[inline]
    pub fn pixels_per_point(&self) -> f32 {
        self.inner.pixels_per_point()
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    #[inline]
    pub fn disable(&mut self) {
        self.inner.disable();
    }

    #[inline]
    pub fn is_visible(&self) -> bool {
        self.inner.is_visible()
    }

    #[inline]
    pub fn set_invisible(&mut self) {
        self.inner.set_invisible();
    }

    #[inline]
    pub fn set_opacity(&mut self, opacity: f32) {
        self.inner.set_opacity(opacity);
    }

    #[inline]
    pub fn multiply_opacity(&mut self, opacity: f32) {
        self.inner.multiply_opacity(opacity);
    }

    #[inline]
    pub fn opacity(&self) -> f32 {
        self.inner.opacity()
    }

    #[inline]
    pub fn layout(&self) -> Layout {
        self.inner.layout()
    }

    #[inline]
    pub fn wrap_mode(&self) -> TextWrapMode {
        self.inner.wrap_mode()
    }

    #[inline]
    pub fn text_valign(&self) -> Align {
        self.inner.text_valign()
    }

    #[inline]
    pub fn painter_at(&self, rect: Rect) -> BunnyPainter {
        self.inner.painter_at(rect)
    }

    #[inline]
    pub fn layer_id(&self) -> LayerId {
        self.inner.layer_id()
    }

    #[inline]
    pub fn text_style_height(&self, style: TextStyle) -> f32 {
        self.inner.text_style_height(style)
    }

    #[inline]
    pub fn clip_rect(&self) -> Rect {
        self.inner.clip_rect()
    }

    #[inline]
    pub fn shrink_clip_rect(&mut self, new_clip_rect: Rect) {
        self.inner.shrink_clip_rect(new_clip_rect);
    }

    #[inline]
    pub fn set_clip_rect(&mut self, clip_rect: Rect) {
        self.inner.set_clip_rect(clip_rect);
    }

    #[inline]
    pub fn is_rect_visible(&self, rect: Rect) -> bool {
        self.inner.is_rect_visible(rect)
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn min_rect(&self) -> Rect {
        self.inner.min_rect()
    }

    #[inline]
    pub fn min_size(&self) -> Vec2 {
        self.inner.min_size()
    }

    #[inline]
    pub fn max_rect(&self) -> Rect {
        self.inner.max_rect()
    }

    #[inline]
    pub fn set_max_size(&mut self, size: impl Into<Vector2<f32>>) {
        self.inner.set_max_size(size.into().into());
    }

    #[inline]
    pub fn set_max_width(&mut self, width: f32) {
        self.inner.set_max_width(width);
    }

    #[inline]
    pub fn set_max_height(&mut self, height: f32) {
        self.inner.set_max_height(height);
    }

    #[inline]
    pub fn set_min_size(&mut self, size: impl Into<Vector2<f32>>) {
        self.inner.set_min_size(size.into().into());
    }

    #[inline]
    pub fn set_min_width(&mut self, width: f32) {
        self.inner.set_min_width(width);
    }

    #[inline]
    pub fn set_min_height(&mut self, height: f32) {
        self.inner.set_min_height(height);
    }

    #[inline]
    pub fn take_available_space(&mut self) {
        self.inner.take_available_space();
    }

    #[inline]
    pub fn take_available_width(&mut self) {
        self.inner.take_available_width();
    }

    #[inline]
    pub fn take_available_height(&mut self) {
        self.inner.take_available_height();
    }

    #[inline]
    pub fn shrink_width_to_current(&mut self) {
        self.inner.shrink_width_to_current();
    }

    #[inline]
    pub fn expand_to_include_rect(&mut self, rect: Rect) {
        self.inner.expand_to_include_rect(rect);
    }

    #[inline]
    pub fn set_width_range(&mut self, width: Rangef) {
        self.inner.set_width_range(width);
    }

    #[inline]
    pub fn set_height_range(&mut self, height: Rangef) {
        self.inner.set_height_range(height);
    }

    #[inline]
    pub fn set_width(&mut self, width: f32) {
        self.inner.set_width(width);
    }

    #[inline]
    pub fn set_height(&mut self, height: f32) {
        self.inner.set_height(height);
    }

    #[inline]
    pub fn expand_to_include_x(&mut self, x: f32) {
        self.inner.expand_to_include_x(x);
    }

    #[inline]
    pub fn expand_to_include_y(&mut self, y: f32) {
        self.inner.expand_to_include_y(y);
    }

    #[inline]
    pub fn available_size(&self) -> Vec2 {
        self.inner.available_size()
    }

    #[inline]
    pub fn available_width(&self) -> f32 {
        self.inner.available_width()
    }

    #[inline]
    pub fn available_height(&self) -> f32 {
        self.inner.available_height()
    }

    #[inline]
    pub fn available_size_before_wrap(&self) -> Vec2 {
        self.inner.available_size_before_wrap()
    }

    #[inline]
    pub fn available_rect_before_wrap(&self) -> Rect {
        self.inner.available_rect_before_wrap()
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn make_persistent_id(&self, id_salt: impl Hash) -> Id {
        let hash = hash_id_salt(id_salt);
        self.inner.make_persistent_id(hash)
    }

    #[inline]
    pub fn next_auto_id(&self) -> Id {
        self.inner.next_auto_id()
    }

    #[inline]
    pub fn auto_id_with(&self, id_salt: impl Hash) -> Id {
        let hash = hash_id_salt(id_salt);
        self.inner.auto_id_with(hash)
    }

    #[inline]
    pub fn skip_ahead_auto_ids(&mut self, count: usize) {
        self.inner.skip_ahead_auto_ids(count);
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn interact(&self, rect: Rect, id: impl Into<Id>, sense: Sense) -> BunnyResponse {
        self.inner.interact(rect, id.into(), sense)
    }

    #[inline]
    pub fn response(&self) -> BunnyResponse {
        self.inner.response()
    }

    #[inline]
    pub fn rect_contains_pointer(&self, rect: Rect) -> bool {
        self.inner.rect_contains_pointer(rect)
    }

    #[inline]
    pub fn ui_contains_pointer(&self) -> bool {
        self.inner.ui_contains_pointer()
    }

    #[inline]
    pub fn close(&self) {
        self.inner.close();
    }

    #[inline]
    pub fn should_close(&self) -> bool {
        self.inner.should_close()
    }

    #[inline]
    pub fn will_parent_close(&self) -> bool {
        self.inner.will_parent_close()
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn allocate_response(
        &mut self,
        desired_size: impl Into<Vector2<f32>>,
        sense: Sense,
    ) -> BunnyResponse {
        self.inner
            .allocate_response(desired_size.into().into(), sense)
    }

    #[inline]
    pub fn allocate_exact_size(
        &mut self,
        desired_size: impl Into<Vector2<f32>>,
        sense: Sense,
    ) -> (Rect, BunnyResponse) {
        let Tuple2(rect, res) = self
            .inner
            .allocate_exact_size(desired_size.into().into(), sense);
        (rect, res)
    }

    #[inline]
    pub fn allocate_at_least(
        &mut self,
        desired_size: impl Into<Vector2<f32>>,
        sense: Sense,
    ) -> (Rect, BunnyResponse) {
        let Tuple2(rect, res) = self
            .inner
            .allocate_at_least(desired_size.into().into(), sense);
        (rect, res)
    }

    #[inline]
    pub fn allocate_space(&mut self, desired_size: impl Into<Vector2<f32>>) -> (Id, Rect) {
        let Tuple2(id, rect) = self.inner.allocate_space(desired_size.into().into());
        (id, rect)
    }

    #[inline]
    pub fn allocate_rect(&mut self, rect: Rect, sense: Sense) -> BunnyResponse {
        self.inner.allocate_rect(rect, sense)
    }

    #[inline]
    pub fn advance_cursor_after_rect(&mut self, rect: Rect) -> Id {
        self.inner.advance_cursor_after_rect(rect)
    }

    #[inline]
    pub fn cursor(&self) -> Rect {
        self.inner.cursor()
    }

    #[inline]
    pub fn next_widget_position(&self) -> Pos2 {
        self.inner.next_widget_position()
    }

    pub fn allocate_ui<R>(
        &mut self,
        desired_size: impl Into<Vector2<f32>>,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.allocate_ui(desired_size.into().into(), closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn allocate_ui_with_layout<R>(
        &mut self,
        desired_size: impl Into<Vector2<f32>>,
        layout: Layout,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response =
            self.inner
                .allocate_ui_with_layout(desired_size.into().into(), layout, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    #[inline]
    pub fn allocate_painter(
        &mut self,
        desired_size: impl Into<Vector2<f32>>,
        sense: Sense,
    ) -> (BunnyResponse, BunnyPainter) {
        let Tuple2(response, painter) = self
            .inner
            .allocate_painter(desired_size.into().into(), sense);
        (response, painter)
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn scroll_to_rect(&self, rect: Rect, align: Option<Align>) {
        self.inner.scroll_to_rect(rect, align.into());
    }

    #[inline]
    pub fn scroll_to_rect_animation(
        &self,
        rect: Rect,
        align: Option<Align>,
        animation: ScrollAnimation,
    ) {
        self.inner
            .scroll_to_rect_animation(rect, align.into(), animation);
    }

    #[inline]
    pub fn scroll_to_cursor(&self, align: Option<Align>) {
        self.inner.scroll_to_cursor(align.into());
    }

    #[inline]
    pub fn scroll_to_cursor_animation(&self, align: Option<Align>, animation: ScrollAnimation) {
        self.inner
            .scroll_to_cursor_animation(align.into(), animation);
    }

    #[inline]
    pub fn scroll_with_delta(&self, delta: impl Into<Vector2<f32>>) {
        self.inner.scroll_with_delta(delta.into().into());
    }

    #[inline]
    pub fn scroll_with_delta_animation(
        &self,
        delta: impl Into<Vector2<f32>>,
        animation: ScrollAnimation,
    ) {
        self.inner
            .scroll_with_delta_animation(delta.into().into(), animation);
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn add<'w>(&mut self, widget: impl Into<Widget<'w>>) -> BunnyResponse {
        self.inner.add(widget.into())
    }

    #[inline]
    pub fn add_sized<'w>(
        &mut self,
        max_size: impl Into<Vector2<f32>>,
        widget: impl Into<Widget<'w>>,
    ) -> BunnyResponse {
        self.inner.add_sized(max_size.into().into(), widget.into())
    }

    #[inline]
    pub fn place<'w>(&mut self, max_rect: Rect, widget: impl Into<Widget<'w>>) -> BunnyResponse {
        self.inner.place(max_rect, widget.into())
    }

    #[inline]
    pub fn put<'w>(&mut self, max_rect: Rect, widget: impl Into<Widget<'w>>) -> BunnyResponse {
        self.inner.put(max_rect, widget.into())
    }

    #[inline]
    pub fn add_enabled<'w>(
        &mut self,
        enabled: bool,
        widget: impl Into<Widget<'w>>,
    ) -> BunnyResponse {
        self.inner.add_enabled(enabled, widget.into())
    }

    pub fn add_enabled_ui<R>(
        &mut self,
        enabled: bool,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.add_enabled_ui(enabled, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    #[inline]
    pub fn add_visible<'w>(
        &mut self,
        visible: bool,
        widget: impl Into<Widget<'w>>,
    ) -> BunnyResponse {
        self.inner.add_visible(visible, widget.into())
    }

    #[inline]
    pub fn add_space(&mut self, amount: f32) {
        self.inner.add_space(amount);
    }

    #[inline]
    pub fn label(&mut self, text: impl Into<WidgetText>) -> BunnyResponse {
        self.inner.label(text.into())
    }

    #[inline]
    pub fn colored_label(&mut self, color: Color32, text: impl Into<RichText>) -> BunnyResponse {
        self.inner.colored_label(color, text.into())
    }

    #[inline]
    pub fn heading(&mut self, text: impl Into<RichText>) -> BunnyResponse {
        self.inner.heading(text.into())
    }

    #[inline]
    pub fn monospace(&mut self, text: impl Into<RichText>) -> BunnyResponse {
        self.inner.monospace(text.into())
    }

    #[inline]
    pub fn code(&mut self, text: impl Into<RichText>) -> BunnyResponse {
        self.inner.code(text.into())
    }

    #[inline]
    pub fn small(&mut self, text: impl Into<RichText>) -> BunnyResponse {
        self.inner.small(text.into())
    }

    #[inline]
    pub fn strong(&mut self, text: impl Into<RichText>) -> BunnyResponse {
        self.inner.strong(text.into())
    }

    #[inline]
    pub fn weak(&mut self, text: impl Into<RichText>) -> BunnyResponse {
        self.inner.weak(text.into())
    }

    #[inline]
    pub fn link(&mut self, text: impl Into<WidgetText>) -> BunnyResponse {
        self.inner.link(text.into())
    }

    #[inline]
    pub fn text_edit_singleline(&mut self, text: &mut BunnyString) -> BunnyResponse {
        self.inner.text_edit_singleline(text)
    }

    #[inline]
    pub fn text_edit_multiline(&mut self, text: &mut BunnyString) -> BunnyResponse {
        self.inner.text_edit_multiline(text)
    }

    #[inline]
    pub fn code_editor(&mut self, text: &mut BunnyString) -> BunnyResponse {
        self.inner.code_editor(text)
    }

    #[inline]
    pub fn button(&mut self, text: impl Into<WidgetText>) -> BunnyResponse {
        self.inner.button(text.into())
    }

    #[inline]
    pub fn small_button(&mut self, text: impl Into<WidgetText>) -> BunnyResponse {
        self.inner.small_button(text.into())
    }

    #[inline]
    pub fn checkbox(&mut self, checked: &mut bool, text: impl Into<WidgetText>) -> BunnyResponse {
        self.inner.checkbox(checked, text.into())
    }

    #[inline]
    pub fn toggle_value(
        &mut self,
        selected: &mut bool,
        text: impl Into<WidgetText>,
    ) -> BunnyResponse {
        self.inner.toggle_value(selected, text.into())
    }

    #[inline]
    pub fn radio(&mut self, selected: bool, text: impl Into<WidgetText>) -> BunnyResponse {
        self.inner.radio(selected, text.into())
    }

    #[inline]
    pub fn radio_value<Value: PartialEq>(
        &mut self,
        current_value: &mut Value,
        alternative: Value,
        text: impl Into<WidgetText>,
    ) -> BunnyResponse {
        let mut response = self.inner.radio(*current_value == alternative, text.into());
        if response.clicked() && *current_value != alternative {
            *current_value = alternative;
            response.mark_changed();
        }
        response
    }

    #[inline]
    pub fn selectable_label(
        &mut self,
        checked: bool,
        text: impl Into<WidgetText>,
    ) -> BunnyResponse {
        self.inner.selectable_label(checked, text.into())
    }

    #[inline]
    pub fn selectable_value<Value: PartialEq>(
        &mut self,
        current_value: &mut Value,
        selected_value: Value,
        text: impl Into<WidgetText>,
    ) -> BunnyResponse {
        let mut response = self
            .inner
            .selectable_label(*current_value == selected_value, text.into());
        if response.clicked() && *current_value != selected_value {
            *current_value = selected_value;
            response.mark_changed();
        }
        response
    }

    #[inline]
    pub fn separator(&mut self) -> BunnyResponse {
        self.inner.separator()
    }

    #[inline]
    pub fn spinner(&mut self) -> BunnyResponse {
        self.inner.spinner()
    }

    #[inline]
    pub fn drag_angle(&mut self, radians: &mut f32) -> BunnyResponse {
        self.inner.drag_angle(radians)
    }

    #[inline]
    pub fn drag_angle_tau(&mut self, radians: &mut f32) -> BunnyResponse {
        self.inner.drag_angle_tau(radians)
    }

    #[inline]
    pub fn image(&mut self, source: impl Into<ImageSource<'a>>) -> BunnyResponse {
        self.inner.image(source.into())
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn color_edit_button_srgba(&mut self, srgba: &mut Color32) -> BunnyResponse {
        self.inner.color_edit_button_srgba(srgba)
    }

    #[inline]
    pub fn color_edit_button_hsva(&mut self, hsva: &mut Hsva) -> BunnyResponse {
        self.inner.color_edit_button_hsva(hsva)
    }

    #[inline]
    pub fn color_edit_button_srgb(&mut self, srgb: &mut [u8; 3]) -> BunnyResponse {
        self.inner.color_edit_button_srgb(srgb)
    }

    #[inline]
    pub fn color_edit_button_rgb(&mut self, rgb: &mut [f32; 3]) -> BunnyResponse {
        self.inner.color_edit_button_rgb(rgb)
    }

    #[inline]
    pub fn color_edit_button_srgba_premultiplied(&mut self, srgba: &mut [u8; 4]) -> BunnyResponse {
        self.inner.color_edit_button_srgba_premultiplied(srgba)
    }

    #[inline]
    pub fn color_edit_button_srgba_unmultiplied(&mut self, srgba: &mut [u8; 4]) -> BunnyResponse {
        self.inner.color_edit_button_srgba_unmultiplied(srgba)
    }

    #[inline]
    pub fn color_edit_button_rgba_premultiplied(
        &mut self,
        rgba_premul: &mut [f32; 4],
    ) -> BunnyResponse {
        self.inner.color_edit_button_rgba_premultiplied(rgba_premul)
    }

    #[inline]
    pub fn color_edit_button_rgba_unmultiplied(
        &mut self,
        rgba_unmul: &mut [f32; 4],
    ) -> BunnyResponse {
        self.inner.color_edit_button_rgba_unmultiplied(rgba_unmul)
    }
}

impl<'a> BunnyUi<'a> {
    pub fn group<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.group(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn push_id<R>(
        &mut self,
        id_salt: impl Hash,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let hash = hash_id_salt(id_salt);
        let response = self.inner.push_id(hash, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn scope<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.scope(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn scope_builder<R>(
        &mut self,
        ui_builder: UiBuilder,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.scope_builder(ui_builder, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn collapsing<R>(
        &mut self,
        heading: impl Into<WidgetText>,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyCollapsingResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let collapsing_ffi = self.inner.collapsing(heading.into(), closure);
        let body_returned = collapsing_ffi
            .body_returned
            .then(|| unsafe { ret.assume_init() });
        BunnyCollapsingResponse {
            header_response: collapsing_ffi.header_response,
            body_response: collapsing_ffi.body_response.into(),
            body_returned,
            openness: collapsing_ffi.openness,
        }
    }

    pub fn indent<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.indent(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn horizontal<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.horizontal(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn horizontal_centered<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.horizontal_centered(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn horizontal_top<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.horizontal_top(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn horizontal_wrapped<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.horizontal_wrapped(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn vertical<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.vertical(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn vertical_centered<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.vertical_centered(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn vertical_centered_justified<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.vertical_centered_justified(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn with_layout<R>(
        &mut self,
        layout: Layout,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.with_layout(layout, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub fn centered_and_justified<R>(
        &mut self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.centered_and_justified(closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    #[inline]
    pub fn end_row(&mut self) {
        self.inner.end_row();
    }

    #[inline]
    pub fn set_row_height(&mut self, height: f32) {
        self.inner.set_row_height(height);
    }

    pub fn with_visual_transform<R>(
        &mut self,
        transform: TSTransform,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.with_visual_transform(transform, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }
}

impl<'a> BunnyUi<'a> {
    pub fn menu_button<R>(
        &mut self,
        text: impl Into<WidgetText>,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<Option<R>> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let Tuple2(response, inner_returned) = self.inner.menu_button(text.into(), closure);
        let inner = inner_returned.then(|| unsafe { ret.assume_init() });
        BunnyInnerResponse::new(inner, response)
    }
}

impl<'a> BunnyUi<'a> {
    pub fn input<R>(&self, mut input: impl FnMut(&mut BunnyInputState) -> R) -> R {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = InputStateClosure::new(&mut input, &mut ret);
        self.inner.input(closure);
        unsafe { ret.assume_init() }
    }

    #[inline]
    pub fn fonts_layout_job(&self, job: LayoutJob) -> BunnyGalley {
        self.inner.fonts_layout_job(job)
    }

    #[inline]
    pub fn fonts_layout(
        &self,
        text: impl Into<RString>,
        font_id: FontId,
        color: Color32,
        wrap_width: f32,
    ) -> BunnyGalley {
        self.inner
            .fonts_layout(text.into(), font_id, color, wrap_width)
    }

    #[inline]
    pub fn fonts_layout_no_wrap(
        &self,
        text: impl Into<RString>,
        font_id: FontId,
        color: Color32,
    ) -> BunnyGalley {
        self.inner.fonts_layout_no_wrap(text.into(), font_id, color)
    }

    #[inline]
    pub fn fonts_layout_delayed_color(
        &self,
        text: impl Into<RString>,
        font_id: FontId,
        wrap_width: f32,
    ) -> BunnyGalley {
        self.inner
            .fonts_layout_delayed_color(text.into(), font_id, wrap_width)
    }

    #[inline]
    pub fn read_response(&self, id: impl Into<Id>) -> Option<BunnyResponse> {
        self.inner.read_response(id.into()).into_option()
    }

    #[inline]
    pub fn layer_painter(&self, layer_id: LayerId) -> BunnyPainter {
        self.inner.layer_painter(layer_id)
    }

    #[inline]
    pub fn debug_painter(&self) -> BunnyPainter {
        self.inner.debug_painter()
    }

    #[inline]
    pub fn time(&self) -> f64 {
        self.inner.time()
    }

    #[inline]
    pub fn copy_text(&self, text: impl Into<RString>) {
        self.inner.copy_text(text.into());
    }

    #[inline]
    pub fn cumulative_frame_nr(&self) -> u64 {
        self.inner.cumulative_frame_nr()
    }

    #[inline]
    pub fn cumulative_pass_nr(&self) -> u64 {
        self.inner.cumulative_pass_nr()
    }

    #[inline]
    pub fn try_load_texture<'uri>(
        &self,
        uri: impl Into<RStr<'uri>>,
        texture_options: TextureOptions,
        size_hint: SizeHint,
    ) -> Option<TexturePoll> {
        self.inner
            .try_load_texture(uri.into(), texture_options, size_hint)
            .into()
    }
}

impl<'a> BunnyUi<'a> {
    pub(crate) fn area_show<R>(
        &mut self,
        area: Area,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.area_show(area, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub(crate) fn collapsing_header_show<R>(
        &mut self,
        collapsing_header: CollapsingHeader,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyCollapsingResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let collapsing_ffi = self
            .inner
            .collapsing_header_show(collapsing_header, closure);
        let body_returned = collapsing_ffi
            .body_returned
            .then(|| unsafe { ret.assume_init() });
        BunnyCollapsingResponse {
            header_response: collapsing_ffi.header_response,
            body_response: collapsing_ffi.body_response.into(),
            body_returned,
            openness: collapsing_ffi.openness,
        }
    }

    pub(crate) fn combo_box_show<R>(
        &mut self,
        combo_box: ComboBox,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<Option<R>> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let Tuple2(response, inner_returned) = self.inner.combo_box_show(combo_box, closure);
        let inner = inner_returned.then(|| unsafe { ret.assume_init() });
        BunnyInnerResponse::new(inner, response)
    }

    pub(crate) fn frame_show<R>(
        &mut self,
        frame: Frame,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.frame_show(frame, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub(crate) fn grid_show<R>(
        &mut self,
        grid: Grid,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.grid_show(grid, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub(crate) fn modal_show<R>(
        &self,
        modal: Modal,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyModalResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.modal_show(modal, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyModalResponse {
            response: response.response,
            backdrop_response: response.backdrop_response,
            inner,
            is_top_modal: response.is_top_modal,
            any_popup_open: response.any_popup_open,
        }
    }

    pub(crate) fn central_panel_show<R>(
        &mut self,
        central_panel: CentralPanel,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.central_panel_show(central_panel, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub(crate) fn panel_show<R>(
        &mut self,
        panel: Panel,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.panel_show(panel, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub(crate) fn panel_show_animated<R>(
        &mut self,
        panel: Panel,
        is_expanded: bool,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> Option<BunnyInnerResponse<R>> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.panel_show_animated(panel, is_expanded, closure);
        response
            .map(|response| {
                let inner = unsafe { ret.assume_init() };
                BunnyInnerResponse::new(inner, response)
            })
            .into_option()
    }

    pub(crate) fn panel_show_animated_between<R>(
        &mut self,
        is_expanded: bool,
        collapsed_panel: Panel,
        expanded_panel: Panel,
        mut add_contents: impl FnMut(&mut BunnyUi, f32) -> R,
    ) -> BunnyInnerResponse<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PanelAnimatedBetweenClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.panel_show_animated_between(
            is_expanded,
            collapsed_panel,
            expanded_panel,
            closure,
        );
        let inner = unsafe { ret.assume_init() };
        BunnyInnerResponse::new(inner, response)
    }

    pub(crate) fn popup_show<R>(
        &mut self,
        popup: Popup,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> Option<BunnyInnerResponse<R>> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.popup_show(popup, closure);
        response
            .map(|response| {
                let inner = unsafe { ret.assume_init() };
                BunnyInnerResponse::new(inner, response)
            })
            .into_option()
    }

    pub(crate) fn scroll_area_show<R>(
        &mut self,
        scroll_area: ScrollArea,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyScrollAreaOutput<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let output = self.inner.scroll_area_show(scroll_area, closure);
        let inner = unsafe { ret.assume_init() };
        BunnyScrollAreaOutput {
            inner,
            id: output.id,
            offset: output.offset,
            velocity: output.velocity,
            content_size: output.content_size,
            inner_rect: output.inner_rect,
        }
    }

    pub(crate) fn scroll_area_show_rows<R>(
        &mut self,
        scroll_area: ScrollArea,
        row_height_sans_spacing: f32,
        total_rows: usize,
        mut add_contents: impl FnMut(&mut BunnyUi, std::ops::Range<usize>) -> R,
    ) -> BunnyScrollAreaOutput<R> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = ScrollAreaRowsClosure::new(&mut add_contents, &mut ret);
        let output = self.inner.scroll_area_show_rows(
            scroll_area,
            row_height_sans_spacing,
            total_rows,
            closure,
        );
        let inner = unsafe { ret.assume_init() };
        BunnyScrollAreaOutput {
            inner,
            id: output.id,
            offset: output.offset,
            velocity: output.velocity,
            content_size: output.content_size,
            inner_rect: output.inner_rect,
        }
    }

    pub(crate) fn sides_show<RetL, RetR>(
        &mut self,
        sides: Sides,
        mut add_contents_left: impl FnMut(&mut BunnyUi) -> RetL,
        mut add_contents_right: impl FnMut(&mut BunnyUi) -> RetR,
    ) -> (RetL, RetR) {
        let mut ret_l = MaybeUninit::<RetL>::uninit();
        let mut ret_r = MaybeUninit::<RetR>::uninit();
        let closure_left = PluginClosure::new(&mut add_contents_left, &mut ret_l);
        let closure_right = PluginClosure::new(&mut add_contents_right, &mut ret_r);
        self.inner.sides_show(sides, closure_left, closure_right);
        unsafe { (ret_l.assume_init(), ret_r.assume_init()) }
    }

    pub(crate) fn window_show<R>(
        &mut self,
        window: Window,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> Option<BunnyInnerResponse<Option<R>>> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let inner_response = self.inner.window_show(window, closure);
        inner_response
            .map(|Tuple2(response, inner_returned)| {
                let inner = inner_returned.then(|| unsafe { ret.assume_init() });
                BunnyInnerResponse::new(inner, response)
            })
            .into_option()
    }
}
