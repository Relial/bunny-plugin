use abi_stable::std_types::{
    RArc, RHashMap,
    ROption::{self, RSome},
};
use egui::{Ui, Vec2};
use rapidhash::fast::RandomState;

use crate::{
    elements::{Container, Id, UiContainer},
    input::PointerState,
    response::Response,
    ui::BunnyUi,
};

#[repr(C)]
pub struct AllocateUi<'a> {
    contents: BunnyUi<'a>,
    desired_size: Vec2,
    layout: ROption<crate::layout::Layout>,
}

impl<'a> AllocateUi<'a> {
    pub fn new(desired_size: Vec2, layout: Option<crate::layout::Layout>, ui: BunnyUi<'a>) -> Self {
        Self {
            contents: ui,
            desired_size,
            layout: layout.into(),
        }
    }
}

impl UiContainer for AllocateUi<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
    ) -> egui::Response {
        if let RSome(layout) = self.layout {
            ui.allocate_ui_with_layout(self.desired_size, layout.into(), |ui| {
                self.contents.ui(ui, responses, input)
            })
            .response
        } else {
            ui.allocate_ui(self.desired_size, |ui| {
                self.contents.ui(ui, responses, input)
            })
            .response
        }
    }
}

impl<'a> From<AllocateUi<'a>> for Container<'a> {
    fn from(value: AllocateUi<'a>) -> Self {
        Self::AllocateUi(value)
    }
}
