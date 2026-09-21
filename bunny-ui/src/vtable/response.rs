use abi_stable::std_types::ROption;
use egui::{Id, Pos2, Rect, Response, Sense, Vec2};
use vtable::{VBox, VRef, VRefMut, vtable};

use crate::{
    Align, LayerId, PointerButton, WidgetText,
    closure::{PluginClosure, PluginNoReturnClosure},
    response::BunnyResponse,
    style::ScrollAnimation,
    ui::BunnyUi,
};

#[vtable]
#[repr(C)]
pub struct ResponseFfiVTable {
    layer_id: fn(VRef<ResponseFfiVTable>) -> LayerId,
    id: fn(VRef<ResponseFfiVTable>) -> Id,
    rect: fn(VRef<ResponseFfiVTable>) -> Rect,
    interact_rect: fn(VRef<ResponseFfiVTable>) -> Rect,
    sense: fn(VRef<ResponseFfiVTable>) -> Sense,

    parent_id: fn(VRef<ResponseFfiVTable>) -> Id,
    clicked: fn(VRef<ResponseFfiVTable>) -> bool,
    clicked_by: fn(VRef<ResponseFfiVTable>, button: PointerButton) -> bool,
    secondary_clicked: fn(VRef<ResponseFfiVTable>) -> bool,
    middle_clicked: fn(VRef<ResponseFfiVTable>) -> bool,
    double_clicked: fn(VRef<ResponseFfiVTable>) -> bool,
    triple_clicked: fn(VRef<ResponseFfiVTable>) -> bool,
    double_clicked_by: fn(VRef<ResponseFfiVTable>, button: PointerButton) -> bool,
    triple_clicked_by: fn(VRef<ResponseFfiVTable>, button: PointerButton) -> bool,
    clicked_with_open_in_background: fn(VRef<ResponseFfiVTable>) -> bool,
    clicked_elsewhere: fn(VRef<ResponseFfiVTable>) -> bool,
    enabled: fn(VRef<ResponseFfiVTable>) -> bool,
    hovered: fn(VRef<ResponseFfiVTable>) -> bool,
    contains_pointer: fn(VRef<ResponseFfiVTable>) -> bool,
    gained_focus: fn(VRef<ResponseFfiVTable>) -> bool,
    lost_focus: fn(VRef<ResponseFfiVTable>) -> bool,
    request_focus: fn(VRef<ResponseFfiVTable>),
    surrender_focus: fn(VRef<ResponseFfiVTable>),
    drag_started: fn(VRef<ResponseFfiVTable>) -> bool,
    drag_started_by: fn(VRef<ResponseFfiVTable>, button: PointerButton) -> bool,
    dragged: fn(VRef<ResponseFfiVTable>) -> bool,
    dragged_by: fn(VRef<ResponseFfiVTable>, button: PointerButton) -> bool,
    drag_stopped: fn(VRef<ResponseFfiVTable>) -> bool,
    drag_stopped_by: fn(VRef<ResponseFfiVTable>, button: PointerButton) -> bool,
    drag_delta: fn(VRef<ResponseFfiVTable>) -> Vec2,
    total_drag_delta: fn(VRef<ResponseFfiVTable>) -> ROption<Vec2>,
    drag_motion: fn(VRef<ResponseFfiVTable>) -> Vec2,
    // dnd_set_drag_payload
    // dnd_hover_payload
    // dnd_release_payload
    interact_pointer_pos: fn(VRef<ResponseFfiVTable>) -> ROption<Pos2>,
    intrinsic_size: fn(VRef<ResponseFfiVTable>) -> ROption<Vec2>,
    set_intrinsic_size: fn(VRefMut<ResponseFfiVTable>, size: Vec2),
    hover_pos: fn(VRef<ResponseFfiVTable>) -> ROption<Pos2>,
    is_pointer_button_down_on: fn(VRef<ResponseFfiVTable>) -> bool,
    changed: fn(VRef<ResponseFfiVTable>) -> bool,
    mark_changed: fn(VRefMut<ResponseFfiVTable>),
    should_close: fn(VRef<ResponseFfiVTable>) -> bool,
    set_close: fn(VRefMut<ResponseFfiVTable>),
    on_hover_ui: fn(VRef<ResponseFfiVTable>, contents: PluginNoReturnClosure),
    on_disabled_hover_ui: fn(VRef<ResponseFfiVTable>, contents: PluginNoReturnClosure),
    on_hover_ui_at_pointer: fn(VRef<ResponseFfiVTable>, contents: PluginNoReturnClosure),
    show_tooltip_ui: fn(VRef<ResponseFfiVTable>, contents: PluginNoReturnClosure),
    show_tooltip_text: fn(VRef<ResponseFfiVTable>, text: WidgetText),
    is_tooltip_open: fn(VRef<ResponseFfiVTable>) -> bool,
    on_hover_text_at_pointer: fn(VRef<ResponseFfiVTable>, text: WidgetText),
    on_hover_text: fn(VRef<ResponseFfiVTable>, text: WidgetText),
    highlight: fn(VRefMut<ResponseFfiVTable>),
    on_disabled_hover_text: fn(VRef<ResponseFfiVTable>, text: WidgetText),
    // on_hover_cursor
    // on_hover_and_drag_cursor
    interact: fn(VRef<ResponseFfiVTable>, sense: Sense) -> BunnyResponse,
    scroll_to_me: fn(VRef<ResponseFfiVTable>, align: ROption<Align>),
    scroll_to_me_animation:
        fn(VRef<ResponseFfiVTable>, align: ROption<Align>, animation: ScrollAnimation),
    // widget_info
    // output_event
    // labelled_by
    context_menu: fn(VRef<ResponseFfiVTable>, contents: PluginClosure) -> ROption<BunnyResponse>,
    context_menu_opened: fn(VRef<ResponseFfiVTable>) -> bool,
    paint_debug_info: fn(VRef<ResponseFfiVTable>),

    drop: fn(VRefMut<ResponseFfiVTable>),
}

impl ResponseFfi for Response {
    #[inline]
    fn layer_id(&self) -> LayerId {
        self.layer_id.into()
    }

    #[inline]
    fn id(&self) -> Id {
        self.id
    }

    #[inline]
    fn rect(&self) -> Rect {
        self.rect
    }

    #[inline]
    fn interact_rect(&self) -> Rect {
        self.interact_rect
    }

    #[inline]
    fn sense(&self) -> Sense {
        self.sense
    }

    #[inline]
    fn parent_id(&self) -> Id {
        self.parent_id()
    }

    #[inline]
    fn clicked(&self) -> bool {
        self.clicked()
    }

    #[inline]
    fn clicked_by(&self, button: PointerButton) -> bool {
        self.clicked_by(button.into())
    }

    #[inline]
    fn secondary_clicked(&self) -> bool {
        self.secondary_clicked()
    }

    #[inline]
    fn middle_clicked(&self) -> bool {
        self.middle_clicked()
    }

    #[inline]
    fn double_clicked(&self) -> bool {
        self.double_clicked()
    }

    #[inline]
    fn triple_clicked(&self) -> bool {
        self.triple_clicked()
    }

    #[inline]
    fn double_clicked_by(&self, button: PointerButton) -> bool {
        self.double_clicked_by(button.into())
    }

    #[inline]
    fn triple_clicked_by(&self, button: PointerButton) -> bool {
        self.triple_clicked_by(button.into())
    }

    #[inline]
    fn clicked_with_open_in_background(&self) -> bool {
        self.clicked_with_open_in_background()
    }

    #[inline]
    fn clicked_elsewhere(&self) -> bool {
        self.clicked_elsewhere()
    }

    #[inline]
    fn enabled(&self) -> bool {
        self.enabled()
    }

    #[inline]
    fn hovered(&self) -> bool {
        self.hovered()
    }

    #[inline]
    fn contains_pointer(&self) -> bool {
        self.contains_pointer()
    }

    #[inline]
    fn gained_focus(&self) -> bool {
        self.gained_focus()
    }

    #[inline]
    fn lost_focus(&self) -> bool {
        self.lost_focus()
    }

    #[inline]
    fn request_focus(&self) {
        self.request_focus();
    }

    #[inline]
    fn surrender_focus(&self) {
        self.surrender_focus();
    }

    #[inline]
    fn drag_started(&self) -> bool {
        self.drag_started()
    }

    #[inline]
    fn drag_started_by(&self, button: PointerButton) -> bool {
        self.drag_started_by(button.into())
    }

    #[inline]
    fn dragged(&self) -> bool {
        self.dragged()
    }

    #[inline]
    fn dragged_by(&self, button: PointerButton) -> bool {
        self.dragged_by(button.into())
    }

    #[inline]
    fn drag_stopped(&self) -> bool {
        self.drag_stopped()
    }

    #[inline]
    fn drag_stopped_by(&self, button: PointerButton) -> bool {
        self.drag_stopped_by(button.into())
    }

    #[inline]
    fn drag_delta(&self) -> Vec2 {
        self.drag_delta()
    }

    #[inline]
    fn total_drag_delta(&self) -> ROption<Vec2> {
        self.total_drag_delta().into()
    }

    #[inline]
    fn drag_motion(&self) -> Vec2 {
        self.drag_motion()
    }

    #[inline]
    fn interact_pointer_pos(&self) -> ROption<Pos2> {
        self.interact_pointer_pos().into()
    }

    #[inline]
    fn intrinsic_size(&self) -> ROption<Vec2> {
        self.intrinsic_size().into()
    }

    #[inline]
    fn set_intrinsic_size(&mut self, size: Vec2) {
        self.set_intrinsic_size(size);
    }

    #[inline]
    fn hover_pos(&self) -> ROption<Pos2> {
        self.hover_pos().into()
    }

    #[inline]
    fn is_pointer_button_down_on(&self) -> bool {
        self.is_pointer_button_down_on()
    }

    #[inline]
    fn changed(&self) -> bool {
        self.changed()
    }

    #[inline]
    fn mark_changed(&mut self) {
        self.mark_changed();
    }

    #[inline]
    fn should_close(&self) -> bool {
        self.should_close()
    }

    #[inline]
    fn set_close(&mut self) {
        self.set_close();
    }

    #[inline]
    fn on_hover_ui(&self, contents: PluginNoReturnClosure) {
        egui::Tooltip::for_enabled(self).show(|ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
    }

    #[inline]
    fn on_disabled_hover_ui(&self, contents: PluginNoReturnClosure) {
        egui::Tooltip::for_disabled(self).show(|ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
    }

    #[inline]
    fn on_hover_ui_at_pointer(&self, contents: PluginNoReturnClosure) {
        egui::Tooltip::for_enabled(self)
            .at_pointer()
            .gap(12.0)
            .show(|ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            });
    }

    #[inline]
    fn show_tooltip_ui(&self, contents: PluginNoReturnClosure) {
        self.show_tooltip_ui(|ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
    }

    #[inline]
    fn show_tooltip_text(&self, text: WidgetText) {
        self.show_tooltip_text(text);
    }

    #[inline]
    fn is_tooltip_open(&self) -> bool {
        self.is_tooltip_open()
    }

    #[inline]
    fn on_hover_text_at_pointer(&self, text: WidgetText) {
        egui::Tooltip::for_enabled(self)
            .at_pointer()
            .gap(12.0)
            .show(|ui| {
                ui.set_max_width(ui.spacing().tooltip_width);
                ui.label(text)
            });
    }

    #[inline]
    fn on_hover_text(&self, text: WidgetText) {
        egui::Tooltip::for_enabled(self).show(|ui| {
            ui.set_max_width(ui.spacing().tooltip_width);
            ui.label(text)
        });
    }

    #[inline]
    fn highlight(&mut self) {
        self.ctx.highlight_widget(self.id);
        self.flags.set(egui::response::Flags::HIGHLIGHTED, true);
    }

    #[inline]
    fn on_disabled_hover_text(&self, text: WidgetText) {
        egui::Tooltip::for_disabled(self).show(|ui| {
            ui.set_max_width(ui.spacing().tooltip_width);
            ui.label(text);
        });
    }

    #[inline]
    fn interact(&self, sense: Sense) -> BunnyResponse {
        let res = self.interact(sense);
        BunnyResponse::new(res)
    }

    #[inline]
    fn scroll_to_me(&self, align: ROption<Align>) {
        self.scroll_to_me(align.map(|a| a.into()).into_option());
    }

    #[inline]
    fn scroll_to_me_animation(&self, align: ROption<Align>, animation: ScrollAnimation) {
        self.scroll_to_me_animation(align.map(|a| a.into()).into_option(), animation.into());
    }

    #[inline]
    fn context_menu(&self, contents: PluginClosure) -> ROption<BunnyResponse> {
        let res = self.context_menu(|ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
        res.map(|i| BunnyResponse::new(i.response)).into()
    }

    #[inline]
    fn context_menu_opened(&self) -> bool {
        self.context_menu_opened()
    }

    #[inline]
    fn paint_debug_info(&self) {
        self.paint_debug_info();
    }
}

ResponseFfiVTable_static!(static RESPONSEFFI_VT for Response);
