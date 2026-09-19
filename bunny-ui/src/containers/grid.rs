use abi_stable::std_types::ROption::{self, RNone, RSome};
use emath::Vec2;

use crate::{Id, elements::Container, layout::Layout, response::InnerResponse, ui_old::BunnyUi};

#[repr(C)]
pub struct Grid {
    num_columns: ROption<usize>,
    spacing: ROption<Vec2>,
    id: Id,
    min_col_width: ROption<f32>,
    min_row_height: ROption<f32>,
    max_col_width: f32,
    striped: bool,
}

impl Grid {
    #[inline]
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
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

    pub fn show<'a, R>(
        self,
        ui: &mut BunnyUi<'a>,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
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

#[cfg(feature = "manager")]
impl crate::elements::UiContainer for GridComponent<'_> {
    fn ui(
        self,
        ui: &mut egui::Ui,
        responses: &mut abi_stable::std_types::RHashMap<
            crate::Id,
            crate::response::Response,
            rapidhash::fast::RandomState,
        >,
        pointer_state: abi_stable::std_types::RArc<crate::input_state::PointerState>,
        id: crate::Id,
    ) -> crate::response::Response {
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
                self.contents.ui(ui, responses, pointer_state.clone());
            })
            .response;
        crate::response::Response::new(id, egui_resp, pointer_state)
    }
}

impl<'a> From<GridComponent<'a>> for Container<'a> {
    #[inline]
    fn from(value: GridComponent<'a>) -> Self {
        Self::Grid(value)
    }
}
