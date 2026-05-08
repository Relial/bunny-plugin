use abi_stable::std_types::ROption::{self, RNone, RSome};
use egui::Ui;

use crate::{elements::Widget, num::Num};

#[repr(C)]
pub struct DragValue<'a> {
    value: &'a mut Num,
    speed: ROption<f64>,
    range: ROption<[Num; 2]>,
    min_decimals: usize,
    max_decimals: ROption<usize>,
    update_while_editing: bool,
}

impl<'a> DragValue<'a> {
    pub fn new(value: &'a mut Num) -> Self {
        Self {
            value,
            speed: RNone,
            range: RNone,
            min_decimals: 0,
            max_decimals: RNone,
            update_while_editing: true,
        }
    }

    #[inline]
    pub fn speed(mut self, speed: impl Into<f64>) -> Self {
        self.speed = RSome(speed.into());
        self
    }

    #[inline]
    pub fn range(mut self, [r1, r2]: [impl Into<Num>; 2]) -> Self {
        self.range = RSome([r1.into(), r2.into()]);
        self
    }

    #[inline]
    pub fn min_decimals(mut self, min_decimals: usize) -> Self {
        self.min_decimals = min_decimals;
        self
    }

    #[inline]
    pub fn max_decimals(mut self, max_decimals: usize) -> Self {
        self.max_decimals = RSome(max_decimals);
        self
    }

    #[inline]
    pub fn fixed_decimals(mut self, num_decimals: usize) -> Self {
        self.min_decimals = num_decimals;
        self.max_decimals = RSome(num_decimals);
        self
    }

    #[inline]
    pub fn update_while_editing(mut self, update: bool) -> Self {
        self.update_while_editing = update;
        self
    }
}

impl egui::Widget for DragValue<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let value = self.value;
        let mut widget = match value {
            Num::Integer(i) => egui::DragValue::new(i),
            Num::Float(f) => egui::DragValue::new(f),
        }
        .min_decimals(self.min_decimals)
        .update_while_editing(self.update_while_editing);
        if let RSome(speed) = self.speed {
            widget = widget.speed(speed);
        }
        if let RSome(range) = self.range {
            widget = widget.range::<f64>(range[0].into()..=range[1].into());
        }
        if let RSome(decimals) = self.max_decimals {
            widget = widget.max_decimals(decimals);
        }
        ui.add(widget)
    }
}

impl<'a> From<DragValue<'a>> for Widget<'a> {
    fn from(value: DragValue<'a>) -> Self {
        Self::DragValue(value)
    }
}
