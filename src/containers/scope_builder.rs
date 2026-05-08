use abi_stable::std_types::{RArc, RHashMap};
use egui::Ui;
use rapidhash::fast::RandomState;

use crate::{
    elements::{Container, Id, UiContainer},
    input::PointerState,
    response::Response,
    ui::BunnyUi,
    ui_builder::UiBuilder,
};

#[repr(C)]
pub struct ScopeBuilder<'a> {
    contents: BunnyUi<'a>,
    ui_builder: UiBuilder,
}

impl<'a> ScopeBuilder<'a> {
    pub fn new(ui_builder: UiBuilder, ui: BunnyUi<'a>) -> Self {
        Self {
            contents: ui,
            ui_builder,
        }
    }
}

impl UiContainer for ScopeBuilder<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
        id: Id,
    ) -> Response {
        let egui_resp = ui
            .scope_builder(self.ui_builder.into(), |ui| {
                self.contents.ui(ui, responses, input.clone());
            })
            .response;
        Response::new(id, egui_resp, input)
    }
}

impl<'a> From<ScopeBuilder<'a>> for Container<'a> {
    fn from(value: ScopeBuilder<'a>) -> Self {
        Self::Scope(value)
    }
}
