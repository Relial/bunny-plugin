use std::hash::Hash;

use abi_stable::std_types::ROption::{self, RNone, RSome};
use emath::Vec2;
use mint::Vector2;

#[cfg(feature = "manager")]
use crate::{BunnyResponse, closure::PluginClosure};
use crate::{Id, response::BunnyInnerResponse, ui::BunnyUi};

#[repr(C)]
pub struct Grid {
    num_columns: ROption<usize>,
    spacing: ROption<Vec2>,
    id_salt: Id,
    min_col_width: ROption<f32>,
    min_row_height: ROption<f32>,
    max_col_width: f32,
    start_row: usize,
    striped: bool,
}

impl Grid {
    #[inline]
    pub fn new(id_salt: impl Hash) -> Self {
        Self {
            id_salt: Id::new(id_salt),
            num_columns: RNone,
            min_col_width: RNone,
            min_row_height: RNone,
            max_col_width: f32::INFINITY,
            spacing: RNone,
            start_row: 0,
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
    pub fn spacing(mut self, spacing: impl Into<Vector2<f32>>) -> Self {
        self.spacing = RSome(spacing.into().into());
        self
    }

    #[inline]
    pub fn start_row(mut self, start_row: usize) -> Self {
        self.start_row = start_row;
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        ui.grid_show(self, add_contents)
    }
}

#[cfg(feature = "manager")]
impl Grid {
    pub(crate) fn show_impl(self, ui: &mut egui::Ui, contents: PluginClosure) -> BunnyResponse {
        let Grid {
            num_columns,
            spacing,
            id_salt: id,
            min_col_width,
            min_row_height,
            max_col_width,
            start_row,
            striped,
        } = self;
        let mut grid = egui::Grid::new(id)
            .max_col_width(max_col_width)
            .striped(striped)
            .start_row(start_row);
        if let RSome(num_columns) = num_columns {
            grid = grid.num_columns(num_columns);
        }
        if let RSome(min_col_width) = min_col_width {
            grid = grid.min_col_width(min_col_width);
        }
        if let RSome(min_row_height) = min_row_height {
            grid = grid.min_row_height(min_row_height);
        }
        if let RSome(spacing) = spacing {
            grid = grid.spacing(spacing);
        }

        let response = grid
            .show(ui, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;

        BunnyResponse::new(response)
    }
}
