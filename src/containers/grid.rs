use std::hash::Hash;

use abi_stable::std_types::{
    RArc, RHashMap,
    ROption::{self, RNone, RSome},
};
use egui::{Id, Ui, Vec2};
use rapidhash::fast::RandomState;

use crate::{
    elements::{Container, UiContainer},
    input::PointerState,
    layout::Layout,
    response::{InnerResponse, Response},
    ui::BunnyUi,
};

#[repr(C)]
pub struct Grid {
    id: Id,
    num_columns: ROption<usize>,
    min_col_width: ROption<f32>,
    min_row_height: ROption<f32>,
    max_col_width: f32,
    spacing: ROption<Vec2>,
    striped: bool,
}

impl Grid {
    pub fn new(id_salt: impl Hash) -> Self {
        Self {
            id: Id::new(id_salt),
            num_columns: RNone,
            min_col_width: RNone,
            min_row_height: RNone,
            max_col_width: f32::INFINITY,
            spacing: RNone,
            striped: false,
        }
    }

    #[inline]
    pub fn num_columns(mut self, num_columns: usize) -> Self {
        self.num_columns = RSome(num_columns);
        self
    }

    #[inline]
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    #[inline]
    pub fn min_col_width(mut self, min_col_width: f32) -> Self {
        self.min_col_width = RSome(min_col_width);
        self
    }

    #[inline]
    pub fn min_row_height(mut self, min_row_height: f32) -> Self {
        self.min_row_height = RSome(min_row_height);
        self
    }

    #[inline]
    pub fn max_col_width(mut self, max_col_width: f32) -> Self {
        self.max_col_width = max_col_width;
        self
    }

    #[inline]
    pub fn spacing(mut self, spacing: impl Into<Vec2>) -> Self {
        self.spacing = RSome(spacing.into());
        self
    }

    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnOnce(&mut BunnyUi) -> R,
    ) -> InnerResponse<R> {
        let mut new = ui.new_child(Some(Layout::default()));
        let ret = add_contents(&mut new);
        let response = ui.add_component_auto_id(Container::Grid(GridComponent {
            contents: new,
            grid: self,
        }));
        InnerResponse::new(ret, response)
    }
}

#[repr(C)]
pub struct GridComponent<'a> {
    contents: BunnyUi<'a>,
    grid: Grid,
}

impl UiContainer for GridComponent<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, RandomState>,
        input: RArc<PointerState>,
        id: Id,
    ) -> Response {
        let mut grid = egui::Grid::new(self.grid.id)
            .max_col_width(self.grid.max_col_width)
            .striped(self.grid.striped);
        if let RSome(num_columns) = self.grid.num_columns {
            grid = grid.num_columns(num_columns);
        }
        if let RSome(min_col_width) = self.grid.min_col_width {
            grid = grid.min_col_width(min_col_width);
        }
        if let RSome(min_row_height) = self.grid.min_row_height {
            grid = grid.min_row_height(min_row_height);
        }
        if let RSome(spacing) = self.grid.spacing {
            grid = grid.spacing(spacing);
        }

        let egui_resp = grid
            .show(ui, |ui| {
                self.contents.ui(ui, responses, input.clone());
            })
            .response;
        Response::new(id, egui_resp, input)
    }
}

impl<'a> From<GridComponent<'a>> for Container<'a> {
    fn from(value: GridComponent<'a>) -> Self {
        Self::Grid(value)
    }
}
