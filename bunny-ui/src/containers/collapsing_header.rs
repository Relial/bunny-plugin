use std::hash::Hash;

use abi_stable::std_types::ROption::{self, RNone, RSome};

use crate::{WidgetText, id::Id, response::BunnyResponse, ui::BunnyUi};
#[cfg(feature = "manager")]
use crate::{closure::PluginClosure, vtable::ui::CollapsingFfiResponse};

#[repr(C)]
pub struct CollapsingHeader<'a> {
    text: WidgetText<'a>,
    id_salt: ROption<Id>,
    open: ROption<bool>,
    show_background: bool,
    indented: bool,
    default_open: bool,
}

impl<'a> CollapsingHeader<'a> {
    pub fn new(text: impl Into<WidgetText<'a>>) -> Self {
        let text = text.into();
        Self {
            text,
            id_salt: RNone,
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

    /// A source for the Id.
    ///
    /// Note that this gets hashed twice, so the Id in the response won't match an Id created out of the same salt.
    #[inline]
    pub fn id_salt(mut self, id_salt: impl Hash) -> Self {
        self.id_salt = RSome(Id::new(id_salt));
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
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyCollapsingResponse<R> {
        ui.collapsing_header_show(self, add_contents)
    }
}

#[cfg(feature = "manager")]
impl CollapsingHeader<'_> {
    pub(crate) fn show_impl(
        self,
        ui: &mut egui::Ui,
        contents: PluginClosure,
    ) -> CollapsingFfiResponse {
        let mut header = egui::CollapsingHeader::new(self.text)
            .default_open(self.default_open)
            .open(self.open.into())
            .show_background(self.show_background);
        if let RSome(id) = self.id_salt {
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
        CollapsingFfiResponse::new(res)
    }
}

pub struct BunnyCollapsingResponse<R> {
    pub header_response: BunnyResponse,
    pub body_response: Option<BunnyResponse>,
    pub body_returned: Option<R>,
    pub openness: f32,
}

impl<R> BunnyCollapsingResponse<R> {
    #[inline]
    pub fn fully_closed(&self) -> bool {
        self.openness <= 0.0
    }

    #[inline]
    pub fn fully_open(&self) -> bool {
        self.openness >= 1.0
    }
}
