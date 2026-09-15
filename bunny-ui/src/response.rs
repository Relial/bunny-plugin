use egui::{Id, Pos2, Rect, Sense, Vec2};
use mint::Vector2;
use vtable::VBox;

use crate::{
    Align, PointerButton, WidgetText, closure::PluginClosure, style::ScrollAnimation, ui::BunnyUi,
    vtable::response::ResponseFfiVTable,
};

#[repr(C)]
pub struct BunnyResponse(VBox<ResponseFfiVTable>);

impl BunnyResponse {
    #[inline]
    pub fn id(&self) -> Id {
        self.0.id()
    }

    #[inline]
    pub fn rect(&self) -> Rect {
        self.0.rect()
    }

    #[inline]
    pub fn interact_rect(&self) -> Rect {
        self.0.interact_rect()
    }

    #[inline]
    pub fn sense(&self) -> Sense {
        self.0.sense()
    }

    #[inline]
    pub fn parent_id(&self) -> Id {
        self.0.parent_id()
    }

    #[inline]
    pub fn clicked(&self) -> bool {
        self.0.clicked()
    }

    #[inline]
    pub fn clicked_by(&self, button: PointerButton) -> bool {
        self.0.clicked_by(button)
    }

    #[inline]
    pub fn secondary_clicked(&self) -> bool {
        self.0.secondary_clicked()
    }

    #[inline]
    pub fn middle_clicked(&self) -> bool {
        self.0.middle_clicked()
    }

    #[inline]
    pub fn double_clicked(&self) -> bool {
        self.0.double_clicked()
    }

    #[inline]
    pub fn triple_clicked(&self) -> bool {
        self.0.triple_clicked()
    }

    #[inline]
    pub fn double_clicked_by(&self, button: PointerButton) -> bool {
        self.0.double_clicked_by(button)
    }

    #[inline]
    pub fn triple_clicked_by(&self, button: PointerButton) -> bool {
        self.0.triple_clicked_by(button)
    }

    #[inline]
    pub fn clicked_with_open_in_background(&self) -> bool {
        self.0.clicked_with_open_in_background()
    }

    #[inline]
    pub fn clicked_elsewhere(&self) -> bool {
        self.0.clicked_elsewhere()
    }

    #[inline]
    pub fn enabled(&self) -> bool {
        self.0.enabled()
    }

    #[inline]
    pub fn hovered(&self) -> bool {
        self.0.hovered()
    }

    #[inline]
    pub fn contains_pointer(&self) -> bool {
        self.0.contains_pointer()
    }

    #[inline]
    pub fn gained_focus(&self) -> bool {
        self.0.gained_focus()
    }

    #[inline]
    pub fn lost_focus(&self) -> bool {
        self.0.lost_focus()
    }

    #[inline]
    pub fn request_focus(&self) {
        self.0.request_focus();
    }

    #[inline]
    pub fn surrender_focus(&self) {
        self.0.surrender_focus();
    }

    #[inline]
    pub fn drag_started(&self) -> bool {
        self.0.drag_started()
    }

    #[inline]
    pub fn drag_started_by(&self, button: PointerButton) -> bool {
        self.0.drag_started_by(button)
    }

    #[inline]
    pub fn dragged(&self) -> bool {
        self.0.dragged()
    }

    #[inline]
    pub fn dragged_by(&self, button: PointerButton) -> bool {
        self.0.dragged_by(button)
    }

    #[inline]
    pub fn drag_stopped(&self) -> bool {
        self.0.drag_stopped()
    }

    #[inline]
    pub fn drag_stopped_by(&self, button: PointerButton) -> bool {
        self.0.drag_stopped_by(button)
    }

    #[inline]
    pub fn drag_delta(&self) -> Vec2 {
        self.0.drag_delta()
    }

    #[inline]
    pub fn total_drag_delta(&self) -> Option<Vec2> {
        self.0.total_drag_delta().into_option()
    }

    #[inline]
    pub fn drag_motion(&self) -> Vec2 {
        self.0.drag_motion()
    }

    #[inline]
    pub fn interact_pointer_pos(&self) -> Option<Pos2> {
        self.0.interact_pointer_pos().into_option()
    }

    #[inline]
    pub fn intrinsic_size(&self) -> Option<Vec2> {
        self.0.intrinsic_size().into_option()
    }

    #[inline]
    pub fn set_intrinsic_size(&mut self, size: impl Into<Vector2<f32>>) {
        self.0.set_intrinsic_size(size.into().into());
    }

    #[inline]
    pub fn hover_pos(&self) -> Option<Pos2> {
        self.0.hover_pos().into_option()
    }

    #[inline]
    pub fn is_pointer_button_down_on(&self) -> bool {
        self.0.is_pointer_button_down_on()
    }

    #[inline]
    pub fn changed(&self) -> bool {
        self.0.changed()
    }

    #[inline]
    pub fn mark_changed(&mut self) {
        self.0.mark_changed();
    }

    #[inline]
    pub fn should_close(&self) -> bool {
        self.0.should_close()
    }

    #[inline]
    pub fn set_close(&mut self) {
        self.0.set_close();
    }

    #[inline]
    pub fn on_hover_ui(self, mut add_contents: impl FnMut(&mut BunnyUi)) -> Self {
        let closure = PluginClosure::new(&mut add_contents);
        self.0.on_hover_ui(closure);
        self
    }

    #[inline]
    pub fn on_disabled_hover_ui(self, mut add_contents: impl FnMut(&mut BunnyUi)) -> Self {
        let closure = PluginClosure::new(&mut add_contents);
        self.0.on_disabled_hover_ui(closure);
        self
    }

    #[inline]
    pub fn on_hover_ui_at_pointer(self, mut add_contents: impl FnMut(&mut BunnyUi)) -> Self {
        let closure = PluginClosure::new(&mut add_contents);
        self.0.on_hover_ui_at_pointer(closure);
        self
    }

    #[inline]
    pub fn show_tooltip_ui(&self, mut add_contents: impl FnMut(&mut BunnyUi)) {
        let closure = PluginClosure::new(&mut add_contents);
        self.0.show_tooltip_ui(closure);
    }

    #[inline]
    pub fn show_tooltip_text(&self, text: impl Into<WidgetText>) {
        self.0.show_tooltip_text(text.into());
    }

    #[inline]
    pub fn is_tooltip_open(&self) -> bool {
        self.0.is_tooltip_open()
    }

    #[inline]
    pub fn on_hover_text_at_pointer(self, text: impl Into<WidgetText>) -> Self {
        self.0.on_hover_text_at_pointer(text.into());
        self
    }

    #[inline]
    pub fn on_hover_text(self, text: impl Into<WidgetText>) -> Self {
        self.0.on_hover_text(text.into());
        self
    }

    #[inline]
    pub fn highlight(mut self) -> Self {
        self.0.highlight();
        self
    }

    #[inline]
    pub fn on_disabled_hover_text(self, text: impl Into<WidgetText>) -> Self {
        self.0.on_disabled_hover_text(text.into());
        self
    }

    #[inline]
    pub fn interact(&self, sense: Sense) -> Self {
        self.0.interact(sense)
    }

    #[inline]
    pub fn scroll_to_me(&self, align: Option<Align>) {
        self.0.scroll_to_me(align.into());
    }

    #[inline]
    pub fn scroll_to_me_animation(&self, align: Option<Align>, animation: ScrollAnimation) {
        self.0.scroll_to_me_animation(align.into(), animation);
    }

    #[inline]
    pub fn context_menu(&self, mut add_contents: impl FnMut(&mut BunnyUi)) -> Option<Self> {
        let closure = PluginClosure::new(&mut add_contents);
        self.0.context_menu(closure).into_option()
    }

    #[inline]
    pub fn context_menu_opened(&self) -> bool {
        self.0.context_menu_opened()
    }

    #[inline]
    pub fn paint_debug_info(&self) {
        self.0.paint_debug_info();
    }
}

impl From<VBox<ResponseFfiVTable>> for BunnyResponse {
    #[inline]
    fn from(value: VBox<ResponseFfiVTable>) -> Self {
        Self(value)
    }
}

#[repr(C)]
pub struct BunnyInnerResponse<R> {
    pub inner: R,
    pub response: BunnyResponse,
}
