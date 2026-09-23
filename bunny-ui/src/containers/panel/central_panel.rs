use abi_stable::std_types::ROption::{self, RNone, RSome};

#[cfg(feature = "manager")]
use crate::{BunnyResponse, closure::PluginClosure};
use crate::{containers::Frame, response::BunnyInnerResponse, ui::BunnyUi};

#[derive(Default)]
#[repr(C)]
pub struct CentralPanel {
    frame: ROption<Frame>,
}

impl CentralPanel {
    #[inline]
    pub fn no_frame() -> Self {
        Self {
            frame: RSome(Frame::NONE),
        }
    }

    #[inline]
    pub fn default_margins() -> Self {
        Self { frame: RNone }
    }

    #[inline]
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = RSome(frame);
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        ui.central_panel_show(self, add_contents)
    }
}

#[cfg(feature = "manager")]
impl CentralPanel {
    pub(crate) fn show_impl(self, ui: &mut egui::Ui, contents: PluginClosure) -> BunnyResponse {
        let mut panel = egui::CentralPanel::default();
        if let RSome(frame) = self.frame {
            panel = panel.frame(frame.into());
        }
        let response = panel
            .show_inside(ui, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(response)
    }
}
