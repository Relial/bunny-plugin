use abi_stable::std_types::ROption::{self, RNone, RSome};
use egui::Id;

use crate::{WidgetText, closure::PluginClosure, response::BunnyResponse, ui::BunnyUi};

#[repr(C)]
pub struct CollapsingHeader {
    text: WidgetText,
    id: ROption<Id>,
    open: ROption<bool>,
    show_background: bool,
    indented: bool,
    default_open: bool,
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
    pub fn id(mut self, id: Id) -> Self {
        self.id = RSome(id);
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
    pub fn show(self, ui: &mut BunnyUi, add_contents: impl FnMut(&mut BunnyUi)) {
        ui.collapsing_header_show(self, add_contents);
    }
}

impl CollapsingHeader {
    #[inline]
    pub(crate) fn show_impl(
        self,
        ui: &mut egui::Ui,
        contents: PluginClosure,
    ) -> BunnyCollapsingResponse {
        let mut header = egui::CollapsingHeader::new(self.text)
            .default_open(self.default_open)
            .open(self.open.into())
            .show_background(self.show_background);
        if let RSome(id) = self.id {
            header = header.id_salt(id);
        }
        let res = if self.indented {
            header.show(ui, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
        } else {
            header.show_unindented(ui, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
        };
        BunnyCollapsingResponse::new(res)
    }
}

#[repr(C)]
pub struct BunnyCollapsingResponse {
    pub header_response: BunnyResponse,
    pub body_response: ROption<BunnyResponse>,
    pub openness: f32,
}

impl BunnyCollapsingResponse {
    #[inline]
    pub fn new<R>(response: egui::CollapsingResponse<R>) -> Self {
        Self {
            header_response: BunnyResponse::new(response.header_response),
            body_response: response.body_response.map(BunnyResponse::new).into(),
            openness: response.openness,
        }
    }
}

impl BunnyCollapsingResponse {
    #[inline]
    pub fn fully_closed(&self) -> bool {
        self.openness <= 0.0
    }

    #[inline]
    pub fn fully_open(&self) -> bool {
        self.openness >= 1.0
    }
}
