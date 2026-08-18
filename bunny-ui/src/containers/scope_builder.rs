use crate::{elements::Container, ui::BunnyUi, ui_builder::UiBuilder};

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

#[cfg(feature = "manager")]
impl crate::elements::UiContainer for ScopeBuilder<'_> {
    fn ui(
        self,
        ui: &mut egui::Ui,
        responses: &mut abi_stable::std_types::RHashMap<
            egui::Id,
            crate::response::Response,
            rapidhash::fast::RandomState,
        >,
        pointer_state: abi_stable::std_types::RArc<crate::input_state::PointerState>,
        id: egui::Id,
    ) -> crate::response::Response {
        let egui_resp = ui
            .scope_builder(self.ui_builder.into(), |ui| {
                self.contents.ui(ui, responses, pointer_state.clone());
            })
            .response;
        crate::response::Response::new(id, egui_resp, pointer_state)
    }
}

impl<'a> From<ScopeBuilder<'a>> for Container<'a> {
    fn from(value: ScopeBuilder<'a>) -> Self {
        Self::Scope(value)
    }
}
