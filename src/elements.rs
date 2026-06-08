use abi_stable::std_types::{RBox, RHashMap};
use egui::{Id, Ui, Vec2, Widget as _};
use rapidhash::fast::RandomState;

use crate::{
    containers::{
        allocate_ui::AllocateUi, collapsing_header::CollapsingHeaderComponent,
        combo_box::ComboBoxComponent, grid::GridComponent, popup::PopupComponent,
        scope_builder::ScopeBuilder, tooltip::TooltipComponent, window::WindowComponent,
    },
    input_state::Input,
    response::Response,
    widgets::{
        button::Button, checkbox::CheckBox, drag_value::DragValue, image::Image,
        interact::Interact, label::Label, link::Link, progress_bar::ProgressBar,
        radio_button::RadioButton, separator::Separator, slider::Slider, spinner::Spinner,
    },
};

pub trait UiComponent {
    fn ui(self, ui: &mut Ui, input: Input, id: Id) -> Response;
}

pub trait UiContainer {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: Input,
        id: Id,
    ) -> Response;
}

#[repr(C)]
pub enum Container<'a> {
    CollapsingHeader(CollapsingHeaderComponent<'a>),
    Scope(ScopeBuilder<'a>),
    AllocateUi(AllocateUi<'a>),
    Grid(GridComponent<'a>),
    Window(WindowComponent<'a>),
    ComboBox(ComboBoxComponent<'a>),
    Popup(PopupComponent<'a>),
    Tooltip(TooltipComponent<'a>),
}

impl UiContainer for Container<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: Input,
        id: Id,
    ) -> Response {
        match self {
            Container::CollapsingHeader(collapsing_header) => {
                collapsing_header.ui(ui, responses, input, id)
            }
            Container::Scope(scope_builder) => scope_builder.ui(ui, responses, input, id),
            Container::AllocateUi(allocate_ui) => allocate_ui.ui(ui, responses, input, id),
            Container::Grid(grid) => grid.ui(ui, responses, input, id),
            Container::Window(window) => window.ui(ui, responses, input, id),
            Container::ComboBox(combo_box_component) => {
                combo_box_component.ui(ui, responses, input, id)
            }
            Container::Popup(popup_component) => popup_component.ui(ui, responses, input, id),
            Container::Tooltip(tooltip_component) => tooltip_component.ui(ui, responses, input, id),
        }
    }
}

#[repr(C)]
pub enum Widget<'a> {
    Label(Label),
    CheckBox(CheckBox<'a>),
    DragValue(DragValue<'a>),
    Button(Button),
    Slider(Slider<'a>),
    Separator(Separator),
    Image(RBox<Image<'a>>),
    Interact(Interact),
    Link(Link),
    ProgressBar(ProgressBar),
    RadioButton(RadioButton),
    Spinner(Spinner),
}

impl egui::Widget for Widget<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        match self {
            Widget::Label(label) => label.ui(ui),
            Widget::CheckBox(check_box) => check_box.ui(ui),
            Widget::DragValue(drag_value) => drag_value.ui(ui),
            Widget::Button(button) => button.ui(ui),
            Widget::Slider(slider) => slider.ui(ui),
            Widget::Separator(separator) => separator.ui(ui),
            Widget::Image(image) => RBox::into_inner(image).ui(ui),
            Widget::Interact(interact) => interact.ui(ui),
            Widget::Link(link) => link.ui(ui),
            Widget::ProgressBar(progress_bar) => progress_bar.ui(ui),
            Widget::RadioButton(radio_button) => radio_button.ui(ui),
            Widget::Spinner(spinner) => spinner.ui(ui),
        }
    }
}

#[repr(C)]
pub enum MiscComponent {
    Space(f32),
    Disable,
    EndRow,
    AllocateSpace(Vec2),
}

impl UiComponent for MiscComponent {
    fn ui(self, ui: &mut Ui, input: Input, id: Id) -> Response {
        match self {
            MiscComponent::Space(space) => {
                ui.add_space(space);
                Response::default()
            }
            MiscComponent::Disable => {
                ui.disable();
                Response::default()
            }
            MiscComponent::EndRow => {
                ui.end_row();
                Response::default()
            }
            MiscComponent::AllocateSpace(size) => {
                let (_, rect) = ui.allocate_space(size);
                Response::rect_only(id, rect, input)
            }
        }
    }
}

#[repr(C)]
pub enum Component<'a> {
    Container(RBox<Container<'a>>),
    Widget(Widget<'a>),
    MiscComponent(MiscComponent),
}

impl UiContainer for Component<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: Input,
        id: Id,
    ) -> Response {
        match self {
            Component::Container(container) => {
                RBox::into_inner(container).ui(ui, responses, input, id)
            }
            Component::Widget(widget) => {
                let egui_resp = widget.ui(ui);
                Response::new(id, egui_resp, input)
            }
            Component::MiscComponent(misc_component) => misc_component.ui(ui, input, id),
        }
    }
}

impl<'a> From<Container<'a>> for Component<'a> {
    fn from(value: Container<'a>) -> Self {
        Self::Container(RBox::new(value))
    }
}

impl<'a> From<Widget<'a>> for Component<'a> {
    fn from(value: Widget<'a>) -> Self {
        Self::Widget(value)
    }
}

impl From<MiscComponent> for Component<'_> {
    fn from(value: MiscComponent) -> Self {
        Self::MiscComponent(value)
    }
}
