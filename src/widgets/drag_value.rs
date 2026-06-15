use std::ops::RangeInclusive;

use abi_stable::std_types::{
    ROption::{self, RNone, RSome},
    RString,
};
use egui::Ui;

use crate::{elements::Widget, num::Num, widgets::slider::NumberCustomFormat};

#[repr(C)]
pub struct DragValue<'a> {
    value: Num<'a>,
    speed: f64,
    prefix: ROption<RString>,
    suffix: ROption<RString>,
    range: [f64; 2],
    clamp_existing_to_range: bool,
    min_decimals: usize,
    max_decimals: ROption<usize>,
    update_while_editing: bool,
    custom_format: ROption<NumberCustomFormat>,
}

impl<'a> DragValue<'a> {
    pub fn new(value: impl Into<Num<'a>>) -> Self {
        let v = value.into();
        if v.integer() {
            let min = v.min();
            let max = v.max();
            Self {
                value: v,
                speed: 0.25,
                prefix: RNone,
                suffix: RNone,
                range: [min, max],
                clamp_existing_to_range: true,
                min_decimals: 0,
                max_decimals: RSome(0),
                update_while_editing: true,
                custom_format: RNone,
            }
        } else {
            Self {
                value: v,
                speed: 1.0,
                prefix: RNone,
                suffix: RNone,
                range: [f64::NEG_INFINITY, f64::INFINITY],
                clamp_existing_to_range: true,
                min_decimals: 0,
                max_decimals: RNone,
                update_while_editing: true,
                custom_format: RNone,
            }
        }
    }

    #[inline]
    pub fn speed(mut self, speed: impl Into<f64>) -> Self {
        self.speed = speed.into();
        self
    }

    #[inline]
    pub fn range(mut self, range: RangeInclusive<f64>) -> Self {
        self.range = [*range.start(), *range.end()];
        self
    }

    #[inline]
    pub fn clamp_existing_to_range(mut self, clamp_existing_to_range: bool) -> Self {
        self.clamp_existing_to_range = clamp_existing_to_range;
        self
    }

    #[inline]
    pub fn prefix(mut self, prefix: impl Into<RString>) -> Self {
        self.prefix = RSome(prefix.into());
        self
    }

    #[inline]
    pub fn suffix(mut self, suffix: impl Into<RString>) -> Self {
        self.suffix = RSome(suffix.into());
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
    pub fn max_decimals_ops(mut self, max_decimals: Option<usize>) -> Self {
        self.max_decimals = max_decimals.into();
        self
    }

    #[inline]
    pub fn fixed_decimals(mut self, num_decimals: usize) -> Self {
        self.min_decimals = num_decimals;
        self.max_decimals = RSome(num_decimals);
        self
    }

    #[inline]
    pub fn binary(mut self, min_width: usize, twos_complement: bool) -> Self {
        self.custom_format = RSome(NumberCustomFormat::Binary {
            min_width,
            twos_complement,
        });
        self
    }

    #[inline]
    pub fn octal(mut self, min_width: usize, twos_complement: bool) -> Self {
        self.custom_format = RSome(NumberCustomFormat::Octal {
            min_width,
            twos_complement,
        });
        self
    }

    #[inline]
    pub fn hexadecimal(mut self, min_width: usize, twos_complement: bool, upper: bool) -> Self {
        self.custom_format = RSome(NumberCustomFormat::Hexadecimal {
            min_width,
            twos_complement,
            upper,
        });
        self
    }

    #[inline]
    pub fn update_while_editing(mut self, update: bool) -> Self {
        self.update_while_editing = update;
        self
    }
}

impl egui::Widget for DragValue<'_> {
    fn ui(mut self, ui: &mut Ui) -> egui::Response {
        let mut drag_value = egui::DragValue::from_get_set(|v: Option<f64>| {
            if let Some(v) = v {
                self.value.set(v);
            }
            self.value.to_f64()
        })
        .speed(self.speed)
        .range(self.range[0]..=self.range[1])
        .clamp_existing_to_range(self.clamp_existing_to_range)
        .min_decimals(self.min_decimals)
        .update_while_editing(self.update_while_editing);

        if let RSome(prefix) = self.prefix {
            drag_value = drag_value.prefix(prefix.as_str());
        }
        if let RSome(suffix) = self.suffix {
            drag_value = drag_value.suffix(suffix.as_str());
        }
        if let RSome(max_decimals) = self.max_decimals {
            drag_value = drag_value.max_decimals(max_decimals);
        }
        if let RSome(custom_format) = self.custom_format {
            drag_value = match custom_format {
                NumberCustomFormat::Binary {
                    min_width,
                    twos_complement,
                } => drag_value.binary(min_width, twos_complement),
                NumberCustomFormat::Octal {
                    min_width,
                    twos_complement,
                } => drag_value.octal(min_width, twos_complement),
                NumberCustomFormat::Hexadecimal {
                    min_width,
                    twos_complement,
                    upper,
                } => drag_value.hexadecimal(min_width, twos_complement, upper),
            };
        }

        ui.add(drag_value)
    }
}

impl<'a> From<DragValue<'a>> for Widget<'a> {
    fn from(value: DragValue<'a>) -> Self {
        Self::DragValue(value)
    }
}
