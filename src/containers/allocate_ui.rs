use abi_stable::std_types::RHashMap;
use egui::{Id, Ui, Vec2};
use rapidhash::fast::RandomState;

use crate::{
    elements::{Container, UiContainer},
    input_state::Input,
    layout::Layout,
    response::Response,
    ui::BunnyUi,
};

#[repr(C)]
pub struct AllocateUi<'a> {
    contents: BunnyUi<'a>,
    desired_size: Vec2,
    layout: Layout,
}

impl<'a> AllocateUi<'a> {
    pub fn new(desired_size: Vec2, layout: Layout, ui: BunnyUi<'a>) -> Self {
        Self {
            contents: ui,
            desired_size,
            layout,
        }
    }
}

impl UiContainer for AllocateUi<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: Input,
        id: Id,
    ) -> Response {
        let egui_resp = ui
            .allocate_ui_with_layout(self.desired_size, self.layout.into(), |ui| {
                self.contents.ui(ui, responses, input.clone())
            })
            .response;
        Response::new(id, egui_resp, input)
    }
}

impl<'a> From<AllocateUi<'a>> for Container<'a> {
    fn from(value: AllocateUi<'a>) -> Self {
        Self::AllocateUi(value)
    }
}
