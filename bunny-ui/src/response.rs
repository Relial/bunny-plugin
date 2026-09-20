use std::mem::MaybeUninit;

use egui::{Id, Pos2, Rect, Sense, Vec2};
use mint::Vector2;
use vtable::VBox;

use crate::{
    Align, PointerButton, WidgetText,
    closure::{PluginClosure, PluginNoReturnClosure},
    style::ScrollAnimation,
    ui::BunnyUi,
    vtable::response::ResponseFfiVTable,
};

#[repr(transparent)]
pub struct BunnyResponse {
    inner: VBox<ResponseFfiVTable>,
}

impl BunnyResponse {
    #[inline]
    pub fn new(response: egui::Response) -> Self {
        Self {
            inner: VBox::new(response),
        }
    }
}

impl BunnyResponse {
    #[inline]
    pub fn id(&self) -> Id {
        self.inner.id()
    }

    #[inline]
    pub fn rect(&self) -> Rect {
        self.inner.rect()
    }

    #[inline]
    pub fn interact_rect(&self) -> Rect {
        self.inner.interact_rect()
    }

    #[inline]
    pub fn sense(&self) -> Sense {
        self.inner.sense()
    }

    #[inline]
    pub fn parent_id(&self) -> Id {
        self.inner.parent_id()
    }

    #[inline]
    pub fn clicked(&self) -> bool {
        self.inner.clicked()
    }

    #[inline]
    pub fn clicked_by(&self, button: PointerButton) -> bool {
        self.inner.clicked_by(button)
    }

    #[inline]
    pub fn secondary_clicked(&self) -> bool {
        self.inner.secondary_clicked()
    }

    #[inline]
    pub fn middle_clicked(&self) -> bool {
        self.inner.middle_clicked()
    }

    #[inline]
    pub fn double_clicked(&self) -> bool {
        self.inner.double_clicked()
    }

    #[inline]
    pub fn triple_clicked(&self) -> bool {
        self.inner.triple_clicked()
    }

    #[inline]
    pub fn double_clicked_by(&self, button: PointerButton) -> bool {
        self.inner.double_clicked_by(button)
    }

    #[inline]
    pub fn triple_clicked_by(&self, button: PointerButton) -> bool {
        self.inner.triple_clicked_by(button)
    }

    #[inline]
    pub fn clicked_with_open_in_background(&self) -> bool {
        self.inner.clicked_with_open_in_background()
    }

    #[inline]
    pub fn clicked_elsewhere(&self) -> bool {
        self.inner.clicked_elsewhere()
    }

    #[inline]
    pub fn enabled(&self) -> bool {
        self.inner.enabled()
    }

    #[inline]
    pub fn hovered(&self) -> bool {
        self.inner.hovered()
    }

    #[inline]
    pub fn contains_pointer(&self) -> bool {
        self.inner.contains_pointer()
    }

    #[inline]
    pub fn gained_focus(&self) -> bool {
        self.inner.gained_focus()
    }

    #[inline]
    pub fn lost_focus(&self) -> bool {
        self.inner.lost_focus()
    }

    #[inline]
    pub fn request_focus(&self) {
        self.inner.request_focus();
    }

    #[inline]
    pub fn surrender_focus(&self) {
        self.inner.surrender_focus();
    }

    #[inline]
    pub fn drag_started(&self) -> bool {
        self.inner.drag_started()
    }

    #[inline]
    pub fn drag_started_by(&self, button: PointerButton) -> bool {
        self.inner.drag_started_by(button)
    }

    #[inline]
    pub fn dragged(&self) -> bool {
        self.inner.dragged()
    }

    #[inline]
    pub fn dragged_by(&self, button: PointerButton) -> bool {
        self.inner.dragged_by(button)
    }

    #[inline]
    pub fn drag_stopped(&self) -> bool {
        self.inner.drag_stopped()
    }

    #[inline]
    pub fn drag_stopped_by(&self, button: PointerButton) -> bool {
        self.inner.drag_stopped_by(button)
    }

    #[inline]
    pub fn drag_delta(&self) -> Vec2 {
        self.inner.drag_delta()
    }

    #[inline]
    pub fn total_drag_delta(&self) -> Option<Vec2> {
        self.inner.total_drag_delta().into_option()
    }

    #[inline]
    pub fn drag_motion(&self) -> Vec2 {
        self.inner.drag_motion()
    }

    #[inline]
    pub fn interact_pointer_pos(&self) -> Option<Pos2> {
        self.inner.interact_pointer_pos().into_option()
    }

    #[inline]
    pub fn intrinsic_size(&self) -> Option<Vec2> {
        self.inner.intrinsic_size().into_option()
    }

    #[inline]
    pub fn set_intrinsic_size(&mut self, size: impl Into<Vector2<f32>>) {
        self.inner.set_intrinsic_size(size.into().into());
    }

    #[inline]
    pub fn hover_pos(&self) -> Option<Pos2> {
        self.inner.hover_pos().into_option()
    }

    #[inline]
    pub fn is_pointer_button_down_on(&self) -> bool {
        self.inner.is_pointer_button_down_on()
    }

    #[inline]
    pub fn changed(&self) -> bool {
        self.inner.changed()
    }

    #[inline]
    pub fn mark_changed(&mut self) {
        self.inner.mark_changed();
    }

    #[inline]
    pub fn should_close(&self) -> bool {
        self.inner.should_close()
    }

    #[inline]
    pub fn set_close(&mut self) {
        self.inner.set_close();
    }

    #[inline]
    pub fn on_hover_ui(self, mut add_contents: impl FnMut(&mut BunnyUi)) -> Self {
        let closure = PluginNoReturnClosure::new(&mut add_contents);
        self.inner.on_hover_ui(closure);
        self
    }

    #[inline]
    pub fn on_disabled_hover_ui(self, mut add_contents: impl FnMut(&mut BunnyUi)) -> Self {
        let closure = PluginNoReturnClosure::new(&mut add_contents);
        self.inner.on_disabled_hover_ui(closure);
        self
    }

    #[inline]
    pub fn on_hover_ui_at_pointer(self, mut add_contents: impl FnMut(&mut BunnyUi)) -> Self {
        let closure = PluginNoReturnClosure::new(&mut add_contents);
        self.inner.on_hover_ui_at_pointer(closure);
        self
    }

    #[inline]
    pub fn show_tooltip_ui(&self, mut add_contents: impl FnMut(&mut BunnyUi)) {
        let closure = PluginNoReturnClosure::new(&mut add_contents);
        self.inner.show_tooltip_ui(closure);
    }

    #[inline]
    pub fn show_tooltip_text(&self, text: impl Into<WidgetText>) {
        self.inner.show_tooltip_text(text.into());
    }

    #[inline]
    pub fn is_tooltip_open(&self) -> bool {
        self.inner.is_tooltip_open()
    }

    #[inline]
    pub fn on_hover_text_at_pointer(self, text: impl Into<WidgetText>) -> Self {
        self.inner.on_hover_text_at_pointer(text.into());
        self
    }

    #[inline]
    pub fn on_hover_text(self, text: impl Into<WidgetText>) -> Self {
        self.inner.on_hover_text(text.into());
        self
    }

    #[inline]
    pub fn highlight(mut self) -> Self {
        self.inner.highlight();
        self
    }

    #[inline]
    pub fn on_disabled_hover_text(self, text: impl Into<WidgetText>) -> Self {
        self.inner.on_disabled_hover_text(text.into());
        self
    }

    #[inline]
    pub fn interact(&self, sense: Sense) -> Self {
        self.inner.interact(sense)
    }

    #[inline]
    pub fn scroll_to_me(&self, align: Option<Align>) {
        self.inner.scroll_to_me(align.into());
    }

    #[inline]
    pub fn scroll_to_me_animation(&self, align: Option<Align>, animation: ScrollAnimation) {
        self.inner.scroll_to_me_animation(align.into(), animation);
    }

    #[inline]
    pub fn context_menu<R>(
        &self,
        mut add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> Option<BunnyInnerResponse<R>> {
        let mut ret = MaybeUninit::<R>::uninit();
        let closure = PluginClosure::new(&mut add_contents, &mut ret);
        let response = self.inner.context_menu(closure);
        response
            .map(|response| {
                let inner = unsafe { ret.assume_init() };
                BunnyInnerResponse::new(inner, response)
            })
            .into_option()
    }

    #[inline]
    pub fn context_menu_opened(&self) -> bool {
        self.inner.context_menu_opened()
    }

    #[inline]
    pub fn paint_debug_info(&self) {
        self.inner.paint_debug_info();
    }
}

pub struct BunnyInnerResponse<R> {
    pub response: BunnyResponse,
    pub inner: R,
}

impl<R> BunnyInnerResponse<R> {
    #[inline]
    pub fn new(inner: R, response: BunnyResponse) -> Self {
        Self { inner, response }
    }
}
