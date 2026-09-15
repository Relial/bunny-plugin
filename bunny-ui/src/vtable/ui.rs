use abi_stable::std_types::Tuple2;
use egui::{Id, Pos2, Rangef, Rect, Sense, Ui, Vec2};
use vtable::{VBox, VRef, VRefMut, vtable};

use crate::{
    Align, LayerId, Layout as BunnyLayout, WidgetText,
    closure::PluginClosure,
    containers::collapsing_header::CollapsingHeader,
    paint::text::text_layout_types::TextWrapMode,
    painter::BunnyPainter,
    response::BunnyResponse,
    style::TextStyle,
    ui::BunnyUi,
    vtable::{
        interaction::InteractionFfiVTable, layout::LayoutFfiVTable, painter::PainterFfiVTable,
        response::ResponseFfiVTable, spacing::SpacingFfiVTable, style::StyleFfiVTable,
        visuals::VisualsFfiVTable,
    },
};

#[vtable]
#[repr(C)]
pub struct UiFfiVTable {
    // new
    // new_child
    is_sizing_pass: fn(VRef<UiFfiVTable>) -> bool,
    id: fn(VRef<UiFfiVTable>) -> Id,
    unique_id: fn(VRef<UiFfiVTable>) -> Id,
    style: fn(VRef<UiFfiVTable>) -> VRef<StyleFfiVTable>,
    style_mut: fn(VRefMut<UiFfiVTable>) -> VRefMut<StyleFfiVTable>,
    reset_style: fn(VRefMut<UiFfiVTable>),
    spacing: fn(VRef<UiFfiVTable>) -> VRef<SpacingFfiVTable>,
    spacing_mut: fn(VRefMut<UiFfiVTable>) -> VRefMut<SpacingFfiVTable>,
    interaction: fn(VRef<UiFfiVTable>) -> VRef<InteractionFfiVTable>,
    interaction_mut: fn(VRefMut<UiFfiVTable>) -> VRefMut<InteractionFfiVTable>,
    visuals: fn(VRef<UiFfiVTable>) -> VRef<VisualsFfiVTable>,
    visuals_mut: fn(VRefMut<UiFfiVTable>) -> VRefMut<VisualsFfiVTable>,
    is_tooltip: fn(VRef<UiFfiVTable>) -> bool,
    // stack
    // ctx
    painter: fn(VRef<UiFfiVTable>) -> VRef<PainterFfiVTable>,
    pixels_per_point: fn(VRef<UiFfiVTable>) -> f32,
    is_enabled: fn(VRef<UiFfiVTable>) -> bool,
    disable: fn(VRefMut<UiFfiVTable>),
    is_visible: fn(VRef<UiFfiVTable>) -> bool,
    set_invisible: fn(VRefMut<UiFfiVTable>),
    set_opacity: fn(VRefMut<UiFfiVTable>, opacity: f32),
    multiply_opacity: fn(VRefMut<UiFfiVTable>, opacity: f32),
    opacity: fn(VRef<UiFfiVTable>) -> f32,
    layout: fn(VRef<UiFfiVTable>) -> VRef<LayoutFfiVTable>,
    wrap_mode: fn(VRef<UiFfiVTable>) -> TextWrapMode,
    text_valign: fn(VRef<UiFfiVTable>) -> Align,
    painter_at: fn(VRef<UiFfiVTable>, rect: Rect) -> VBox<PainterFfiVTable>,
    layer_id: fn(VRef<UiFfiVTable>) -> LayerId,
    text_style_height: fn(VRef<UiFfiVTable>, style: TextStyle) -> f32,
    clip_rect: fn(VRef<UiFfiVTable>) -> Rect,
    shrink_clip_rect: fn(VRefMut<UiFfiVTable>, new_clip_rect: Rect),
    set_clip_rect: fn(VRefMut<UiFfiVTable>, clip_rect: Rect),
    is_rect_visible: fn(VRef<UiFfiVTable>, rect: Rect) -> bool,

    min_rect: fn(VRef<UiFfiVTable>) -> Rect,
    min_size: fn(VRef<UiFfiVTable>) -> Vec2,
    max_rect: fn(VRef<UiFfiVTable>) -> Rect,
    set_max_size: fn(VRefMut<UiFfiVTable>, size: Vec2),
    set_max_width: fn(VRefMut<UiFfiVTable>, width: f32),
    set_max_height: fn(VRefMut<UiFfiVTable>, height: f32),
    set_min_size: fn(VRefMut<UiFfiVTable>, size: Vec2),
    set_min_width: fn(VRefMut<UiFfiVTable>, width: f32),
    set_min_height: fn(VRefMut<UiFfiVTable>, height: f32),
    take_available_space: fn(VRefMut<UiFfiVTable>),
    take_available_width: fn(VRefMut<UiFfiVTable>),
    take_available_height: fn(VRefMut<UiFfiVTable>),
    shrink_width_to_current: fn(VRefMut<UiFfiVTable>),
    expand_to_include_rect: fn(VRefMut<UiFfiVTable>, rect: Rect),
    set_width_range: fn(VRefMut<UiFfiVTable>, width: Rangef),
    set_height_range: fn(VRefMut<UiFfiVTable>, height: Rangef),
    set_width: fn(VRefMut<UiFfiVTable>, width: f32),
    set_height: fn(VRefMut<UiFfiVTable>, height: f32),
    expand_to_include_x: fn(VRefMut<UiFfiVTable>, x: f32),
    expand_to_include_y: fn(VRefMut<UiFfiVTable>, y: f32),
    available_size: fn(VRef<UiFfiVTable>) -> Vec2,
    available_width: fn(VRef<UiFfiVTable>) -> f32,
    available_height: fn(VRef<UiFfiVTable>) -> f32,
    available_size_before_wrap: fn(VRef<UiFfiVTable>) -> Vec2,
    available_rect_before_wrap: fn(VRef<UiFfiVTable>) -> Rect,

    // make_persistent_id
    next_auto_id: fn(VRef<UiFfiVTable>) -> Id,
    // auto_id_with
    skip_ahead_auto_ids: fn(VRefMut<UiFfiVTable>, count: usize),

    interact: fn(VRef<UiFfiVTable>, rect: Rect, id: Id, sense: Sense) -> BunnyResponse,
    // interact_opt
    response: fn(VRef<UiFfiVTable>) -> BunnyResponse,
    rect_contains_pointer: fn(VRef<UiFfiVTable>, rect: Rect) -> bool,
    ui_contains_pointer: fn(VRef<UiFfiVTable>) -> bool,
    close: fn(VRef<UiFfiVTable>),
    // close_kind
    should_close: fn(VRef<UiFfiVTable>) -> bool,
    will_parent_close: fn(VRef<UiFfiVTable>) -> bool,

    allocate_response: fn(VRefMut<UiFfiVTable>, desired_size: Vec2, sense: Sense) -> BunnyResponse,
    allocate_exact_size:
        fn(VRefMut<UiFfiVTable>, desired_size: Vec2, sense: Sense) -> Tuple2<Rect, BunnyResponse>,
    allocate_at_least:
        fn(VRefMut<UiFfiVTable>, desired_size: Vec2, sense: Sense) -> Tuple2<Rect, BunnyResponse>,
    allocate_space: fn(VRefMut<UiFfiVTable>, desired_size: Vec2) -> Tuple2<Id, Rect>,
    allocate_rect: fn(VRefMut<UiFfiVTable>, rect: Rect, sense: Sense) -> BunnyResponse,
    advance_cursor_after_rect: fn(VRefMut<UiFfiVTable>, rect: Rect) -> Id,
    cursor: fn(VRef<UiFfiVTable>) -> Rect,
    next_widget_position: fn(VRef<UiFfiVTable>) -> Pos2,
    allocate_ui:
        fn(VRefMut<UiFfiVTable>, desired_size: Vec2, contents: PluginClosure) -> BunnyResponse,
    allocate_ui_with_layout: fn(
        VRefMut<UiFfiVTable>,
        desired_size: Vec2,
        layout: BunnyLayout,
        contents: PluginClosure,
    ) -> BunnyResponse,
    allocate_painter: fn(
        VRefMut<UiFfiVTable>,
        desired_size: Vec2,
        sense: Sense,
    ) -> Tuple2<BunnyResponse, BunnyPainter>,

    label: fn(VRefMut<UiFfiVTable>, text: WidgetText) -> VBox<ResponseFfiVTable>,
    horizontal: fn(VRefMut<UiFfiVTable>, contents: PluginClosure),

    collapsing_header_show:
        fn(VRefMut<UiFfiVTable>, collapsing_header: CollapsingHeader, contents: PluginClosure),
}

impl UiFfi for Ui {
    #[inline]
    fn is_sizing_pass(&self) -> bool {
        self.is_sizing_pass()
    }

    #[inline]
    fn id(&self) -> Id {
        self.id()
    }

    #[inline]
    fn unique_id(&self) -> Id {
        self.unique_id()
    }

    #[inline]
    fn style(&self) -> VRef<'_, StyleFfiVTable> {
        VRef::new(self.style().as_ref())
    }

    #[inline]
    fn style_mut(&mut self) -> VRefMut<'_, StyleFfiVTable> {
        VRefMut::new(self.style_mut())
    }

    #[inline]
    fn reset_style(&mut self) {
        self.reset_style();
    }

    #[inline]
    fn spacing(&self) -> VRef<'_, SpacingFfiVTable> {
        VRef::new(self.spacing())
    }

    #[inline]
    fn spacing_mut(&mut self) -> VRefMut<'_, SpacingFfiVTable> {
        VRefMut::new(self.spacing_mut())
    }

    #[inline]
    fn interaction(&self) -> VRef<'_, InteractionFfiVTable> {
        VRef::new(&self.style().interaction)
    }

    #[inline]
    fn interaction_mut(&mut self) -> VRefMut<'_, InteractionFfiVTable> {
        VRefMut::new(&mut self.style_mut().interaction)
    }

    #[inline]
    fn visuals(&self) -> VRef<'_, VisualsFfiVTable> {
        VRef::new(self.visuals())
    }

    #[inline]
    fn visuals_mut(&mut self) -> VRefMut<'_, VisualsFfiVTable> {
        VRefMut::new(self.visuals_mut())
    }

    #[inline]
    fn is_tooltip(&self) -> bool {
        self.is_tooltip()
    }

    #[inline]
    fn painter(&self) -> VRef<'_, PainterFfiVTable> {
        VRef::new(self.painter())
    }

    #[inline]
    fn pixels_per_point(&self) -> f32 {
        self.pixels_per_point()
    }

    #[inline]
    fn is_enabled(&self) -> bool {
        self.is_enabled()
    }

    #[inline]
    fn disable(&mut self) {
        self.disable();
    }

    #[inline]
    fn is_visible(&self) -> bool {
        self.is_visible()
    }

    #[inline]
    fn set_invisible(&mut self) {
        self.set_invisible();
    }

    #[inline]
    fn set_opacity(&mut self, opacity: f32) {
        self.set_opacity(opacity);
    }

    #[inline]
    fn multiply_opacity(&mut self, opacity: f32) {
        self.multiply_opacity(opacity);
    }

    #[inline]
    fn opacity(&self) -> f32 {
        self.opacity()
    }

    #[inline]
    fn layout(&self) -> VRef<'_, LayoutFfiVTable> {
        VRef::new(self.layout())
    }

    #[inline]
    fn wrap_mode(&self) -> TextWrapMode {
        self.wrap_mode().into()
    }

    #[inline]
    fn text_valign(&self) -> Align {
        self.text_valign().into()
    }

    #[inline]
    fn painter_at(&self, rect: Rect) -> VBox<PainterFfiVTable> {
        let painter = self.painter_at(rect);
        VBox::new(painter)
    }

    #[inline]
    fn layer_id(&self) -> LayerId {
        self.layer_id().into()
    }

    #[inline]
    fn text_style_height(&self, style: TextStyle) -> f32 {
        self.text_style_height(&style.into())
    }

    #[inline]
    fn clip_rect(&self) -> Rect {
        self.clip_rect()
    }

    #[inline]
    fn shrink_clip_rect(&mut self, new_clip_rect: Rect) {
        self.shrink_clip_rect(new_clip_rect);
    }

    #[inline]
    fn set_clip_rect(&mut self, clip_rect: Rect) {
        self.set_clip_rect(clip_rect);
    }

    #[inline]
    fn is_rect_visible(&self, rect: Rect) -> bool {
        self.is_rect_visible(rect)
    }

    #[inline]
    fn min_rect(&self) -> Rect {
        self.min_rect()
    }

    #[inline]
    fn min_size(&self) -> Vec2 {
        self.min_size()
    }

    #[inline]
    fn max_rect(&self) -> Rect {
        self.max_rect()
    }

    #[inline]
    fn set_max_size(&mut self, size: Vec2) {
        self.set_max_size(size);
    }

    #[inline]
    fn set_max_width(&mut self, width: f32) {
        self.set_max_width(width);
    }

    #[inline]
    fn set_max_height(&mut self, height: f32) {
        self.set_max_height(height);
    }

    #[inline]
    fn set_min_size(&mut self, size: Vec2) {
        self.set_min_size(size);
    }

    #[inline]
    fn set_min_width(&mut self, width: f32) {
        self.set_min_width(width);
    }

    #[inline]
    fn set_min_height(&mut self, height: f32) {
        self.set_min_height(height);
    }

    #[inline]
    fn take_available_space(&mut self) {
        self.take_available_space();
    }

    #[inline]
    fn take_available_width(&mut self) {
        self.take_available_width();
    }

    #[inline]
    fn take_available_height(&mut self) {
        self.take_available_height();
    }

    #[inline]
    fn shrink_width_to_current(&mut self) {
        self.shrink_width_to_current();
    }

    #[inline]
    fn expand_to_include_rect(&mut self, rect: Rect) {
        self.expand_to_include_rect(rect);
    }

    #[inline]
    fn set_width_range(&mut self, width: Rangef) {
        self.set_width_range(width);
    }

    #[inline]
    fn set_height_range(&mut self, height: Rangef) {
        self.set_height_range(height);
    }

    #[inline]
    fn set_width(&mut self, width: f32) {
        self.set_width(width);
    }

    #[inline]
    fn set_height(&mut self, height: f32) {
        self.set_height(height);
    }

    #[inline]
    fn expand_to_include_x(&mut self, x: f32) {
        self.expand_to_include_x(x);
    }

    #[inline]
    fn expand_to_include_y(&mut self, y: f32) {
        self.expand_to_include_y(y);
    }

    #[inline]
    fn available_size(&self) -> Vec2 {
        self.available_size()
    }

    #[inline]
    fn available_width(&self) -> f32 {
        self.available_width()
    }

    #[inline]
    fn available_height(&self) -> f32 {
        self.available_height()
    }

    #[inline]
    fn available_size_before_wrap(&self) -> Vec2 {
        self.available_size_before_wrap()
    }

    #[inline]
    fn available_rect_before_wrap(&self) -> Rect {
        self.available_rect_before_wrap()
    }

    #[inline]
    fn next_auto_id(&self) -> Id {
        self.next_auto_id()
    }

    #[inline]
    fn skip_ahead_auto_ids(&mut self, count: usize) {
        self.skip_ahead_auto_ids(count);
    }

    #[inline]
    fn interact(&self, rect: Rect, id: Id, sense: Sense) -> BunnyResponse {
        let res = self.interact(rect, id, sense);
        VBox::new(res).into()
    }

    #[inline]
    fn response(&self) -> BunnyResponse {
        let res = self.response();
        VBox::new(res).into()
    }

    #[inline]
    fn rect_contains_pointer(&self, rect: Rect) -> bool {
        self.rect_contains_pointer(rect)
    }

    #[inline]
    fn ui_contains_pointer(&self) -> bool {
        self.ui_contains_pointer()
    }

    #[inline]
    fn close(&self) {
        self.close();
    }

    #[inline]
    fn should_close(&self) -> bool {
        self.should_close()
    }

    #[inline]
    fn will_parent_close(&self) -> bool {
        self.will_parent_close()
    }

    #[inline]
    fn allocate_response(&mut self, desired_size: Vec2, sense: Sense) -> BunnyResponse {
        let res = self.allocate_response(desired_size, sense);
        VBox::new(res).into()
    }

    #[inline]
    fn allocate_exact_size(
        &mut self,
        desired_size: Vec2,
        sense: Sense,
    ) -> Tuple2<Rect, BunnyResponse> {
        let (rect, res) = self.allocate_exact_size(desired_size, sense);
        Tuple2(rect, VBox::new(res).into())
    }

    #[inline]
    fn allocate_at_least(
        &mut self,
        desired_size: Vec2,
        sense: Sense,
    ) -> Tuple2<Rect, BunnyResponse> {
        let (rect, res) = self.allocate_at_least(desired_size, sense);
        Tuple2(rect, VBox::new(res).into())
    }

    #[inline]
    fn allocate_space(&mut self, desired_size: Vec2) -> Tuple2<Id, Rect> {
        let (id, rect) = self.allocate_space(desired_size);
        Tuple2(id, rect)
    }

    #[inline]
    fn allocate_rect(&mut self, rect: Rect, sense: Sense) -> BunnyResponse {
        let res = self.allocate_rect(rect, sense);
        VBox::new(res).into()
    }

    #[inline]
    fn advance_cursor_after_rect(&mut self, rect: Rect) -> Id {
        self.advance_cursor_after_rect(rect)
    }

    #[inline]
    fn cursor(&self) -> Rect {
        self.cursor()
    }

    #[inline]
    fn next_widget_position(&self) -> Pos2 {
        self.next_widget_position()
    }

    #[inline]
    fn allocate_ui(&mut self, desired_size: Vec2, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .allocate_ui(desired_size, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        VBox::new(res).into()
    }

    #[inline]
    fn allocate_ui_with_layout(
        &mut self,
        desired_size: Vec2,
        layout: crate::Layout,
        contents: PluginClosure,
    ) -> BunnyResponse {
        let res = self
            .allocate_ui_with_layout(desired_size, layout.into(), |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        VBox::new(res).into()
    }

    #[inline]
    fn allocate_painter(
        &mut self,
        desired_size: Vec2,
        sense: Sense,
    ) -> Tuple2<BunnyResponse, BunnyPainter<'_>> {
        let (res, painter) = self.allocate_painter(desired_size, sense);
        Tuple2(VBox::new(res).into(), VBox::new(painter).into())
    }

    #[inline]
    fn label(&mut self, text: WidgetText) -> VBox<ResponseFfiVTable> {
        let res = self.label(text);
        VBox::new(res)
    }

    #[inline]
    fn horizontal(&mut self, contents: PluginClosure) {
        self.horizontal(|ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
    }

    #[inline]
    fn collapsing_header_show(
        &mut self,
        collapsing_header: CollapsingHeader,
        contents: PluginClosure,
    ) {
        collapsing_header.show_impl(self, contents);
    }
}

UiFfiVTable_static!(static UIFFI_VT for Ui);
