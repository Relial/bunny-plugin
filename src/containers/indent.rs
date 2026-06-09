use abi_stable::std_types::{RArc, RHashMap};
use rapidhash::fast::RandomState;

use crate::{
    elements::{Container, UiContainer},
    input_state::PointerState,
    response::Response,
    ui::BunnyUi,
};

#[repr(C)]
pub(crate) struct Indent<'a> {
    contents: BunnyUi<'a>,
}

impl<'a> Indent<'a> {
    pub(crate) fn new(ui: BunnyUi<'a>) -> Self {
        Self { contents: ui }
    }
}

impl UiContainer for Indent<'_> {
    fn ui(
        self,
        ui: &mut egui::Ui,
        responses: &mut RHashMap<egui::Id, Response, RandomState>,
        pointer_state: RArc<PointerState>,
        id: egui::Id,
    ) -> crate::response::Response {
        let egui_resp = ui
            .indent(id, |ui| {
                self.contents.ui(ui, responses, pointer_state.clone());
            })
            .response;
        Response::new(id, egui_resp, pointer_state)
    }
}

impl<'a> From<Indent<'a>> for Container<'a> {
    fn from(value: Indent<'a>) -> Self {
        Self::Indent(value)
    }
}
