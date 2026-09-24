#[cfg(feature = "manager")]
use crate::{BunnyInputState, BunnyUi};
use abi_stable::std_types::{ROption, RStr, Tuple2};
use ecolor::{Color32, Hsva};
use emath::{Pos2, Rangef, Rect, TSTransform, Vec2};
use vtable::{VRef, VRefMut, vtable};

use crate::{
    Align, Area, BunnyGalley, BunnyInteractionMut, BunnyInteractionRef, BunnyPainter,
    BunnyPainterRef, BunnyResponse, BunnySpacingMut, BunnySpacingRef, BunnyString, BunnyStyleMut,
    BunnyStyleRef, BunnyVisualsMut, BunnyVisualsRef, CentralPanel, CollapsingHeader, ComboBox,
    Frame, Grid, Id, ImageSource, LayerId, Layout as BunnyLayout, Modal, Panel, Popup, RichText,
    ScrollAnimation, ScrollArea, Sense, Sides, SizeHint, Style, TextStyle, UiBuilder, Widget,
    WidgetText, Window,
    closure::{
        InputStateClosure, PanelAnimatedBetweenClosure, PluginClosure, ScrollAreaRowsClosure,
    },
    load::TexturePoll,
    paint::{FontId, LayoutJob, TextWrapMode, TextureOptions},
};

#[vtable]
#[repr(C)]
pub struct UiFfiVTable {
    // new
    // new_child
    is_sizing_pass: fn(VRef<UiFfiVTable>) -> bool,
    id: fn(VRef<UiFfiVTable>) -> Id,
    unique_id: fn(VRef<UiFfiVTable>) -> Id,
    style: fn(VRef<UiFfiVTable>) -> BunnyStyleRef,
    style_mut: fn(VRefMut<UiFfiVTable>) -> BunnyStyleMut,
    style_clone: fn(VRef<UiFfiVTable>) -> Style,
    set_style: fn(VRefMut<UiFfiVTable>, style: &Style),
    reset_style: fn(VRefMut<UiFfiVTable>),
    spacing: fn(VRef<UiFfiVTable>) -> BunnySpacingRef,
    spacing_mut: fn(VRefMut<UiFfiVTable>) -> BunnySpacingMut,
    interaction: fn(VRef<UiFfiVTable>) -> BunnyInteractionRef,
    interaction_mut: fn(VRefMut<UiFfiVTable>) -> BunnyInteractionMut,
    visuals: fn(VRef<UiFfiVTable>) -> BunnyVisualsRef,
    visuals_mut: fn(VRefMut<UiFfiVTable>) -> BunnyVisualsMut,
    is_tooltip: fn(VRef<UiFfiVTable>) -> bool,
    // stack
    // ctx
    painter: fn(VRef<UiFfiVTable>) -> BunnyPainterRef,
    pixels_per_point: fn(VRef<UiFfiVTable>) -> f32,
    is_enabled: fn(VRef<UiFfiVTable>) -> bool,
    disable: fn(VRefMut<UiFfiVTable>),
    is_visible: fn(VRef<UiFfiVTable>) -> bool,
    set_invisible: fn(VRefMut<UiFfiVTable>),
    set_opacity: fn(VRefMut<UiFfiVTable>, opacity: f32),
    multiply_opacity: fn(VRefMut<UiFfiVTable>, opacity: f32),
    opacity: fn(VRef<UiFfiVTable>) -> f32,
    layout: fn(VRef<UiFfiVTable>) -> BunnyLayout,
    wrap_mode: fn(VRef<UiFfiVTable>) -> TextWrapMode,
    text_valign: fn(VRef<UiFfiVTable>) -> Align,
    painter_at: fn(VRef<UiFfiVTable>, rect: Rect) -> BunnyPainter,
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

    // make_persistent_id: fn(VRef<UiFfiVTable>, hash: u64) -> Id,
    next_auto_id: fn(VRef<UiFfiVTable>) -> Id,
    // auto_id_with: fn(VRef<UiFfiVTable>, hash: u64) -> Id,
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

    scroll_to_rect: fn(VRef<UiFfiVTable>, rect: Rect, align: ROption<Align>),
    scroll_to_rect_animation:
        fn(VRef<UiFfiVTable>, rect: Rect, align: ROption<Align>, animation: ScrollAnimation),
    scroll_to_cursor: fn(VRef<UiFfiVTable>, align: ROption<Align>),
    scroll_to_cursor_animation:
        fn(VRef<UiFfiVTable>, align: ROption<Align>, animation: ScrollAnimation),
    scroll_with_delta: fn(VRef<UiFfiVTable>, delta: Vec2),
    scroll_with_delta_animation: fn(VRef<UiFfiVTable>, delta: Vec2, animation: ScrollAnimation),

    add: fn(VRefMut<UiFfiVTable>, widget: Widget) -> BunnyResponse,
    add_sized: fn(VRefMut<UiFfiVTable>, max_size: Vec2, widget: Widget) -> BunnyResponse,
    place: fn(VRefMut<UiFfiVTable>, max_rect: Rect, widget: Widget) -> BunnyResponse,
    put: fn(VRefMut<UiFfiVTable>, max_rect: Rect, widget: Widget) -> BunnyResponse,
    add_enabled: fn(VRefMut<UiFfiVTable>, enabled: bool, widget: Widget) -> BunnyResponse,
    add_enabled_ui:
        fn(VRefMut<UiFfiVTable>, enabled: bool, contents: PluginClosure) -> BunnyResponse,
    add_visible: fn(VRefMut<UiFfiVTable>, visible: bool, widget: Widget) -> BunnyResponse,
    add_space: fn(VRefMut<UiFfiVTable>, amount: f32),
    label: fn(VRefMut<UiFfiVTable>, text: WidgetText) -> BunnyResponse,
    colored_label: fn(VRefMut<UiFfiVTable>, color: Color32, text: RichText) -> BunnyResponse,
    heading: fn(VRefMut<UiFfiVTable>, text: RichText) -> BunnyResponse,
    monospace: fn(VRefMut<UiFfiVTable>, text: RichText) -> BunnyResponse,
    code: fn(VRefMut<UiFfiVTable>, text: RichText) -> BunnyResponse,
    small: fn(VRefMut<UiFfiVTable>, text: RichText) -> BunnyResponse,
    strong: fn(VRefMut<UiFfiVTable>, text: RichText) -> BunnyResponse,
    weak: fn(VRefMut<UiFfiVTable>, text: RichText) -> BunnyResponse,
    link: fn(VRefMut<UiFfiVTable>, text: WidgetText) -> BunnyResponse,
    // hyperlink
    text_edit_singleline: fn(VRefMut<UiFfiVTable>, text: &mut BunnyString) -> BunnyResponse,
    text_edit_multiline: fn(VRefMut<UiFfiVTable>, text: &mut BunnyString) -> BunnyResponse,
    code_editor: fn(VRefMut<UiFfiVTable>, text: &mut BunnyString) -> BunnyResponse,
    button: fn(VRefMut<UiFfiVTable>, text: WidgetText) -> BunnyResponse,
    small_button: fn(VRefMut<UiFfiVTable>, text: WidgetText) -> BunnyResponse,
    checkbox: fn(VRefMut<UiFfiVTable>, checked: &mut bool, text: WidgetText) -> BunnyResponse,
    toggle_value: fn(VRefMut<UiFfiVTable>, selected: &mut bool, text: WidgetText) -> BunnyResponse,
    radio: fn(VRefMut<UiFfiVTable>, selected: bool, text: WidgetText) -> BunnyResponse,
    // radio_value
    selectable_label: fn(VRefMut<UiFfiVTable>, checked: bool, text: WidgetText) -> BunnyResponse,
    // selectable_value
    separator: fn(VRefMut<UiFfiVTable>) -> BunnyResponse,
    spinner: fn(VRefMut<UiFfiVTable>) -> BunnyResponse,
    drag_angle: fn(VRefMut<UiFfiVTable>, radians: &mut f32) -> BunnyResponse,
    drag_angle_tau: fn(VRefMut<UiFfiVTable>, radians: &mut f32) -> BunnyResponse,
    image: fn(VRefMut<UiFfiVTable>, source: ImageSource) -> BunnyResponse,

    color_edit_button_srgba: fn(VRefMut<UiFfiVTable>, srgba: &mut Color32) -> BunnyResponse,
    color_edit_button_hsva: fn(VRefMut<UiFfiVTable>, hsva: &mut Hsva) -> BunnyResponse,
    color_edit_button_srgb: fn(VRefMut<UiFfiVTable>, srgb: &mut [u8; 3]) -> BunnyResponse,
    color_edit_button_rgb: fn(VRefMut<UiFfiVTable>, rgb: &mut [f32; 3]) -> BunnyResponse,
    color_edit_button_srgba_premultiplied:
        fn(VRefMut<UiFfiVTable>, srgba: &mut [u8; 4]) -> BunnyResponse,
    color_edit_button_srgba_unmultiplied:
        fn(VRefMut<UiFfiVTable>, srgba: &mut [u8; 4]) -> BunnyResponse,
    color_edit_button_rgba_premultiplied:
        fn(VRefMut<UiFfiVTable>, rgba_premul: &mut [f32; 4]) -> BunnyResponse,
    color_edit_button_rgba_unmultiplied:
        fn(VRefMut<UiFfiVTable>, rgba_unmul: &mut [f32; 4]) -> BunnyResponse,

    group: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    push_id: fn(VRefMut<UiFfiVTable>, id_salt: u64, contents: PluginClosure) -> BunnyResponse,
    scope: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    scope_builder:
        fn(VRefMut<UiFfiVTable>, ui_builder: UiBuilder, contents: PluginClosure) -> BunnyResponse,
    // scope_dyn
    collapsing: fn(
        VRefMut<UiFfiVTable>,
        heading: WidgetText,
        contents: PluginClosure,
    ) -> CollapsingFfiResponse,
    indent: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    horizontal: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    horizontal_centered: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    horizontal_top: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    horizontal_wrapped: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    vertical: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    vertical_centered: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    vertical_centered_justified: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    with_layout:
        fn(VRefMut<UiFfiVTable>, layout: BunnyLayout, contents: PluginClosure) -> BunnyResponse,
    centered_and_justified: fn(VRefMut<UiFfiVTable>, contents: PluginClosure) -> BunnyResponse,
    end_row: fn(VRefMut<UiFfiVTable>),
    set_row_height: fn(VRefMut<UiFfiVTable>, height: f32),
    // columns
    // columns_const
    // dnd_drag_source
    // dnd_drop_zone
    with_visual_transform:
        fn(VRefMut<UiFfiVTable>, transform: TSTransform, contents: PluginClosure) -> BunnyResponse,

    menu_button: fn(
        VRefMut<UiFfiVTable>,
        text: WidgetText,
        contents: PluginClosure,
    ) -> Tuple2<BunnyResponse, bool>,
    // menu_image_button
    // menu_image_text_button
    // debug_paint_cursor

    // run_ui
    // begin_pass
    input: fn(VRef<UiFfiVTable>, InputStateClosure: InputStateClosure),
    // ...
    fonts_layout_job: fn(VRef<UiFfiVTable>, job: LayoutJob) -> BunnyGalley,
    fonts_layout: fn(
        VRef<UiFfiVTable>,
        text: RStr,
        font_id: FontId,
        color: Color32,
        wrap_width: f32,
    ) -> BunnyGalley,
    fonts_layout_no_wrap:
        fn(VRef<UiFfiVTable>, text: RStr, font_id: FontId, color: Color32) -> BunnyGalley,
    fonts_layout_delayed_color:
        fn(VRef<UiFfiVTable>, text: RStr, font_id: FontId, wrap_width: f32) -> BunnyGalley,
    // ...
    read_response: fn(VRef<UiFfiVTable>, id: Id) -> ROption<BunnyResponse>,
    layer_painter: fn(VRef<UiFfiVTable>, layer_id: LayerId) -> BunnyPainter,
    debug_painter: fn(VRef<UiFfiVTable>) -> BunnyPainter,
    // debug_text
    time: fn(VRef<UiFfiVTable>) -> f64,
    // ...
    copy_text: fn(VRef<UiFfiVTable>, text: RStr),
    // ...
    cumulative_frame_nr: fn(VRef<UiFfiVTable>) -> u64,
    // cumulative_frame_nr_for
    cumulative_pass_nr: fn(VRef<UiFfiVTable>) -> u64,
    // cumulative_pass_nr_for
    // ...
    try_load_texture: fn(
        VRef<UiFfiVTable>,
        uri: RStr,
        texture_options: TextureOptions,
        size_hint: SizeHint,
    ) -> ROption<TexturePoll>,
    // ...
    area_show: fn(VRefMut<UiFfiVTable>, area: Area, contents: PluginClosure) -> BunnyResponse,
    collapsing_header_show: fn(
        VRefMut<UiFfiVTable>,
        collapsing_header: CollapsingHeader,
        contents: PluginClosure,
    ) -> CollapsingFfiResponse,
    combo_box_show: fn(
        VRefMut<UiFfiVTable>,
        combo_box: ComboBox,
        contents: PluginClosure,
    ) -> Tuple2<BunnyResponse, bool>,
    frame_show: fn(VRefMut<UiFfiVTable>, frame: Frame, contents: PluginClosure) -> BunnyResponse,
    grid_show: fn(VRefMut<UiFfiVTable>, grid: Grid, contents: PluginClosure) -> BunnyResponse,
    modal_show: fn(VRef<UiFfiVTable>, modal: Modal, contents: PluginClosure) -> ModalFfiResponse,
    central_panel_show: fn(
        VRefMut<UiFfiVTable>,
        central_panel: CentralPanel,
        contents: PluginClosure,
    ) -> BunnyResponse,
    panel_show: fn(VRefMut<UiFfiVTable>, panel: Panel, contents: PluginClosure) -> BunnyResponse,
    panel_show_animated: fn(
        VRefMut<UiFfiVTable>,
        panel: Panel,
        is_expanded: bool,
        contents: PluginClosure,
    ) -> ROption<BunnyResponse>,
    panel_show_animated_between: fn(
        VRefMut<UiFfiVTable>,
        is_expanded: bool,
        collapsed_panel: Panel,
        expanded_panel: Panel,
        contents: PanelAnimatedBetweenClosure,
    ) -> BunnyResponse,
    popup_show:
        fn(VRefMut<UiFfiVTable>, popup: Popup, contents: PluginClosure) -> ROption<BunnyResponse>,
    scroll_area_show: fn(
        VRefMut<UiFfiVTable>,
        scroll_area: ScrollArea,
        contents: PluginClosure,
    ) -> ScrollAreaFfiOutput,
    scroll_area_show_rows: fn(
        VRefMut<UiFfiVTable>,
        scroll_area: ScrollArea,
        row_height_sans_spacing: f32,
        total_rows: usize,
        contents: ScrollAreaRowsClosure,
    ) -> ScrollAreaFfiOutput,
    sides_show: fn(
        VRefMut<UiFfiVTable>,
        sides: Sides,
        contents_left: PluginClosure,
        contents_right: PluginClosure,
    ),
    window_show: fn(
        VRefMut<UiFfiVTable>,
        window: Window,
        contents: PluginClosure,
    ) -> ROption<Tuple2<BunnyResponse, bool>>,
}

#[cfg(feature = "manager")]
impl UiFfi for egui::Ui {
    #[inline]
    fn is_sizing_pass(&self) -> bool {
        self.is_sizing_pass()
    }

    #[inline]
    fn id(&self) -> Id {
        self.id().into()
    }

    #[inline]
    fn unique_id(&self) -> Id {
        self.unique_id().into()
    }

    #[inline]
    fn style(&self) -> BunnyStyleRef<'_> {
        BunnyStyleRef::new(self.style().as_ref())
    }

    #[inline]
    fn style_mut(&mut self) -> BunnyStyleMut<'_> {
        BunnyStyleMut::new(self.style_mut())
    }

    #[inline]
    fn style_clone(&self) -> Style {
        self.style().as_ref().clone().into()
    }

    #[inline]
    fn set_style(&mut self, style: &Style) {
        let style: egui::Style = style.into();
        self.set_style(style);
    }

    #[inline]
    fn reset_style(&mut self) {
        self.reset_style();
    }

    #[inline]
    fn spacing(&self) -> BunnySpacingRef<'_> {
        BunnySpacingRef::new(self.spacing())
    }

    #[inline]
    fn spacing_mut(&mut self) -> BunnySpacingMut<'_> {
        BunnySpacingMut::new(self.spacing_mut())
    }

    #[inline]
    fn interaction(&self) -> BunnyInteractionRef<'_> {
        BunnyInteractionRef::new(&self.style().interaction)
    }

    #[inline]
    fn interaction_mut(&mut self) -> BunnyInteractionMut<'_> {
        BunnyInteractionMut::new(&mut self.style_mut().interaction)
    }

    #[inline]
    fn visuals(&self) -> BunnyVisualsRef<'_> {
        BunnyVisualsRef::new(self.visuals())
    }

    #[inline]
    fn visuals_mut(&mut self) -> BunnyVisualsMut<'_> {
        BunnyVisualsMut::new(self.visuals_mut())
    }

    #[inline]
    fn is_tooltip(&self) -> bool {
        self.is_tooltip()
    }

    #[inline]
    fn painter(&self) -> BunnyPainterRef<'_> {
        BunnyPainterRef::new(self.painter())
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
    fn layout(&self) -> BunnyLayout {
        let layout = *self.layout();
        layout.into()
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
    fn painter_at(&self, rect: Rect) -> BunnyPainter {
        let painter = self.painter_at(rect);
        BunnyPainter::new(painter)
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

    // #[inline]
    // fn make_persistent_id(&self, hash: u64) -> Id {
    //     self.make_persistent_id(hash).into()
    // }

    #[inline]
    fn next_auto_id(&self) -> Id {
        self.next_auto_id().into()
    }

    // #[inline]
    // fn auto_id_with(&self, hash: u64) -> Id {
    //     self.auto_id_with(hash).into()
    // }

    #[inline]
    fn skip_ahead_auto_ids(&mut self, count: usize) {
        self.skip_ahead_auto_ids(count);
    }

    #[inline]
    fn interact(&self, rect: Rect, id: Id, sense: Sense) -> BunnyResponse {
        let res = self.interact(rect, id.into(), sense.into());
        BunnyResponse::new(res)
    }

    #[inline]
    fn response(&self) -> BunnyResponse {
        let res = self.response();
        BunnyResponse::new(res)
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
        let res = self.allocate_response(desired_size, sense.into());
        BunnyResponse::new(res)
    }

    #[inline]
    fn allocate_exact_size(
        &mut self,
        desired_size: Vec2,
        sense: Sense,
    ) -> Tuple2<Rect, BunnyResponse> {
        let (rect, res) = self.allocate_exact_size(desired_size, sense.into());
        Tuple2(rect, BunnyResponse::new(res))
    }

    #[inline]
    fn allocate_at_least(
        &mut self,
        desired_size: Vec2,
        sense: Sense,
    ) -> Tuple2<Rect, BunnyResponse> {
        let (rect, res) = self.allocate_at_least(desired_size, sense.into());
        Tuple2(rect, BunnyResponse::new(res))
    }

    #[inline]
    fn allocate_space(&mut self, desired_size: Vec2) -> Tuple2<Id, Rect> {
        let (id, rect) = self.allocate_space(desired_size);
        Tuple2(id.into(), rect)
    }

    #[inline]
    fn allocate_rect(&mut self, rect: Rect, sense: Sense) -> BunnyResponse {
        let res = self.allocate_rect(rect, sense.into());
        BunnyResponse::new(res)
    }

    #[inline]
    fn advance_cursor_after_rect(&mut self, rect: Rect) -> Id {
        self.advance_cursor_after_rect(rect).into()
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
        BunnyResponse::new(res)
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
        BunnyResponse::new(res)
    }

    #[inline]
    fn allocate_painter(
        &mut self,
        desired_size: Vec2,
        sense: Sense,
    ) -> Tuple2<BunnyResponse, BunnyPainter> {
        let (res, painter) = self.allocate_painter(desired_size, sense.into());
        Tuple2(BunnyResponse::new(res), BunnyPainter::new(painter))
    }

    #[inline]
    fn scroll_to_rect(&self, rect: Rect, align: ROption<Align>) {
        self.scroll_to_rect(rect, align.map(|a| a.into()).into_option());
    }

    #[inline]
    fn scroll_to_rect_animation(
        &self,
        rect: Rect,
        align: ROption<Align>,
        animation: ScrollAnimation,
    ) {
        self.scroll_to_rect_animation(
            rect,
            align.map(|a| a.into()).into_option(),
            animation.into(),
        );
    }

    #[inline]
    fn scroll_to_cursor(&self, align: ROption<Align>) {
        self.scroll_to_cursor(align.map(|a| a.into()).into_option());
    }

    #[inline]
    fn scroll_to_cursor_animation(&self, align: ROption<Align>, animation: ScrollAnimation) {
        self.scroll_to_cursor_animation(align.map(|a| a.into()).into_option(), animation.into());
    }

    #[inline]
    fn scroll_with_delta(&self, delta: Vec2) {
        self.scroll_with_delta(delta);
    }

    #[inline]
    fn scroll_with_delta_animation(&self, delta: Vec2, animation: ScrollAnimation) {
        self.scroll_with_delta_animation(delta, animation.into());
    }

    #[inline]
    fn add(&mut self, widget: Widget) -> BunnyResponse {
        let res = self.add(widget);
        BunnyResponse::new(res)
    }

    #[inline]
    fn add_sized(&mut self, max_size: Vec2, widget: Widget) -> BunnyResponse {
        let res = self.add_sized(max_size, widget);
        BunnyResponse::new(res)
    }

    #[inline]
    fn place(&mut self, max_rect: Rect, widget: Widget) -> BunnyResponse {
        let res = self.place(max_rect, widget);
        BunnyResponse::new(res)
    }

    #[inline]
    fn put(&mut self, max_rect: Rect, widget: Widget) -> BunnyResponse {
        let res = self.put(max_rect, widget);
        BunnyResponse::new(res)
    }

    #[inline]
    fn add_enabled(&mut self, enabled: bool, widget: Widget) -> BunnyResponse {
        let res = self.add_enabled(enabled, widget);
        BunnyResponse::new(res)
    }

    #[inline]
    fn add_enabled_ui(&mut self, enabled: bool, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .add_enabled_ui(enabled, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn add_visible(&mut self, visible: bool, widget: Widget) -> BunnyResponse {
        let res = self.add_visible(visible, widget);
        BunnyResponse::new(res)
    }

    #[inline]
    fn add_space(&mut self, amount: f32) {
        self.add_space(amount);
    }

    #[inline]
    fn label(&mut self, text: WidgetText) -> BunnyResponse {
        let res = self.label(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn colored_label(&mut self, color: Color32, text: RichText) -> BunnyResponse {
        let res = self.colored_label(color, text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn heading(&mut self, text: RichText) -> BunnyResponse {
        let res = self.heading(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn monospace(&mut self, text: RichText) -> BunnyResponse {
        let res = self.monospace(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn code(&mut self, text: RichText) -> BunnyResponse {
        let res = self.code(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn small(&mut self, text: RichText) -> BunnyResponse {
        let res = self.small(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn strong(&mut self, text: RichText) -> BunnyResponse {
        let res = self.strong(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn weak(&mut self, text: RichText) -> BunnyResponse {
        let res = self.weak(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn link(&mut self, text: WidgetText) -> BunnyResponse {
        let res = self.link(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn text_edit_singleline(&mut self, text: &mut BunnyString) -> BunnyResponse {
        let res = self.text_edit_singleline(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn text_edit_multiline(&mut self, text: &mut BunnyString) -> BunnyResponse {
        let res = self.text_edit_multiline(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn code_editor(&mut self, text: &mut BunnyString) -> BunnyResponse {
        let res = self.code_editor(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn button(&mut self, text: WidgetText) -> BunnyResponse {
        let res = self.button(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn small_button(&mut self, text: WidgetText) -> BunnyResponse {
        let res = self.small_button(text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn checkbox(&mut self, checked: &mut bool, text: WidgetText) -> BunnyResponse {
        let res = self.checkbox(checked, text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn toggle_value(&mut self, selected: &mut bool, text: WidgetText) -> BunnyResponse {
        let res = self.toggle_value(selected, text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn radio(&mut self, selected: bool, text: WidgetText) -> BunnyResponse {
        let res = self.radio(selected, text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn selectable_label(&mut self, checked: bool, text: WidgetText) -> BunnyResponse {
        let res = self.selectable_label(checked, text);
        BunnyResponse::new(res)
    }

    #[inline]
    fn separator(&mut self) -> BunnyResponse {
        let res = self.separator();
        BunnyResponse::new(res)
    }

    #[inline]
    fn spinner(&mut self) -> BunnyResponse {
        let res = self.spinner();
        BunnyResponse::new(res)
    }

    #[inline]
    fn drag_angle(&mut self, radians: &mut f32) -> BunnyResponse {
        let res = self.drag_angle(radians);
        BunnyResponse::new(res)
    }

    #[inline]
    fn drag_angle_tau(&mut self, radians: &mut f32) -> BunnyResponse {
        let res = self.drag_angle_tau(radians);
        BunnyResponse::new(res)
    }

    #[inline]
    fn image(&mut self, source: ImageSource) -> BunnyResponse {
        let res = self.image(source);
        BunnyResponse::new(res)
    }

    #[inline]
    fn color_edit_button_srgba(&mut self, srgba: &mut Color32) -> BunnyResponse {
        let res = self.color_edit_button_srgba(srgba);
        BunnyResponse::new(res)
    }

    #[inline]
    fn color_edit_button_hsva(&mut self, hsva: &mut Hsva) -> BunnyResponse {
        let res = self.color_edit_button_hsva(hsva);
        BunnyResponse::new(res)
    }

    #[inline]
    fn color_edit_button_srgb(&mut self, srgb: &mut [u8; 3]) -> BunnyResponse {
        let res = self.color_edit_button_srgb(srgb);
        BunnyResponse::new(res)
    }

    #[inline]
    fn color_edit_button_rgb(&mut self, rgb: &mut [f32; 3]) -> BunnyResponse {
        let res = self.color_edit_button_rgb(rgb);
        BunnyResponse::new(res)
    }

    #[inline]
    fn color_edit_button_srgba_premultiplied(&mut self, srgba: &mut [u8; 4]) -> BunnyResponse {
        let res = self.color_edit_button_srgba_premultiplied(srgba);
        BunnyResponse::new(res)
    }

    #[inline]
    fn color_edit_button_srgba_unmultiplied(&mut self, srgba: &mut [u8; 4]) -> BunnyResponse {
        let res = self.color_edit_button_srgba_unmultiplied(srgba);
        BunnyResponse::new(res)
    }

    #[inline]
    fn color_edit_button_rgba_premultiplied(
        &mut self,
        rgba_premul: &mut [f32; 4],
    ) -> BunnyResponse {
        let res = self.color_edit_button_rgba_premultiplied(rgba_premul);
        BunnyResponse::new(res)
    }

    #[inline]
    fn color_edit_button_rgba_unmultiplied(&mut self, rgba_unmul: &mut [f32; 4]) -> BunnyResponse {
        let res = self.color_edit_button_rgba_unmultiplied(rgba_unmul);
        BunnyResponse::new(res)
    }

    #[inline]
    fn group(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .group(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn push_id(&mut self, hash: u64, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .push_id(hash, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn scope(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .scope(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn scope_builder(&mut self, ui_builder: UiBuilder, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .scope_builder(ui_builder.into(), |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn collapsing(
        &mut self,
        heading: WidgetText,
        contents: PluginClosure,
    ) -> CollapsingFfiResponse {
        let res = self.collapsing(heading, |ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
        CollapsingFfiResponse::new(res)
    }

    #[inline]
    fn indent(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .indent(self.next_auto_id(), |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn horizontal(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .horizontal(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn horizontal_centered(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .horizontal_centered(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn horizontal_top(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .horizontal_top(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn horizontal_wrapped(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .horizontal_wrapped(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn vertical(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .vertical(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn vertical_centered(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .vertical_centered(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn vertical_centered_justified(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .vertical_centered_justified(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn with_layout(&mut self, layout: BunnyLayout, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .with_layout(layout.into(), |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn centered_and_justified(&mut self, contents: PluginClosure) -> BunnyResponse {
        let res = self
            .centered_and_justified(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn end_row(&mut self) {
        self.end_row();
    }

    #[inline]
    fn set_row_height(&mut self, height: f32) {
        self.set_row_height(height);
    }

    #[inline]
    fn with_visual_transform(
        &mut self,
        transform: TSTransform,
        contents: PluginClosure,
    ) -> BunnyResponse {
        let res = self
            .with_visual_transform(transform, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(res)
    }

    #[inline]
    fn menu_button(
        &mut self,
        text: WidgetText,
        contents: PluginClosure,
    ) -> Tuple2<BunnyResponse, bool> {
        let inner_res = self.menu_button(text, |ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
        Tuple2(
            BunnyResponse::new(inner_res.response),
            inner_res.inner.is_some(),
        )
    }

    #[inline]
    fn input(&self, closure: InputStateClosure) {
        self.ctx().input_mut(|i| {
            let mut b = BunnyInputState::new(i);
            closure.call(&mut b);
        });
    }

    #[inline]
    fn fonts_layout_job(&self, job: LayoutJob) -> BunnyGalley {
        self.ctx()
            .fonts_mut(|f| BunnyGalley::new(f.layout_job(job.into())))
    }

    #[inline]
    fn fonts_layout(
        &self,
        text: RStr<'_>,
        font_id: FontId,
        color: Color32,
        wrap_width: f32,
    ) -> BunnyGalley {
        self.ctx().fonts_mut(|f| {
            BunnyGalley::new(f.layout(text.into(), font_id.into(), color, wrap_width))
        })
    }

    #[inline]
    fn fonts_layout_no_wrap(&self, text: RStr<'_>, font_id: FontId, color: Color32) -> BunnyGalley {
        self.ctx()
            .fonts_mut(|f| BunnyGalley::new(f.layout_no_wrap(text.into(), font_id.into(), color)))
    }

    #[inline]
    fn fonts_layout_delayed_color(
        &self,
        text: RStr<'_>,
        font_id: FontId,
        wrap_width: f32,
    ) -> BunnyGalley {
        self.ctx().fonts_mut(|f| {
            BunnyGalley::new(f.layout_delayed_color(text.into(), font_id.into(), wrap_width))
        })
    }

    #[inline]
    fn read_response(&self, id: Id) -> ROption<BunnyResponse> {
        self.ctx()
            .read_response(id.into())
            .map(BunnyResponse::new)
            .into()
    }

    #[inline]
    fn layer_painter(&self, layer_id: LayerId) -> BunnyPainter {
        let painter = self.ctx().layer_painter(layer_id.into());
        BunnyPainter::new(painter)
    }

    #[inline]
    fn debug_painter(&self) -> BunnyPainter {
        let painter = self.ctx().debug_painter();
        BunnyPainter::new(painter)
    }

    #[inline]
    fn time(&self) -> f64 {
        self.ctx().time()
    }

    #[inline]
    fn copy_text(&self, text: RStr<'_>) {
        self.ctx().copy_text(text.into());
    }

    #[inline]
    fn cumulative_frame_nr(&self) -> u64 {
        self.ctx().cumulative_frame_nr()
    }

    #[inline]
    fn cumulative_pass_nr(&self) -> u64 {
        self.ctx().cumulative_pass_nr()
    }

    #[inline]
    fn try_load_texture(
        &self,
        uri: RStr,
        texture_options: TextureOptions,
        size_hint: SizeHint,
    ) -> ROption<TexturePoll> {
        self.ctx()
            .try_load_texture(uri.into(), texture_options.into(), size_hint.into())
            .ok()
            .map(|poll| poll.into())
            .into()
    }

    #[inline]
    fn area_show(&mut self, area: Area, contents: PluginClosure) -> BunnyResponse {
        area.show_impl(self, contents)
    }

    #[inline]
    fn collapsing_header_show(
        &mut self,
        collapsing_header: CollapsingHeader,
        contents: PluginClosure,
    ) -> CollapsingFfiResponse {
        collapsing_header.show_impl(self, contents)
    }

    #[inline]
    fn combo_box_show(
        &mut self,
        combo_box: ComboBox,
        contents: PluginClosure,
    ) -> Tuple2<BunnyResponse, bool> {
        combo_box.show_impl(self, contents)
    }

    #[inline]
    fn frame_show(&mut self, frame: Frame, contents: PluginClosure) -> BunnyResponse {
        frame.show_impl(self, contents)
    }

    #[inline]
    fn grid_show(&mut self, grid: Grid, contents: PluginClosure) -> BunnyResponse {
        grid.show_impl(self, contents)
    }

    #[inline]
    fn modal_show(&self, modal: Modal, contents: PluginClosure) -> ModalFfiResponse {
        modal.show_impl(self.ctx(), contents)
    }

    #[inline]
    fn central_panel_show(
        &mut self,
        central_panel: CentralPanel,
        contents: PluginClosure,
    ) -> BunnyResponse {
        central_panel.show_impl(self, contents)
    }

    #[inline]
    fn panel_show(&mut self, panel: Panel, contents: PluginClosure) -> BunnyResponse {
        panel.show_impl(self, contents)
    }

    #[inline]
    fn panel_show_animated(
        &mut self,
        panel: Panel,
        is_expanded: bool,
        contents: PluginClosure,
    ) -> ROption<BunnyResponse> {
        panel.show_animated_impl(self, is_expanded, contents)
    }

    #[inline]
    fn panel_show_animated_between(
        &mut self,
        is_expanded: bool,
        collapsed_panel: Panel,
        expanded_panel: Panel,
        contents: PanelAnimatedBetweenClosure,
    ) -> BunnyResponse {
        Panel::show_animated_between_impl(
            self,
            is_expanded,
            collapsed_panel,
            expanded_panel,
            contents,
        )
    }

    #[inline]
    fn popup_show(&mut self, popup: Popup, contents: PluginClosure) -> ROption<BunnyResponse> {
        popup.show_impl(self, contents)
    }

    #[inline]
    fn scroll_area_show(
        &mut self,
        scroll_area: ScrollArea,
        contents: PluginClosure,
    ) -> ScrollAreaFfiOutput {
        scroll_area.show_impl(self, contents)
    }

    #[inline]
    fn scroll_area_show_rows(
        &mut self,
        scroll_area: ScrollArea,
        row_height_sans_spacing: f32,
        total_rows: usize,
        contents: ScrollAreaRowsClosure,
    ) -> ScrollAreaFfiOutput {
        scroll_area.show_rows_impl(self, row_height_sans_spacing, total_rows, contents)
    }

    #[inline]
    fn sides_show(
        &mut self,
        sides: Sides,
        contents_left: PluginClosure,
        contents_right: PluginClosure,
    ) {
        sides.show_impl(self, contents_left, contents_right);
    }

    #[inline]
    fn window_show(
        &mut self,
        window: Window,
        contents: PluginClosure,
    ) -> ROption<Tuple2<BunnyResponse, bool>> {
        window.show_impl(self, contents)
    }
}

#[cfg(feature = "manager")]
UiFfiVTable_static!(static UIFFI_VT for egui::Ui);

#[repr(C)]
pub struct CollapsingFfiResponse {
    pub body_response: ROption<BunnyResponse>,
    pub header_response: BunnyResponse,
    pub openness: f32,
    pub body_returned: bool,
}

#[cfg(feature = "manager")]
impl CollapsingFfiResponse {
    #[inline]
    pub fn new<R>(response: egui::CollapsingResponse<R>) -> Self {
        Self {
            header_response: BunnyResponse::new(response.header_response),
            body_response: response.body_response.map(BunnyResponse::new).into(),
            openness: response.openness,
            body_returned: response.body_returned.is_some(),
        }
    }
}

#[repr(C)]
pub struct ScrollAreaFfiOutput {
    pub inner_rect: Rect,
    pub id: Id,
    pub offset: Vec2,
    pub velocity: Vec2,
    pub content_size: Vec2,
}

#[cfg(feature = "manager")]
impl ScrollAreaFfiOutput {
    #[inline]
    pub fn new<R>(output: egui::scroll_area::ScrollAreaOutput<R>) -> Self {
        let egui::scroll_area::ScrollAreaOutput {
            inner: _,
            id,
            state,
            content_size,
            inner_rect,
        } = output;
        Self {
            inner_rect,
            id: id.into(),
            offset: state.offset,
            velocity: state.velocity(),
            content_size,
        }
    }
}

#[repr(C)]
pub struct ModalFfiResponse {
    pub response: BunnyResponse,
    pub backdrop_response: BunnyResponse,
    pub is_top_modal: bool,
    pub any_popup_open: bool,
}

#[cfg(feature = "manager")]
impl ModalFfiResponse {
    #[inline]
    pub fn new<R>(response: egui::modal::ModalResponse<R>) -> Self {
        let egui::modal::ModalResponse {
            response,
            backdrop_response,
            inner: _,
            is_top_modal,
            any_popup_open,
        } = response;
        Self {
            response: BunnyResponse::new(response),
            backdrop_response: BunnyResponse::new(backdrop_response),
            is_top_modal,
            any_popup_open,
        }
    }
}
