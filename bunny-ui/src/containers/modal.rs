use abi_stable::std_types::ROption::{self, RNone, RSome};
use ecolor::Color32;
use emath::Vec2;

use crate::{
    Align2, Id, Order, Sense, UiKind,
    containers::{Area, Frame},
    response::BunnyResponse,
    ui::BunnyUi,
};
#[cfg(feature = "manager")]
use crate::{closure::PluginClosure, vtable::ui::ModalFfiResponse};

#[repr(C)]
pub struct Modal {
    pub area: Area,
    pub frame: ROption<Frame>,
    pub backdrop_color: Color32,
}

impl Modal {
    #[inline]
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            area: Self::default_area(id),
            backdrop_color: Color32::from_black_alpha(100),
            frame: RNone,
        }
    }

    #[inline]
    pub fn default_area(id: impl Into<Id>) -> Area {
        Area::new(id)
            .kind(UiKind::Modal)
            .sense(Sense::hover())
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .order(Order::Foreground)
            .interactable(true)
    }

    #[inline]
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = RSome(frame);
        self
    }

    #[inline]
    pub fn backdrop_color(mut self, color: Color32) -> Self {
        self.backdrop_color = color;
        self
    }

    #[inline]
    pub fn area(mut self, area: Area) -> Self {
        self.area = area;
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyModalResponse<R> {
        ui.modal_show(self, add_contents)
    }
}

#[cfg(feature = "manager")]
impl Modal {
    pub(crate) fn show_impl(
        self,
        ctx: &egui::Context,
        contents: PluginClosure,
    ) -> ModalFfiResponse {
        let Modal {
            area,
            backdrop_color,
            frame,
        } = self;
        let modal = egui::Modal {
            area: area.into(),
            backdrop_color,
            frame: frame.map(|f| f.into()).into_option(),
        };
        let response = modal.show(ctx, |ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
        ModalFfiResponse::new(response)
    }
}

pub struct BunnyModalResponse<R> {
    pub response: BunnyResponse,
    pub backdrop_response: BunnyResponse,
    pub inner: R,
    pub is_top_modal: bool,
    pub any_popup_open: bool,
}

impl<R> BunnyModalResponse<R> {
    pub fn should_close(&self) -> bool {
        let escape_pressed = || {
            self.response
                .input(|i| i.consume_key(crate::Modifiers::NONE, crate::Key::Escape))
        };
        let ui_close_called = self.response.should_close();
        self.backdrop_response.clicked()
            || ui_close_called
            || (self.is_top_modal && !self.any_popup_open && escape_pressed())
    }
}
