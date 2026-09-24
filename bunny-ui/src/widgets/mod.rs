mod button;
mod checkbox;
mod color_picker;
mod drag_value;
mod image;
mod label;
mod link;
mod progress_bar;
mod radio_button;
mod separator;
mod shortcut_button;
mod slider;
mod spinner;
mod text_edit;

pub use button::*;
pub use checkbox::*;
pub use color_picker::*;
pub use drag_value::*;
pub use image::*;
pub use label::*;
pub use link::*;
pub use progress_bar::*;
pub use radio_button::*;
pub use separator::*;
pub use shortcut_button::*;
pub use slider::*;
pub use spinner::*;
pub use text_edit::*;

#[repr(C)]
pub enum Widget<'a> {
    Button(Button<'a>),
    CheckBox(CheckBox<'a>),
    ColorPicker(ColorPicker<'a>),
    DragValue(DragValue<'a>),
    Image(Image<'a>),
    Label(Label<'a>),
    Link(Link<'a>),
    ProgressBar(ProgressBar<'a>),
    RadioButton(RadioButton<'a>),
    Separator(Separator),
    ShortcutButton(ShortcutButton<'a>),
    Slider(Slider<'a>),
    Spinner(Spinner),
    TextEdit(TextEdit<'a>),
}

#[cfg(feature = "manager")]
impl egui::Widget for Widget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        match self {
            Widget::Button(button) => button.ui(ui),
            Widget::CheckBox(check_box) => check_box.ui(ui),
            Widget::ColorPicker(color_picker) => color_picker.ui(ui),
            Widget::DragValue(drag_value) => drag_value.ui(ui),
            Widget::Image(image) => image.ui(ui),
            Widget::Label(label) => label.ui(ui),
            Widget::Link(link) => link.ui(ui),
            Widget::ProgressBar(progress_bar) => progress_bar.ui(ui),
            Widget::RadioButton(radio_button) => radio_button.ui(ui),
            Widget::Separator(separator) => separator.ui(ui),
            Widget::ShortcutButton(shortcut_button) => shortcut_button.ui(ui),
            Widget::Slider(slider) => slider.ui(ui),
            Widget::Spinner(spinner) => spinner.ui(ui),
            Widget::TextEdit(text_edit) => text_edit.ui(ui),
        }
    }
}
