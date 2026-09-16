use crate::widgets::button::Button;

pub mod button;
pub mod checkbox;
// pub mod drag_value;
pub mod image;
pub mod label;
pub mod separator;
// pub mod slider;
pub mod color_picker;
pub mod link;
pub mod progress_bar;
pub mod radio_button;
pub mod shortcut_button;
pub mod spinner;
pub mod text_edit;

#[repr(C)]
pub enum Widget {
    Button(Button),
}

impl egui::Widget for Widget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        match self {
            Widget::Button(button) => button.ui(ui),
        }
    }
}
