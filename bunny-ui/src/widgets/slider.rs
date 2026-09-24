use std::ops::RangeInclusive;

use abi_stable::std_types::{
    RCowStr,
    ROption::{self, RNone, RSome},
};

use crate::{HandleShape, Num, Widget, WidgetText};

#[derive(Clone, Copy)]
#[repr(C)]
struct SliderSpec {
    smallest_positive: f64,
    largest_finite: f64,
    logarithmic: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

#[cfg(feature = "manager")]
impl From<SliderOrientation> for egui::SliderOrientation {
    #[inline]
    fn from(value: SliderOrientation) -> Self {
        match value {
            SliderOrientation::Horizontal => Self::Horizontal,
            SliderOrientation::Vertical => Self::Vertical,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub enum SliderClamping {
    Never,
    Edits,
    #[default]
    Always,
}

#[cfg(feature = "manager")]
impl From<SliderClamping> for egui::SliderClamping {
    #[inline]
    fn from(value: SliderClamping) -> Self {
        match value {
            SliderClamping::Never => Self::Never,
            SliderClamping::Edits => Self::Edits,
            SliderClamping::Always => Self::Always,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum NumberCustomFormat {
    Binary {
        min_width: usize,
        twos_complement: bool,
    },
    Octal {
        min_width: usize,
        twos_complement: bool,
    },
    Hexadecimal {
        min_width: usize,
        twos_complement: bool,
        upper: bool,
    },
}

#[repr(C)]
pub struct Slider<'a> {
    text: ROption<WidgetText<'a>>,
    prefix: ROption<RCowStr<'a>>,
    suffix: ROption<RCowStr<'a>>,
    custom_format: ROption<NumberCustomFormat>,
    spec: SliderSpec,
    range: [f64; 2],
    drag_value_speed: ROption<f64>,
    max_decimals: ROption<usize>,
    step: ROption<f64>,
    value: Num<'a>,
    handle_shape: ROption<HandleShape>,
    min_decimals: usize,
    clamping: SliderClamping,
    orientation: SliderOrientation,
    trailing_fill: ROption<bool>,
    smart_aim: bool,
    show_value: bool,
    update_while_editing: bool,
}

impl<'a> Slider<'a> {
    pub fn new(value: impl Into<Num<'a>>, range: RangeInclusive<f64>) -> Self {
        let v = value.into();
        let int = v.integer();
        let slf = Self {
            value: v,
            range: [*range.start(), *range.end()],
            spec: SliderSpec {
                logarithmic: false,
                smallest_positive: 1e-6,
                largest_finite: f64::INFINITY,
            },
            clamping: Default::default(),
            smart_aim: true,
            show_value: true,
            orientation: SliderOrientation::Horizontal,
            prefix: RNone,
            suffix: RNone,
            text: RNone,
            step: RNone,
            drag_value_speed: RNone,
            min_decimals: 0,
            max_decimals: RNone,
            trailing_fill: RNone,
            handle_shape: RNone,
            update_while_editing: true,
            custom_format: RNone,
        };
        if int { slf.integer() } else { slf }
    }

    #[inline]
    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    #[inline]
    pub fn prefix(mut self, prefix: impl Into<RCowStr<'a>>) -> Self {
        self.prefix = RSome(prefix.into());
        self
    }

    #[inline]
    pub fn suffix(mut self, suffix: impl Into<RCowStr<'a>>) -> Self {
        self.suffix = RSome(suffix.into());
        self
    }

    #[inline]
    pub fn text(mut self, text: impl Into<WidgetText<'a>>) -> Self {
        self.text = RSome(text.into());
        self
    }

    #[inline]
    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    #[inline]
    pub fn vertical(mut self) -> Self {
        self.orientation = SliderOrientation::Vertical;
        self
    }

    #[inline]
    pub fn logarithmic(mut self, logarithmic: bool) -> Self {
        self.spec.logarithmic = logarithmic;
        self
    }

    #[inline]
    pub fn smallest_positive(mut self, smallest_positive: f64) -> Self {
        self.spec.smallest_positive = smallest_positive;
        self
    }

    #[inline]
    pub fn largest_finite(mut self, largest_finite: f64) -> Self {
        self.spec.largest_finite = largest_finite;
        self
    }

    #[inline]
    pub fn clamping(mut self, clamping: SliderClamping) -> Self {
        self.clamping = clamping;
        self
    }

    #[inline]
    pub fn smart_aim(mut self, smart_aim: bool) -> Self {
        self.smart_aim = smart_aim;
        self
    }

    #[inline]
    pub fn step_by(mut self, step: f64) -> Self {
        self.step = RSome(step);
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
    pub fn max_decimals_opt(mut self, max_decimals: Option<usize>) -> Self {
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
    pub fn trailing_fill(mut self, trailing_fill: bool) -> Self {
        self.trailing_fill = RSome(trailing_fill);
        self
    }

    #[inline]
    pub fn handle_shape(mut self, handle_shape: HandleShape) -> Self {
        self.handle_shape = RSome(handle_shape);
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
    pub fn integer(self) -> Self {
        self.fixed_decimals(0).smallest_positive(1.0).step_by(1.0)
    }

    #[inline]
    pub fn update_while_editing(mut self, update: bool) -> Self {
        self.update_while_editing = update;
        self
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for Slider<'_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut slider =
            egui::Slider::from_get_set(self.range[0]..=self.range[1], |v: Option<f64>| {
                if let Some(v) = v {
                    self.value.set(v);
                }
                self.value.to_f64()
            })
            .show_value(self.show_value)
            .orientation(self.orientation.into())
            .logarithmic(self.spec.logarithmic)
            .smallest_positive(self.spec.smallest_positive)
            .largest_finite(self.spec.largest_finite)
            .clamping(self.clamping.into())
            .smart_aim(self.smart_aim)
            .min_decimals(self.min_decimals)
            .update_while_editing(self.update_while_editing);

        if let RSome(prefix) = self.prefix {
            slider = slider.prefix(prefix);
        }
        if let RSome(suffix) = self.suffix {
            slider = slider.suffix(suffix);
        }
        if let RSome(text) = self.text {
            slider = slider.text(text);
        }
        if let RSome(step) = self.step {
            slider = slider.step_by(step);
        }
        if let RSome(drag_value_speed) = self.drag_value_speed {
            slider = slider.drag_value_speed(drag_value_speed);
        }
        if let RSome(max_decimals) = self.max_decimals {
            slider = slider.max_decimals(max_decimals);
        }
        if let RSome(trailing_fill) = self.trailing_fill {
            slider = slider.trailing_fill(trailing_fill);
        }
        if let RSome(handle_shape) = self.handle_shape {
            slider = slider.handle_shape(handle_shape.into());
        }
        if let RSome(custom_format) = self.custom_format {
            slider = match custom_format {
                NumberCustomFormat::Binary {
                    min_width,
                    twos_complement,
                } => slider.binary(min_width, twos_complement),
                NumberCustomFormat::Octal {
                    min_width,
                    twos_complement,
                } => slider.octal(min_width, twos_complement),
                NumberCustomFormat::Hexadecimal {
                    min_width,
                    twos_complement,
                    upper,
                } => slider.hexadecimal(min_width, twos_complement, upper),
            }
        }

        ui.add(slider)
    }
}

impl<'a> From<Slider<'a>> for Widget<'a> {
    #[inline]
    fn from(value: Slider<'a>) -> Self {
        Self::Slider(value)
    }
}
