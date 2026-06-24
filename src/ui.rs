use abi_stable::{
    external_types::RRwLock,
    std_types::{RArc, RHashMap, RVec, Tuple2},
};
use egui::{Color32, Id, Rect, Sense, Ui, Vec2};
use rapidhash::fast::RandomState;

use crate::{
    align::Align,
    containers::{
        allocate_ui::AllocateUi, collapsing_header::CollapsingHeader, indent::Indent,
        scope_builder::ScopeBuilder,
    },
    elements::{Component, Container, MiscComponent, UiContainer, Widget},
    input_state::{Input, InputState, PointerState},
    layout::Layout,
    paint::paintlist::PaintList,
    painter::Painter,
    response::{InnerResponse, Response},
    style::{Interaction, Spacing, Style, Visuals},
    ui_builder::UiBuilder,
    widget_text::{RichText, WidgetText},
    widgets::{
        button::Button, checkbox::CheckBox, interact::Interact, label::Label, link::Link,
        radio_button::RadioButton, separator::Separator, spinner::Spinner,
    },
};

#[repr(C)]
pub struct BunnyUi<'a> {
    components: RVec<Tuple2<Id, Component<'a>>>,
    next_salt: u64,
    painter: Painter<'a>,
    pub layout: Layout,
    last_frame_responses: RArc<RHashMap<Id, Response, RandomState>>,
    input: Input,
    available_rect: Rect,
    pixels_per_point: f32,
    style: RArc<Style>,
    opacity_factor: f32,
    enabled: bool,
}

impl<'a> BunnyUi<'a> {
    pub fn ui(
        self,
        ui: &mut Ui,
        new_responses: &mut RHashMap<Id, Response, RandomState>,
        pointer_state: RArc<PointerState>,
    ) {
        self.style.to_egui(ui.style_mut());
        ui.set_opacity(self.opacity_factor);
        if !self.enabled {
            ui.disable();
        }
        for Tuple2(id, component) in self.components {
            let response = component.ui(ui, new_responses, pointer_state.clone(), id);
            new_responses.insert(id, response);
        }
    }

    pub fn new(
        initial_id: Id,
        last_frame_responses: RArc<RHashMap<Id, Response, RandomState>>,
        input: Input,
        paint_list: RArc<RRwLock<PaintList<'a>>>,
        available_rect: Rect,
        pixels_per_point: f32,
        style: Style,
    ) -> Self {
        Self {
            components: RVec::new(),
            next_salt: initial_id.value(),
            painter: Painter::new(paint_list, available_rect, pixels_per_point),
            layout: Layout::default(),
            last_frame_responses,
            input,
            available_rect,
            pixels_per_point,
            style: RArc::new(style),
            opacity_factor: 1.0,
            enabled: true,
        }
    }

    pub fn available_size(&self) -> Vec2 {
        self.available_rect.size()
    }

    pub fn available_width(&self) -> f32 {
        self.available_rect.width()
    }

    pub fn available_height(&self) -> f32 {
        self.available_rect.height()
    }

    pub fn available_rect(&self) -> Rect {
        self.available_rect
    }

    pub fn new_child(&mut self, layout: Option<Layout>) -> Self {
        let next_salt = self.next_salt;
        let id = self.next_id();
        self.next_salt = next_salt;
        let mut style = self.style.clone();
        if style.changed {
            RArc::make_mut(&mut style).changed = false;
        }
        BunnyUi {
            components: RVec::new(),
            next_salt: id.value(),
            painter: self.painter.clone(),
            layout: layout.unwrap_or(self.layout),
            last_frame_responses: self.last_frame_responses.clone(),
            input: self.input.clone(),
            available_rect: self.available_rect,
            pixels_per_point: self.pixels_per_point,
            style,
            opacity_factor: self.opacity_factor,
            enabled: self.enabled,
        }
    }

    pub fn scope<R>(
        &mut self,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(UiBuilder::new(), add_contents)
    }

    pub fn scope_builder<R>(
        &mut self,
        ui_builder: UiBuilder,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        let mut new = self.new_child(ui_builder.layout.into());
        let ret = add_contents(&mut new);
        let builder = ScopeBuilder::new(ui_builder, new);
        let response = self.add_component_auto_id(Container::Scope(builder));
        InnerResponse::new(ret, response)
    }

    pub fn with_layout<R>(
        &mut self,
        layout: Layout,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(UiBuilder::new().layout(layout), add_contents)
    }

    pub fn allocate_response(&mut self, desired_size: impl Into<Vec2>, sense: Sense) -> Response {
        let rect = self.allocate_space(desired_size);
        self.interact(rect, sense)
    }

    pub fn allocate_space(&mut self, desired_size: impl Into<Vec2>) -> Rect {
        let response =
            self.add_component_auto_id(MiscComponent::AllocateSpace(desired_size.into()));
        response.rect
    }

    pub fn allocate_ui<R>(
        &mut self,
        desired_size: impl Into<Vec2>,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        self.allocate_ui_with_layout(desired_size, self.layout, add_contents)
    }

    pub fn allocate_ui_with_layout<R>(
        &mut self,
        desired_size: impl Into<Vec2>,
        layout: Layout,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        let mut new = self.new_child(Some(layout));
        let ret = add_contents(&mut new);
        let allocate = AllocateUi::new(desired_size.into(), layout, new);
        let response = self.add_component_auto_id(Container::AllocateUi(allocate));
        InnerResponse::new(ret, response)
    }

    pub fn allocate_ui_at_rect<R>(
        &mut self,
        max_rect: Rect,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(UiBuilder::new().max_rect(max_rect), add_contents)
    }

    pub fn allocate_painter(
        &mut self,
        desired_size: impl Into<Vec2>,
        sense: Sense,
    ) -> (Response, Painter<'a>) {
        let response = self.allocate_response(desired_size, sense);
        let clip_rect = self.available_rect.intersect(response.rect);
        let painter = self.painter().with_clip_rect(clip_rect);
        (response, painter)
    }

    #[allow(private_bounds)]
    pub fn add_sized(
        &mut self,
        max_size: impl Into<Vec2>,
        widget: impl Into<Widget<'a>>,
    ) -> Response {
        let layout = Layout::centered_and_justified(self.layout.main_dir);
        self.allocate_ui_with_layout(max_size, layout, |ui| {
            ui.add_component_auto_id(widget.into())
        })
        .inner
    }

    fn next_id(&mut self) -> Id {
        let id = Id::new(self.next_salt);
        self.next_salt = self.next_salt.wrapping_add(1);
        id
    }

    pub(crate) fn add_component_auto_id(
        &mut self,
        component: impl Into<Component<'a>>,
    ) -> Response {
        let id = self.next_id();
        self.add_component(id, component);
        self.last_frame_responses
            .get(&id)
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn add_component(&mut self, id: Id, component: impl Into<Component<'a>>) {
        self.components.push((id, component.into()).into());
    }

    #[allow(private_bounds)]
    pub fn add(&mut self, widget: impl Into<Widget<'a>>) -> Response {
        self.add_component_auto_id(widget.into())
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn add_enabled_ui<R>(
        &mut self,
        enabled: bool,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        self.scope(|ui| {
            if !enabled {
                ui.disable();
            }
            add_contents(ui)
        })
    }

    pub fn label(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add(Label::new(text))
    }

    pub fn colored_label(
        &mut self,
        color: impl Into<Color32>,
        text: impl Into<RichText>,
    ) -> Response {
        self.add(Label::new(text.into().color(color.into())))
    }

    pub fn heading(&mut self, text: impl Into<RichText>) -> Response {
        self.add(Label::new(text.into().heading()))
    }

    pub fn monospace(&mut self, text: impl Into<RichText>) -> Response {
        self.add(Label::new(text.into().monospace()))
    }

    pub fn code(&mut self, text: impl Into<RichText>) -> Response {
        self.add(Label::new(text.into().monospace()))
    }

    pub fn small(&mut self, text: impl Into<RichText>) -> Response {
        self.add(Label::new(text.into().small()))
    }

    pub fn strong(&mut self, text: impl Into<RichText>) -> Response {
        self.add(Label::new(text.into().strong()))
    }

    pub fn weak(&mut self, text: impl Into<RichText>) -> Response {
        self.add(Label::new(text.into().weak()))
    }

    pub fn checkbox(&mut self, value: &'a mut bool, text: impl Into<WidgetText>) -> Response {
        self.add(CheckBox::new(value, text))
    }

    pub fn horizontal<R>(
        &mut self,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        let layout = if self.layout.prefer_right_to_left() {
            Layout::right_to_left(Align::Center)
        } else {
            Layout::left_to_right(Align::Center)
        };
        self.scope_builder(UiBuilder::new().layout(layout), add_contents)
    }

    pub fn vertical<R>(
        &mut self,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(
            UiBuilder::new().layout(Layout::top_down(Align::Min)),
            add_contents,
        )
    }

    pub fn vertical_centered<R>(
        &mut self,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(
            UiBuilder::new().layout(Layout::top_down(Align::Center)),
            add_contents,
        )
    }

    pub fn vertical_centered_justified<R>(
        &mut self,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(
            UiBuilder::new().layout(Layout::top_down(Align::Center).with_cross_justify(true)),
            add_contents,
        )
    }

    pub fn collapsing<R>(
        &mut self,
        text: impl Into<WidgetText>,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        CollapsingHeader::new(text).show(self, add_contents)
    }

    pub fn button(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add(Button::new(text))
    }

    pub fn small_button(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add(Button::new(text).small())
    }

    pub fn selectable_label(&mut self, selected: bool, text: impl Into<WidgetText>) -> Response {
        self.add(Button::selectable(selected, text))
    }

    pub fn selectable_value<Value: PartialEq>(
        &mut self,
        current_value: &mut Value,
        selected_value: Value,
        text: impl Into<WidgetText>,
    ) -> Response {
        let mut response = self.selectable_label(*current_value == selected_value, text);
        if response.clicked() && *current_value != selected_value {
            *current_value = selected_value;
            response.mark_changed();
        }
        response
    }

    pub fn separator(&mut self) -> Response {
        self.add(Separator::default())
    }

    pub fn add_space(&mut self, space: f32) {
        self.add_component_auto_id(MiscComponent::Space(space));
    }

    pub fn end_row(&mut self) {
        self.add_component_auto_id(MiscComponent::EndRow);
    }

    pub fn input<R>(&self, reader: impl FnOnce(&InputState) -> R) -> R {
        self.input.read(reader)
    }

    pub fn input_mut<R>(&self, writer: impl FnOnce(&mut InputState) -> R) -> R {
        self.input.write(writer)
    }

    pub fn painter(&self) -> &Painter<'a> {
        &self.painter
    }

    pub fn max_rect(&self) -> Rect {
        self.available_rect
    }

    pub fn interact(&mut self, rect: Rect, sense: Sense) -> Response {
        self.add(Interact::new(rect, sense))
    }

    pub fn link(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add(Link::new(text))
    }

    pub fn radio(&mut self, selected: bool, text: impl Into<WidgetText>) -> Response {
        self.add(RadioButton::new(selected, text))
    }

    pub fn radio_value<Value: PartialEq>(
        &mut self,
        current_value: &mut Value,
        selected_value: Value,
        text: impl Into<WidgetText>,
    ) -> Response {
        let mut response = self.radio(*current_value == selected_value, text);
        if response.clicked() && *current_value != selected_value {
            *current_value = selected_value;
            response.mark_changed();
        }
        response
    }

    pub fn painter_at(&self, rect: Rect) -> Painter<'a> {
        self.painter().with_clip_rect(rect)
    }

    pub fn spinner(&mut self) -> Response {
        self.add(Spinner::new())
    }

    pub fn response(&self, id: Id) -> Option<&Response> {
        self.last_frame_responses.get(&id)
    }

    #[inline]
    pub fn style(&self) -> &Style {
        &self.style
    }

    #[inline]
    pub fn style_mut(&mut self) -> &mut Style {
        let style = RArc::make_mut(&mut self.style);
        style.changed = true;
        style
    }

    #[inline]
    pub fn spacing(&self) -> &Spacing {
        self.style.spacing()
    }

    #[inline]
    pub fn spacing_mut(&mut self) -> &mut Spacing {
        self.style_mut().spacing_mut()
    }

    #[inline]
    pub fn interaction(&self) -> &Interaction {
        self.style.interaction()
    }

    #[inline]
    pub fn interaction_mut(&mut self) -> &mut Interaction {
        self.style_mut().interaction_mut()
    }

    #[inline]
    pub fn visuals(&self) -> &Visuals {
        self.style.visuals()
    }

    #[inline]
    pub fn visuals_mut(&mut self) -> &mut Visuals {
        self.style_mut().visuals_mut()
    }

    pub fn set_style(&mut self, style: impl Into<RArc<Style>>) {
        self.style = style.into()
    }

    #[inline]
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity_factor = opacity;
    }

    #[inline]
    pub fn indent<R>(
        &mut self,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        let mut new = self.new_child(None);
        let ret = add_contents(&mut new);
        let indent = Indent::new(new);
        let response = self.add_component_auto_id(Container::Indent(indent));
        InnerResponse::new(ret, response)
    }
}
