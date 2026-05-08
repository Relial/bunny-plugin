use std::hash::Hash;

use abi_stable::std_types::{RArc, RHashMap, RVec, Tuple2};
use egui::{Color32, Ui, Vec2};
use rapidhash::fast::RandomState;

use crate::{
    align::Align,
    collapsing_header::CollapsingHeader,
    elements::{
        AllocateUi, Button, CheckBox, Component, Container, Grid, Id, Label, MiscComponent,
        ScopeBuilder, Separator, UiContainer, Widget,
    },
    input::PointerState,
    layout::Layout,
    painter::Painter,
    response::{InnerResponse, Response},
    ui_builder::UiBuilder,
    widget_text::{RichText, WidgetText},
};

#[repr(C)]
pub struct BunnyUi<'a> {
    components: RVec<Tuple2<Id, Component<'a>>>,
    pub painter: Painter<'a>,
    next_salt: u64,
    pub layout: Layout,
    last_frame_responses: RArc<RHashMap<Id, Response, RandomState>>,
    input: RArc<PointerState>,
}

impl<'a> BunnyUi<'a> {
    pub fn ui(
        self,
        ui: &mut Ui,
        new_responses: &mut RHashMap<Id, Response, RandomState>,
        new_input: RArc<PointerState>,
    ) {
        for component in self.components {
            let egui_resp = component.1.ui(ui, new_responses, new_input.clone());
            let resp = Response::new(component.0, egui_resp, new_input.clone());
            new_responses.insert(component.0, resp);
        }
        self.painter.ui(ui);
    }

    pub fn new(
        initial_id: Id,
        last_frame_responses: RArc<RHashMap<Id, Response, RandomState>>,
        last_frame_input: RArc<PointerState>,
    ) -> Self {
        Self {
            components: RVec::new(),
            painter: Painter::new(),
            next_salt: initial_id.value(),
            layout: Layout::default(),
            last_frame_responses,
            input: last_frame_input,
        }
    }

    pub fn new_child(&mut self, layout: Option<Layout>) -> Self {
        let next_salt = self.next_salt;
        let id = self.next_id();
        self.next_salt = next_salt;
        BunnyUi {
            components: RVec::new(),
            painter: Painter::new(),
            next_salt: id.value(),
            layout: layout.unwrap_or(self.layout),
            last_frame_responses: self.last_frame_responses.clone(),
            input: self.input.clone(),
        }
    }

    pub fn scope<R>(&mut self, add_contents: impl FnOnce(&mut BunnyUi) -> R) -> InnerResponse<R> {
        self.scope_builder(UiBuilder::new(), add_contents)
    }

    pub fn scope_builder<R>(
        &mut self,
        ui_builder: UiBuilder,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        let mut new = self.new_child(ui_builder.layout.into());
        let ret = add_contents(&mut new);
        let builder = ScopeBuilder::new(ui_builder, new);
        let response = self.add_component(Container::Scope(builder));
        InnerResponse::new(ret, response)
    }

    pub fn with_layout<R>(
        &mut self,
        layout: Layout,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(UiBuilder::new().layout(layout), add_contents)
    }

    pub fn allocate_ui<R>(
        &mut self,
        desired_size: impl Into<Vec2>,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        self.allocate_ui_with_layout(desired_size, self.layout, add_contents)
    }

    pub fn allocate_ui_with_layout<R>(
        &mut self,
        desired_size: impl Into<Vec2>,
        layout: Layout,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        let mut new = self.new_child(Some(layout));
        let ret = add_contents(&mut new);
        let allocate = AllocateUi::new(desired_size.into(), Some(layout), new);
        let response = self.add_component(Container::AllocateUi(allocate));
        InnerResponse::new(ret, response)
    }

    pub fn add_sized(&mut self, max_size: impl Into<Vec2>, widget: impl Into<Widget>) -> Response {
        let layout = Layout::centered_and_justified(self.layout.main_dir);
        self.allocate_ui_with_layout(max_size, layout, |ui| ui.add_component(widget.into()))
            .inner
    }

    fn next_id(&mut self) -> Id {
        let id = Id::new(self.next_salt);
        self.next_salt = self.next_salt.wrapping_add(1);
        id
    }

    pub(crate) fn add_component(&mut self, component: impl Into<Component<'a>>) -> Response {
        let id = self.next_id();
        self.components.push((id, component.into()).into());
        self.last_frame_responses
            .get(&id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn add(&mut self, widget: impl Into<Widget>) -> Response {
        self.add_component(widget.into())
    }

    pub fn disable(&mut self) {
        self.add_component(MiscComponent::Disable);
    }

    pub fn add_enabled_ui<R>(
        &mut self,
        enabled: bool,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
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

    pub fn checkbox(&mut self, value: &mut bool, text: impl Into<WidgetText>) -> Response {
        self.add(CheckBox::new(value, text))
    }

    pub fn horizontal<R>(
        &mut self,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
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
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(
            UiBuilder::new().layout(Layout::top_down(Align::Min)),
            add_contents,
        )
    }

    pub fn vertical_centered<R>(
        &mut self,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(
            UiBuilder::new().layout(Layout::top_down(Align::Center)),
            add_contents,
        )
    }

    pub fn vertical_centered_justified<R>(
        &mut self,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        self.scope_builder(
            UiBuilder::new().layout(Layout::top_down(Align::Center).with_cross_justify(true)),
            add_contents,
        )
    }

    pub fn collapsing<R>(
        &mut self,
        text: impl Into<WidgetText>,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
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

    pub fn separator(&mut self) -> Response {
        self.add(Separator::default())
    }

    pub fn add_space(&mut self, space: f32) {
        self.add_component(MiscComponent::Space(space));
    }

    pub fn grid<R>(
        &mut self,
        id: impl Hash,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        let mut new = self.new_child(Some(Layout::top_down(Align::Min)));
        let ret = add_contents(&mut new);
        let response = self.add_component(Container::Grid(Grid::new(id, new)));
        InnerResponse::new(ret, response)
    }

    pub fn end_row(&mut self) {
        self.add_component(MiscComponent::EndRow);
    }

    pub fn input(&self) -> &PointerState {
        &self.input
    }
}
