use crate::{elements::Container, ui::BunnyUi};

#[repr(C)]
pub(crate) struct Indent<'a> {
    contents: BunnyUi<'a>,
}

impl<'a> Indent<'a> {
    pub(crate) fn new(ui: BunnyUi<'a>) -> Self {
        Self { contents: ui }
    }
}

#[cfg(feature = "manager")]
impl crate::elements::UiContainer for Indent<'_> {
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
            .indent(id, |ui| {
                self.contents.ui(ui, responses, pointer_state.clone());
            })
            .response;
        crate::response::Response::new(id, egui_resp, pointer_state)
    }
}

impl<'a> From<Indent<'a>> for Container<'a> {
    fn from(value: Indent<'a>) -> Self {
        Self::Indent(value)
    }
}
