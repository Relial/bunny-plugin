use emath::Vec2;

use crate::{elements::Container, layout::Layout, ui::BunnyUi};

#[repr(C)]
pub struct AllocateUi<'a> {
    contents: BunnyUi<'a>,
    layout: Layout,
    desired_size: Vec2,
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

#[cfg(feature = "manager")]
impl crate::elements::UiContainer for AllocateUi<'_> {
    fn ui(
        self,
        ui: &mut egui::Ui,
        responses: &mut abi_stable::std_types::RHashMap<
            crate::Id,
            crate::response::Response,
            rapidhash::fast::RandomState,
        >,
        pointer_state: abi_stable::std_types::RArc<crate::input_state::PointerState>,
        id: crate::Id,
    ) -> crate::response::Response {
        let egui_resp = ui
            .allocate_ui_with_layout(self.desired_size, self.layout.into(), |ui| {
                self.contents.ui(ui, responses, pointer_state.clone())
            })
            .response;
        crate::response::Response::new(id, egui_resp, pointer_state)
    }
}

impl<'a> From<AllocateUi<'a>> for Container<'a> {
    fn from(value: AllocateUi<'a>) -> Self {
        Self::AllocateUi(value)
    }
}
