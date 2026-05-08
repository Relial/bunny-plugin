use std::{
    hash::{BuildHasher, Hash, Hasher},
    num::NonZeroU64,
};

use abi_stable::std_types::{RArc, RBox, RHashMap};
use egui::{Ui, Widget as _};
use rapidhash::fast::RandomState;

use crate::{
    containers::{
        allocate_ui::AllocateUi, collapsing_header::CollapsingHeaderComponent, grid::GridComponent,
        scope_builder::ScopeBuilder, window::WindowComponent,
    },
    input::PointerState,
    response::Response,
    widgets::{
        button::Button, checkbox::CheckBox, drag_value::DragValue, image::Image,
        interact::Interact, label::Label, separator::Separator, slider::Slider,
    },
};

#[repr(C)]
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Id(NonZeroU64);

impl Id {
    pub const NULL: Self = Self(NonZeroU64::MAX);

    pub const fn from_hash(hash: u64) -> Self {
        if let Some(nonzero) = NonZeroU64::new(hash) {
            Self(nonzero)
        } else {
            Self(NonZeroU64::MIN)
        }
    }

    pub fn new(source: impl Hash) -> Self {
        Self::from_hash(rapidhash::fast::GlobalState::new().hash_one(source))
    }

    pub fn with(self, child: impl Hash) -> Self {
        let mut hasher = rapidhash::fast::GlobalState::new().build_hasher();
        hasher.write_u64(self.0.get());
        child.hash(&mut hasher);
        Self::from_hash(hasher.finish())
    }

    pub fn value(&self) -> u64 {
        self.0.get()
    }
}

pub trait UiComponent {
    fn ui(self, ui: &mut Ui, id: Id) -> Response;
}

pub trait UiContainer {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
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
}

impl UiContainer for Container<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
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
        }
    }
}

#[repr(C)]
pub enum MiscComponent {
    Space(f32),
    Disable,
    EndRow,
}

impl UiComponent for MiscComponent {
    fn ui(self, ui: &mut Ui, _id: Id) -> Response {
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
        input: RArc<PointerState>,
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
            Component::MiscComponent(misc_component) => misc_component.ui(ui, id),
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
