use std::{
    hash::{BuildHasher, Hash, Hasher},
    num::NonZeroU64,
};

use abi_stable::std_types::{
    RArc, RBox, RHashMap,
    ROption::{self, RNone, RSome},
};
use egui::{Color32, Ui, Vec2, Widget as _};
use rapidhash::fast::RandomState;

use crate::{
    collapsing_header::CollapsingHeaderComponent, input::PointerState, paint::{corner_radius::CornerRadius, stroke::Stroke}, response::Response, ui::BunnyUi, ui_builder::UiBuilder, widget_text::WidgetText, window::WindowComponent
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

#[repr(C)]
#[derive(Clone, Copy)]
pub enum Num {
    Integer(i32),
    Float(f64),
}

impl From<Num> for f64 {
    fn from(value: Num) -> Self {
        match value {
            Num::Integer(i) => i.into(),
            Num::Float(f) => f,
        }
    }
}

impl From<Num> for i32 {
    fn from(value: Num) -> Self {
        match value {
            Num::Integer(i) => i,
            Num::Float(f) => f as i32,
        }
    }
}

impl From<i32> for Num {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Num {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

pub trait UiComponent {
    fn ui(self, ui: &mut Ui) -> egui::Response;
}

pub trait UiContainer {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
    ) -> egui::Response;
}

#[repr(C)]
pub enum Container {
    CollapsingHeader(CollapsingHeaderComponent),
    Scope(ScopeBuilder),
    AllocateUi(AllocateUi),
    Grid(Grid),
    Window(WindowComponent),
}

impl UiContainer for Container {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
    ) -> egui::Response {
        match self {
            Container::CollapsingHeader(collapsing_header) => {
                collapsing_header.ui(ui, responses, input.clone())
            }
            Container::Scope(scope_builder) => scope_builder.ui(ui, responses, input.clone()),
            Container::AllocateUi(allocate_ui) => allocate_ui.ui(ui, responses, input.clone()),
            Container::Grid(grid) => grid.ui(ui, responses, input.clone()),
            Container::Window(window) => window.ui(ui, responses, input),
        }
    }
}

impl From<CollapsingHeaderComponent> for Container {
    fn from(value: CollapsingHeaderComponent) -> Self {
        Self::CollapsingHeader(value)
    }
}

impl From<ScopeBuilder> for Container {
    fn from(value: ScopeBuilder) -> Self {
        Self::Scope(value)
    }
}

impl From<AllocateUi> for Container {
    fn from(value: AllocateUi) -> Self {
        Self::AllocateUi(value)
    }
}

impl From<Grid> for Container {
    fn from(value: Grid) -> Self {
        Self::Grid(value)
    }
}

impl From<WindowComponent> for Container {
    fn from(value: WindowComponent) -> Self {
        Self::Window(value)
    }
}

#[repr(C)]
pub enum Widget {
    Label(Label),
    CheckBox(CheckBox),
    DragValue(DragValue),
    Button(Button),
    Slider(Slider),
    Separator(Separator),
}

impl egui::Widget for Widget {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        match self {
            Widget::Label(label) => label.ui(ui),
            Widget::CheckBox(check_box) => check_box.ui(ui),
            Widget::DragValue(drag_value) => drag_value.ui(ui),
            Widget::Button(button) => button.ui(ui),
            Widget::Slider(slider) => slider.ui(ui),
            Widget::Separator(separator) => separator.ui(ui),
        }
    }
}

impl From<Label> for Widget {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}

impl From<CheckBox> for Widget {
    fn from(value: CheckBox) -> Self {
        Self::CheckBox(value)
    }
}

impl From<DragValue> for Widget {
    fn from(value: DragValue) -> Self {
        Self::DragValue(value)
    }
}

impl From<Button> for Widget {
    fn from(value: Button) -> Self {
        Self::Button(value)
    }
}

impl From<Slider> for Widget {
    fn from(value: Slider) -> Self {
        Self::Slider(value)
    }
}

impl From<Separator> for Widget {
    fn from(value: Separator) -> Self {
        Self::Separator(value)
    }
}

#[repr(C)]
pub enum MiscComponent {
    Space(f32),
    Disable,
    EndRow,
}

impl UiComponent for MiscComponent {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        match self {
            MiscComponent::Space(space) => {
                ui.add_space(space);
                ui.response()
            }
            MiscComponent::Disable => {
                ui.disable();
                ui.response()
            }
            MiscComponent::EndRow => {
                ui.end_row();
                ui.response()
            }
        }
    }
}

#[repr(C)]
pub enum Component {
    Container(RBox<Container>),
    Widget(Widget),
    MiscComponent(MiscComponent),
}

impl UiContainer for Component {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
    ) -> egui::Response {
        match self {
            Component::Container(container) => RBox::into_inner(container).ui(ui, responses, input),
            Component::Widget(widget) => widget.ui(ui),
            Component::MiscComponent(misc_component) => misc_component.ui(ui),
        }
    }
}

impl From<Container> for Component {
    fn from(value: Container) -> Self {
        Self::Container(RBox::new(value))
    }
}

impl From<Widget> for Component {
    fn from(value: Widget) -> Self {
        Self::Widget(value)
    }
}

impl From<MiscComponent> for Component {
    fn from(value: MiscComponent) -> Self {
        Self::MiscComponent(value)
    }
}

#[repr(C)]
pub struct Label(WidgetText);

impl Label {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self(text.into())
    }
}

impl egui::Widget for Label {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.label(self.0)
    }
}

#[repr(C)]
pub struct CheckBox {
    value: *mut bool,
    text: WidgetText,
}

impl CheckBox {
    pub fn new(value: &mut bool, text: impl Into<WidgetText>) -> Self {
        Self {
            value,
            text: text.into(),
        }
    }
}

impl egui::Widget for CheckBox {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        unsafe {
            let checked = self.value.as_mut_unchecked();
            ui.checkbox(checked, self.text)
        }
    }
}

#[repr(C)]
pub struct ScopeBuilder {
    contents: BunnyUi,
    ui_builder: UiBuilder,
}

impl ScopeBuilder {
    pub fn new(ui_builder: UiBuilder, ui: BunnyUi) -> Self {
        Self {
            contents: ui,
            ui_builder,
        }
    }
}

impl UiContainer for ScopeBuilder {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
    ) -> egui::Response {
        ui.scope_builder(self.ui_builder.into(), |ui| {
            self.contents.ui(ui, responses, input);
        })
        .response
    }
}

#[repr(C)]
pub struct AllocateUi {
    contents: BunnyUi,
    desired_size: Vec2,
    layout: ROption<crate::layout::Layout>,
}

impl AllocateUi {
    pub fn new(desired_size: Vec2, layout: Option<crate::layout::Layout>, ui: BunnyUi) -> Self {
        Self {
            contents: ui,
            desired_size,
            layout: layout.into(),
        }
    }
}

impl UiContainer for AllocateUi {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
    ) -> egui::Response {
        if let RSome(layout) = self.layout {
            ui.allocate_ui_with_layout(self.desired_size, layout.into(), |ui| {
                self.contents.ui(ui, responses, input)
            })
            .response
        } else {
            ui.allocate_ui(self.desired_size, |ui| {
                self.contents.ui(ui, responses, input)
            })
            .response
        }
    }
}

#[repr(C)]
pub struct DragValue {
    value: *mut Num,
    speed: ROption<f64>,
    range: ROption<[Num; 2]>,
    decimals: ROption<usize>,
}

impl DragValue {
    pub fn new(value: &mut Num) -> Self {
        Self {
            value,
            speed: RNone,
            range: RNone,
            decimals: RNone,
        }
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = RSome(speed);
        self
    }

    pub fn range(mut self, range: [Num; 2]) -> Self {
        self.range = RSome(range);
        self
    }

    pub fn decimals(mut self, decimals: usize) -> Self {
        self.decimals = RSome(decimals);
        self
    }
}

impl egui::Widget for DragValue {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let value = unsafe { self.value.as_mut_unchecked() };
        let mut widget = match value {
            Num::Integer(i) => egui::DragValue::new(i),
            Num::Float(f) => egui::DragValue::new(f),
        };
        if let RSome(speed) = self.speed {
            widget = widget.speed(speed);
        }
        if let RSome(range) = self.range {
            widget = widget.range::<f64>(range[0].into()..=range[1].into());
        }
        if let RSome(decimals) = self.decimals {
            widget = widget.fixed_decimals(decimals);
        }
        ui.add(widget)
    }
}

#[repr(C)]
pub struct Slider {
    value: *mut Num,
    range: [Num; 2],
}

impl Slider {
    pub fn new(value: &mut Num, range: [Num; 2]) -> Self {
        Self { value, range }
    }
}

impl egui::Widget for Slider {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let value = unsafe { self.value.as_mut_unchecked() };
        let widget = match value {
            Num::Integer(i) => {
                let range = self.range[0].into()..=self.range[1].into();
                egui::Slider::new(i, range)
            }
            Num::Float(f) => {
                let range = self.range[0].into()..=self.range[1].into();
                egui::Slider::new(f, range)
            }
        };
        ui.add(widget)
    }
}

#[repr(C)]
pub struct Button {
    text: WidgetText,
    fill: ROption<Color32>,
    stroke: ROption<Stroke>,
    small: bool,
    frame: ROption<bool>,
    frame_when_inactive: bool,
    corner_radius: ROption<CornerRadius>,
    selected: bool,
}

impl Button {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            fill: RNone,
            stroke: RNone,
            small: false,
            frame: RNone,
            frame_when_inactive: true,
            corner_radius: RNone,
            selected: false,
        }
    }

    pub fn selectable(selected: bool, text: impl Into<WidgetText>) -> Self {
        Self::new(text)
            .selected(selected)
            .frame_when_inactive(selected)
            .frame(true)
    }

    pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
        self.fill = RSome(fill.into());
        self
    }

    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = RSome(stroke.into());
        self.frame = RSome(true);
        self
    }

    pub fn small(mut self) -> Self {
        self.small = true;
        self
    }

    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = RSome(frame);
        self
    }

    pub fn frame_when_inactive(mut self, frame_when_inactive: bool) -> Self {
        self.frame_when_inactive = frame_when_inactive;
        self
    }

    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = RSome(corner_radius.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl egui::Widget for Button {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let mut button = egui::Button::new(self.text);
        if let RSome(fill) = self.fill {
            button = button.fill(fill);
        }
        if let RSome(stroke) = self.stroke {
            button = button.stroke(stroke);
        }
        if self.small {
            button = button.small();
        }
        if let RSome(frame) = self.frame {
            button = button.frame(frame);
        }
        if let RSome(corner_radius) = self.corner_radius {
            button = button.corner_radius(corner_radius);
        }
        ui.add(button)
    }
}

#[repr(C)]
pub struct Separator {
    spacing: ROption<f32>,
    grow: f32,
    is_horizontal_line: ROption<bool>,
}

impl Separator {
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = RSome(spacing);
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.is_horizontal_line = RSome(true);
        self
    }

    pub fn vertical(mut self) -> Self {
        self.is_horizontal_line = RSome(false);
        self
    }

    pub fn grow(mut self, grow: f32) -> Self {
        self.grow += grow;
        self
    }

    pub fn shrink(mut self, shrink: f32) -> Self {
        self.grow -= shrink;
        self
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            spacing: RNone,
            grow: 0.0,
            is_horizontal_line: RNone,
        }
    }
}

impl egui::Widget for Separator {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let mut separator = egui::Separator::default();
        if let RSome(spacing) = self.spacing {
            separator = separator.spacing(spacing);
        }
        if self.grow > 0.0 {
            separator = separator.grow(self.grow)
        } else if self.grow < 0.0 {
            separator = separator.shrink(self.grow)
        }
        if let RSome(horizontal) = self.is_horizontal_line {
            if horizontal {
                separator = separator.horizontal();
            } else {
                separator = separator.vertical();
            }
        }
        ui.add(separator)
    }
}

#[repr(C)]
pub struct Grid {
    id: Id,
    contents: BunnyUi,
    num_columns: ROption<usize>,
    min_col_width: ROption<f32>,
    min_row_height: ROption<f32>,
    max_cell_size: Vec2,
    spacing: ROption<Vec2>,
    striped: bool,
}

impl Grid {
    pub fn new(id_salt: impl Hash, ui: BunnyUi) -> Self {
        Self {
            id: Id::new(id_salt),
            contents: ui,
            num_columns: RNone,
            min_col_width: RNone,
            min_row_height: RNone,
            max_cell_size: Vec2::INFINITY,
            spacing: RNone,
            striped: false,
        }
    }
}

impl UiContainer for Grid {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
    ) -> egui::Response {
        let mut grid = egui::Grid::new(self.id);
        if let RSome(num_columns) = self.num_columns {
            grid = grid.num_columns(num_columns);
        }
        if let RSome(min_col_width) = self.min_col_width {
            grid = grid.min_col_width(min_col_width);
        }
        if let RSome(min_row_height) = self.min_row_height {
            grid = grid.min_row_height(min_row_height);
        }
        grid = grid.max_col_width(self.max_cell_size.x);
        if let RSome(spacing) = self.spacing {
            grid = grid.spacing(spacing);
        }
        if self.striped {
            grid = grid.striped(true);
        }

        grid.show(ui, |ui| {
            self.contents.ui(ui, responses, input);
        })
        .response
    }
}
