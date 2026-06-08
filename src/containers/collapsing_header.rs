use std::hash::Hash;

use abi_stable::std_types::{
    RHashMap,
    ROption::{self, RNone, RSome},
};
use egui::{Id, Ui};
use rapidhash::fast::RandomState;

use crate::{
    elements::{Container, UiContainer},
    input_state::Input,
    layout::Layout,
    response::{InnerResponse, Response},
    ui::BunnyUi,
    widget_text::WidgetText,
};

#[repr(C)]
pub struct CollapsingHeader {
    text: WidgetText,
    id: ROption<Id>,
    default_open: bool,
    open: ROption<bool>,
    show_background: bool,
    indented: bool,
}

impl CollapsingHeader {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        let text = text.into();
        Self {
            text,
            id: RNone,
            default_open: false,
            open: RNone,
            show_background: false,
            indented: true,
        }
    }

    #[inline]
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    #[inline]
    pub fn open(mut self, open: Option<bool>) -> Self {
        self.open = open.into();
        self
    }

    #[inline]
    pub fn id_salt(mut self, id_salt: impl Hash) -> Self {
        self.id = RSome(Id::new(id_salt));
        self
    }

    #[inline]
    pub fn show_background(mut self, show_background: bool) -> Self {
        self.show_background = show_background;
        self
    }

    #[inline]
    pub fn indented(mut self, indented: bool) -> Self {
        self.indented = indented;
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        let mut new = ui.new_child(Some(Layout::default()));
        let ret = add_contents(&mut new);
        let response =
            ui.add_component_auto_id(Container::CollapsingHeader(CollapsingHeaderComponent {
                collapsing_header: self,
                contents: new,
            }));
        InnerResponse::new(ret, response)
    }
}

#[repr(C)]
pub struct CollapsingHeaderComponent<'a> {
    collapsing_header: CollapsingHeader,
    contents: BunnyUi<'a>,
}

impl UiContainer for CollapsingHeaderComponent<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: Input,
        id: Id,
    ) -> Response {
        let mut header = egui::CollapsingHeader::new(self.collapsing_header.text)
            .id_salt(self.collapsing_header.id)
            .default_open(self.collapsing_header.default_open)
            .open(self.collapsing_header.open.into())
            .show_background(self.collapsing_header.show_background);
        if let RSome(id) = self.collapsing_header.id {
            header = header.id_salt(id);
        }
        let resp = if self.collapsing_header.indented {
            header.show(ui, |ui| self.contents.ui(ui, responses, input.clone()))
        } else {
            header.show_unindented(ui, |ui| self.contents.ui(ui, responses, input.clone()))
        };
        Response::new(id, resp.header_response, input)
    }
}

impl<'a> From<CollapsingHeaderComponent<'a>> for Container<'a> {
    fn from(value: CollapsingHeaderComponent<'a>) -> Self {
        Self::CollapsingHeader(value)
    }
}
